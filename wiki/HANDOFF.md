---
title: "email-alert-cli handoff"
wiki_slug: wiki/handoff
doc_type: handoff
type: page
---

<!-- Consumer template from centexmsp/repository-restructuring config/cursor-program/templates/HANDOFF.wiki.md. -->

# email-alert-cli handoff

## Status

- Default branch `main` at `31e3228`.
- Open PRs: this slice (`docs/eac-handoff-obs-hmac`) — fleet wiki templates, HMAC dedicated-repo docs, caller `correlation_id` contract fixtures. Never merge; in81 reviews.
- Blockers: none for this PR. Live HMAC smoke still needs vault fields. Hub `workers/email-alerts` JSON ingress log is on hub origin; live Worker deploy is a separate HITL.
- Hub [#387](https://github.com/centexmsp/repository-restructuring/pull/387) shell-hook rewire is **not** needed here: this repo has no `.cursor/hooks.json` and does not vendor `devops-command-gate-rs`. `.cursor/hooks/.gitignore` is already on `main` (#30).

## Work order

Fields per [STANDARDS_SSOT.md §13](https://github.com/centexmsp/repository-restructuring/blob/main/docs/STANDARDS_SSOT.md#13-work-ordering-standard-mandatory--2026-10-21).

- **Priority class:** P5 (consumer fleet-sync / docs hygiene batch)
- **Queue position:** hub [`wiki/HANDOFF.md`](https://github.com/centexmsp/repository-restructuring/blob/main/wiki/HANDOFF.md) consumer template wave; this repo `wiki/ROADMAP.md` step 0
- **Blocked-on:** none

## Next steps

1. Merge this PR (in81). HMAC pin stays dedicated [`email-alert-hmac-v1`](https://github.com/centexmsp/email-alert-hmac-v1) @ `Cargo.toml` `rev`.
2. Later: ROADMAP step 1 (optional local-gates `check.sh`) only if `verify-local.sh` is not enough.

## How to resume in Cursor

1. Check free space first: `df --output=pcent /` (stop if under 20% free — [DISK_BUDGET_STANDARD.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/DISK_BUDGET_STANDARD.md)).
2. Open this repo in Cursor (or use `gh` / a sparse blobless clone for small edits).
3. Read `AGENTS.md`, then this file, then `wiki/ROADMAP.md`.
4. Program standards: [centexmsp/repository-restructuring](https://github.com/centexmsp/repository-restructuring) `docs/STANDARDS_SSOT.md`.

## Last updated

2026-10-28 by Cursor (M5) (`docs/eac-handoff-obs-hmac`).
