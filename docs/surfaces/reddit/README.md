# Reddit surface dossier

- Surface: Reddit
- Operator/company: Reddit, Inc.
- Canonical domain: `reddit.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: unranked relative to the non-X inventory
- Overall access tier: **provisional Tier C — Adversarial**
- Important exception: the currently approved non-commercial Data API is closer to **Tier B — Workable** for several ordinary read paths
- Confidence: high for current policy/access facts; medium for undocumented first-party protocol details pending direct observation

## Executive summary

Reddit presents a mixed access relationship rather than a simple open/closed result.

The current Data API still provides an OAuth-authenticated programmatic interface. Reddit's June 2026 Responsible Builder Policy explicitly says API access requires a request plus **explicit approval**, and the current Data API Wiki documents **100 queries per minute per OAuth client ID** for users eligible for free access. Access is not anonymous: OAuth/login credentials are required for the normal rate-limited path.

The built-in API documentation continues to expose broad account-oriented functionality including identity, authored posts/comments, saved items, upvoted/downvoted items, hidden items, subreddit subscriptions, messages, and other account data. Reddit's current Data API Wiki warns that some legacy technical documentation may be stale, so endpoint existence in the built-in docs is evidence but not a substitute for live validation.

The most serious conflict with Sociarium is not price. It is **control over retained history**. Reddit's current Data API guidance says API clients must remove content that users delete from Reddit, including author-identifying information after account deletion, and says retaining deleted content even after anonymization violates policy. Reddit strongly recommends routinely deleting stored user data/content within 48 hours. Its Developer Terms similarly require updating/deleting locally held Reddit data under multiple conditions.

That is in direct tension with Sociarium's provenance-preserving historical-corpus purpose.

Reddit is also actively moving away from the legacy Data API as the general third-party integration surface. In August 2026 Reddit announced that it will continue limited public API access but gradually restrict new requests and require third-party apps to transition to the Developer Platform (Devvit). Devvit, however, explicitly does **not** expose several private logged-in user data classes, including subscribed subreddits, up/downvoted content, saved content, recent views, private profile information, and follows/friends.

The official account-data export is available, but Reddit says preparation can take **up to 30 days**. This is useful for portability/recovery but unacceptable as a normal continuous acquisition loop.

Public browser/network observations show the current first-party web app uses an undocumented `/svc/shreddit/graphql` endpoint for at least some authenticated operations, alongside other `/svc/shreddit/*` endpoints. That protocol deserves documentation, but Reddit's current User Agreement prohibits scraping without prior written consent and restricts automated collection outside permitted terms/agreements.

## Why provisional Tier C

Reddit earns a provisional overall Tier C for a different reason than X.

Positive factors:

- a real OAuth Data API still exists;
- eligible approved non-commercial access can be free;
- the free rate limit is substantial for a personal corpus;
- stable Reddit fullnames/IDs and generic Listing pagination are well established;
- broad self-data endpoint coverage exists in the built-in API docs.

Adversarial factors:

- Data API access requires approval and is being strategically narrowed;
- Reddit announced a migration toward Devvit while Devvit withholds important private self-data classes;
- API data retention/deletion rules conflict with durable historical preservation;
- official export may take up to 30 days;
- current User Agreement prohibits scraping without written consent;
- the private first-party protocol is undocumented.

This means **data-class/source-specific tiers matter**:

- authored public posts/comments through an approved Data API: approximately Tier B;
- saved/voting/subscription/follow data under the future Devvit-only direction: currently trends Tier C;
- historical preservation of deleted API-acquired content: Tier C conflict with Sociarium's goals;
- official export: useful recovery/bootstrap but operationally poor for continuous acquisition.

## R2 current-policy validation — 2026-09-18

Issue #13 re-checked the baseline against current official Reddit sources.

Confirmed:

- Reddit's Responsible Builder Policy requires explicit approval before API access;
- eligible free Data API access remains documented at 100 QPM per OAuth client ID;
- the Data API Wiki warns that some legacy technical docs may be stale;
- Reddit's August 2026 platform announcement says limited public API access will continue while new requests are gradually restricted and third-party apps are moved toward Devvit;
- current Devvit documentation explicitly withholds subscribed communities, vote history, saved content, recent views, private profile information, and follows/friends from apps;
- the current Data API guidance requires deletion of remotely deleted content and says retaining deleted content even after anonymization violates policy;
- the official data-request flow remains current and may take up to 30 days.

This strengthens, rather than weakens, the preservation conflict behind Reddit's provisional Tier C classification. Live endpoint validation and first-party observation remain necessary before the tier and implementation recommendation are finalized.

## Implementation recommendation

**Do not implement Reddit yet.**

The current research questions are more important than code:

1. verify which legacy Data API self-data endpoints remain approval-accessible today;
2. determine whether a personal/local archival use case would actually be approved;
3. establish practical listing/history ceilings for authored posts/comments, saved, and vote history;
4. inspect a current official data export and document its exact format/completeness;
5. directly observe current first-party Shreddit reads for the operator's own account;
6. determine what parts of Reddit's deletion/retention policy apply to self-owned personal historical preservation and whether that makes a durable API-backed corpus operationally incompatible.

Only after that should Sociarium decide whether Reddit merits a Data API adapter, export importer, documentation-only status, or another strategy.

## Files

- [Access matrix](access-matrix.md)
- [Official API and Developer Platform](official-api.md)
- [Official export](export.md)
- [Private first-party protocol](private-protocol.md)
- [Terms and retention constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
