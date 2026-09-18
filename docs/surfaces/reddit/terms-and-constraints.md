# Reddit terms, retention, and operational constraints

Research date: 2026-09-18.

This is project/technical research, not legal advice.

## User Agreement: collection/scraping

Reddit's current July 1, 2026 User Agreement for users outside the EEA/UK/Switzerland restricts accessing, searching, or collecting data from the Services by automated or other means except as permitted by the Terms or a separate agreement.

It conditionally permits crawling according to `robots.txt` while stating that scraping without Reddit's prior written consent is prohibited.

The Agreement also restricts reverse engineering except where such restrictions are impermissible under applicable law.

## Developer/Data API access

Current Data API Terms require:

- accurate registration/contact information;
- authorized Access Info such as OAuth;
- no masking of OAuth identity/User-Agent;
- compliance with API limitations;
- no circumvention of technical restrictions;
- use/retention only within the approved use case.

Reddit reserves the right to charge future Data API fees and can change/suspend/discontinue API access.

## Deletion and retention conflict

This is the most important Sociarium constraint.

Current Data API guidance says:

- delete locally held post/comment content if it is deleted from Reddit;
- after account deletion, delete related user IDs and author-identifying information;
- retaining deleted content even anonymized violates Reddit policies;
- routinely deleting stored user data/content within 48 hours is strongly recommended.

Current Developer Terms additionally require deleting or modifying User Content when it is deleted, protected, suspended, withheld, modified, or removed, and require deletion of locally held Reddit Services/Data in several circumstances.

### Sociarium conflict

Sociarium's corpus model treats an observation as historical evidence that may remain valuable after the remote object changes or disappears.

Reddit's developer-data rules instead require substantial remote-deletion propagation into local holdings.

Those are different authority models.

A future Reddit adapter cannot silently pretend they are compatible. Implementation admission must decide whether:

- Sociarium accepts destructive deletion propagation for Reddit API-derived evidence;
- self-owned data has a separately defensible preservation basis outside the developer API;
- export/another source has different retention conditions;
- Reddit remains documentation-only.

## Data API transition risk

Reddit announced in August 2026 that it plans to gradually restrict new public/Data API requests and move third-party applications toward Devvit.

That is a current platform-direction risk, not merely a historical concern.

## Commercial/research restrictions

Reddit says commercial developer use requires permission/contract.

Reddit also says formal academic research using Reddit data must use the Reddit For Researchers program.

Sociarium should always describe its actual use case truthfully rather than selecting a category for convenience.

## Current conclusion

Reddit is technically more open than X's pay-per-read developer interface today, but its **retention authority and platform migration direction** are materially adverse to a durable personal corpus.

That supports the current provisional Tier C classification despite a workable free Data API path for some data classes.
