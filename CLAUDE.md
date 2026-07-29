@AGENTS.md

# Claude Code adapter (thin)

This repository’s **canonical** agent contract is **`AGENTS.md`** (vendor-neutral; used by Cursor + Grok and other tools).

Claude Code: treat **`AGENTS.md`** as source of truth. Do **not** grow this file into a second standards master.

**Dual-host (M5 + horeb):** from hub `repository-restructuring` run only:

```bash
./scripts/dual-host-bootstrap.sh --agent
# Build: --build · horeb orch: --orch
```

Then read hub `state/dual-host/agent-context.md`. Do **not** invent `GROK.md`.

Program SSOT: https://github.com/centexmsp/repository-restructuring  
Automation: https://github.com/centexmsp/repository-restructuring/blob/main/docs/DUAL_HOST_AGENT_AUTOMATION_SSOT.md
