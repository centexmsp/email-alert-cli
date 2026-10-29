---
title: "Cloudflare entitlement catalog — email-alert-cli"
wiki_slug: docs/cloudflare-entitlement-catalog-eac
doc_type: reference
type: page
---

# Cloudflare entitlement catalog — `email-alert-cli`

**Date:** 2026-07-30 (obs + HMAC pin docs 2026-10-28)  
**Policy:** [hub CLOUDFLARE_ENTITLEMENT_FULL_STACK_UTILIZATION_SSOT](https://github.com/centexmsp/repository-restructuring/blob/main/docs/CLOUDFLARE_ENTITLEMENT_FULL_STACK_UTILIZATION_SSOT.md)

## Purpose

Standalone **Rust HMAC producer** for hub Worker **`email-alerts`** (`POST /alert`). **No Worker** ships here — edge compute entitlement is realized on the **hub Worker** (already Logs+Traces + live redeploy 2026-07-30).

## Edge compute & data

| Capability | Status | Notes |
|------------|--------|-------|
| Workers | **N/A here** | Consumer of hub `workers/email-alerts` |
| Workers Logs+Traces | **use (hub)** | Hub P0 spine — not this CLI |
| Browser / CBI | **N/A** | |

## Security (primary)

| Capability | Status | Notes |
|------------|--------|-------|
| SC-HMAC-EA wire | **use** | Pin dedicated [`centexmsp/email-alert-hmac-v1`](https://github.com/centexmsp/email-alert-hmac-v1) git `rev` — never fork crypto |
| 1Password `op://` | **use** | Session gate per hub ONEPASSWORD_CLI_SESSION_GATE |
| credential-manager | **use (optional)** | Vault materialize paths |

## Observability

| Control | Status |
|---------|--------|
| CLI structured errors | **use** — no secret bodies in logs |
| Correlation with Worker traces | **use** — caller-supplied JSON `correlation_id` (`--correlation-id` / `ALERT_CORRELATION_ID`). Hub `workers/email-alerts` logs that field on ingress. Do not generate ids here; do not add `cf-observability-tracing` (ADR-0057). No `X-Correlation-Id` header — the Worker join key is the JSON field. |

## Reuse

| Pattern | Decision |
|---------|----------|
| Pin this CLI | Submodule/git dep for producers |
| Worker source | Stay on hub — do not copy email-alerts |
| HMAC crate | Dedicated `email-alert-hmac-v1` repo only — bump `rev` when SC-HMAC-EA changes |

## Gaps / deferred

| ID | Item |
|----|------|
| EAC-PIN-1 | Periodic bump of dedicated `email-alert-hmac-v1` `rev` after SC-HMAC-EA changes (ROADMAP step 2) |
| EAC-CORR-1 | **shipped** — `--correlation-id` / `ALERT_CORRELATION_ID` → JSON `correlation_id` (fixtures in `tests/fixtures/`) |

## Local gates

```bash
./scripts/verify-local.sh
```
