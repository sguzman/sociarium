# Reddit source ledger

Last reviewed: 2026-09-18.

## Official current sources

### Reddit Data API Wiki

https://support.reddithelp.com/hc/en-us/articles/16160319875092-Reddit-Data-API-Wiki

Updated May 11, 2026.

Used for:

- approval/request requirement;
- OAuth requirement;
- descriptive User-Agent requirement;
- free-access 100 QPM rate limit;
- non-OAuth traffic blocking;
- deletion and retention requirements;
- warning that legacy support/API documentation may be stale.

### Developer Platform & Accessing Reddit Data

https://support.reddithelp.com/hc/en-us/articles/14945211791892-Developer-Platform-Accessing-Reddit-Data

Updated May 28, 2026.

Used for:

- distinction among Devvit, Data API, Ads API, RFR, Embeds;
- approved-developer framing;
- free vs paid access;
- commercial-use contract requirement;
- Data API sign-up requirement;
- academic-research restriction.

### Built-in Reddit API documentation

https://www.reddit.com/dev/api

Used cautiously because the current Data API Wiki warns legacy technical material may be out of date.

Used for:

- current displayed endpoint inventory;
- `/api/v1/me`;
- submitted/comments/saved/upvoted/downvoted/hidden user listings;
- subreddit subscription listing;
- private-message listings;
- generic Listing pagination;
- fullname type semantics.

### Reddit API Overview / Developer Platform

https://developers.reddit.com/docs/capabilities/server/reddit-api

Used for:

- Devvit's Reddit API model;
- explicit list of private user data that Devvit apps cannot access.

### Modernizing Reddit's Infrastructure and Moderation Tools

https://redditinc.com/news/modernizing-reddits-infrastructure-and-moderation-tools

Published August 5, 2026.

Used for:

- current announcement that limited public API access will continue;
- gradual restriction of new requests;
- planned migration of third-party apps toward Developer Platform;
- anti-scraping direction.

### Data API Terms

https://redditinc.com/policies/data-api-terms

Last revised July 20, 2026.

Used for:

- registration/access-info requirements;
- user-content license;
- future-fee reservation;
- retention limited to approved use case;
- anti-circumvention/reverse-engineering restrictions for Data APIs;
- right to change/suspend APIs.

### Developer Terms

https://redditinc.com/policies/developer-terms

Last revised March 24, 2026.

Used for:

- content-removal obligations;
- data sharing/retention duties;
- authorized access requirements;
- platform update requirements.

### User Agreement

https://redditinc.com/policies/user-agreement

Current agreement effective July 1, 2026.

Used for:

- restriction on automated/other collection outside permitted terms or agreement;
- scraping prohibition without prior written consent;
- reverse-engineering/technical-access limitations.

### Request a copy of Reddit data

https://support.reddithelp.com/hc/en-us/articles/360043048352-How-do-I-request-a-copy-of-my-Reddit-data-and-information

Updated March 19, 2026.

Used for:

- official data-request path;
- up-to-30-day preparation window;
- notification/download flow.

### Where and how can I access my Reddit data?

https://support.reddithelp.com/hc/en-us/articles/360043483511-Where-and-how-can-I-access-my-Reddit-data-and-information

Updated March 20, 2026.

Used for first-party availability of:

- own posts/comments;
- up/downvotes;
- saved/hidden/recent views;
- joined/moderated communities;
- messages/Chat/mod mail;
- follows/friends/blocks;
- preferences/security/integrations.

## Public technical observations / secondary sources

### Recent Reddit browser report: Shreddit GraphQL join operation

Public 2026 Reddit reports show first-party requests to:

`https://www.reddit.com/svc/shreddit/graphql`

with operation/variables/CSRF-shaped payloads.

These are secondary/public observations, not yet Sociarium direct evidence.

### agents-io/reddit-unofficial-api

https://github.com/agents-io/reddit-unofficial-api/blob/main/docs/api-reference.md

Used as secondary evidence for:

- modern `/svc/shreddit/*` HTML-fragment endpoints;
- `<shreddit-post>` / `<shreddit-comment>` web-component representation;
- anti-bot brittleness.

Do not promote these details above direct current first-party observation.

## Evidence hierarchy

1. direct Sociarium observation of the operator's first-party client;
2. current official Reddit help/terms/developer documentation;
3. Reddit's current built-in API docs, with its explicit staleness warning;
4. multiple recent public browser/network observations;
5. third-party reverse-engineering docs;
6. historical memory.

Unknowns stay unknown until stronger evidence resolves them.
