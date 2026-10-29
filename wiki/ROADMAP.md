---
title: "email-alert-cli roadmap"
wiki_slug: wiki/roadmap
doc_type: roadmap
type: page
---

<!-- Consumer template from centexmsp/repository-restructuring config/cursor-program/templates/ROADMAP.wiki.md. Ordering rules: hub STANDARDS_SSOT.md §13 — do not copy them here. -->

# email-alert-cli roadmap — small sequential PRs

Take the first step that is not DONE and whose deps are DONE. Ordering across repos: hub [`wiki/QUEUE.md`](https://github.com/centexmsp/repository-restructuring/blob/main/wiki/QUEUE.md) · rules: [STANDARDS_SSOT.md §13](https://github.com/centexmsp/repository-restructuring/blob/main/docs/STANDARDS_SSOT.md#13-work-ordering-standard-mandatory--2026-10-21). Resume card: `wiki/HANDOFF.md`.

Each PR: branch per step, test-first, local gates green with evidence in the PR body ([LOCAL_PR_MERGE_EVIDENCE_SSOT.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/LOCAL_PR_MERGE_EVIDENCE_SSOT.md)), mark the step DONE here in the same PR.

**Status 2026-10-28 CT:** step 0 in this PR. Hub #387 hook rewire does not apply (no `hooks.json`). HMAC stays SC-HMAC-EA dedicated-repo pin. Next after merge: step 1 only if local-gates templates are wanted.

Horizons for step 0: **long** — this CLI stays a thin HMAC producer for hub `email-alerts`, pinning dedicated `email-alert-hmac-v1`, with caller `correlation_id` joining Worker observability. **medium** — durable `wiki/HANDOFF.md` + `wiki/ROADMAP.md` plus docs that match the dedicated pin and the JSON join key. **short** — this slice.

| # | Step | Deps | Priority class | Queue position | Blocked-on | Status |
|---|---|---|---|---|---|---|
| 0 | fleet-handoff-obs-hmac | — | P5 | hub HANDOFF consumer template wave | none | DONE (this PR) |
| 1 | optional-local-gates | 0 | P5 | — | none | TODO |
| 2 | hmac-pin-bump | 0 | P4 | EAC-PIN-1 | hub SC-HMAC-EA rev moves | TODO (no bump this PR) |

## 0 fleet-handoff-obs-hmac

Seed consumer `wiki/HANDOFF.md`, `wiki/ROADMAP.md`, `.cursor/BUGBOT.md`, `.github/pull_request_template.md`, and `work-order.mdc`. Point HMAC docs at dedicated [`centexmsp/email-alert-hmac-v1`](https://github.com/centexmsp/email-alert-hmac-v1) (do not retarget the hub crate). Record that `--correlation-id` / `ALERT_CORRELATION_ID` already ships as JSON `correlation_id` (Worker join key; no `cf-observability-tracing` dep; ADR-0057). Fixture tests for with/without id. Do not add `.cursor/hooks.json`.
**Done when:** `test -f wiki/HANDOFF.md && test -f wiki/ROADMAP.md && test -f .cursor/BUGBOT.md && grep -q 'centexmsp/email-alert-hmac-v1' Cargo.toml AGENTS.md docs/SSOT_POINTER.md && ./scripts/verify-local.sh`
**DONE 2026-10-28** (this PR).

## 1 optional-local-gates

Copy hub `templates/local-gates/` (`check.sh`, `check-heavy.sh`, fail-closed `pre-push`) only if `verify-local.sh` is not enough. Fail-closed tools must exist on the host first.
**Done when:** `test -f scripts/check.sh` and `./scripts/check.sh` exits 0, or this step stays TODO with the reason recorded here.

## 2 hmac-pin-bump

Bump `Cargo.toml` `rev` of `email-alert-hmac-v1` after a dedicated-repo SC-HMAC-EA change. Full 40-character SHA. Do not switch the git URL to the hub crate path.
**Done when:** `Cargo.toml` + `Cargo.lock` `rev` match the dedicated repo tip used, `./scripts/verify-local.sh` exits 0.
