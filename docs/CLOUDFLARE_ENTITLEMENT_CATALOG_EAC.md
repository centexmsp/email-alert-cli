---
title: "Cloudflare entitlement catalog — email-alert-cli"
wiki_slug: docs/cloudflare-entitlement-catalog-eac
doc_type: reference
type: page
---

# Cloudflare entitlement catalog — `email-alert-cli`

**Date:** 2026-07-30  
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
| SC-HMAC-EA wire | **use** | Pin hub `email-alert-hmac-v1` git rev — never fork crypto |
| 1Password `op://` | **use** | Session gate per hub ONEPASSWORD_CLI_SESSION_GATE |
| credential-manager | **use (optional)** | Vault materialize paths |

## Observability

| Control | Status |
|---------|--------|
| CLI structured errors | **use** — no secret bodies in logs |
| Correlation with Worker traces | **defer** — pass `X-Correlation-Id` when hub documents header (align with SC-CFOT) |

## Reuse

| Pattern | Decision |
|---------|----------|
| Pin this CLI | Submodule/git dep for producers |
| Worker source | Stay on hub — do not copy email-alerts |
| HMAC crate | Hub only — bump `rev` when SC-HMAC-EA changes |

## Gaps / deferred

| ID | Item |
|----|------|
| EAC-PIN-1 | Periodic bump of `email-alert-hmac-v1` rev after hub HMAC changes |
| EAC-CORR-1 | Optional correlation_id flag on CLI for Worker trace join |

## Local gates

```bash
./scripts/verify-local.sh
```
