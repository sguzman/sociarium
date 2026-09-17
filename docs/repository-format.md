# Repository format

Sociarium distinguishes two repositories/directories with different authority and privacy roles:

```text
sguzman/sociarium/        # software source repository
<operator corpus>/        # durable social-data corpus repository/directory
```

The **software source repository** contains Rust source, tests, public examples, architecture, and build state. It is not the default home for real acquired social data.

The **corpus repository** is operator-owned durable data. It is intended to outlive any particular executable build and may be versioned with Git independently of the software. If pushed to a remote Git host, private visibility is the safe default because later capabilities may preserve authorized/private observations even though credentials themselves are forbidden from the corpus.

Git is optional for persistence: Sociarium's filesystem durability contract must work before or without `git init`. Git adds history/transport; it is not the storage API.

## M0 corpus physical layout

Real persistence pressure from the first X slice settled one important question: the **atomic durable unit is an acquisition bundle**. Keeping one completed remote acquisition in a single directory lets Sociarium publish the bundle with one directory rename and prevents a partially written set of raw/normalized files from masquerading as a completed sync.

Conceptually an initialized corpus is:

```text
sociarium.toml             # non-secret corpus/profile configuration; may be tracked deliberately
.gitignore                 # corpus policy, not the software-source .gitignore
acquisitions/
  <surface>/
    <profile>/
      <acquisition-id>/
        manifest.json
        raw/
          ... preserved response bodies ...
        normalized/
          records.jsonl
state/
  profiles/
    <profile>/
      <acquisition-id>.json
derived/
indexes/
```

The exact initialization/scaffolding behavior is tracked in issue #6. Until that is implemented, do not use the public software checkout's `corpus/` subdirectory for real account data merely because `cargo run` executes there.

Surface, profile, and acquisition identifiers are encoded into safe filesystem components by the store; their semantic values remain inside manifests/records.

### Configuration policy differs by repository

The public software source repo ignores its root `sociarium.toml` by default because that file is machine/operator-local during development and smoke testing.

A dedicated corpus repo may deliberately track its **non-secret** `sociarium.toml` because tracked profiles and non-secret surface configuration are part of durable corpus state. This does not relax the credential boundary: access tokens, refresh tokens, client secrets, passwords, authorization codes, PKCE verifiers, and session cookies remain forbidden from any corpus configuration or tracked path.

### Acquisition manifest

`manifest.json` identifies the profile, surface, observation time, acquisition ID, normalized-record count, the next remote cursor (when one exists), and each preserved raw file with media type, byte length, and SHA-256 digest.

### Raw evidence

`raw/` contains exactly the **successful social-response evidence** selected by the adapter for preservation. Adapter-suggested paths must be relative and may not escape the acquisition directory.

HTTP authorization headers, OAuth token responses, OAuth/API failure bodies, PKCE verifiers, and other credential/protocol material are not raw social evidence and must never be placed here merely for diagnostics.

### Normalized records

`normalized/records.jsonl` is an ordinary UTF-8 JSON Lines stream of `NormalizedRecord` values. It is durable corpus evidence, not a query index.

The same remote object may legitimately appear in more than one acquisition because Sociarium preserves observations over time rather than pretending a mutable remote object was observed only once.

### Checkpoint state

A successful acquisition writes an immutable profile checkpoint under `state/profiles/` **after** the acquisition directory becomes visible. State never advances first.

The cursor is also recorded in the acquisition manifest, so profile state can be reconstructed by scanning completed acquisition manifests if a process crashes after publishing the acquisition but before writing its checkpoint file. Checkpoints are therefore operational accelerators, not the only copy of synchronization progress.

## Crash-consistency boundary

The write sequence is:

```text
build hidden pending acquisition directory
  -> write raw evidence
  -> write normalized records
  -> write manifest
  -> rename pending directory to final acquisition directory
  -> write immutable profile checkpoint
```

Errors before the rename remove the pending directory on a best-effort basis. An application crash may leave a hidden `.pending-*` directory; readers ignore those directories. A crash after the rename cannot make the completed acquisition disappear from recovery logic merely because its checkpoint was not written.

Corpus Git policy must also ignore `.pending-*` staging directories so a crash followed by `git add .` cannot promote partial evidence into canonical history. Issue #6 owns this initialization policy.

This is an application-crash consistency contract, not yet a claim of full power-loss durability across every filesystem.

## Conceptual layers

The earlier architectural distinction remains unchanged even though the M0 physical layout groups evidence by acquisition:

- **raw** — what a remote surface successfully returned as retained social evidence;
- **normalized** — Sociarium's stable social-domain representation;
- **provenance** — manifest metadata connecting acquisition, raw files, profile, and normalized observations;
- **derived** — human/agent views built from durable records;
- **indexes** — disposable query machinery.

Future materialized `objects/` views may provide convenient latest-object projections, but they must remain rebuildable from acquisition history and must not erase historical observations.

## Canonical versus disposable

Canonical/durable:

- non-secret configuration describing tracked profiles and surface settings;
- completed acquisition bundles;
- preserved successful raw evidence selected by retention policy;
- normalized observation records;
- acquisition manifests/provenance;
- schema/migration metadata.

Recoverable operational state:

- profile cursor checkpoint files, because completed manifests contain the same cursor boundary.

Disposable/rebuildable:

- SQLite databases;
- FTS/vector indexes;
- caches;
- generated Markdown/agent context views unless explicitly promoted to durable material.

Deleting indexes should be boring. Deleting acquisitions should not be.

The generated corpus ignore policy should make this distinction visible to Git: completed canonical acquisitions stay stageable, while `indexes/`, staging directories, and other explicitly disposable state do not become history by accident.

## Secrets and private data are different concepts

OAuth access tokens, refresh tokens, client secrets where applicable, and other credentials must never be committed to the corpus repository.

A corpus can still contain **private or authorized social data** that is not itself a credential, such as future bookmarks or private observations a surface legitimately returned to the authorized user. Therefore "contains no secrets" does not imply "safe to publish publicly." Remote Git visibility is an operator decision; private is the conservative default.

## Git

Git versions the corpus repository; it is not the query engine or persistence API.

A sync can update corpus files without requiring an automatic commit. Auto-commit may later be a policy option rather than an architectural prerequisite. GitHub is likewise a possible projection/transport destination, not corpus authority.
