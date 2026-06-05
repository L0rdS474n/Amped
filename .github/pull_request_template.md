<!-- Thanks for contributing to Amped! Keep PRs single-objective and test-first. -->

## What & why

<!-- What does this change do, and why? Link any related issue: Closes #123 -->

## Milestone / scope

<!-- e.g. M2 — Wasmtime host. Confirm this PR has ONE objective. -->

## How to verify

<!-- Exact commands a reviewer runs, and what they should see. -->

```sh
cargo test --workspace
```

## Checklist

- [ ] One clear objective; unrelated changes split out
- [ ] Tests added/updated **before** implementation (test-first); positive **and** negative cases
- [ ] `cargo test --workspace` is green (output pasted or summarized below)
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [ ] `cargo fmt --all -- --check` is clean
- [ ] No `unsafe` (or it is justified in an ADR)
- [ ] No secrets, tokens, or personal data anywhere in the diff
- [ ] Contract/boundary/dependency changes are recorded in an ADR under `docs/adr/`
- [ ] Docs/README updated if behavior or usage changed

## Evidence

<!-- Paste the relevant test / clippy output. "Tests pass" alone is not enough. -->
