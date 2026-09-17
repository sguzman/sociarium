# Search index

Sociarium's search database is a disposable projection of the durable corpus. It is not authoritative state.

## M0 implementation

The M0 index is SQLite with FTS5 and lives at:

```text
corpus/indexes/search.sqlite
```

It is rebuilt exclusively from completed acquisition bundles under `corpus/acquisitions/`. Pending acquisition directories are ignored. Every normalized record is checked against its acquisition manifest before it is admitted to the index.

For posts observed more than once, the index keeps the latest normalized observation for that stable object id. Older observations remain in the durable acquisition history and can be reinterpreted later; they are not deleted by indexing.

## Rebuildability invariant

Deleting `corpus/indexes/search.sqlite` must never destroy social data. The supported recovery path is:

```text
sociarium index rebuild
```

Rebuild writes a new database to a temporary sibling path and replaces the visible index only after construction succeeds. Queries open the resulting database read-only.

## Query surface

M0 exposes:

```text
sociarium posts list
sociarium posts list --profile x-main
sociarium posts search sociarium
sociarium posts search "fallen village" --profile x-main
```

The CLI deliberately talks to the local index rather than to a remote surface. This keeps ordinary corpus interrogation offline, fast, and independent of API billing or availability.

## Boundary

SQLite schema is an implementation detail of the search projection. MCP and future agent-facing APIs must not depend directly on its tables. They should depend on a semantic query layer so the index implementation can later change without becoming corpus ontology.
