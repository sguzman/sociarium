# YouTube source ledger

Last reviewed: 2026-09-18.

## Official YouTube / Google sources

### YouTube Data API overview

https://developers.google.com/youtube/v3/getting-started

Used for:

- API model;
- default quota allocation;
- quota-extension process.

### YouTube Data API reference

https://developers.google.com/youtube/v3/docs

Used for endpoint/resource inventory.

### Videos: list

https://developers.google.com/youtube/v3/docs/videos/list

Used for:

- `myRating=like|dislike`;
- maximum 1000 returned rated videos;
- video field access.

### Activities: list

https://developers.google.com/youtube/v3/docs/activities/list

Used for:

- authenticated `mine=true` channel activity;
- pagination/time filters.

### Playlists / Ratings implementation guides

https://developers.google.com/youtube/v3/docs/playlists

https://developers.google.com/youtube/v3/guides/implementation/ratings

Used for:

- owned playlists;
- special uploads/liked playlists;
- rating retrieval.

### Subscriptions implementation guide

https://developers.google.com/youtube/v3/guides/implementation/subscriptions

Used for authenticated subscription listing.

### API revision history

https://developers.google.com/youtube/v3/revision_history

Used for the historical removal/deprecation of:

- Watch History special playlist API property;
- Watch Later special playlist API property.

### YouTube API Services Developer Policies

https://developers.google.com/youtube/terms/developer-policies

Used for:

- 30-day refresh/delete rules;
- revocation deletion;
- current-state consistency;
- historical-display allowance;
- audiovisual-content backup restrictions;
- undocumented API/reverse-engineering prohibition;
- scraping prohibition.

### YouTube API Services Terms

https://developers.google.com/youtube/terms/api-services-terms-of-service

Used for API-project/termination/data-deletion obligations.

### YouTube Terms of Service

https://www.youtube.com/static?template=terms

Used for automated-access/scraping restrictions and general service boundaries.

### Google Takeout / Download your Google data

https://support.google.com/accounts/answer/3024190

Used for:

- user export/archive mechanism;
- explicit YouTube-video export;
- product/account activity export;
- Brand Account caution.

### Your Data in YouTube

https://support.google.com/youtube/answer/9315727

Used for first-party account-data/control categories.

## Public technical evidence for Innertube

### yt-dlp

https://github.com/yt-dlp/yt-dlp

Current source/wiki used for:

- multiple Innertube client contexts;
- client-version drift;
- Proof-of-Origin token enforcement;
- account/cookie operational risk;
- continuations/client behavior.

### YouTube.js

https://github.com/LuanRT/YouTube.js

Current open-source Innertube client used as secondary evidence for browse/player/navigation operation families.

### YouTube Internal Clients research

https://github.com/zerodytrash/YouTube-Internal-Clients

Historical/public technical reference for Innertube client identities and request structure.

## Evidence hierarchy

1. direct Sociarium observation;
2. current official YouTube Data API/Terms/Help;
3. current Google Takeout/My Activity documentation;
4. multiple current mature open-source Innertube clients;
5. single reverse-engineering references;
6. historical memory.

Private protocol details must remain dated because YouTube actively changes client requirements.
