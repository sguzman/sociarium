# X auth/session boundary

Research date: 2026-09-18.

Purpose: document the current authenticated X Web session boundary **by structural name only** without storing credential values.

This note deliberately separates direct Sociarium observation from secondary technical corroboration.

## Directly observed in Sociarium browser captures

Microsoft Edge 153 sanitized HAR captures from the operator's own authenticated X session directly exposed these X-specific request-header names on first-party requests:

- `x-client-transaction-id`
- `x-csrf-token`
- `x-twitter-active-user`
- `x-twitter-auth-type`
- `x-twitter-client-language`

The captures also establish that these names accompany the same GraphQL request family used for profile, timeline, relationship, History, Lists, Notifications, and other observed reads.

No live values are recorded here.

## What Edge sanitization removes

Edge's sanitized HAR export removed:

- the ordinary request `Cookie` header;
- the ordinary request `Authorization` header;
- the request-cookie array itself.

Therefore the sanitized HAR cannot directly reveal the browser cookie names or the web-client Authorization value.

At the same time, the sanitized HAR **did retain non-empty `x-csrf-token` values**. This is why even sanitized HAR files remain private evidence and are never committed.

## Current secondary corroboration

Multiple current independent technical implementations of the X web protocol agree on the following logged-in browser-session names:

- `auth_token` — session/authentication cookie;
- `ct0` — CSRF cookie;
- `x-csrf-token` — header carrying the CSRF value corresponding to `ct0`;
- `Authorization` — web-client bearer/app-identity header;
- `x-twitter-auth-type` — X-specific auth-mode header;
- `x-client-transaction-id` — per-request client transaction/proof identifier.

Current corroborating sources:

- `mudrii/gobird`, `docs/wire-protocol.md`;
- `alarok/x-agent-sdk`, README credential/auth documentation.

These are **secondary evidence**, not direct Sociarium observation of the stripped cookie/header names.

## Boundary model

The baseline authenticated request model is therefore:

```text
browser session
  -> Cookie names: auth_token + ct0        [secondary corroboration]
  -> Authorization header                 [secondary corroboration]
  -> x-csrf-token                         [direct name observation]
  -> x-twitter-auth-type                  [direct name observation]
  -> x-twitter-active-user                [direct name observation]
  -> x-twitter-client-language            [direct name observation]
  -> x-client-transaction-id              [direct name observation]
  -> X first-party request
```

This is a **name-level boundary map**, not a credential recipe. Sociarium does not store or publish live cookie, bearer, CSRF, session, or proof values.

## What is not claimed

This baseline does **not** claim:

- that only `auth_token` and `ct0` ever exist in an X browser cookie jar;
- that every X endpoint requires the identical subset of session material;
- that the bearer/app-identity value is immutable;
- that the transaction-ID algorithm or proof requirements are stable;
- that a sanitized HAR is safe to publish;
- that documenting the boundary authorizes replay or automation.

It records the minimum current boundary needed to understand the observed first-party client.

## R2 completion interpretation

The R2 gate asks for current auth/session header/cookie **names** without values.

That gate is satisfied at the documentation level because:

1. the X-specific header names are directly observed;
2. Edge's sanitization behavior and its evidentiary limitation are explicitly recorded;
3. the stripped cookie/header names are independently corroborated by multiple current technical sources;
4. no credential values are exposed.

The direct-observation limitation remains visible rather than being silently erased.

## Implementation status

This boundary map does **not** admit a private-protocol client.

The X dossier's implementation recommendation remains documentation-only for the private protocol under current X Terms/Automation Rules unless a later I0 decision or explicit human-principal override changes that status.
