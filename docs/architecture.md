# Architecture

## Purpose

Sociarium has two first-class planes:

1. a **knowledge plane** that forensically maps how social surfaces permit an authorized user to access and preserve their own data;
2. a **software/corpus plane** that implements selected acquisition strategies and preserves acquired evidence locally.

The Surface Atlas is not disposable planning material. It is durable project state and can remain valuable even for surfaces that never receive an adapter.

The long-term software substrate maintains a durable local corpus of deliberately selected social profiles across multiple remote social surfaces. It acquires observations, preserves evidence, normalizes genuinely shared concepts into a surface-independent model, and exposes that corpus through human and agent interfaces.

No particular remote API defines the project.

## System boundary

```text
                 SOCIAL SURFACE
                       |
          +------------+------------+
          |                         |
          v                         v
 documented interfaces      first-party clients
 exports / public web       web / mobile / live channels
          |                         |
          +------------+------------+
                       |
                       v
              SURFACE ATLAS RESEARCH
       docs + observations + provenance
                       |
                       v
        per-data-class access assessment
                       |
                       v
             dated access tier
                       |
                       v
        implementation admission decision
                       |
           +-----------+-----------+
           |                       |
        document only          implement source
                                   |
                  +----------------+----------------+
                  |                                 |
                  v                                 v
          live surface adapter              archive/import source
                  |                                 |
                  +----------------+----------------+
                                   |
                                   v
                           raw/source evidence
                                   |
                                   v
                              normalization
                                   |
                                   v
                             durable corpus
                                   |
                  +----------------+----------------+
                  |                |                |
                  v                v                v
               search            CLI          agent/MCP views
```

The knowledge plane is upstream of new implementation.

A surface can stop at **document only**. That is not failure. The dossier may establish that access is too expensive, too brittle, too incomplete, or currently crosses a boundary Sociarium should not build around.

Remote surfaces own their live systems. Sociarium owns its research record, acquired evidence, and local representations. Credentials remain operational authority outside both the public research corpus and the social-data corpus.

A social **surface** and an **acquisition source** are different concepts. X is the surface whether evidence arrives through an official API, an account archive, public rendering, or an authorized first-party private protocol. See ADR 0006.

Research-before-code and the Surface Atlas are architectural policy, not project-management preference. See ADR 0007.

## Core concepts

### Surface dossier

A dated, provenance-heavy research record for one social surface.

A dossier distinguishes documented facts, direct observations, inference, and unknowns. It records per-data-class access, cost, friction, history, authentication, protocol behavior, and candidate acquisition sources.

### Access tier

A dated summary classification of the surface's access relationship:

- Tier A — Sovereign-friendly;
- Tier B — Workable;
- Tier C — Adversarial;
- Tier D — Inaccessible.

The tier is not a social/political rating of the platform. Per-data-class matrices remain more precise than the overall tier.

### Implementation admission

A deliberate decision to spend engineering effort on a specific acquisition source after research establishes why it is worthwhile.

No surface is automatically entitled to implementation.

### Surface

A remote social system such as X, Reddit, Bluesky, Mastodon, or YouTube.

A surface exists independently of any particular developer API. Its dossier may describe several acquisition mechanisms with very different access conditions.

The software core stores a stable `SurfaceId`, but surface-specific protocol types remain outside the core.

### Surface dossier

A dated, provenance-heavy research record describing how an authorized user can access and preserve their own data on one surface.

A dossier records documented interfaces, observed first-party behavior, exports, identifiers, pagination/history, cost, friction, authentication boundaries, protocol drift, per-data-class access, and explicit unknowns.

Dossiers distinguish documented fact, direct observation, inference, and unknowns.

### Access tier

A dated summary of the access relationship:

- Tier A — Sovereign-friendly;
- Tier B — Workable;
- Tier C — Adversarial;
- Tier D — Inaccessible.

The tier does not replace the detailed per-data-class access matrix.

Tier C explicitly makes first-party private protocols a research target when sanctioned developer interfaces materially obstruct self-data access.

### Acquisition source

A mechanism by which Sociarium obtains evidence about a social surface.

Examples include a live remote API and a first-party account archive. Acquisition source is provenance, not identity: importing an X archive does not create a new surface or a second remote profile.

### Adapter

A Rust implementation for a live remote acquisition source. It understands one surface's authentication protocol, endpoints, pagination, rate limits, object semantics, and capability constraints.

An adapter translates remote observations into generic records plus lossless or sufficiently rich surface-specific extensions. Persistent secret storage is not embedded in the adapter contract; orchestration supplies authenticated adapter instances after resolving profile-scoped credentials.

Offline importers are separate acquisition implementations and do not need to implement the network-oriented `SocialAdapter` trait.

### Remote profile

A stable profile/account identity on one surface. It is identified by the surface plus that surface's stable remote identifier. Handles and display names are mutable observations, not identity keys.

### Person/entity

An optional higher-level identity that may explicitly link multiple remote profiles. Sociarium must never infer this equivalence as unquestionable truth merely because names or metadata match.

### Observation

A statement that a remote surface returned or exhibited some state at a particular time. Repeated observations can coexist and form history rather than being destructively overwritten.

## Authority model

Sociarium distinguishes authority by layer:

- official documentation is evidence of what the surface/operator currently claims;
- dated direct observations are evidence of what a first-party client/export/protocol actually exhibited in that context;
- inferences are interpretations and must remain labeled as such;
- the original acquisition source is authoritative for the source material it supplied at acquisition/import time;
- preserved raw/source evidence is authoritative for what Sociarium actually received;
- normalized records are Sociarium's typed interpretation of that evidence;
- derived indexes, summaries, and agent views are disposable projections;
- credential stores contain operational authority, not historical evidence.

The local corpus is the durable historical record after acquisition. This does not imply that the local state controls or overrides live remote state.

## Research, read, and write asymmetry

Research and local analysis should be broad.

Remote acquisition must respect the documented authorization boundary even when the acquisition source is an undocumented first-party protocol.

Remote mutations should be explicit, separately authorized, auditable, and capability-scoped.

The current project phase is research-first. New remote write work is not active.

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
- `sociarium-adapter` defines live remote-adapter behavior without depending on X;
- source-neutral acquisition envelopes should not require a network adapter;
- surface adapters and importers depend inward on generic acquisition/core types;
- `sociarium-credentials` stores opaque secret bytes keyed by generic profile identity and does not know X token semantics;
- `sociarium-sync` orchestrates adapters and durable storage but does not parse surface-specific cursors;
- `sociarium-search` is a projection over durable corpus records and remains disposable;
- `sociarium-cli` is the composition root that joins configuration, credentials, adapters, sync, storage, and search.

Future MCP/agent crates should depend on corpus/query abstractions rather than becoming a prerequisite for core behavior.

The Surface Atlas is deliberately not generated from adapter code. Research can precede, outlive, contradict, or decide against an implementation. Code may link back to dossier assumptions, but implementation is downstream of the knowledge plane.

## Toolchain and dependency resolution

Sociarium declares Rust 1.85 as its minimum supported Rust version and uses Cargo resolver 3 so fresh dependency resolution honors that floor where upstream metadata permits it.

Because MSRV metadata can still be inaccurate, the root `Cargo.lock` is committed as reproducible application build state. CI verifies current stable Rust, native Windows behavior, and Rust 1.85 against that exact graph with `--locked`.

The X network stack also exact-pins `idna_adapter` 1.1.0. This intentionally selects its `unicode-rs` IDNA backend instead of the newer ICU4X backend whose Yoke chain failed an actual Rust 1.85 compile despite nominal compatibility metadata. The Windows credential backend similarly pins the keyring line known to respect the current MSRV.

These are part of the architecture contract: dependency resolution or a dependency update that raises the executable's Rust floor is a compatibility change, not routine ambient drift. See ADR 0005.

## Non-goals

Sociarium is not attempting to recreate social platforms.

The research program does not treat "adversarial" as permission for credential theft, impersonation, or defeating meaningful access controls to reach data the operator is not authorized to access.

Recommendation feeds, ad systems, general-purpose public social search, notification ecosystems, or GUI clones are out of scope unless later justified by acquisition/preservation/query needs.

Sociarium also does not assume every researched surface should receive code. Documentation-only outcomes are first-class.


## Research architecture

The Surface Atlas lives under `docs/surfaces/` and is durable project state, not disposable planning material.

Each dossier should preserve:

- dates and client/context;
- evidence class (documented / observed / inferred / unknown);
- source provenance;
- per-data-class access matrix;
- monetary cost and non-monetary friction;
- stable identity/object semantics;
- authorization boundary;
- history/pagination behavior;
- protocol volatility;
- acquisition candidates;
- implementation recommendation.

For adversarial surfaces, public documentation may be supplemented by sanitized observations of the authorized first-party client. Sensitive raw captures remain outside the public repository.

See ADR 0007 and `docs/surface-research-doctrine.md`.
