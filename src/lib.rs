//! HMAC v1 signing for the `email-alerts` Cloudflare Worker `POST /alert` contract.
//! Crypto lives in hub **`email-alert-hmac-v1`** (G8); this crate re-exports the producer API
//! and adds `op://` materialization + `AlertJson`.
//!
//! SSOT: centexmsp/repository-restructuring `workers/email-alerts` + `crates/email-alert-hmac-v1`.

use serde::Serialize;
use thiserror::Error;

pub use email_alert_hmac_v1::{
    email_alert_hmac_verify, email_alert_sign_hex, email_alert_signature_header_value,
    email_alert_ts_within_skew, hmac_sha256_hex, hmac_sha256_hex_message, signature_header_v1,
    strip_v1_email_alert_sig,
};

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
        let secret = "s";
        let body = r#"{"subject":"x"}"#;
        let ts = "1000";
        let got = hmac_sha256_hex(secret, ts, body);
        assert_eq!(got.len(), 64);
        let sig = signature_header_v1(secret, ts, body);
        assert!(sig.starts_with("v1="));
        assert!(email_alert_hmac_verify(secret, ts, body, &sig));
    }
}
