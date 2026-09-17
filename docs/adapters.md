# Adapter model

Adapters isolate remote-surface behavior from the Sociarium ontology.

## Responsibilities

An adapter owns:

- authentication and token refresh;
- HTTP/API transport;
- endpoint paths and request parameters;
- pagination and cursors;
- rate-limit interpretation;
- remote error translation;
- surface payload types;
- capability discovery/declaration;
- conversion of acquired data into normalized records plus preserved raw evidence.

An adapter does not own repository authority, global identity resolution, indexing, MCP semantics, or Git policy.

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

Capabilities may be further constrained by authentication context or profile ownership.

## Profile-scoped synchronization

The primitive synchronization operation targets a configured profile.

Convenience commands such as `sync --surface x`, `sync --owned`, or `sync --all` should expand into profile-scoped work rather than bypassing that model.

## X adapter policy

X is the first adapter only.

The X adapter will talk directly to the X API from Rust and will eventually own OAuth 2.0 Authorization Code + PKCE, refresh tokens, rate limiting, pagination, and X payloads. `xurl`, XMCP, and other external platform CLIs are explicitly not runtime dependencies.

The X Developer App remains remote registration/credential state required by X; it is not a local runtime component to replace.
