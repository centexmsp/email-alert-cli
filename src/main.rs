//! `email-alert` — send a signed `POST` to the Cloudflare `email-alerts` Worker from a **dev machine** (Cursor, terminal).
//! Consumer SSOT: <https://github.com/centexmsp/repository-restructuring/blob/main/docs/EMAIL_ALERT_DEV_ENV_SSOT.md>
use clap::Parser;
use email_alert::{materialize_op_or_literal, signature_header_v1, AlertJson, OpReadError};

#[derive(Parser, Debug)]
#[command(name = "email-alert")]
#[command(about = "HMAC POST to email-alerts Worker (dev). Secrets: env, or op:// refs via 1Password CLI.", long_about = None)]
struct Opt {
    /// Email subject (JSON)
    #[arg(short = 's', long, env = "ALERT_SUBJECT")]
    subject: String,

    /// Plain text body
    #[arg(short, long, env = "ALERT_TEXT", default_value = "(empty)")]
    text: String,

    /// Optional correlation_id in JSON
    #[arg(long, env = "ALERT_CORRELATION_ID")]
    correlation_id: Option<String>,

    /// HMAC shared secret, or `op://vault/Item/field` (1Password CLI)
    #[arg(long, env = "HMAC_SECRET")]
    hmac_secret: Option<String>,

    /// Same as --hmac-secret; preferred for 1P refs: `op://...`
    #[arg(long, env = "EMAIL_ALERT_HMAC_OP")]
    hmac_op: Option<String>,

    /// Worker URL ending in `/alert`, or `op://` ref
    #[arg(long, env = "EMAIL_ALERT_WORKER_URL")]
    url: Option<String>,

    /// Same as --url; `op://` ref allowed
    #[arg(long, env = "EMAIL_ALERT_URL_OP")]
    url_op: Option<String>,

    /// If set, print request details but do not POST (redact HMAC)
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), String> {
    let opt = Opt::parse();
    let hmac_src = opt.hmac_op.or(opt.hmac_secret).ok_or_else(|| {
        "set --hmac-secret, --hmac-op, or HMAC_SECRET / EMAIL_ALERT_HMAC_OP".to_string()
    })?;
    let url_src = opt.url_op.or(opt.url).ok_or_else(|| {
        "set --url, --url-op, or EMAIL_ALERT_WORKER_URL / EMAIL_ALERT_URL_OP".to_string()
    })?;

    let hmac = materialize_op_or_literal(&hmac_src).map_err(|e: OpReadError| e.to_string())?;
    let url = materialize_op_or_literal(&url_src).map_err(|e: OpReadError| e.to_string())?;
    if !url.contains("/alert") {
        eprintln!("email-alert: warning: URL should end with /alert (got: {url})");
    }

    let alert = AlertJson {
        subject: opt.subject,
        text: opt.text,
        correlation_id: opt.correlation_id,
    };
    let body = alert.to_body_string().map_err(|e| e.to_string())?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs()
        .to_string();
    let sig = signature_header_v1(&hmac, &ts, &body);
    if opt.dry_run {
        println!("{ts} would POST to {url} body_len={}", body.len());
        println!("X-Email-Alert-Signature: (redacted, len {})", sig.len());
        return Ok(());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("X-Email-Alert-Timestamp", &ts)
        .header("X-Email-Alert-Signature", &sig)
        .body(body)
        .send()
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let t = resp.text().map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("HTTP {status}: {t}"));
    }
    println!("{t}");
    Ok(())
}
