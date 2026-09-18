# YouTube first-party private protocol (Innertube)

Research date: 2026-09-18.

Evidence class: **public technical evidence**, pending direct Sociarium observation.

## Innertube

YouTube's web/mobile/TV clients use an internal API ecosystem commonly called **Innertube**.

Public open-source clients and extractors document endpoint families under paths conceptually like:

- `/youtubei/v1/browse`;
- `/youtubei/v1/player`;
- `/youtubei/v1/next`;
- search/navigation/account-specific request families.

Requests carry a client context identifying a YouTube client name/version and other environment state.

## Continuations

First-party feeds/playlists/search results commonly use opaque continuation tokens rather than the Data API's public `pageToken` contract.

Public projects document substantial variation across web/mobile/TV clients.

## Client plurality

YouTube ships many first-party client identities, including Web, mobile web, Android, iOS, TV, creator and other variants.

Current yt-dlp source tracks numerous `INNERTUBE_CONTEXT` client configurations and has to update versions/behavior as YouTube changes them.

This is protocol archaeology, not a stable published interface.

## Proof of Origin tokens

YouTube has increasingly deployed **PO Tokens** / client attestation for some playback/subtitle/resource flows.

Current public technical documentation from yt-dlp describes enforcement varying by client and request class.

This is strong evidence that YouTube actively differentiates genuine/authorized client behavior from generic replay.

Sociarium should not treat finding a working internal client identity as a durable acquisition contract.

## Logged-in state

Authenticated first-party requests can involve:

- browser cookies/session authority;
- account/context identifiers;
- client version/context;
- anti-abuse/proof state.

None of those live values belong in public project documentation.

## Why Innertube matters to Sociarium

Important user-visible state exists outside the Data API, including:

- Watch History;
- Watch Later;
- search/activity state;
- home/recommendation feeds;
- notifications;
- some private account views.

Innertube observation can answer:

> How does the first-party client actually retrieve the data that the supported API omits?

That is useful documentation even if no replay client is ever built.

## Policy boundary

YouTube's Terms prohibit automated access such as robots/botnets/scrapers without prior written permission (apart from public-search-engine robots.txt access).

YouTube API Developer Policies additionally prohibit undocumented API use/reverse engineering and scraping by API clients.

Therefore:

- direct browser observation: research target;
- sanitized protocol documentation: research target;
- private Innertube acquisition adapter: **not admitted**;
- bypassing PO-token/client-attestation/security systems: outside ordinary Sociarium acquisition doctrine.

## Current unknowns

Direct observation should establish:

- exact current endpoint families for Watch History and Watch Later;
- history/search pagination depth;
- stable ID presence;
- deletion handling;
- continuation semantics;
- account/channel identity context;
- current auth/session header names;
- which requests require PO/client proof;
- whether Google My Activity uses separate Google account APIs from YouTube Innertube.
