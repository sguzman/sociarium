# Reddit official API and Developer Platform

Research date: 2026-09-18.

## Two developer surfaces

Reddit currently distinguishes:

1. **Data API** — OAuth programmatic access to Reddit data for approved developers;
2. **Developer Platform / Devvit** — Reddit-hosted app platform with automatic Reddit API authentication and a curated API surface.

These are not equivalent.

## Data API access

Current Reddit Help says the Data API is available to **approved developers**.

For non-commercial use, developers must sign up/request access. Clients must authenticate with a registered OAuth token and use a unique descriptive User-Agent. Unauthenticated/non-login traffic may be blocked.

For users eligible for free Data API access, the current documented rate limit is:

- **100 queries per minute per OAuth client ID**, averaged over a 10-minute window.

Current Data API Terms reserve Reddit's right to charge future fees. Broader/commercial access can require a separate contract.

## Built-in API documentation

Reddit's own current Data API Wiki links to the site's built-in live API documentation while warning that some legacy documentation/support may be out of date.

That built-in documentation currently lists account/self-data endpoints including:

- `/api/v1/me`;
- `/user/{username}/submitted`;
- `/user/{username}/comments`;
- `/user/{username}/saved`;
- `/user/{username}/upvoted`;
- `/user/{username}/downvoted`;
- `/user/{username}/hidden`;
- `/subreddits/mine/subscriber`;
- private message inbox/sent/unread listings;
- account blocked/friends/preferences-related endpoints.

Generic listings use `after`/`before` cursor-style pagination and typically cap a slice at 100 items.

### Stable IDs

The built-in docs define Reddit **fullnames** as a typed globally unique object identifier:

- `t1_` — Comment
- `t2_` — Account
- `t3_` — Link/Post
- `t4_` — Message
- `t5_` — Subreddit
- `t6_` — Award

This maps well to Sociarium's stable-ID doctrine if live validation confirms the relevant objects still expose these values consistently.

## Current policy hazard: retention

The Data API Wiki imposes unusually strong deletion obligations:

- deleted posts/comments must be removed from the client's possession;
- deleted accounts require removal of related user ID and author-identifying data;
- retaining deleted content even after de-identification is described as a policy violation;
- Reddit strongly recommends routinely deleting stored user data/content within 48 hours.

The Data API Terms also prohibit retaining data beyond the approved use case and require deletion of data not required for it.

This is a first-class architectural constraint for Sociarium, whose durable corpus is specifically meant to preserve prior observations.

## Reddit's announced platform direction

In August 2026 Reddit announced infrastructure/developer changes:

- limited public API access will continue;
- Reddit will **gradually restrict new requests**;
- third-party apps are expected to transition to the Developer Platform.

This means a new Sociarium integration cannot assume today's legacy Data API admission path is a durable long-term contract.

## Devvit and private self-data

Devvit handles authentication automatically for installed apps, but Reddit's current Developer Platform docs explicitly say Devvit apps **cannot access** several logged-in user's private data classes:

- subscribed subreddits;
- upvoted/downvoted content;
- saved content;
- recently viewed posts;
- non-public profile information;
- follows/friends.

For a conventional community app this may be sensible. For a user-sovereign self-archive it leaves major gaps.

That creates a strategic contradiction:

```text
legacy Data API
    broad self-data listings
        ↓
platform direction says migrate
        ↓
Devvit
    deliberately hides several private self-data classes
```

This contradiction is one of the main reasons Reddit is provisionally Tier C overall despite the existence of current free Data API access.

## Research classification caution

Reddit Help says academic research using Reddit data must use Reddit For Researchers rather than normal developer APIs.

Sociarium is presently a personal user-sovereignty/software project, not being asserted as academic research. Do not misrepresent the use case either way when applying for any future access.

## Current implementation recommendation

No adapter yet.

Before implementation, verify:

- whether Sociarium's actual personal non-commercial use case would be approved;
- live access to each desired self-data endpoint;
- practical history depths;
- whether retention/deletion requirements are acceptable for the intended corpus;
- Reddit's migration timetable from Data API to Devvit.
