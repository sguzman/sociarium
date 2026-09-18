# Bluesky access matrix

Status: baseline desk research, 2026-09-18.

| Data class | Signed repository | Public AppView/API | Authenticated/private API | Export/migration | Live/incremental | Assessment |
| --- | --- | --- | --- | --- | --- | --- |
| account identity | DID document + repo identity references | profile lookup by handle/DID | account/session APIs | DID/PDS migration documented | identity resolution is live | Tier A |
| authored posts | repository records | public post/profile/feed APIs | not normally required to read public posts | included in repo CAR | repo commit stream | Tier A |
| replies/quotes/reposts | repository records/strong refs where authored by account | normalized post/thread views | not normally required for public data | included as repo records where owned | repo commit stream | Tier A |
| likes | repository records | public counts/views where exposed | own private convenience views may differ | repo-owned like records portable | repo commit stream | Tier A |
| follows | repository records | graph APIs | not normally required for public graph | repo-owned follow records portable | repo commit stream | Tier A |
| lists/starter-pack-like records | repository/app records depending feature | public app APIs | feature-specific | repo records portable where stored there | repo stream | generally Tier A |
| profile record | repository record | public profile API | write requires auth | portable in repo | repo stream | Tier A |
| media blobs | repo records reference blobs by CID; blobs stored separately on PDS | CDN/AppView access paths | upload/write auth required | blob export handled separately from repo CAR | blob arrival tied to repo changes | Tier A with separate blob transfer |
| actor preferences | not part of public signed repo | not public | authenticated `app.bsky.actor.getPreferences` / put/update flows | official migration exports/imports preferences separately | service-specific | Tier A/B |
| bookmarks | private application state, not assumed public repo data | public post view may expose bookmark state only to authorized context | dedicated bookmark RPCs exist | portability mechanism needs current verification | authenticated service state | Tier A/B, research incomplete |
| Direct Messages/chat | not part of public repo | not public | `chat.bsky.*` service APIs | chat is external/centralized service state; account-data export support requires dedicated verification | chat service | Tier B-ish compared with public repo |
| moderation preferences/labels | mixed: user-created repo records plus service-specific preferences | label services/AppView expose relevant views | authenticated prefs/moderation APIs | depends on class | mixed | source-specific |

## Stable identifiers

Bluesky/AT Protocol offers unusually strong identity semantics:

- **DID**: stable account identity.
- **Handle**: human-readable mutable identifier resolved to DID.
- **AT URI**: record identity based on repository DID, collection NSID, and record key.
- **CID**: content-addressed record/version identity where applicable.

These semantics map cleanly to Sociarium's distinction between stable remote identity and mutable display/handle observations.

## Repository completeness nuance

A CAR export is a complete export of the account's AT repository, not automatically every byte of every service associated with the account.

Blobs are transferred separately, and private/service-specific data such as preferences and chat may require separate APIs/export paths.

Therefore "complete repo" and "complete account" must remain distinct phrases in Sociarium documentation.
