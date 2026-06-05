# ADR-0001: M0+M1 — Workspace scaffold, plugin contract, edition & sandbox decisions

Status: ACCEPTED

Date: 2026-06-05

Deciders: Amped maintainer (user) + pipeline gates (planner, security, architecture, plugin-dev)

Scope: M0 (workspace scaffold) + M1 (WIT contract + manifest/registry parsing) ONLY.

## (a) Bootstrap exception — test-first waiver, skeleton-only [B1]

Context: Embedded HARD RULE + user CLAUDE.md require test-first. M1 test files live inside
crates/amped-plugin-api and crates/amped-registry, which do not exist yet. Those tests
cannot compile until a minimal workspace skeleton exists — a strict chicken/egg.

Decision: Grant a ONE-TIME bootstrap exception scoped to EXACTLY the workspace skeleton:
root Cargo.toml ([workspace]-only), the 6 member Cargo.toml files, and stub src/lib.rs
(and one stub src-tauri bin) that compile and do nothing else. No behavioral logic ships
under this exception. ALL M1 behavior (manifest parsing, schema_version checks, registry
loading, checksum rejection, refresh-model parsing, WIT compile/validate) remains FULLY
test-first with NO exception.

Scope (exhaustive — nothing else is covered):
- root Cargo.toml [workspace] + resolver="2" + members list
- crates/{amped-core,amped-plugin-api,amped-registry,amped-host,amped-plugin-easee}/Cargo.toml + src/lib.rs (stub)
- src-tauri/Cargo.toml + src/main.rs (stub, NO tauri dep yet)
- deletion of old root [package] + src/main.rs

Date: 2026-06-05

Consequences: Commit 1 of the PR has only a "compiles + members resolve" DoD (acceptable
ONLY under this exception). Every subsequent commit obeys tests-first.

Verbatim user approval (MUST be filled before merge):
> "Ja, godkänn undantaget"
> — Amped maintainer (user), via /pipeline AskUserQuestion, 2026-06-05. Scope: workspace skeleton only (root Cargo.toml -> [workspace]; the 6 member Cargo.toml + stub lib.rs/main.rs; deletion of old root [package] + src/main.rs). All M1 behavior remained test-first.

(Per user CLAUDE.md Pipeline-disciplin: bootstrap exception requires verbatim approval +
date + scope. Until this line is populated, the skeleton MUST NOT be committed.)

## (b) Rust edition mix [EDITION CONCERN]

Context: Existing root Cargo.toml is edition 2024; plan §2 specifies "edition 2021 i bibliotek".

Decision: After the M0 transform the root Cargo.toml has NO [package] section (it is
[workspace]-only), so the root carries NO edition at all. All library/member crates under
crates/* and src-tauri declare edition = "2021" per plan §2. The previous edition=2024
root package is removed (its src/main.rs is deleted; src-tauri replaces the app entrypoint).

Rationale: A [workspace]-only root needs no edition; pinning members to 2021 matches the plan's
stated library edition and avoids 2024-only behaviors leaking into the stable plugin contract.
resolver="2" is set explicitly at the workspace level.

Consequences: No edition mismatch warnings; the old 2024 root is intentionally retired. If a
member later needs edition 2024, that is a new, per-crate ADR (not covered here).

## (c) WIT toolchain choice: cargo-component vs wit-bindgen [feeds B2]

Context: Plan §2/§4 names `cargo component` as the build tool. Whether cargo-component (or
wasm-tools) is installed in this environment is UNKNOWN at planning time (not probed).

Decision: Adopt cargo-component + Component Model (WIT) as the canonical contract tooling
(consistent with plan). For M1, the WIT need only PARSE/VALIDATE — full guest codegen is
an M2+ concern. The Test Engineer MUST probe the toolchain:
- If cargo-component/wasm-tools present: T-API-wit-compiles runs fully (parse + world check).
- If absent: T-API-wit-compiles DEGRADES to a structural/syntax check (wasm-tools/wit-parser
  dev-dep, or lowest-tier textual structural assertion) and is annotated
  #[ignore = "SKIP: toolchain absent — ADR-0001(c)/B2"]. The SKIP MUST be visible in output.

Rationale: Decouples M1 (contract definition) from M2 (guest execution); avoids blocking on a
possibly-missing tool while keeping the WIT honestly validated at the best available tier.

Consequences: If cargo-component proves ergonomically blocking later, see decision (d)
process-isolation contingency — same security model, different transport.

## (d) WASM sandbox decision + process-isolation contingency [plan §4/§9]

Context: Plan locks "WASM-sandbox (wasmtime) from day one" — host-mediated capabilities, no
ambient authority (plan §1d, §4, §9). NOTE: actual wasmtime EXECUTION is M2, OUT OF SCOPE
for M0+M1. This ADR records the decision and its M0/M1 footprint only.

Decision: The contract is shaped for WASM Component Model now (WIT world in amped-plugin-api).
The security model — no ambient authority, host owns all I/O, capabilities granted per
manifest ∩ approval — is fixed. M0/M1 commit ONLY the WIT contract + manifest/registry types
that this model requires; no wasmtime dependency is added in M0/M1.

Contingency: If WASM guest ergonomics block progress in M2+, switch the capability TRANSPORT to
process-isolation (separate OS process + IPC) via a FUTURE ADR. The security model (no ambient
authority, host-injected auth, manifest-gated caps) is IDENTICAL; only the transport differs.
The WIT-shaped manifest/permission types defined in M1 remain valid under either transport.

Rationale: Recording the decision now keeps M1's contract shape coherent without pulling heavy
runtime deps into the scaffold.

Consequences: No wasmtime/cargo-component RUNTIME code in M0/M1; only contract + types.

## (e) Auth-header ownership resolution [B4]

Context: Plan §4 http-request carries `headers: list<tuple<string,string>>`, which would let an
untrusted plugin set its own Authorization header and bypass the host-owned token model (plan §7).

Decision: Authorization is HOST-OWNED and HOST-INJECTED. The M1 contract MUST make it impossible
for a plugin to set Authorization. Resolution direction (Security Reviewer + Plugin Developer
finalize the mechanism):
- Option A (preferred): the plugin-supplied request carries NO arbitrary header map; instead a
  constrained allow-list of non-auth headers, with Authorization/Proxy-Authorization excluded
  by construction.
- Option B: keep `headers` but the host STRIPS/OVERWRITES any Authorization/Proxy-Authorization
  (case-insensitive) before injecting its own bearer token; enforced + tested.

Rationale: Plugins are untrusted by default (plan §9); credential injection must be unforgeable.

Consequences: WIT shape for http-request is finalized in M1 to ENABLE enforcement; the runtime
negative test ("plugin-supplied Authorization is ignored") lands in M2 with the capability gate,
but the enabling contract decision and a host-stub unit test land in M1 (AC-M1-10).

## ADR triggers (what would require a NEW ADR beyond ADR-0001)

- Any **breaking change** to the WIT world (`amped:plugin` exports/imports) or a `package` version bump. `[CONFIRMED: C5]`
- Bumping any crate to **edition 2024** (or mixing editions beyond the 2021 baseline). `[CONFIRMED: §4b]`
- Introducing any **`unsafe`** block. `[CONFIRMED: C10]`
- Switching capability transport from **WASM → process-isolation** (plan §4 contingency). `[CONFIRMED: §4d]`
- Adding a **runtime dependency** not in the stack table (wasmtime, reqwest, tokio, keyring) — those belong to M2+ and each entry into M0/M1 would be an out-of-scope ADR.
- Changing manifest `schema_version` to accept >1, or relaxing the **mandatory checksum** rule. `[CONFIRMED: C7, AC-M1-3/5]`
- Allowing a plugin to set `Authorization` (reversing §4e). `[CONFIRMED: B4]`
- Making `src-tauri` depend on `tauri` (pulls heavy tree) — defer to M3; entering it in M0 is an ADR. `[PROBABLE]`
