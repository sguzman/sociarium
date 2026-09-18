# Synchronization

> **Status:** technical reference for the existing software substrate. The current Surface Atlas phase is research-first; this document does not define the active milestone.

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

## Bootstrap coverage is not lifetime completeness

A first synchronization can only acquire history that the remote surface currently exposes. Durable local preservation does not imply unlimited retrospective remote access.

The pre-Atlas X implementation was designed around a bounded-history assumption inherited from older X/Twitter behavior. Surface Atlas research on 2026-09-18 found that X Help still documents a 3,200-post bound for the **profile UI**, but the current `GET /2/users/{id}/tweets` reference does not state the same total-history ceiling.

Accordingly, future live X work must treat the API's lifetime-history ceiling as **unknown until verified**. A terminal traversal proves only that the chosen acquisition source has no further page under the observed contract; it must not be translated into "account fully archived" without source-specific coverage evidence.

The durable `since_id` model remains useful for forward incremental continuity if/when the official API backend is resumed. Archive/import or other historical sources can add older evidence without redefining existing acquisitions.

## Pagination is not incremental state

A remote pagination token answers: "where is the next page of this traversal?"

An incremental checkpoint answers: "what remote state had been completely incorporated when the last traversal finished?"

Those are different concepts. `SocialAdapter::cursor_has_more` lets an adapter return a non-empty durable checkpoint even when the current traversal is complete.

For the existing X adapter, the cursor contains:

- the previous completed `since_id` high-water mark;
- a `pagination_token` while a traversal still has more pages;
- the newest Post ID observed in the current traversal.

During a multi-page traversal, `since_id` remains the prior completed high-water mark while `newest_id` advances. When the final page is durable, `newest_id` becomes the new `since_id` and the pagination token is cleared. The next invocation can therefore request only Posts newer than the completed checkpoint.

This also makes interruption recovery safe: a crash after a page is persisted resumes from the cursor stored with that page rather than repeating or skipping the page transition.

## Stable remote identity before sync

Profile synchronization resolves corpus identity before contacting the remote surface.

For a configured local profile, `CorpusStore::profile_binding` scans completed durable `ProfileSnapshot` observations for that profile/surface and reports `Unbound`, `Bound`, or `Conflicted`.

The CLI composition layer then applies these rules:

1. an explicitly configured `remote_id` must agree with a durable binding if one exists;
2. if configuration omits `remote_id` but durable evidence is bound, inject the durable ID into the working profile passed to the adapter;
3. conflicted durable identity evidence is terminal and no remote request is made;
4. for an unbound X profile, the X adapter requires either an explicit remote ID or a configured handle that matches the authenticated username case-insensitively;
5. after binding, X checks the stable remote ID and does not require the mutable handle to remain unchanged.

The corpus store repeats stable-ID validation before an acquisition is made canonical. Adapter-side rejection is therefore an early safety check, not the only integrity boundary.

This prevents a logout/login or credential swap from silently changing what a local profile means while allowing legitimate remote handle changes.

## Credential resolution before sync

Credential lookup is deliberately outside the generic synchronization runner. The runner receives an already-constructed adapter; it does not know how a surface authenticates.

For X, the CLI resolves credentials in this order:

1. if a nonblank `SOCIARIUM_X_ACCESS_TOKEN` process environment override exists, use it for that invocation;
2. otherwise load the profile-scoped OAuth token envelope from the native credential store;
3. if the access token is inside the refresh leeway, use the configured X OAuth application settings to refresh it;
4. persist the refreshed envelope back to the same profile-scoped credential key;
5. construct the authenticated X adapter and hand it to the generic sync runner.

The normal desktop path is therefore:

```text
sociarium auth login x-main
sociarium sync x-main
```

The environment variable remains an emergency operational escape hatch rather than the normal credential path. Neither path writes bearer material into corpus acquisitions, raw evidence, manifests, normalized records, or indexes. See `security-and-credentials.md`.

## CLI

The generic command shape is:

```text
sociarium sync <profile-id>
```

Existing synchronization controls:

```text
sociarium sync x-main --max-pages 10000
sociarium sync x-main --no-index
```

A successful sync rebuilds the disposable search index unless `--no-index` is supplied.

The existing X user-post request uses `tweet.fields=created_at,referenced_tweets,note_tweet` and `exclude=retweets`. Replies and quote Posts remain eligible and their portable relationships are normalized when X supplies them; long-form Posts prefer full `note_tweet.text`; retweets are deliberately excluded until Sociarium has a portable repost/reblog relation. Live paid-X validation is preserved as an optional/deferred path; further X implementation now waits on the Surface Atlas research/admission process.

## Failure rules

The runner rejects:

- disabled profiles;
- adapter/profile surface mismatches;
- zero page limits;
- cursor cycles;
- non-terminal cursors that fail to advance;
- persistence failures before cursor advancement.

Authentication failures occur before the generic runner starts or are returned by the adapter as remote/authentication errors. A failed index rebuild does not invalidate previously persisted acquisition bundles. The durable corpus remains the authority; the index is a projection.
