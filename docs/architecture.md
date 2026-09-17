# Architecture

## Purpose

Sociarium maintains a durable local corpus of deliberately selected social profiles across multiple remote social surfaces. It acquires remote observations, preserves evidence, normalizes genuinely shared concepts into a surface-independent model, and exposes that corpus through human and agent interfaces.

The center of the system is the corpus, not any particular remote API.

## System boundary

```text
remote surfaces
    |
    v
surface adapters <------ operational credentials
    |                         |
    |                         v
    |                  native credential store
    |
    +--> raw evidence
    |
    v
normalization
    |
    v
durable corpus
    |
    +--> rebuildable indexes/search
    +--> CLI
    +--> MCP / agent views
    +--> future UI / exports
```

Remote surfaces own their live systems. Sociarium owns its acquired evidence and local representations. Credentials remain operational authority outside the corpus.

## Core concepts

### Surface

A remote social system such as X, Reddit, Bluesky, Mastodon, or YouTube.

The core stores a stable `SurfaceId`, but surface-specific API types remain in adapter crates.

### Adapter

A Rust implementation that understands one surface's authentication protocol, endpoints, pagination, rate limits, object semantics, and capability constraints.

An adapter translates remote observations into generic records plus lossless or sufficiently rich surface-specific extensions. Persistent secret storage is not embedded in the adapter contract; orchestration supplies authenticated adapter instances after resolving profile-scoped credentials.

### Remote profile

A stable profile/account identity on one surface. It is identified by the surface plus that surface's stable remote identifier. Handles and display names are mutable observations, not identity keys.

### Person/entity

An optional higher-level identity that may explicitly link multiple remote profiles. Sociarium must never infer this equivalence as unquestionable truth merely because names or metadata match.

### Observation

A statement that a remote surface returned or exhibited some state at a particular time. Repeated observations can coexist and form history rather than being destructively overwritten.

## Authority model

Sociarium distinguishes authority by layer:

- a remote surface is authoritative for what its API returned at acquisition time;
- preserved raw evidence is authoritative for what Sociarium actually received;
- normalized records are Sociarium's typed interpretation of that evidence;
- derived indexes, summaries, and agent views are disposable projections;
- credential stores contain operational authority, not historical evidence.

The local corpus is the durable historical record after acquisition. This does not imply that the local state controls or overrides live remote state.

## Read and write asymmetry

Local reads and analysis should be broad and cheap. Remote mutations should be explicit, separately authorized, auditable, and capability-scoped.

M0 is read/acquisition only.

## Dependency direction

```text
                         sociarium-core
                         ^      ^     ^
                        /       |      \
                       /        |       \
          sociarium-adapter  config   credentials
                 ^             ^          ^
                 |             |          |
       sociarium-adapter-x     |          |
                 ^             |          |
                  \            |         /
                   \           |        /
                    +---- sociarium-cli ----+
                    |          ^             |
                    |          |             v
                    |    sociarium-sync   sociarium-search
                    |          ^             ^
                    |          |             |
                    +------ sociarium-store--+
```

The diagram is conceptual rather than a complete Cargo edge list. The important direction rules are:

- `sociarium-core` does not depend on adapters, storage, CLI, MCP, or a particular surface;
- `sociarium-adapter` defines generic acquisition behavior without depending on X;
- surface adapters depend inward on generic adapter/core types;
- `sociarium-credentials` stores opaque secret bytes keyed by generic profile identity and does not know X token semantics;
- `sociarium-sync` orchestrates adapters and durable storage but does not parse surface-specific cursors;
- `sociarium-search` is a projection over durable corpus records and remains disposable;
- `sociarium-cli` is the composition root that joins configuration, credentials, adapters, sync, storage, and search.

Future MCP/agent crates should depend on corpus/query abstractions rather than becoming a prerequisite for core behavior.

## Toolchain and dependency resolution

Sociarium declares Rust 1.85 as its minimum supported Rust version and uses Cargo resolver 3 so fresh dependency resolution honors that floor where upstream metadata permits it.

Because MSRV metadata can still be inaccurate, the root `Cargo.lock` is committed as reproducible application build state. CI verifies current stable Rust, native Windows behavior, and Rust 1.85 against that exact graph with `--locked`.

The X network stack also exact-pins `idna_adapter` 1.1.0. This intentionally selects its `unicode-rs` IDNA backend instead of the newer ICU4X backend whose Yoke chain failed an actual Rust 1.85 compile despite nominal compatibility metadata. The Windows credential backend similarly pins the keyring line known to respect the current MSRV.

These are part of the architecture contract: dependency resolution or a dependency update that raises the executable's Rust floor is a compatibility change, not routine ambient drift. See ADR 0005.

## Non-goals

Sociarium is not attempting to recreate social platforms. Recommendation feeds, ad systems, general-purpose public social search, notification ecosystems, or GUI clones are out of scope unless later justified by corpus acquisition/preservation/query needs.
