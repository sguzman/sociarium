# X source ledger

Last reviewed: 2026-09-18.

This ledger records sources used by the baseline dossier. Retrieval date is 2026-09-18 unless stated otherwise.

## Official X sources

### X API pricing

https://docs.x.com/x-api/getting-started/pricing

Used for:

- prepaid pay-per-use model;
- per-resource read pricing;
- write pricing;
- Owned Reads;
- qualifying owned-read endpoints;
- 3 million monthly Post-read cap;
- deduplication;
- credit balance / spending-limit model.

### OAuth 2.0 Authorization Code + PKCE

https://docs.x.com/fundamentals/authentication/oauth-2-0/authorization-code

Used for:

- public vs confidential client types;
- access-token lifetime;
- `offline.access` refresh tokens;
- scopes;
- exact-match callback requirement.

### User Posts endpoint

https://docs.x.com/x-api/users/get-posts

Used for:

- `GET /2/users/{id}/tweets`;
- pagination;
- `since_id` / `until_id`;
- time filters;
- current field/expansion names.

Important negative finding: this research pass did not find the old 3,200 total-history statement in the current endpoint reference.

### Rate limits

https://docs.x.com/x-api/fundamentals/rate-limits

Used for current endpoint rate-limit tables covering timelines, likes, follows, Direct Messages, Lists, Bookmarks, media, activity, and other API families.

### X account data/archive help

https://help.x.com/en/managing-your-account/accessing-your-x-data

Used for:

- archive request flow;
- machine-readable HTML/JSON description;
- documented archive contents;
- entire post history statement.

### New user FAQ

https://help.x.com/en/resources/new-user-faq

Used for:

- profile timeline displays up to 3,200 recent posts;
- archive recommended for older posts / history from first post.

### X Terms of Service

https://x.com/en/tos

Used for:

- published-interface restriction;
- scraping prohibition;
- technical-limit / reverse-engineering language;
- dated implementation-admission constraint.

### X Automation Rules

https://help.x.com/en/rules-and-policies/x-automation

Used for:

- April 2026 automation policy;
- prohibition on non-API automation such as scripting the X website;
- account-enforcement risk.

## Public technical / reverse-engineering sources

These are **secondary evidence**, not official documentation and not direct Sociarium observations.

### gobird wire protocol

https://github.com/mudrii/gobird/blob/main/docs/wire-protocol.md

Useful for:

- `x.com/i/api/graphql/{queryId}/{operationName}` pattern;
- operation inventory;
- public description of cookie/CSRF/web-client authentication shape;
- variables/features/field-toggle encoding;
- distinction among read/write operation shapes.

### x-agent-sdk

https://github.com/alarok/x-agent-sdk

Useful for:

- independent current corroboration that logged-in browser sessions use cookie names `auth_token` and `ct0`;
- `ct0` mirrored into the CSRF header;
- current `x-client-transaction-id` requirement in a separate implementation.

Cookie/session **values are never copied into Sociarium**. This source is secondary evidence because Edge sanitized HAR exports remove the ordinary Cookie header and request-cookie array.

### tweetkit-x constants

https://github.com/nsozturk/tweetkit-x/blob/main/tweetkit_x/constants.py

Useful for:

- explicit warning that X web GraphQL query IDs rotate with web builds;
- examples of operation names such as `UserTweets`, `UserByScreenName`, `CreateTweet`;
- evidence that private clients refresh IDs from HAR/current web traffic.

### twitter-web-client

https://github.com/uakihir0/twitter-web-client

Useful for:

- independent operation inventory including `UserTweets`, `Bookmarks`, `Following`, `Followers`, `TweetDetail`, and search/timeline operations.

## Evidence hierarchy

For current private-protocol claims:

1. direct local observation of the first-party X client;
2. current official X documentation where applicable;
3. multiple current public technical sources;
4. single public reverse-engineering source;
5. historical memory / old implementation assumptions.

The dossier should be revised when higher-quality evidence conflicts with lower-quality evidence.
