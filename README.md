# email-alert-cli

Standalone **Rust** CLI and library: HMAC-signed `POST` to the [`email-alerts`](https://github.com/centexmsp/repository-restructuring/tree/main/workers/email-alerts) Cloudflare Worker (`POST /alert`), with optional **`op://`** resolution via [1Password CLI](https://developer.1password.com/docs/cli/).

**Repository**: source of truth for this crate. Integrate consumers via **pinned git submodule** or **git dependency** per [SHARED_MODULE_INTEGRATION_STANDARD_SSOT](https://github.com/centexmsp/repository-restructuring/blob/main/docs/SHARED_MODULE_INTEGRATION_STANDARD_SSOT.md).

## Program SSOT

- Program index: <https://github.com/centexmsp/repository-restructuring/blob/main/docs/00_SSOT.md>
- SSOT quick pointer (this repo): `docs/SSOT_POINTER.md`

## Install

```bash
cargo install --git https://github.com/centexmsp/email-alert-cli.git --locked
# or from clone:
cargo install --path . --locked
```

## Usage

```bash
export EMAIL_ALERT_HMAC_OP='op://developer/YourItem/credential'
export EMAIL_ALERT_URL_OP='op://developer/YourItem/url'
email-alert -s "Subject" -t "Body"
```

Use the **developer** vault in 1Password for dev refs (Cursor: Command Palette → **1Password: Choose vault** → `developer`). See consumer SSOT: [EMAIL_ALERT_DEV_ENV_SSOT.md](https://github.com/centexmsp/repository-restructuring/blob/main/docs/EMAIL_ALERT_DEV_ENV_SSOT.md).

## Crate layout

- **Library**: `email_alert` — HMAC helpers + `materialize_op_or_literal`
- **Binary**: `email-alert`

## Related

- [credential-manager](https://github.com/centexmsp/credential-manager) — optional shared `op` / provisioning patterns
- Consumer: [repository-restructuring](https://github.com/centexmsp/repository-restructuring) (submodule `email-alert-cli/`)

## License

MIT — see `LICENSE-MIT`.
