# Bluesky direct observation plan

Status: planned.

Unlike X/Reddit, the purpose here is primarily **validation and source comparison**, not reverse engineering an opaque private protocol.

## Goals

Verify how the flagship Bluesky application projects documented AT Protocol state.

Compare:

1. signed repository records;
2. public AppView responses;
3. first-party web rendering;
4. authenticated private-state APIs where relevant.

## Priority observations

### Identity

- resolve own handle to DID;
- inspect DID document/PDS endpoint;
- confirm handle can be treated as mutable while DID remains stable.

### Public repository

- export/fetch own repo;
- identify profile record;
- identify authored Post records;
- identify Like/Follow/List records;
- inspect AT URIs, rkeys, CIDs, timestamps, and strong references;
- enumerate referenced blobs.

### Incremental stream

- observe a bounded repo event sequence;
- create/update/delete a disposable test record only if intentionally authorized;
- confirm event operation semantics and revisions;
- verify local mirror logic can conceptually resume from a known point.

### AppView

Compare a repository Post with:

- public post view;
- thread view;
- author feed/timeline view;
- counts/derived fields not present in the authored repository.

This should make the authority boundary visible: authored record vs indexed/derived social view.

### Private state

When ready:

- preferences;
- bookmarks;
- chat.

Keep private response contents out of the public repo.

## Evidence handling

Bluesky's public repo can contain public social content, but that does not automatically make every raw account artifact appropriate for a public software repository.

Private OAuth tokens, refresh tokens, app passwords, chat messages, private preferences, and account-security data remain private.

## Deliverables

Sanitized observations under:

`docs/surfaces/bluesky/observations/YYYY-MM-DD-<topic>.md`

The observation should cite both protocol docs and what was directly seen.

## Implementation gate

Direct observation can strengthen the implementation-admission case, but R1 remains documentation-first until the research phase is explicitly closed.
