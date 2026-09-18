# Reddit first-party private web protocol

Research date: 2026-09-18.

Evidence class: **public technical/network observations**, not yet direct Sociarium observation.

## Current observed family: Shreddit

Recent public browser-console/network reports show Reddit's current web application using:

`POST https://www.reddit.com/svc/shreddit/graphql`

Observed payloads use fields shaped like:

- `operation`;
- `variables`;
- `csrf_token`.

Public 2026 examples include authenticated operations such as joining a community and updating subreddit settings/widgets.

Other public technical work documents additional `/svc/shreddit/*` endpoints used by the modern web frontend for pagination and comment loading. Some of these return HTML fragments containing `<shreddit-*>` web components rather than a conventional JSON API response.

## Conflicting historical/current GraphQL evidence

Older and third-party projects also reference `gql.reddit.com`.

The baseline evidence is not strong enough to declare one universal current Reddit GraphQL origin. Recent first-party browser reports are stronger evidence for `/svc/shreddit/graphql`, while other protocol families may coexist.

Direct observation should determine:

- which host/path the current logged-in web client uses by data class;
- whether `gql.reddit.com` remains active for any first-party paths;
- whether GraphQL, REST-like Shreddit endpoints, and HTML-fragment endpoints coexist.

## Authentication shape

Public observations indicate the Shreddit web protocol relies on the ordinary logged-in browser session plus CSRF protection.

Do not commit:

- Reddit session cookies;
- CSRF token values;
- Authorization headers;
- private response bodies.

The dossier should record header/field **names and semantics**, not live authority.

## Why the private protocol matters

Reddit's own Devvit documentation says Developer Platform apps cannot access several private user data classes that the first-party web UI can display, such as saved content and vote history.

That makes the first-party protocol relevant to understanding the real surface even if Sociarium never automates it.

## Policy boundary

Reddit's current User Agreement prohibits scraping the Services without Reddit's prior written consent and limits automated or other collection to what is permitted by the Terms or a separate agreement.

The Developer Terms likewise prohibit circumventing technical/security mechanisms and require use of authorized access information for Developer Services.

Therefore:

- **observe/document first-party behavior:** legitimate research target for Sociarium;
- **automated private-protocol acquisition:** not implementation-admitted;
- **bypassing anti-bot/security controls:** outside ordinary Sociarium acquisition doctrine.

## Current direct unknowns

Sociarium has not yet directly established:

- operation names used for own profile/activity pages;
- Saved data request structure;
- up/downvote history request structure;
- subscriptions/follows request structure;
- pagination cursors;
- practical history depth;
- private message/Chat protocol families;
- rate-limit/error behavior;
- anti-automation/proof mechanisms;
- whether the same operation shape survives site releases.

These should be resolved by bounded browser observation rather than guesswork.
