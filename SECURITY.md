# Security Policy

## Supported versions

Amped is pre-1.0 and under active development. Security fixes are applied to the `main`
branch. There are no released versions yet; once releases begin, the most recent minor
series will be supported.

## Reporting a vulnerability

**Please do not report security issues in public GitHub issues or pull requests.**

Use GitHub's **private vulnerability reporting** for this repository
(*Security → Report a vulnerability*), or contact the maintainer
[@L0rdS474n](https://github.com/L0rdS474n) privately. When reporting, please include:

- a description of the issue and its impact,
- steps to reproduce or a proof of concept,
- affected component(s) and version/commit.

**Never include real credentials, tokens, or personal data in a report.** Redact secrets
before sending; a description of the class of secret is sufficient.

You can expect an acknowledgement within a few days and a coordinated disclosure timeline
once the issue is confirmed.

## Security model (what Amped promises)

Amped is built so that **plugins are untrusted by default**:

- Plugins run as sandboxed WebAssembly components with **no ambient authority** — no direct
  network, filesystem, or secret access. The host mediates every capability and enforces a
  per-plugin allow-list declared in the plugin manifest.
- The **host owns all credentials.** Secrets are stored in the OS keychain; the host
  performs authentication and injects `Authorization`. A plugin never receives a raw
  long-lived token.
- **No secret logging.** Tokens, passwords, and personal data are redacted host-side.

If you find a way to bypass any of these properties, that is a vulnerability — please
report it as above.
