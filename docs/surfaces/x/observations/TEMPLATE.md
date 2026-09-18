# X observation: <topic>

- Observation date/time UTC:
- Browser/version:
- X account context: self-owned authenticated account
- X web build/revision if visible:
- Local private capture filename:
- Local private capture SHA-256:
- Raw capture committed publicly: **no**
- Interaction type: read-only / mutating
- Related R2 issue: #12

## Operator action sequence

Describe only what the operator did in the first-party UI.

Example:

1. opened own profile;
2. reloaded page;
3. scrolled until three additional timeline requests occurred;
4. opened one authored Post.

## Direct observations

Only statements directly supported by the private capture belong here.

### Request family

- Method:
- Host/path pattern:
- Operation name:
- Query/document ID:
- Content type:
- Pagination request:

### Request structure

Record structural field/header names only.

Never record live values for cookies, Authorization, CSRF, session, or proof tokens.

#### Query/variables

- ...

#### Feature/field toggles

- ...

#### Auth/session-related names observed

- ...

### Response structure

- top-level envelope:
- user ID field:
- Post ID field:
- Post text field:
- long-form/note representation:
- reply relation:
- quote relation:
- repost relation:
- media:
- pagination cursor:
- terminal-cursor behavior:

## Pagination observations

Record:

- page/cursor direction;
- cursor location;
- whether the next request reuses the same operation;
- whether older pages preserve the same response shape;
- bounded number of pages observed;
- any visible hard stop.

Do not infer a global history maximum from a small bounded sample.

## Query-ID/build volatility

If the same action was repeated after reload:

- same operation name?
- same query ID?
- same feature set?
- same response schema?

## Confirmed baseline claims

List desk-research claims strengthened by this observation.

## Contradicted baseline claims

List any prior claims that need correction.

Do not silently edit away the old claim without preserving the dated correction.

## Inferences

Clearly label interpretations that are not directly observed.

## Unknowns

List unresolved questions.

## Public-safety review

Before commit confirm:

- [ ] no cookies;
- [ ] no Authorization values;
- [ ] no CSRF/token values;
- [ ] no session IDs;
- [ ] no private message/bookmark/notification bodies;
- [ ] no full HAR;
- [ ] only sanitized public/self-owned identifiers remain.
