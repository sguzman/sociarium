# Facebook surface dossier

- Surface: Facebook
- Operator/company: Meta Platforms, Inc.
- Canonical domain: `facebook.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: unranked relative to the non-X inventory
- Overall access tier: **provisional Tier C — Adversarial**
- Confidence: high for first-party access/export and anti-scraping posture; medium for the exact current personal-profile Graph API surface because Meta's developer documentation is highly permission/version/use-case dependent; medium for private web protocol until direct observation

## Executive summary

Facebook gives users substantial **human-facing access and portability tooling** but a much narrower general-purpose machine interface for personal-profile self-data.

Current first-party tools include:

- Access Your Information;
- Activity Log;
- Download Your Information;
- Transfer a Copy of Your Information;
- Accounts Center information controls;
- additional downloadable "data logs" describing product usage and recommendations.

These tools expose a broad range of categories and are useful for recovery, inspection, and historical export.

Facebook also has a documented Graph API. Current Meta permissions/reference material still exposes selected personal-profile permissions such as:

- `user_posts`;
- `user_photos`;
- `user_videos`;
- basic profile/login-related data.

But Graph API permissions are granular, use-case constrained, and can require App Review, Business Verification/Advanced Access, or periodic data-use review depending on the requested capability and audience. The Graph API is not a generic "give me everything in my own Facebook account" interface.

The first-party Facebook web application itself uses an extensive private Relay/GraphQL request surface under paths such as `/api/graphql`, with operation/document IDs and session/CSRF state. Public technical evidence confirms that this private protocol carries functionality unavailable through a simple public self-data API.

Meta's current anti-scraping posture is exceptionally explicit. Meta Help defines scraping as automated collection from websites/interfaces intended for people, Meta maintains dedicated anti-scraping systems, and its Automated Data Collection Terms require express written permission for automated collection.

## Why provisional Tier C

Positive factors:

- unusually broad human-facing data access tools;
- selectable account download/export;
- direct transfer tools;
- some supported personal-profile Graph API reads;
- stable Facebook object/user IDs exist across official/private interfaces.

Adversarial factors:

- no broad free general self-data API representing the whole account;
- Graph API access is permission/use-case/review constrained;
- several important self-data classes are primarily exposed through first-party interfaces/export rather than ordinary developer endpoints;
- exports are user-initiated snapshots, not a continuous sync channel;
- first-party private protocol is undocumented and build-sensitive;
- Meta aggressively restricts/blocks unauthorized automated collection from the website.

This makes Facebook better than a surface with no portability tools, but substantially worse than a Tier A protocol designed for independent user-controlled synchronization.

## Data portability is strong but not synchronization

Facebook should receive credit for serious portability work.

Current Help/Meta materials show that users can:

- access broad information categories in Accounts Center;
- select categories and date ranges for download;
- transfer certain information to external services;
- access Activity Log history;
- receive additional data logs in Download Your Information.

However, a periodic user-generated archive is still not equivalent to a live self-data protocol.

Sociarium should treat export as a **strong bootstrap/recovery acquisition source**, not as permission to make the human principal a recurring manual synchronization process.

## Implementation recommendation

**Do not implement Facebook yet.**

Research should continue with:

1. direct current Graph API capability validation for an app used only by its own developer/operator;
2. current App Review/Advanced Access requirements per desired self-data class;
3. a real current Download Your Information package, kept private, to map exact formats and stable IDs;
4. direct observation of first-party Facebook web reads for the operator's own account;
5. explicit policy/operational analysis before any private-protocol automation.

Facebook is a strong candidate for a future **export importer** if a current package proves structurally rich. A live official Graph API adapter may be worthwhile for the subset of personal-profile data Meta actually permits. Private-web automation is not currently admitted.

## Files

- [Access matrix](access-matrix.md)
- [Official APIs](official-api.md)
- [Access/export tools](export-and-access-tools.md)
- [Private first-party protocol](private-protocol.md)
- [Terms and constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
