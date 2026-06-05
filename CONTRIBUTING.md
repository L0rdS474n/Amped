# Contributing to Amped

Thanks for your interest in Amped! This document explains how we work so your contribution
lands smoothly. The guiding principle: **build it correctly, not quickly** — security and
honesty about what works beat velocity.

## Ground rules

- **Never push to `main`.** `main` is protected. All changes land via pull request.
- **One objective per PR.** Keep changes reviewable in one sitting. Split unrelated work.
- **Test-first.** Write or update tests *before* the implementation. "Compiles" and "tests
  pass" are necessary but never sufficient — behavior must be demonstrated.
- **No secrets, ever.** Never commit tokens, passwords, refresh tokens, real credentials,
  or personal data — not in code, tests, fixtures, manifests, or logs.
- **No ambient authority.** Plugin code gets capabilities only through the host-mediated
  contract. Don't add direct network/filesystem/secret access to a plugin.

## Development setup

```sh
git clone git@github.com:L0rdS474n/Amped.git
cd Amped
cargo build --workspace
cargo test  --workspace
```

A recent stable Rust toolchain is required (Rust 2021 edition). The WASM component
toolchain (`cargo-component`, `wasm-tools`) is optional until milestone M2.

## Workflow

1. **Branch** off `main` with a descriptive name:
   - `feat/<short-description>` — new functionality
   - `fix/<short-description>` — bug fixes
   - `docs/<short-description>` — documentation
   - `chore/<short-description>` — tooling, CI, refactors
2. **Make the change**, test-first, scoped to one objective.
3. **Verify locally** before pushing:
   ```sh
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. **Open a pull request** against `main`. Fill in the PR template.
5. **CI must be green** and **at least one approving review** is required before merge
   (repository maintainers may merge their own changes without review for now).
6. On merge the branch is **squash-merged** and **automatically deleted**. History on
   `main` stays linear.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):
`feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`. Keep the subject imperative and
under ~72 characters; explain the *why* in the body.

## Architecture & boundaries

Amped enforces a one-way dependency graph (see [README](README.md#architecture-at-a-glance)).
`amped-plugin-api` is the dependency-free contract leaf; `amped-core` must never depend on
Tauri; plugins depend only on `amped-plugin-api`. Changes that cross a module boundary,
add a dependency, or alter the plugin contract require an **ADR** under
[`docs/adr/`](docs/adr/) — copy the format of `0001-*.md`.

## Definition of Done

A change is done when:

- [ ] Acceptance criteria are met and demonstrated by tests (not just "it compiles").
- [ ] `cargo test --workspace` is green; new behavior has positive **and** negative tests.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- [ ] `cargo fmt --all -- --check` is clean.
- [ ] No `unsafe` without an ADR justifying it.
- [ ] No secrets or personal data anywhere in the diff.
- [ ] Contract/boundary changes are recorded in an ADR.
- [ ] The PR has a single, clearly described objective.

## Code of Conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). Be respectful.

## License

By contributing you agree that your contributions are dual licensed under
**MIT OR Apache-2.0**, matching the project license.
