#!/usr/bin/env bash
# Billing-safe local gates for email-alert-cli (no GitHub Actions minutes).
# Usage: ./scripts/verify-local.sh [--quick] [--audit]
#   --quick  fmt only
#   --audit  also run cargo audit (network)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:${PATH:-}"

QUICK=0
AUDIT=0
for a in "$@"; do
  case "$a" in
    --quick) QUICK=1 ;;
    --audit) AUDIT=1 ;;
    -h|--help) sed -n '1,8p' "$0"; exit 0 ;;
  esac
done

echo "=== email-alert-cli local verify (quick=$QUICK audit=$AUDIT) ==="
cargo fmt --all -- --check
if [[ "$QUICK" -eq 0 ]]; then
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-features
fi
if [[ "$AUDIT" -eq 1 ]]; then
  if command -v cargo-audit >/dev/null 2>&1 || cargo audit -h >/dev/null 2>&1; then
    cargo audit
  else
    echo "WARN: cargo-audit not installed — skip (cargo install cargo-audit)"
  fi
fi
echo "PASS email-alert-cli local verify"
