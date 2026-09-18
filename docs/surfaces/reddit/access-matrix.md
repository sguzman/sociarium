# Reddit access matrix

Status: baseline desk research, 2026-09-18.

The table distinguishes **current technical availability** from long-term platform direction.

| Data class | Data API | Devvit / Developer Platform | Official data export / account access | Public web | Private first-party protocol | Preservation notes |
| --- | --- | --- | --- | --- | --- | --- |
| account identity/profile | built-in docs expose `/api/v1/me` and user-about endpoints | public Reddit API subset exists; private profile info explicitly unavailable | account settings/data request | public profile | likely available to first-party client; direct observation pending | stable account fullname/id should be verified live |
| authored posts | built-in docs expose `/user/{username}/submitted` | can read Reddit content in app/community context | Reddit Help says own posts accessible through account; export available | public unless removed/private context | Shreddit feed/profile paths pending direct observation | deleted API-acquired content must be removed under current policy |
| authored comments | built-in docs expose `/user/{username}/comments` | can read Reddit content in app/community context | Reddit Help says own comments accessible through account; export available | public unless removed/private context | pending | same deletion-retention conflict |
| upvoted/downvoted | built-in docs expose `/user/{username}/upvoted` and `downvoted` under history scope | **explicitly unavailable** as private user data | Reddit Help says user can access votes through account; export availability exact format pending | private to user | first-party UI necessarily has access; exact protocol pending | strategically important gap if Data API is retired |
| saved content | built-in docs expose `/user/{username}/saved` | **explicitly unavailable** | Reddit Help says saved posts/comments accessible in account; export package inclusion not yet verified | private to user | first-party UI has Saved view; protocol pending | important self-data class |
| hidden content | built-in docs expose `/user/{username}/hidden` | current Devvit private-data list does not explicitly enumerate hidden; availability unknown | Reddit Help says hidden posts/comments accessible in account | private | pending | unknown |
| joined communities/subscriptions | built-in docs expose `/subreddits/mine/subscriber` | **explicitly unavailable** | Reddit Help says joined communities accessible in account | membership may be private | first-party protocol supports join/leave; public observations show GraphQL mutations | snapshot/history semantics unknown |
| follows/friends | legacy/built-in endpoints exist for friends; exact modern follows coverage needs validation | **explicitly unavailable** | Reddit Help says followed people/friends accessible in account | partly visible depending on feature | pending | exact current API semantics unknown |
| blocks | built-in docs expose blocked/friend account endpoints | capability depends on platform context; needs verification | Reddit Help says blocked users accessible in account | private | pending | snapshot |
| private messages | built-in docs expose inbox/sent/unread message listings | Devvit's private-user-data restrictions plus app context require dedicated validation | Reddit Help lists archived private messages | private | first-party protocol pending | old private messages and newer Chat are distinct systems |
| Chat | Data API Wiki documents free-access chat-message limits, but endpoint model needs current validation | platform capabilities exist but personal-chat access unknown | Reddit Help lists Chats | private | first-party live protocol pending | high-sensitivity evidence |
| mod mail | extensive API/Devvit support | strong platform support | Reddit Help lists mod mail | private/mod-only | first-party protocol exists | not central unless operator moderates |
| recent viewed posts | not established as Data API self-data endpoint | **explicitly unavailable** | Reddit Help exposes Recent Posts in account UI | private | first-party state/protocol pending | likely ephemeral/limited |
| authorized third-party apps/preferences/IP history | not ordinary social Data API objects | n/a | Reddit Help says available through account/data request | private | settings endpoints/UI | portability/security data rather than social corpus |

## API pagination and IDs

Reddit's built-in API docs define generic **Listings** with:

- `after` / `before` cursors;
- `limit` up to 100;
- `count`;
- Reddit **fullnames** as globally unique typed IDs such as `t1_` comments, `t2_` accounts, `t3_` links/posts, `t4_` messages, and `t5_` subreddits.

These are attractive corpus identity semantics, but Sociarium still needs direct current validation for the specific self-data listings it cares about.

## Deletion is an access property

A technically available API source is not automatically preservation-friendly.

Current Reddit Data API policy requires deletion of locally held Reddit content when the source content/account is deleted, and explicitly says retention of deleted content even after de-identification violates policy.

Therefore a Data API adapter could be technically strong while still being architecturally incompatible with Sociarium's desire to preserve historical observations.

That incompatibility must remain visible rather than being hidden inside implementation.
