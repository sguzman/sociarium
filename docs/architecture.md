# Architecture

## Purpose

Sociarium maintains a durable local corpus of deliberately selected social profiles across multiple remote social surfaces. It acquires remote observations, preserves evidence, normalizes genuinely shared concepts into a surface-independent model, and exposes that corpus through human and agent interfaces.

The center of the system is the corpus, not any particular remote API.

## System boundary

```text
remote surfaces
    |
    v
surface adapters
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

Remote surfaces own their live systems. Sociarium owns its acquired evidence and local representations.

## Core concepts

### Surface

A remote social system such as X, Reddit, Bluesky, Mastodon, or YouTube.

The core stores a stable `SurfaceId`, but surface-specific API types remain in adapter crates.

### Adapter

A Rust implementation that understands one surface's authentication, endpoints, pagination, rate limits, object semantics, and capability constraints.

An adapter translates remote observations into generic records plus lossless or sufficiently rich surface-specific extensions.

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
- derived indexes, summaries, and agent views are disposable projections.

The local corpus is the durable historical record after acquisition. This does not imply that the local state controls or overrides live remote state.

## Read and write asymmetry

Local reads and analysis should be broad and cheap. Remote mutations should be explicit, separately authorized, auditable, and capability-scoped.

M0 is read/acquisition only.

## Dependency direction

```text
sociarium-core
      ^
      |
sociarium-adapter <--- sociarium-adapter-x
      ^                     ^
      |                     |
sociarium-store             |
      ^                     |
      +------ sociarium-cli-+
```

Future MCP/search crates depend inward on core/store abstractions. Core never depends on adapters, CLI, MCP, or a particular storage engine.

## Non-goals

Sociarium is not attempting to recreate social platforms. Recommendation feeds, ad systems, general-purpose public social search, notification ecosystems, or GUI clones are out of scope unless later justified by corpus acquisition/preservation/query needs.
