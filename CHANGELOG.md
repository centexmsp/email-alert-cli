# Changelog

## [Unreleased]

### 2026-10-28 — Fleet handoff + observability honesty (P5)

- Seed `wiki/HANDOFF.md`, `wiki/ROADMAP.md`, `.cursor/BUGBOT.md`, `.github/pull_request_template.md`, `.cursor/rules/work-order.mdc`.
- HMAC docs now match the dedicated SC-HMAC-EA pin (`centexmsp/email-alert-hmac-v1` @ `Cargo.toml` `rev`). Pin unchanged.
- Caller `correlation_id` contract fixtures; EAC-CORR-1 marked shipped. No `cf-observability-tracing` dep.
- Hub #387 shell-hook rewire: N/A (no `.cursor/hooks.json`).
- Local gate: `./scripts/verify-local.sh`.

### 2026-07-30 — Cloudflare entitlement (fleet Repo 8)

- `docs/CLOUDFLARE_ENTITLEMENT_CATALOG_EAC.md` — no Workers here; SC-HMAC-EA producer role; hub email-alerts owns edge obs.
- `docs/SSOT_POINTER.md` — CONSUMER item **16** + SC-HMAC-EA index.
- Local gate: `./scripts/verify-local.sh` **PASS** (HMAC vector ok).
- HMAC pin: dedicated `email-alert-hmac-v1` (see `Cargo.toml` `rev`).
