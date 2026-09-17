# ADR 0005: Acquisition bundles are the atomic durable sync unit

- Status: accepted
- Date: 2026-09-17

## Context

A remote sync yields several related artifacts at once: raw response evidence, normalized records, provenance, and a pagination cursor. Writing those independently into top-level directories creates an awkward crash boundary. If the cursor advances before every durable artifact exists, data can be skipped. If some artifacts become visible while others do not, the repository can represent an acquisition that never actually completed.

Git does not solve this at runtime because synchronization must be correct before any optional Git commit occurs.

## Decision

A completed acquisition is stored as one immutable directory containing:

```text
manifest.json
raw/
normalized/records.jsonl
```

Sociarium builds that directory under a hidden pending name and renames it to its final acquisition ID only after every required file has been written.

The acquisition manifest records the next pagination cursor. A separate immutable profile checkpoint is written only after the acquisition directory is published. Recovery derives effective profile state from completed acquisition manifests, so a missing checkpoint cannot cause the last completed page to be forgotten.

Raw, normalized, and provenance semantics remain distinct even though their physical files share one acquisition directory.

## Consequences

- A reader can ignore `.pending-*` directories and treat every other acquisition directory as a completed unit.
- Pagination state never needs to advance ahead of durable evidence.
- A crash between acquisition publication and checkpoint creation is recoverable from the manifest.
- Historical observations are naturally append-only.
- Query-oriented latest-object views and indexes become explicit derived projections rather than accidental authorities.
- The design provides application-crash consistency; stronger power-loss guarantees may require additional filesystem-specific syncing later.
