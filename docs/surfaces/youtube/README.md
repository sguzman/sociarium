# YouTube surface dossier

- Surface: YouTube
- Operator/company: Google LLC / YouTube
- Canonical domain: `youtube.com`
- Research started: 2026-09-18
- Last reviewed: 2026-09-18
- Operator priority: unranked relative to the non-X inventory
- Overall access tier: **provisional Tier B — Workable**
- Important exceptions: private consumption history and API-data preservation have Tier C characteristics
- Confidence: high for official API/quota/policy facts; medium-high for Google Takeout breadth; medium for undocumented Innertube details pending direct observation

## Executive summary

YouTube is materially more workable than X or Facebook for supported machine access, but it is not fully sovereignty-friendly.

The official **YouTube Data API v3** provides a substantial OAuth-enabled interface for:

- channels;
- uploaded videos;
- playlists and playlist items;
- subscriptions;
- likes/ratings;
- comments/comment threads;
- channel activities;
- many creator/channel management operations.

Google currently gives enabled API projects a default **10,000-unit daily quota** for ordinary Data API use. Many list/read operations cost only 1 unit. This is a real free machine interface, not pay-per-resource billing.

Google Takeout provides a first-party archive mechanism for YouTube/Google account data, including uploaded YouTube videos and account/activity data. "Your Data in YouTube" and Google My Activity provide additional human-facing access to private activity/history.

However, the Data API does **not** expose every important self-data class. Notably, YouTube removed the old Watch History and Watch Later related-playlist API properties years ago. Current private viewing/search/recommendation state is therefore not represented as a general supported Data API feed.

YouTube's developer policies also create a preservation conflict: most Authorized API Data must be deleted or refreshed within 30 days, non-authorized API data has similar limits, API clients must keep stored data consistent with current YouTube state, and undocumented API access/reverse engineering/scraping is explicitly prohibited.

The first-party clients use a rich internal protocol commonly called **Innertube**. Public open-source projects document `youtubei` endpoint families, client contexts, continuations, and increasingly complex Proof-of-Origin (PO) token enforcement. That protocol is important forensic evidence, but current YouTube terms/API policies make it a poor candidate for a Sociarium live backend without a separate deliberate review.

## Why Tier B overall

Positive factors:

- free useful documented API quota;
- OAuth for user-authorized self-data;
- stable channel/video/playlist/comment IDs;
- broad creator/channel object model;
- standard pagination;
- first-party Google Takeout portability;
- no requirement to reverse engineer the website for ordinary channel data.

Tier C-like factors:

- watch/search/private consumption history is not generally exposed through the Data API;
- Watch History/Watch Later API surfaces were removed;
- most stored API data has refresh/delete requirements inconsistent with immutable raw-evidence preservation;
- audiovisual-content backup via API is restricted;
- undocumented API use and scraping are explicitly prohibited;
- private Innertube behavior is actively hardened with client/version and PO-token mechanisms.

This makes the supported channel-oriented API **Workable**, while some personal-history acquisition remains adversarial.

## Authority split

For Sociarium, YouTube has at least four distinct acquisition/authority surfaces:

1. **YouTube Data API** — supported structured social/channel data.
2. **Google Takeout** — historical/account portability snapshot.
3. **Google My Activity / Your Data in YouTube** — human-facing consumption/activity history.
4. **Innertube / first-party client protocol** — undocumented live client interface.

These must not be collapsed into one "YouTube API."

## Implementation recommendation

A future official YouTube adapter is **implementation-admissible in principle** for data classes that fit the Data API and its storage rules.

But before coding, Sociarium should decide an architectural policy for YouTube API data:

- Does the corpus store only refreshable current state?
- Can historical observations be stored under the API policy if refreshed/current state is separately maintained?
- Which evidence can come from Takeout instead, under a different portability relationship?
- Should raw API response bodies be treated differently from normalized historical observations?

For watch/search history, research should prioritize Takeout/My Activity and direct first-party observation rather than pretending the Data API is complete.

## Files

- [Access matrix](access-matrix.md)
- [Official Data API](official-api.md)
- [Takeout and account data](export-and-account-data.md)
- [Private first-party Innertube protocol](private-protocol.md)
- [Terms and retention constraints](terms-and-constraints.md)
- [Direct observation plan](observation-plan.md)
- [Source ledger](sources.md)
