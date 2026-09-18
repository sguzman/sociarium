# YouTube access matrix

Status: baseline desk research, 2026-09-18.

| Data class | Data API v3 | Takeout/account tools | Public web | Innertube/first-party protocol | Preservation note |
| --- | --- | --- | --- | --- | --- |
| channel/profile | documented `channels.list`, including authenticated `mine` patterns | export/account data | public | yes | supported API is workable |
| uploaded videos metadata | uploads playlist + videos resources | Takeout can export YouTube videos | public/unlisted/private according to authorization | yes | API metadata refresh rules; raw media backup via API restricted |
| uploaded audiovisual files | API upload/management exists but developer policy forbids download/backup/cache without prior written approval | Google Takeout explicitly supports YouTube video export | public playback according to visibility | player/GVS private protocol | Takeout is the appropriate self-backup path |
| playlists | `playlists.list mine=true`; playlistItems | Takeout | public/private according to account | yes | Tier B |
| subscriptions | `subscriptions.list mine=true` via OAuth | account/Takeout data likely available; exact current package schema pending sample | visibility can be private | yes | Tier B |
| likes | OAuth ratings API; liked-videos playlist; `videos.list myRating=like` | account/Takeout | private/account-dependent | yes | `myRating` API pagination caps at most recent 1000; liked playlist path should be separately validated |
| dislikes | `myRating=dislike` supported but returned set is capped at most recent 1000 | Takeout/account data needs sample | private | yes | bounded API history |
| own comments | comments APIs are object/video/thread centric; no universal "all comments I ever made" endpoint established | Your Data/Takeout are stronger self-history sources | individual public comments | yes | Tier C-ish as complete self-history |
| channel activity | `activities.list mine=true` | account tools | partly public | yes | activity resource is not equivalent to complete personal activity history |
| Watch History | **not exposed as current related playlist API; old property removed** | Google My Activity / Takeout | private UI | first-party protocol | Tier C data class |
| Watch Later | old API related-playlist property removed | first-party account/Takeout path requires current package verification | private UI | first-party protocol | Tier C data class |
| search history | no general Data API endpoint | My Activity / Takeout | private | first-party protocol | Tier C data class |
| recommendations/home feed | no supported self-archive API | some recommendation/activity data may appear in Google account exports/activity | logged-in UI | first-party protocol | derived/private state |
| notifications | no general archival Data API | account tools uncertain | logged-in UI | first-party protocol/live systems | unknown/Tier C |
| purchases/memberships | not ordinary Data API social object set | Your Data in YouTube exposes purchased-video controls; Google account tools | private | first-party systems | source-specific |
| live chat | dedicated liveChatMessages API for authorized live broadcasts | Takeout completeness needs sample | public/private context | first-party protocol | event-specific, not general DM system |

## Stable identifiers

YouTube provides stable-looking typed resource identifiers for:

- channels;
- videos;
- playlists;
- playlist items;
- comments/comment threads;
- subscriptions;
- activities.

Sociarium should still distinguish:

- canonical video/channel identity;
- playlist-item membership identity;
- mutable metadata/title/handle-like presentation fields.

## Likes caveat

`videos.list?myRating=like` is documented to return at most the most recent **1000** matching videos even if totalResults is larger.

YouTube also exposes a channel's special Liked Videos playlist ID, retrievable through channel content details, which can be traversed with `playlistItems.list`.

A future live validation should determine whether the playlist path provides deeper liked-video history than the `myRating` path and whether account/UI limits impose another ceiling.

## History caveat

The current Data API is not a complete record of "everything I did on YouTube."

That distinction is central to the Tier B classification.
