# Synchronization

Synchronization is profile-scoped. A surface adapter acquires one configured `TrackedProfile` at a time and returns durable batches containing normalized records, raw evidence, and an opaque adapter-owned checkpoint cursor.

## Execution contract

The generic runner follows this order for every page:

1. recover the most recent durable profile cursor from acquisition manifests;
2. call the profile's adapter with that cursor;
3. validate the adapter result;
4. persist raw evidence, normalized records, the manifest, and the resulting cursor as one acquisition bundle;
5. only after persistence succeeds, decide whether another page is required;
6. continue until the adapter reports that the returned checkpoint is terminal for the current traversal.

The cursor is intentionally opaque outside the adapter. Core synchronization code must not parse surface-specific cursor contents.

## Pagination is not incremental state

A remote pagination token answers: "where is the next page of this traversal?"

An incremental checkpoint answers: "what remote state had been completely incorporated when the last traversal finished?"

Those are different concepts. `SocialAdapter::cursor_has_more` lets an adapter return a non-empty durable checkpoint even when the current traversal is complete.

For X, the M0 cursor contains:

- the previous completed `since_id` high-water mark;
- a `pagination_token` while a traversal still has more pages;
- the newest Post ID observed in the current traversal.

During a multi-page traversal, `since_id` remains the prior completed high-water mark while `newest_id` advances. When the final page is durable, `newest_id` becomes the new `since_id` and the pagination token is cleared. The next invocation can therefore request only Posts newer than the completed checkpoint.

This also makes interruption recovery safe: a crash after a page is persisted resumes from the cursor stored with that page rather than repeating or skipping the page transition.

## CLI

The generic command shape is:

```text
sociarium sync <profile-id>
```

Useful M0 controls:

```text
sociarium sync x-main --max-pages 10000
sociarium sync x-main --no-index
```

A successful sync rebuilds the disposable search index unless `--no-index` is supplied.

## Current X credential bridge

The durable credential-store and OAuth callback UX are not wired into the CLI yet. Until that slice lands, live X sync accepts an already-authorized user access token from the process environment:

```text
SOCIARIUM_X_ACCESS_TOKEN
```

This is deliberately an operational bridge, not corpus configuration. The token is never written into raw evidence, normalized records, manifests, search indexes, or `sociarium.toml`.

The environment bridge will be replaced by profile-scoped OS credential-store lookup plus OAuth2/PKCE authorization and refresh-token handling. See `security-and-credentials.md`.

## Failure rules

The runner rejects:

- disabled profiles;
- adapter/profile surface mismatches;
- zero page limits;
- cursor cycles;
- non-terminal cursors that fail to advance;
- persistence failures before cursor advancement.

A failed index rebuild does not invalidate previously persisted acquisition bundles. The durable corpus remains the authority; the index is a projection.
