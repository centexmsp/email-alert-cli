# Bugbot review rules

<!-- Consumer template from centexmsp/repository-restructuring config/cursor-program/templates/BUGBOT.consumer.md. -->

Review against the centexmsp hub standards ([STANDARDS_SSOT.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/STANDARDS_SSOT.md)). Flag each gap as a blocking finding with file and line.

## Must hold

1. **Test-first Rust.** New or changed Rust logic ships with tests in the same PR. Tests should fail without the change.
2. **Line coverage 90%.** `cargo llvm-cov --fail-under-lines 90` passes at the crate / repo scope being changed; auth / secret / crypto code holds **95%**. Flag lowered thresholds or new ignore regexes without a documented reason.
3. **Contract coverage 100%.** Every external API response shape the code parses has a fixture test (small JSON under the repo's fixtures dir): success, error, and empty / missing-field cases. A new parsed field without a fixture is a finding.
4. **Consumer backlink minimum.** The 21 rules in [CONSUMER_SSOT_BACKLINK_MINIMUM.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/CONSUMER_SSOT_BACKLINK_MINIMUM.md) still hold (thin root `AGENTS.md`, README "Program SSOT" link, PR-first, no committed hook telemetry, ...).
5. **No secrets.** No tokens, keys, passwords, `op://` values resolved to plaintext, or `.env` files. Secrets come from 1Password / env only.
6. **Disk budget.** No build output, coverage HTML, `*.profraw` / `*.profdata`, or large binaries in git ([DISK_BUDGET_STANDARD.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/DISK_BUDGET_STANDARD.md)).
7. **Work order.** The PR body states **Priority class** (P1–P5), **Queue item** (hub `wiki/QUEUE.md` row or this repo's `wiki/ROADMAP.md` step), and **Blocked-on** ([STANDARDS_SSOT.md §13](https://github.com/centexmsp/repository-restructuring/blob/main/docs/STANDARDS_SSOT.md#13-work-ordering-standard-mandatory--2026-10-21)). P5 hygiene / docs changes are batched into one PR per repo: a P5 PR says it is the repo's hygiene batch; flag unrelated hygiene mixed into a P1–P4 PR.
8. **Merge evidence.** Code PRs paste local gate commands with exit 0 per [LOCAL_PR_MERGE_EVIDENCE_SSOT.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/LOCAL_PR_MERGE_EVIDENCE_SSOT.md) (LOCAL-MERGE-PR-BODY-1): fmt, clippy `-D warnings`, tests, coverage, cargo-deny, cargo-audit, semgrep, secret scan. A missing or skipped gate without a stated reason is a finding; "saves usage" is not a reason.

## Not a finding

- **Frozen Dependabot / GitHub Actions** (disabled or paused) is expected. Do not ask to re-enable them.
- Docs-only PRs with no Rust change do not need coverage output (they still need check 7).
- Missing `.cursor/hooks.json` / unvendored `devops-command-gate-rs` is expected. Pack README: consumers do not copy hub hooks unless they vendor the gate crate. Hub [#387](https://github.com/centexmsp/repository-restructuring/pull/387) does not require a rewire here.

## Style

Short findings. Quote the line, say what rule it breaks, suggest the fix. No Autofix.

## Repo-local rules

- **SC-HMAC-EA pin:** `Cargo.toml` must keep `git = "https://github.com/centexmsp/email-alert-hmac-v1"` with a full `rev`. Do not retarget the hub `crates/email-alert-hmac-v1` path. Do not reimplement `v1=` signing.
- **Observability:** `correlation_id` is caller-supplied JSON (`--correlation-id` / `ALERT_CORRELATION_ID`). Do not add a `cf-observability-tracing` dependency (ADR-0057). Do not log HMAC material or mail bodies.
