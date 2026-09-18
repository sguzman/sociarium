# X browser capture protocol

Purpose: collect bounded, private first-party browser evidence for R2 without turning the observation phase into automation.

## Browser setup

Use the normal browser/account session you already use for X.

Open Developer Tools and select **Network**.

Recommended settings:

- enable **Preserve log**;
- disable cache only if you deliberately want a clean reload and note that choice in metadata;
- clear the network log immediately before each observation;
- do one interaction family per capture.

Do not use third-party request-replay extensions or scripts for the baseline observation.

## Capture A: own profile timeline

1. Navigate to your own X profile.
2. Open Network.
3. Clear the log.
4. Reload the profile.
5. Wait for the initial profile/timeline render.
6. Scroll slowly until at least 3 additional timeline fetches occur.
7. Open one authored Post detail/thread.
8. Wait for requests to settle.
9. Stop interacting.
10. Save/export the HAR **to a private local folder only**.

This capture should let us identify:

- own-user lookup operation;
- initial UserTweets/timeline operation;
- pagination requests/cursors;
- Post detail/thread operation;
- user/Post stable IDs;
- current long-form Post fields;
- reply/quote/repost relation representation;
- auth/proof header names.

## Capture B: query-ID reload comparison

Repeat Capture A's initial profile reload once more in a second clean network log.

Do not need to scroll deeply.

Goal:

- compare operation names;
- compare query IDs;
- compare feature flags;
- compare build/revision hints.

Save as a second private HAR.

## Capture C onward: one self-data view per file

Use separate captures for:

- Likes — current observed UI location: **History → Likes** (`/i/history/likes`);
- Bookmarks — current History UI places Bookmarks beside Likes; capture separately;
- Followers;
- Following;
- Lists;
- Mentions/Notifications.

For each:

1. clear log;
2. navigate to that view;
3. trigger one or two pagination fetches if available;
4. stop;
5. save private HAR.

Do **not** include Direct Messages in these baseline captures.

## Private local naming

Suggested:

```text
x-r2/
  2026-09-18-profile-a.har
  2026-09-18-profile-b.har
  2026-09-18-likes.har
  2026-09-18-bookmarks.har
  ...
```

This directory is not part of the public Sociarium repository.

## Hashing

Optional but useful for provenance:

PowerShell:

```powershell
Get-FileHash .\2026-09-18-profile-a.har -Algorithm SHA256
```

Only the hash may be copied into a public observation note.

## What you can safely give ChatGPT

Best option for analysis is to attach the raw HAR **in the private chat** rather than paste its contents into GitHub or public docs.

Even then, treat it as sensitive account evidence.

**Important observed behavior:** an Edge 153 sanitized HAR captured on 2026-09-18 still retained non-empty `x-csrf-token` values. A browser-provided "sanitized" export is therefore still private and must not be published or committed as-is.

If you prefer to sanitize locally first, remove at minimum:

- `Cookie` request headers;
- `Set-Cookie` response headers;
- `Authorization` request headers;
- `x-csrf-token` values;
- any OAuth/token/session values;
- private response payloads unrelated to the research target.

But aggressive manual sanitization can accidentally destroy useful structure, so private-chat attachment plus careful extraction may be preferable to hand-editing a large HAR.

## What we are not doing

This protocol does not ask the operator to:

- replay requests;
- fabricate tokens;
- bypass Turnstile/captcha/anti-bot systems;
- defeat rate limits;
- write scraping code;
- automate website interaction;
- capture another person's private data.

R2 is first-party observation.
