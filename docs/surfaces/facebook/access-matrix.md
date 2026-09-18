# Facebook access matrix

Status: baseline desk research, 2026-09-18.

| Data class | Graph API | Access/Activity Log | Download/transfer | Public web | Private first-party protocol | Current assessment |
| --- | --- | --- | --- | --- | --- | --- |
| profile/basic identity | basic profile/login APIs exist | yes | yes | partly public by privacy settings | yes | workable but developer scope-limited |
| authored timeline posts | `user_posts` permission exists with specific allowed uses | Activity Log / Access Your Information | Download Your Information; transfer tools cover some content classes | only according to audience/privacy | extensive Relay/GraphQL evidence | Tier B/C by source |
| comments/reactions | no single broad personal-history API established in this baseline | Activity Log explicitly covers comments/likes and other activity | current download tools expose selectable information categories; exact 2026 package schema pending sample | privacy/context dependent | first-party client necessarily queries them | Tier C overall |
| photos | `/{user-id}/photos` with `user_photos` | yes | download/transfer supported | privacy dependent | yes | Tier B/C |
| videos/reels | `user_videos` exists for uploaded videos; Reels/private-profile details need validation | yes | download/transfer supported | privacy dependent | yes | Tier B/C |
| friends/connections/following | public/login APIs expose limited connection concepts; modern friend access is not a generic full social-graph API | Access Your Information has Connections category | download likely contains connection-related data; exact package fields pending sample | public visibility limited | yes | Tier C |
| likes/interests/page follows | broad personal historical API not established | Activity Log / preferences/access tools | downloadable account categories include interaction information | partly visible | yes | Tier C |
| saved items | no supported broad Graph API path established | user-facing Saved/Activity features | package inclusion needs current sample | private | yes | Tier C |
| search/activity history | no ordinary Graph API history endpoint | Activity Log exposes searches and activity | data logs/download tools include significant usage information | private | yes | Tier C |
| groups/group activity | Graph API is role/permission/product constrained | Access Your Information has group activity tools | downloadable categories | groups may be public/private | yes | source-specific |
| messages/Messenger | Messenger Platform is primarily Page/business messaging, not a generic personal-inbox export API | Messenger/Facebook first-party UI | Download Your Information historically/currently covers messaging data; exact 2026 package split needs sample | private | extensive internal protocol | Tier C |
| security/login/IP/device data | not general social Graph API | Accounts Center/security tools | downloadable/access categories include security/login/logged information | private | settings protocols | export/access only |
| ads/inference/view logs | not general self-data Graph API | Access Your Information / ad preferences | Download Your Information includes account/inference/data-log categories | private | first-party systems | human-facing access strong; programmatic access weak |

## Granularity matters

Facebook is not "closed" in the sense of hiding the user's information from the user.

The user-facing access layer is broad.

The adversarial classification comes from the gap between:

```text
what Facebook can show/export to the user
            versus
what an independent user-controlled program can continuously access through a supported machine interface
```

That gap is the central Facebook research problem for Sociarium.
