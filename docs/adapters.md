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

Historical note: earlier Sociarium work inherited the long-standing assumption that X's user-post API exposed approximately the most recent 3,200 Posts. The 2026-09-18 Surface Atlas research pass did not find that ceiling in the current `GET /2/users/{id}/tweets` reference. X Help still documents a 3,200-post bound for the ordinary **profile UI** and directs users to the archive for older history.

Therefore the current API lifetime-history ceiling is now **unverified/unknown** rather than a core architectural fact. Any future X implementation must establish its actual retrievable history from current evidence before making completeness claims. See `docs/surfaces/x/official-api.md`.

Archive/import paths remain additive acquisition sources; they do not change the authority model.

## Existing X adapter policy

X is the first implemented adapter, not the active project milestone and not automatically the next implementation target.

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

### Existing X Post fidelity boundary

The generic `Post` model already supports portable `reply_to` and `quote_of` references. The X payload model understands `referenced_tweets` / relationship kinds and can normalize `replied_to` and `quoted` references.

The live X request must explicitly request the remote `referenced_tweets` field for those relationships to exist in production data. Repost/retweet relationships are not yet represented generically in the existing implementation and must not be silently flattened.

The existing X adapter must preserve the best complete authored text the X payload exposes. X uses the selectable `note_tweet` field for long-form Note Tweet data, and current X documentation states that Posts longer than 280 characters keep their full text there rather than in the ordinary `text` field. When `note_tweet.text` is present, the normalized generic `Post.text` should therefore use that full value and fall back to `text` otherwise.

This requirement does **not** force M0 to normalize every rich-text entity or article feature. It is a fidelity rule for the already-existing portable `Post.text` field. Successful raw evidence remains available for later richer interpretation.

The existing pre-Atlas X adapter uses the historical remote names `referenced_tweets` and `note_tweet`. Current 2026-09-18 X reference material instead advertises names including `referenced_posts` and `note_post`. During the research freeze this is recorded as a compatibility question, not silently "fixed" in code. Any resumed implementation must verify the live wire contract first.

X wire names remain X wire names. The remote API uses parameter/field names such as `tweet.fields`, `referenced_tweets`, and `note_tweet`; the fact that Sociarium's portable ontology calls the object a `Post` does not justify renaming remote protocol parameters.
