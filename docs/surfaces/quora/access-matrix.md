# Quora access matrix

Status: baseline desk research, 2026-09-18.

| Data class | Official developer API | First-party user UI | Official archive | Public web | Private first-party protocol | Assessment |
| --- | --- | --- | --- | --- | --- | --- |
| account/profile | no general self-data API identified | profile/settings | content + personal data archive | public profile | public technical evidence exposes numeric `uid` and user queries | Tier C overall |
| asked questions | no supported general API identified | `Your Content` | expected as account content; exact schema pending sample | public if visible | current public technical evidence: `UserProfileQuestionsList_Questions_Query` | rich private access; no supported machine interface identified |
| answers | no supported general API identified | `Your Content` | expected as content | public if visible | `UserProfileAnswersMostRecent_RecentAnswers_Query` + `AnswerComponentBaseQuery` | private protocol appears deep |
| posts | no supported general API identified | `Your Content` | expected as content | public if visible | not yet mapped in current public research | unknown private op |
| comments | no supported general API identified | visible via content/activity surfaces | archive inclusion/schema unknown | public by context | `UserProfileEditsQuery` exposes recent comment/edit operations in public technical research | history may be server-windowed |
| question follows | no supported general API identified | `Your Content` includes followed questions | archive schema unknown | partly public/partly private | not yet mapped | privately followed questions are explicitly excluded from `Your Content` |
| topic follows | no supported general API identified | supported first-party UI | archive schema unknown | follows can influence public feed/profile behavior | not yet mapped | source-specific |
| people follows | no supported general API identified | supported first-party UI | archive schema unknown | profile relationship may be visible | not yet mapped | source-specific |
| upvotes | no general self-history API identified | first-party feed/content behavior; account history surface requires direct observation | archive inclusion unknown | individual public activity can influence feed/profile; exact visibility varies | activity/private query family likely; direct mapping pending | Tier C/unknown |
| Bookmarks | no supported API identified | dedicated Bookmarks page | archive inclusion unknown | private | private protocol pending | Tier C |
| private Messages | no supported API identified | dedicated Messages UI | archive inclusion unknown | private | protocol pending | Tier C |
| Spaces | no general API identified | first-party Space UI/admin tooling | archive relationship unknown | many Spaces public | protocol pending | source-specific |
| anonymous historical questions/answers | not account-linked by design | not shown in account-centric `Your Content` | should not be assumed recoverable | content itself may remain public | Quora says identity link is not retained | **not reconstructable as account-owned history from platform identity** |
| edit/activity history | no supported API identified | some object logs are user-visible | archive schema unknown | answer/question logs can be public in places | `UserProfileEditsQuery` publicly documented by reverse engineer | private protocol shows bounded recent activity in one tested account |

## Stable identifiers

Current public technical evidence shows numeric identifiers:

- `uid` — user;
- `qid` — question;
- `aid` — answer;
- `opid` — activity/edit operation.

GraphQL responses also include Relay-style encoded `id` values.

These look promising for Sociarium, but they remain **secondary technical evidence** until directly observed against the operator's own account and compared with the archive.

## Public URLs are not enough

Quora commonly exposes human-readable slugs in URLs.

Sociarium should not use slugs as authoritative object identity if numeric native IDs are available, because:

- titles can change;
- questions can be merged;
- answer URLs include mutable user/title presentation;
- community ownership complicates the relationship between asker and object.

## Historical completeness caveats

The current public reverse-engineering source observed:

- profile question/answer GraphQL pagination reaching many thousands of items;
- UI infinite scroll stopping earlier than the private GraphQL path in its test;
- activity-log/comment history stopping at a smaller server-side window for one tested user.

These are **dated observations from one external researcher**, not universal Quora limits.

Direct Sociarium observation is still required.
