# YouTube Data API v3

Research date: 2026-09-18.

Evidence class: **official documented fact** unless marked otherwise.

## Quota model

YouTube Data API access uses a quota-unit system.

Current developer documentation says newly enabled projects receive a default allocation equivalent to:

- 100 `search.list` calls;
- 100 `videos.insert` calls;
- **10,000 units per day combined for all other endpoints**.

The default quota is subject to change.

Developers who need more can request a quota extension/audit.

This is not ordinary pay-per-resource billing.

## Read costs

Many list/read endpoints cost only 1 quota unit.

Search is comparatively expensive.

Quota cost applies even to invalid requests.

For a personal local corpus that uses targeted authenticated list endpoints rather than broad search, the default quota is potentially generous.

## OAuth/self-data patterns

Supported endpoints expose authenticated self-data through parameters such as:

- `mine=true`;
- user authorization scopes such as `youtube.readonly`;
- authenticated rating/filter operations.

Examples:

### Channel activities

`activities.list?mine=true`

Returns activity events associated with the authenticated user's channel.

### Subscriptions

`subscriptions.list?mine=true`

Returns channels to which the authenticated user is subscribed.

### Playlists

`playlists.list` can retrieve playlists owned by the authenticated user.

Special channel playlists include:

- uploads;
- liked videos.

### Ratings

YouTube supports:

- `videos.getRating` for the authenticated user's rating on specified videos;
- `videos.list?myRating=like|dislike`;
- `videos.rate` for writes.

The `myRating` list route is explicitly capped at the most recent 1000 videos.

## Watch History / Watch Later gap

YouTube's API revision history records that:

- `contentDetails.relatedPlaylists.watchHistory`;
- `contentDetails.relatedPlaylists.watchLater`;

were deprecated and subsequently removed.

Therefore a current Data API client cannot simply traverse those old special playlists.

This is an important difference between creator/social-channel data and private consumption history.

## Comments

The Data API has comment and comment-thread resources, but this baseline did not find a single current endpoint meaning:

> return every comment the authenticated user has ever authored across YouTube.

For complete self-history, Takeout/account activity may be the more appropriate source.

## API-data storage rules

The most important architectural constraint is in YouTube's Developer Policies.

Most Authorized Data that does not fall into specific long-lived categories may be stored for no more than 30 calendar days before it must be **deleted or refreshed**.

Non-authorized API data has a similar 30-day limit.

Clients must make reasonable efforts to keep stored API data consistent with YouTube's current state.

YouTube permits display of historical API Data if accurately contextualized in time, but this does not eliminate the storage/refresh obligations.

This policy needs a dedicated Sociarium interpretation before an adapter writes immutable raw API snapshots forever.

## Audiovisual content

YouTube's API developer policy prohibits API clients from downloading/importing/backing up/caching copies of YouTube audiovisual content without prior written approval.

Therefore:

- metadata API != media-backup API;
- Google Takeout is the supported self-backup route for the user's uploaded video files.

## Unsupported/private API restriction

Current Developer Policies explicitly say:

- do not use undocumented APIs without express permission;
- do not reverse engineer undocumented YouTube API services;
- do not scrape YouTube/Google applications;
- do not use technology other than YouTube API Services to retrieve YouTube API Data.

This makes Innertube research useful descriptively but not automatically implementation-admissible.
