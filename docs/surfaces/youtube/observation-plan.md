# YouTube direct observation plan

Status: planned.

The goal is to compare supported Data API, first-party web behavior, Google account activity tools, and Takeout.

## Evidence handling

Raw captures can contain:

- Google login cookies;
- OAuth/session tokens;
- private playlists/history;
- purchases/memberships;
- account identifiers;
- viewing/search history.

Keep raw HAR/network captures private.

## Supported API validation

Using a deliberately registered local test project later, verify:

1. `channels.list mine=true`;
2. uploads playlist traversal;
3. owned playlists;
4. subscriptions `mine=true`;
5. liked videos through both available API patterns;
6. `myRating` 1000-item bound;
7. channel activity;
8. comments relevant to the operator's channel/content.

Record quota cost for each workflow.

No implementation code is needed for this R1 validation plan.

## First-party observation priorities

1. Watch History;
2. Watch Later;
3. Liked Videos;
4. Subscriptions;
5. own channel/videos;
6. comments;
7. playlists;
8. search history;
9. notifications;
10. Your Data in YouTube.

For each, record whether the current web client uses:

- `youtubei` / Innertube;
- Google My Activity infrastructure;
- public Data API;
- another service.

## Cross-source comparison

Compare the same resource among:

- Data API;
- first-party UI/Innertube;
- Takeout.

Questions:

- same stable video/channel/playlist IDs?
- same timestamp?
- deleted/private video representation?
- title snapshot vs current metadata?
- history ordering?
- pagination depth?
- account/channel/Brand Account context?

## Takeout inspection

A real current archive should be treated as private evidence.

Publish only a sanitized manifest/schema and representative redacted shapes.

## Implementation gate

The supported Data API may eventually be admitted independently from private protocol research.

Innertube automation is not admitted by this observation plan.
