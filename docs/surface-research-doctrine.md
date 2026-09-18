# Surface research doctrine

Sociarium is a documentation-first digital-forensics and user-sovereignty project.

Its primary research question is:

> What mechanisms exist for a user to acquire, preserve, query, and continuously observe their own data on a social surface, and how much control does that surface retain over each mechanism?

Code is downstream of that question. A surface does not receive an adapter merely because the operator uses it.

## Research before implementation

Before new surface-specific implementation begins, Sociarium should first establish a dated, evidence-backed dossier for that surface.

The dossier should investigate, where applicable:

- documented public APIs;
- authenticated/self-data APIs;
- official archive/export mechanisms;
- public web representations;
- RSS/Atom or other feed mechanisms;
- undocumented first-party web application protocols;
- materially distinct mobile application protocols;
- WebSocket, SSE, push, or other live-update channels;
- stable profile and object identifiers;
- pagination and historical-window behavior;
- edit and deletion behavior;
- media access;
- follows/followers and other relationship data;
- likes/reactions;
- bookmarks/saves;
- lists/collections;
- direct/private messages where the operator is authorized to access them;
- authentication and credential boundaries;
- rate limits and anti-automation constraints;
- monetary cost and billing prerequisites;
- incremental synchronization possibilities;
- historical completeness;
- data portability;
- protocol stability and observed change frequency.

Research may conclude that a surface is not worth implementing. That is a successful research result.

## Evidence standard

Sociarium documentation distinguishes:

1. **Documented fact** — stated by the surface/operator in current public documentation.
2. **Observed behavior** — directly observed in a first-party client, export, network trace, or other reproducible artifact.
3. **Inference** — a conclusion drawn from documented or observed evidence.
4. **Unknown** — not yet established.

Private or undocumented protocol claims must be time-bounded. Prefer:

> Observed 2026-09-18: X Web issued operation Y with pagination field Z while viewing the authenticated user's own timeline.

over:

> X uses Y.

A protocol may change. Sociarium preserves the history of observations rather than silently rewriting old claims into timeless assertions.

Every important claim should record enough provenance to answer:

- when was this established?
- by what source or observation?
- under what account/client/context?
- what is directly known versus inferred?
- what could make the claim stale?

## Adversarial protocol archaeology

A platform's preferred developer interface is not the ontology of the platform.

When sanctioned interfaces materially obstruct access to the user's own data, the first-party application itself becomes a legitimate research object. Sociarium may document the protocol that the service actually ships to the authorized user, including:

- request families and endpoints;
- GraphQL operation names or equivalent RPC identifiers;
- request/response schemas;
- stable IDs;
- pagination cursors;
- feature flags;
- authentication boundaries;
- session lifetime behavior;
- live-update channels;
- error forms;
- protocol drift across observations.

This is **adversarial acquisition research**, not a euphemism for repeated manual groveling through crippled export flows.

Adversarial does not mean indiscriminate security bypass. Sociarium does not require:

- stealing another person's credentials or session material;
- bypassing meaningful authorization controls;
- defeating access controls to obtain data the operator is not authorized to access;
- committing passwords, bearer tokens, cookies, refresh tokens, or session secrets into the repository.

The useful research territory is deliberately broad: understanding what the operator's own authenticated first-party client sends and receives, and determining what can be reproduced safely and responsibly.

## Per-data-class analysis

A single site-wide tier is only a summary.

A surface may be cooperative for public posts but adversarial for bookmarks, messages, or historical data. Every dossier should therefore include an access matrix by data class and acquisition mechanism.

Example shape:

| Data class | Documented API | Export | Public web | Private first-party protocol | Incremental | Historical completeness |
| --- | --- | --- | --- | --- | --- | --- |
| authored posts | ? | ? | ? | ? | ? | ? |
| likes | ? | ? | ? | ? | ? | ? |
| bookmarks | ? | ? | ? | ? | ? | ? |
| relationships | ? | ? | ? | ? | ? | ? |
| messages | ? | ? | n/a | ? | ? | ? |

Unknowns should remain explicit rather than being filled by assumption.

## Secrets and captures

Research artifacts may contain sensitive material even when they are not credentials.

Do not commit raw browser HAR files, cookies, authorization headers, tokens, private messages, or other account-sensitive captures to the public repository.

Prefer sanitized structural observations:

- endpoint or operation name;
- request field names;
- response schema;
- pagination semantics;
- redacted sample shapes;
- hashes or local references to private captures when useful.

A private local evidence vault may be introduced later if the research program needs durable raw captures. Public documentation should remain safe to publish.

## Implementation admission gate

A new surface-specific adapter/importer should normally not begin until its dossier establishes:

- current access tier;
- per-data-class access matrix;
- authentication model;
- cost model;
- stable identity/object identifiers;
- history and pagination behavior;
- available acquisition sources;
- likely protocol volatility;
- known legal/operational constraints without pretending to provide legal advice;
- the specific acquisition source chosen for implementation and why.

The human principal may override this gate deliberately. Agents must not silently bypass it merely to keep coding.

## Research is a deliverable

The Surface Atlas is not disposable planning material.

Even if no adapter is ever written for a site, a high-quality dossier remains a useful Sociarium artifact: it records how that surface treats user access, where authority sits, how portability works, and what technical interfaces actually exist.
