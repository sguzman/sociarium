# Repository format

Sociarium's repository is intended to outlive any particular executable build.

## M0 physical layout

Real persistence pressure from the first X slice settled one important question: the **atomic durable unit is an acquisition bundle**. Keeping one completed remote acquisition in a single directory lets Sociarium publish the bundle with one directory rename and prevents a partially written set of raw/normalized files from masquerading as a completed sync.

```text
sociarium.toml
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

Surface, profile, and acquisition identifiers are encoded into safe filesystem components by the store; their semantic values remain inside manifests/records.

### Acquisition manifest

`manifest.json` identifies the profile, surface, observation time, acquisition ID, normalized-record count, the next remote cursor (when one exists), and each preserved raw file with media type, byte length, and SHA-256 digest.

### Raw evidence

`raw/` contains exactly the response evidence selected by the adapter for preservation. Adapter-suggested paths must be relative and may not escape the acquisition directory. HTTP authorization headers, OAuth token responses, PKCE verifiers, and other credential material are not raw social evidence and must never be placed here.

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

This is an application-crash consistency contract, not yet a claim of full power-loss durability across every filesystem.

## Conceptual layers

The earlier architectural distinction remains unchanged even though the M0 physical layout groups evidence by acquisition:

- **raw** — what a remote surface returned;
- **normalized** — Sociarium's stable social-domain representation;
- **provenance** — manifest metadata connecting acquisition, raw files, profile, and normalized observations;
- **derived** — human/agent views built from durable records;
- **indexes** — disposable query machinery.

Future materialized `objects/` views may provide convenient latest-object projections, but they must remain rebuildable from acquisition history and must not erase historical observations.

## Canonical versus disposable

Canonical/durable:

- configuration describing tracked profiles (minus secrets);
- completed acquisition bundles;
- preserved raw evidence selected by retention policy;
- normalized observation records;
- acquisition manifests/provenance;
- schema/migration metadata.

Recoverable operational state:

- profile cursor checkpoint files, because completed manifests contain the same cursor boundary.

Disposable/rebuildable:

- SQLite databases;
- FTS/vector indexes;
- caches;
- generated Markdown/agent context views.

Deleting indexes should be boring. Deleting acquisitions should not be.

## Secrets

OAuth access tokens, refresh tokens, client secrets where applicable, and other credentials must never be committed to the corpus repository. Credential storage is an operational concern outside canonical Git-tracked data.

## Git

Git versions the corpus repository; it is not the query engine or persistence API.

A sync can update corpus files without requiring an automatic commit. Auto-commit may later be a policy option rather than an architectural prerequisite.
