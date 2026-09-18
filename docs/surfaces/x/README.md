# X / Twitter surface dossier

- Surface: X / Twitter
- Operator/company: X Corp.
- Canonical domain: `x.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: **highest**
- Overall access tier: **provisional Tier C — Adversarial**
- Confidence: high for official API/export facts; medium for the undocumented first-party protocol until direct observation is added

## Executive summary

X is currently Sociarium's highest-value surface and the first deep Surface Atlas target.

The current documented developer interface is technically capable but commercially hostile to the project's zero-spend self-data goal. X's official API uses prepaid pay-per-use credits. Reads are billed per returned resource. Qualifying **Owned Reads** for the app owner's own posts, bookmarks, followers, likes, lists, and related self-data are discounted to $0.001 per resource, but they are still paid reads.

X separately offers an official account archive through normal account settings. X documents the archive as machine-readable HTML/JSON and says it includes profile information, posts, Direct Messages, attached media, followers, following, Lists, and other account data. This is strong for historical bootstrap and preservation, but it is an asynchronous manual export rather than a continuous acquisition interface.

X Help currently says the ordinary profile timeline displays up to 3,200 of the user's most recent posts and directs users to the archive for older history. The current official `GET /2/users/{id}/tweets` API reference does **not** state that same 3,200 ceiling. Sociarium therefore treats the current API historical ceiling as **unknown pending direct/official evidence** rather than inheriting an older assumption.

Public technical reverse-engineering projects independently document a substantial first-party web protocol under `x.com/i/api/graphql/{queryId}/{operationName}`, with operations such as `UserTweets`, `TweetDetail`, `Bookmarks`, `Likes`, `Following`, and `Followers`. These sources also describe query IDs and feature sets as client-build-sensitive. This is currently **public secondary technical evidence**, not yet a direct Sociarium observation.

X's current Terms of Service expressly prohibit scraping and automated access through interfaces other than X's currently available published interfaces unless separately permitted. X's April 2026 Automation Rules also say not to use non-API automation such as scripting the X website. That creates a major implementation constraint even where the private protocol is technically observable.

## Tier assessment

**Provisional Tier C — Adversarial.**

Reasons:

- useful official self-data reads are metered rather than generally free;
- the comprehensive official export is manual/asynchronous and is not a reasonable continuous synchronization primitive;
- the first-party web client appears to have broad live access through an undocumented protocol;
- X's current terms/policies explicitly constrain automated use of non-published interfaces;
- recurring manual export should not be normalized as the operator's permanent job.

This tier is about the **access relationship**, not the quality of X as a social network.

## Acquisition candidates

### Official X API

Technically clean and already partially implemented in Sociarium.

Strengths:

- documented;
- OAuth 2.0 PKCE support;
- stable numeric user/Post IDs;
- pagination and incremental parameters;
- broad endpoint coverage;
- explicit rate limits.

Weaknesses:

- prepaid pay-per-use;
- zero-spend self-data synchronization is not available under the documented standard pricing model;
- pricing is mutable;
- current history ceiling for the user-post endpoint remains unverified in this research pass.

Status: **preserved optional paid backend; not the active implementation target.**

### Official account archive

Strengths:

- first-party;
- machine-readable;
- broad historical/account coverage;
- suitable for recovery/bootstrap and provenance.

Weaknesses:

- manual request;
- asynchronous preparation;
- poor fit for continuous synchronization;
- exact per-file format and completeness by data class should be verified against a real current archive.

Status: **valuable bootstrap/recovery candidate; not an acceptable sole live-acquisition strategy.**

### First-party private web protocol

Strengths indicated by public technical evidence:

- live;
- broad operation set;
- same protocol family used by the shipped web client;
- likely exposes many data classes important to Sociarium.

Weaknesses:

- undocumented;
- operation/query identifiers and feature flags may drift with web builds;
- authenticated requests use live browser-session authority;
- current X terms/policies materially constrain non-published automated access.

Status: **research target, not implementation-admitted.**

## Implementation recommendation

**Do not resume X implementation yet.**

Next X work should be forensic:

1. preserve this documented/public-research baseline;
2. directly observe the current first-party web client from the operator's own authenticated browser;
3. map read-only operations relevant to self-data;
4. record pagination, history depth, response identity semantics, and protocol drift;
5. keep all cookies/tokens/session material outside the public repository;
6. only then make a deliberate implementation-admission decision.

The existing official-API and archive-import code remains preserved substrate and implementation evidence.

## Files

- [Access matrix](access-matrix.md)
- [Official API](official-api.md)
- [Official archive/export](archive.md)
- [Private first-party protocol](private-protocol.md)
- [Terms and operational constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
