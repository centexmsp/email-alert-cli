<!-- Consumer template from centexmsp/repository-restructuring config/cursor-program/templates/pull_request_template.md. Copy to <repo>/.github/pull_request_template.md (not a workflow; safe under the Actions freeze). -->

## Summary

<!-- What changed and why, in 1–3 lines. -->

## Work order

<!-- Required. Rules: docs/STANDARDS_SSOT.md §13 (ADR-0059). -->

- **Priority class:** <!-- P1–P5 (STANDARDS_SSOT.md §13.2) -->
- **Queue item:** <!-- hub wiki/QUEUE.md row, or this repo's wiki/ROADMAP.md step -->
- **Blocked-on:** <!-- none / in81 — what / other PR -->
- **Depends on:** <!-- PRs that must merge first, or none -->

## Local merge evidence (Actions freeze)

<!-- LOCAL-MERGE-PR-BODY-1: docs/LOCAL_PR_MERGE_EVIDENCE_SSOT.md §3. Paste commands with exit 0. Docs-only: say so and paste the docs verify. -->

- Host: <!-- mobile / box --> · branch: · date (CT):
- Commands (all exit 0):
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - `cargo llvm-cov --fail-under-lines 90` <!-- 95% for auth/secret code; contracts N/N -->
  - `cargo deny check` · `cargo audit`
  - `semgrep` (fleet config) · secret scan (`gitleaks`)
  - `<repo verify script>`
- Socket: <!-- clean / findings linked -->
- Freeze: Dependabot stays `updates: []`; no `gh workflow enable`.

## Review

- [ ] Bugbot ran (`.cursor/BUGBOT.md`); findings fixed or answered
- [ ] Security review done for auth / secret / network code (or N/A)
- [ ] `wiki/HANDOFF.md` / `wiki/ROADMAP.md` updated; CHANGELOG entry
- [ ] Branch + PR only; in81 merges
