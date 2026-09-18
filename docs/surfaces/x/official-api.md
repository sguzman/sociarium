# X official API

Research date: 2026-09-18.

Evidence class: **documented fact** unless otherwise noted.

## Pricing model

X's current documentation describes the standard X API as **pay-per-usage**.

The operator purchases credits upfront in the Developer Console. Credits are deducted as API requests return resources or perform actions. X says there are no subscriptions or minimum spend for this model.

Current documented read prices include:

| Resource | Price |
| --- | ---: |
| Post read | $0.005 / resource |
| User read | $0.010 / resource |
| DM event read | $0.010 / resource |
| Following / Followers read | $0.010 / resource |
| List read | $0.005 / resource |
| Like read | $0.001 / resource |
| Mute read | $0.001 / resource |
| Block read | $0.001 / resource |

Pay-per-usage plans are currently capped at 3 million Post reads per monthly billing cycle.

Prices are explicitly subject to change.

## Owned Reads

X documents a discounted **Owned Read** price of **$0.001 per returned resource** when the authenticated user is also the developer-app owner and the endpoint's `{id}` matches that user.

The current qualifying list includes:

- own posts;
- mentions;
- liked posts;
- bookmarks;
- followers;
- following;
- blocks;
- mutes;
- owned lists;
- followed lists;
- list memberships;
- pinned lists.

This is favorable relative to X's normal read rates but still fails Sociarium's zero-paid-self-data preference.

Example purely for scale: 1,000 qualifying owned resources cost $1 at the documented rate.

## Authentication

X documents OAuth 2.0 Authorization Code Flow with PKCE for user-delegated access.

Relevant facts:

- Native Apps and Single Page Apps are public clients;
- Web Apps and automated/bot apps are confidential clients;
- a PKCE access token lasts two hours by default;
- `offline.access` causes a refresh token to be issued;
- callback URLs require exact-match validation;
- granular scopes include `tweet.read`, `users.read`, `follows.read`, `like.read`, `bookmark.read`, `list.read`, `block.read`, `mute.read`, and `dm.read`.

The existing Sociarium X adapter was designed around this general model.

## User-post timeline

Current endpoint:

`GET /2/users/{id}/tweets`

The current reference documents:

- `max_results`: 5–100;
- `pagination_token`;
- `start_time`;
- `end_time`;
- `since_id`;
- `until_id`;
- exclusions for replies and retweets;
- selectable Post fields and expansions.

Current rate-limit documentation lists:

- 10,000 requests / 15 minutes per app;
- 900 requests / 15 minutes per user.

### Historical-ceiling caution

Older Sociarium documentation inherited the long-standing 3,200-user-timeline assumption.

This 2026-09-18 research pass did **not** find a 3,200 total-history statement in the current user-post endpoint documentation.

X Help separately states that the **profile timeline UI** shows up to 3,200 recent posts and that the archive can be used to browse from the first post.

Until current API evidence establishes otherwise, the API's lifetime-history ceiling is therefore **unknown**.

## Other self-data endpoint coverage

Current official documentation/rate-limit tables expose read endpoints for:

- mentions;
- liked posts;
- followers/following;
- blocks/mutes;
- bookmarks and bookmark folders;
- owned/list membership/pinned Lists;
- Direct Messages;
- Spaces;
- Activity/webhook products;
- media-related operations.

This is broad technical coverage. The core problem for Sociarium is not that the official API is weak; it is that live self-data access is metered.

## Wire-name drift warning

The current 2026 API reference advertises field/expansion names including `note_post` and `referenced_posts`.

Existing pre-Atlas Sociarium X code/docs were written against older names such as `note_tweet` and `referenced_tweets`.

Do not "fix" code during the research freeze. Record this as a compatibility question for any future implementation-admission phase:

- Are the old names still accepted?
- Were the wire names actually renamed?
- Is this only a documentation-schema naming normalization?

A live official-API validation would be required before changing the adapter.

## Sources

See [sources.md](sources.md), especially the current X pricing, OAuth, rate-limit, and user-post endpoint documentation.
