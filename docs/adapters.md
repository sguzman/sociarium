# Adapter model

> **Research-first gate:** an adapter is an implementation consequence, not the starting point for understanding a surface. Before new surface-specific adapter work normally begins, the Surface Atlas should have a current dossier, access tier, per-data-class matrix, and explicit implementation-admission decision. See `surface-research-doctrine.md` and ADR 0007.

Adapters isolate remote-surface behavior from the Sociarium ontology.

## Responsibilities

An adapter owns:

- authentication protocol and token-refresh semantics;
- HTTP/API transport;
- endpoint paths, remote wire names, and request parameters;
- pagination and cursors;
- rate-limit/entitlement interpretation;
- remote error translation;
- surface payload types;
- capability discovery/declaration;
- conversion of acquired data into normalized records plus preserved raw evidence.

An adapter does not own repository authority, persistent secret storage, global identity resolution, indexing, MCP semantics, or Git policy.

## Capabilities

Adapters declare capabilities rather than forcing core code to assume all surfaces support the same operations.

Initial capability vocabulary includes:

- profile snapshots;
- posts;
- replies;
- reactions;
- relationships;
- collections/bookmarks;
- remote writes.

Capabilities may be further constrained by authentication context, current remote API access, or profile ownership.

## Profile-scoped synchronization

The primitive synchronization operation targets a configured profile.

Convenience commands such as `sync --surface x`, `sync --owned`, or `sync --all` should expand into profile-scoped work rather than bypassing that model.

## Remote availability versus local durability

A durable local corpus can preserve everything Sociarium successfully acquires, but an adapter cannot manufacture history the remote surface does not expose.

Initial synchronization therefore has two separate concepts:

- **bootstrap coverage:** the historical window the remote API currently makes retrievable;
- **forward continuity:** observations Sociarium keeps acquiring and preserving after synchronization begins.

For X, the user-post timeline currently exposes only a bounded recent history (approximately the most recent 3,200 Posts). A first X sync must therefore not be described as a complete lifetime account export. Once Sociarium is running incrementally, its local history can remain durable even as older remote timeline entries fall out of X's retrievable window.

Later archive-import or full-archive acquisition paths may fill older history when the operator has an export or current X access permits it. Those are additive acquisition sources; they do not change the authority model.

## X adapter policy

X is the first adapter only.

The X adapter talks directly to the X API from Rust and currently owns:

- OAuth 2.0 Authorization Code + S256 PKCE protocol semantics;
- access/refresh token response semantics while persistent secret storage remains in `sociarium-credentials`;
- current X HTTP endpoints and remote query names;
- pagination and incremental `since_id` cursor behavior;
- X payload types;
- normalization of profile snapshots and Posts into generic records;
- preserved raw evidence for successful social acquisitions.

`xurl`, XMCP, and other external platform CLIs are explicitly not runtime dependencies.

The X Developer App remains remote registration state required by X; it is not a local runtime component to replace.

### M0 Post fidelity boundary

The generic `Post` model already supports portable `reply_to` and `quote_of` references. The X payload model understands `referenced_tweets` / relationship kinds and can normalize `replied_to` and `quoted` references.

The live X request must explicitly request the remote `referenced_tweets` field for those relationships to exist in production data. Repost/retweet relationships are not yet represented generically in M0 and must not be silently flattened.

M0 must also preserve the best complete authored text the X payload exposes. X uses the selectable `note_tweet` field for long-form Note Tweet data, and current X documentation states that Posts longer than 280 characters keep their full text there rather than in the ordinary `text` field. When `note_tweet.text` is present, the normalized generic `Post.text` should therefore use that full value and fall back to `text` otherwise.

This requirement does **not** force M0 to normalize every rich-text entity or article feature. It is a fidelity rule for the already-existing portable `Post.text` field. Successful raw evidence remains available for later richer interpretation.

M0 now uses the remote `tweet.fields` parameter with `created_at,referenced_tweets,note_tweet`. It preserves supported reply/quote references, prefers full `note_tweet.text` when present, and explicitly sends `exclude=retweets` so reposts are not flattened into ordinary authored Posts before a portable repost relation exists.

X wire names remain X wire names. The remote API uses parameter/field names such as `tweet.fields`, `referenced_tweets`, and `note_tweet`; the fact that Sociarium's portable ontology calls the object a `Post` does not justify renaming remote protocol parameters.
