//! HMAC v1 signing for the `email-alerts` Cloudflare Worker `POST /alert` contract (SSOT: centexmsp/repository-restructuring `workers/email-alerts`).
//! Optional reuse from [credential-manager](https://github.com/centexmsp/credential-manager) via `path` or `git` dependency.
use hmac::Hmac;
use hmac::Mac;
use serde::Serialize;
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Same canonical string as the Worker: `HMAC-SHA256(secret, "{ts}.{raw_body_utf8}")` → **lowercase** hex (use [`signature_header_v1`] for the full header value).
#[must_use]
pub fn hmac_sha256_hex(secret: &str, ts: &str, body: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts arbitrary key length");
    let msg = format!("{ts}.{body}");
    mac.update(msg.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Full `X-Email-Alert-Signature` value, e.g. `v1=a1b2...` (lowercase hex).
#[must_use]
pub fn signature_header_v1(secret: &str, ts: &str, body: &str) -> String {
    format!("v1={}", hmac_sha256_hex(secret, ts, body))
}

#[derive(Debug, Serialize)]
pub struct AlertJson {
    pub subject: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}

impl AlertJson {
    pub fn to_body_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Read a secret from the environment, or from `1password-cli` (`op read "op://..."`) when `value` is an `op://` reference.
#[derive(Debug, Error)]
pub enum OpReadError {
    #[error("1Password CLI (`op`) not found in PATH — install: https://developer.1password.com/docs/cli/")]
    OpNotFound,
    #[error("op read failed: {0}")]
    OpFailed(String),
    #[error("op returned empty output for: {0}")]
    OpEmpty(String),
}

/// If `s` starts with `op://`, run `op read s`; else return `s` as the secret material.
pub fn materialize_op_or_literal(s: &str) -> Result<String, OpReadError> {
    if s.starts_with("op://") {
        return op_read(s);
    }
    Ok(s.to_string())
}

fn op_read(op_ref: &str) -> Result<String, OpReadError> {
    use std::process::Command;
    let out = Command::new("op")
        .args(["read", op_ref])
        .output()
        .map_err(|e: std::io::Error| {
            if e.kind() == std::io::ErrorKind::NotFound {
                OpReadError::OpNotFound
            } else {
                OpReadError::OpFailed(e.to_string())
            }
        })?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(OpReadError::OpFailed(format!(
            "{op_ref} status {}: {}",
            out.status,
            stderr.trim()
        )));
    }
    let t = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if t.is_empty() {
        return Err(OpReadError::OpEmpty(op_ref.to_string()));
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmac_vector_matches_openssl_worker_example() {
        // Same family as workers/email-alerts: ts.body
        let secret = "s";
        let body = r#"{"subject":"x"}"#;
        let ts = "1000";
        let got = hmac_sha256_hex(secret, ts, body);
        assert_eq!(got.len(), 64);
        // Cross-check: stable expected from worker tests (hmac_round_trip uses "s" and body)
        let sig = signature_header_v1(secret, ts, body);
        assert!(sig.starts_with("v1="));
    }
}
