# X access matrix

Status: baseline desk research, 2026-09-18.

Legend:

- **documented** — established from current X documentation/help;
- **public technical evidence** — documented by independent reverse-engineering projects, not yet directly observed by Sociarium;
- **unknown** — do not fill from memory or old assumptions.

| Data class | Official API | Official archive | Public web | Private first-party protocol | Incremental/live potential | Historical completeness |
| --- | --- | --- | --- | --- | --- | --- |
| profile/account | documented, billed User reads; `/2/users/me` and lookup endpoints exist | documented profile information | public profile rendering | public technical evidence for `UserByScreenName` / related account queries | API yes | archive broad; exact private-protocol depth n/a |
| authored posts | documented `GET /2/users/{id}/tweets`; qualifying self read is $0.001/resource | documented; X says archive includes posts / entire post history | profile UI documented up to 3,200 most recent posts | public technical evidence for `UserTweets` | API has pagination + `since_id`; private protocol pending direct verification | archive: entire history per X Help; API current lifetime ceiling unknown |
| replies / quotes / repost relations | API exposes post fields/expansions; exact current normalization details require verification | posts present; exact relationship fidelity in current archive unknown | rendered | likely in timeline/detail responses; direct verification pending | likely | unknown by source until sampled |
| mentions | documented `GET /2/users/{id}/mentions`; qualifies for Owned Read pricing | not explicitly enumerated in current archive help page | rendered in notifications/search contexts | likely; direct verification pending | API yes | unknown |
| likes | documented `GET /2/users/{id}/liked_tweets`; qualifying self read $0.001/resource | not explicitly enumerated in current archive help page | user-visible when available | public technical evidence for `Likes` | API yes | unknown |
| bookmarks | documented `GET /2/users/{id}/bookmarks` and folders; qualifying self read $0.001/resource | not explicitly enumerated in current archive help page | authenticated UI only | public technical evidence for `Bookmarks` and bookmark folders | API yes | unknown |
| followers | documented `GET /2/users/{id}/followers`; qualifying self read $0.001/resource | documented follower list | public profile count/list subject to visibility | public technical evidence for `Followers` | API yes | archive snapshot; private/API historical changes not inherently complete |
| following | documented `GET /2/users/{id}/following`; qualifying self read $0.001/resource | documented following list | public/profile dependent | public technical evidence for `Following` | API yes | archive snapshot |
| lists | documented owned/followed/membership/pinned list endpoints; several qualify for Owned Read pricing | documented Lists created/member/followed | partially public depending on list | public technical evidence for ownership/membership/timeline operations | API yes | archive snapshot; full history not expected |
| blocks / mutes | documented lookup endpoints; qualifying self reads listed in pricing page | X Data surfaces these elsewhere; exact archive inclusion not yet verified | private UI | private-protocol status not yet directly researched | API yes | snapshot semantics |
| Direct Messages | documented DM lookup endpoints; standard DM Event read is $0.010/resource | documented Direct Messages | not public | known first-party UI exists; protocol details pending dedicated observation | official API can read incrementally by conversations/events; private protocol TBD | archive likely strong historical snapshot; exact completeness pending sample |
| attached media | API can expose post media metadata/attachments and has media endpoints | documented attached images/videos/GIFs | rendered | likely carried in post/detail responses | depends on source | archive explicitly includes attached media |
| live notifications / activity | Activity API/webhooks documented and billed by delivered event | archive is snapshot, not live | first-party web is live | likely live channels/requests; dedicated observation pending | yes through official activity products; private path TBD | not historical by itself |

## Important distinctions

### The 3,200 number

X Help currently says the **profile timeline UI** shows up to 3,200 recent posts and recommends the archive for older history.

The current official user-post API reference exposes pagination, `start_time`, `end_time`, `since_id`, and `until_id`, but this research pass did not find a current official statement imposing the same 3,200 total-history ceiling on the API.

Therefore:

- **web profile UI:** documented 3,200 recent-post display bound;
- **archive:** documented entire post history;
- **official API lifetime ceiling:** unknown until current evidence establishes it.

Do not silently copy the web/UI limit into the API column.

### Cost is per acquisition source

A data class being technically available does not mean it is available under an acceptable acquisition relationship.

For example, bookmarks and own posts are well-supported by the official API but still metered. Their technical accessibility and their sovereignty/access tier are separate facts.
