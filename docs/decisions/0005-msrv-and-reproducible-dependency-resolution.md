# ADR 0005: Enforce MSRV with reproducible dependency resolution

- Status: accepted
- Date: 2026-09-17

## Context

Sociarium declares Rust 1.85 as its minimum supported Rust version (MSRV) and is an application workspace rather than a library intended to be embedded with an arbitrary downstream lockfile.

A stable-Rust-only CI job initially allowed the dependency graph to drift to releases that no longer built on Rust 1.85. Adding an explicit Rust 1.85 CI lane exposed two distinct failure modes:

1. with Cargo resolver 2, fresh resolution selected transitive ICU4X and IDNA releases whose declared Rust requirements were above 1.85;
2. after switching to resolver 3, Cargo correctly selected versions whose published metadata claimed Rust 1.85 compatibility, but `yoke-derive 0.8.3` still failed to compile on Rust 1.85.

Therefore neither a `rust-version` field nor MSRV-aware resolution alone is sufficient to make the executable reproducible at the declared floor.

The URL stack also intentionally permits selecting among multiple `idna_adapter` 1.x Unicode backends. The 1.2.x line uses ICU4X; the 1.1.x line uses `unicode-rs`; the 1.0.x line is a stub backend. This gives the application a supported place to make an explicit compatibility/backend choice.

## Decision

Sociarium uses all of the following together:

1. workspace `resolver = "3"` so Cargo prefers dependency releases compatible with the workspace `rust-version`;
2. a committed root `Cargo.lock` as part of application build state;
3. CI commands use `--locked` so normal verification cannot silently rewrite dependency resolution;
4. a dedicated CI job runs `cargo +1.85.0 check --workspace --all-targets --locked`;
5. native Windows CI remains separate because Windows-only code paths, especially credential storage, must compile and test on the actual target;
6. `idna_adapter` is exact-pinned to `1.1.0` at the X network-stack boundary, selecting its full `unicode-rs` backend and avoiding the ICU4X/Yoke chain that failed the declared MSRV;
7. the Windows `keyring` dependency remains exact-pinned to the known-compatible 3.6.3 line because the current 4.x line requires a newer Rust toolchain.

The `idna_adapter` dependency in `sociarium-adapter-x` is intentionally present even though Sociarium does not import it directly. It is a backend-selection dependency: `url -> idna` accepts any 1.x adapter, so the exact application-level requirement constrains the shared graph to the selected Unicode backend.

## Consequences

- A source checkout resolves the same tested dependency graph instead of selecting a different graph based on the day it is built.
- Raising the Rust floor becomes an explicit repository change rather than ambient dependency drift.
- Dependency upgrades require updating `Cargo.lock` deliberately and passing stable Linux, native Windows, and Rust 1.85 CI.
- The X network stack retains internationalized-domain-name support while avoiding an unnecessary ICU4X dependency chain.
- Some dependencies are intentionally pinned more tightly than ordinary semver ranges; those pins must be revisited consciously when the MSRV changes or upstream compatibility improves.

## Updating dependencies

Dependency refreshes should be deliberate:

1. update manifest constraints if needed;
2. regenerate/update `Cargo.lock` intentionally;
3. inspect meaningful version/backend changes;
4. run the full CI matrix with locked resolution;
5. update this ADR if a compatibility pin is removed, replaced, or becomes architectural debt.

Do not delete `Cargo.lock` to obtain a "cleaner" dependency graph. For Sociarium, the lockfile is reproducible build state.

## Rejected alternatives

### Stable-Rust CI only

Rejected because it verifies only today's toolchain, not the declared Rust 1.85 compatibility contract.

### Resolver 3 without a lockfile

Rejected because MSRV metadata can be inaccurate or insufficient to guarantee that a selected release actually compiles at the floor.

### Pin arbitrary ICU4X/Yoke transitive crates

Rejected because those crates are implementation details several layers below Sociarium. Selecting the supported `idna_adapter` backend is a cleaner boundary.

### Use `idna_adapter` 1.0.x stub backend

Rejected because it would avoid ICU4X by giving up a real Unicode backend. The 1.1.x `unicode-rs` backend preserves IDNA behavior while satisfying the compatibility goal.
