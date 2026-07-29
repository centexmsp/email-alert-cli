# AGENTS.md — consumer contract

**Repo role:** Implementation consumer of the Centex program SSOT.  
**Canonical standards / enforcement / status:**  
https://github.com/centexmsp/repository-restructuring

Do **not** fork program-wide status or token policy here. Extend local `docs/` only for this codebase.

## Cold start

1. Paste only:  
   https://github.com/centexmsp/repository-restructuring/blob/main/docs/NEW_CHAT_SINGLE_FILE_HANDOFF_SSOT.md  
2. `git pull` on this repo + SSOT `main` when standards move.  
3. Install/refresh Cursor pack from SSOT:  
   `repository-restructuring/scripts/install-cursor-program-pack.sh "$(pwd)"`

## Non-negotiables (inherit SSOT)

1. **Secrets:** 1Password / `credential-manager` / platform store — never commit bearers.  
2. **Cloudflare mutations:** guard preflight + portfolio tokens — SSOT `TOKEN_*` + playbook.  
3. **Rust-first** for new Workers and program automation (ADR if exception).  
4. **PR-first** into default branch; no parallel machine integration branches.  
5. **No second master status canvas** — link SSOT `COMPREHENSIVE_PROJECT_STATUS_REPORT.md`.  
6. **Vendor docs first** for Cloudflare and xAI/Grok (SSOT + MCP).  
7. **Deception / neutral public naming** when this repo has public security surfaces.

## Local only (edit this section)

- **Default branch:** `main` (or document exception).  
- **Primary languages / packages:** _fill in_  
- **Key services / paths:** _fill in_  
- **Local verify commands:** _fill in_  
- **Deploy entrypoints:** _fill in_  
- **HITL blockers known here:** _fill in_

## Cursor + Grok

- Rules: `.cursor/rules/` from program pack (do not duplicate SSOT prose).  
- Models: `.cursor/ai-gateway.json` (Cloudflare AI Gateway `/compat`). Prefer program default (**grok-4.5** when pack is current).  
- Optional subagents: copy from SSOT `.cursor/agents/` only when needed.  
- Loops: durable state in git/SSOT Continue queue — not chat history.

## Git + docs (every merge-ready slice)

1. Commit with machine-lane trailer (SSOT `git-commit-with-machine-lane.sh` or consumer equivalent).  
2. Push (or open PR when branch protection requires).  
3. Update local docs + link SSOT COMPREHENSIVE/CHANGELOG when status moves.  
4. Never leave merge-ready work uncommitted.

## Upstream links

- https://github.com/centexmsp/repository-restructuring/blob/main/AGENTS.md  
- https://github.com/centexmsp/repository-restructuring/blob/main/docs/00_SSOT.md  
- https://github.com/centexmsp/repository-restructuring/blob/main/docs/STANDARDS_SSOT.md  
- https://github.com/centexmsp/repository-restructuring/blob/main/docs/CURSOR_GROK_AGENT_HIERARCHY_AND_LOOP_OPTIMIZATION_SSOT.md  
- https://github.com/centexmsp/repository-restructuring/blob/main/config/cursor-program/README.md  

## Program hub (dual-host / Grok) — do not remove

- **ONE command (do not type multi-step lists):** from hub run `./scripts/dual-host-bootstrap.sh --agent` (Build: `--build` · orch: `--orch`) then read hub `state/dual-host/agent-context.md`
- **Automation SSOT:** https://github.com/centexmsp/repository-restructuring/blob/main/docs/DUAL_HOST_AGENT_AUTOMATION_SSOT.md

- **Standards hub:** https://github.com/centexmsp/repository-restructuring
- **M5 always-read:** https://github.com/centexmsp/repository-restructuring/blob/main/docs/M5_READ_ALWAYS.md
- **Horeb always-read:** https://github.com/centexmsp/repository-restructuring/blob/main/docs/HOREB_READ_ALWAYS.md
- **Dual-host sync system:** https://github.com/centexmsp/repository-restructuring/blob/main/docs/DUAL_HOST_SYNC_SYSTEM_SSOT.md
- **Consumer contract:** https://github.com/centexmsp/repository-restructuring/blob/main/docs/CONSUMER_SSOT_BACKLINK_MINIMUM.md
- **Cursor pack:** from hub run `./scripts/install-cursor-program-pack.sh` on this repo
- **Models:** `grok/grok-4.5` (default) · `grok/grok-build-0.1` (long Build) · not Auto
- **Git law:** claim branch → push → PR → both machines pull default branch after merge
