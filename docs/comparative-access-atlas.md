# Comparative access atlas

Review date: 2026-09-18.

This document compares the initial six Sociarium surfaces using the completed R1 dossiers plus the completed X R2 baseline.

It compares **access relationships**, not the quality or social value of the services.

## Summary matrix

| Surface | Overall tier | Supported machine access | Official portability | Incremental/live potential | Private first-party protocol | Main sovereignty obstruction | Current implementation posture |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Bluesky / AT Protocol | **Tier A — Sovereign-friendly** | strong documented public/authenticated protocol; many public reads free | complete signed repository export plus migration/recovery mechanisms | first-class repository event streams and diff semantics | core protocol is documented, so archaeology is optional for ordinary public state | private/app-specific state is not all in the public repo; default DID key custody has nuance | **admissible in principle** when implementation phase resumes |
| YouTube | **provisional Tier B — Workable** with Tier C pockets | substantial OAuth Data API with free daily quota | Google Takeout plus My Activity / Your Data in YouTube | good for creator/channel objects; weak for private consumption history | rich Innertube protocol, undocumented and increasingly proof-token-sensitive | API storage/refresh rules and missing watch/search-history API coverage | **admissible in principle** for policy-compatible official-API subsets |
| X / Twitter | **Tier C — Adversarial** | broad official API, but useful self-data reads are metered | broad machine-readable archive, but manual/asynchronous | technically strong in both official and first-party surfaces | **directly observed** broad GraphQL/internal read surface | paid self-data reads plus contractual restrictions on non-published automation | private protocol **not admitted**; official API optional paid backend; archive bootstrap/recovery |
| Reddit | **provisional Tier C — Adversarial** with Tier B-like Data API paths | approved OAuth Data API can be free and broad | official account export, but preparation may take up to 30 days | technically workable for many listings if access is approved | current Shreddit web uses undocumented service/GraphQL families | retention/deletion policy conflicts with durable historical preservation; API direction is narrowing | **not yet admitted** pending live validation and retention analysis |
| Facebook | **provisional Tier C — Adversarial** | Graph API exposes selected personal-profile capabilities under permission/review constraints | unusually broad download/access/transfer tooling | supported live self-data coverage is fragmented | rich private Relay/GraphQL surface | no broad general self-data API; strong anti-scraping posture; review/use-case gates | **do not implement yet**; export importer remains plausible |
| Quora | **provisional Tier C — Adversarial** | no broad supported ordinary-user self-data API identified | official user archive/data-copy path | no supported general continuous self-data channel identified | rich persisted-query GraphQL surface in public technical evidence | no general supported self-data API; manual archive; session/anti-abuse-bound private protocol | **do not implement yet** |

## What the tiers are actually measuring

The comparison makes clear that API availability alone is not enough.

### Portability is not synchronization

Facebook, X, Reddit, YouTube, and Quora all provide meaningful first-party export or account-data mechanisms.

Those mechanisms are valuable for:

- bootstrap;
- recovery;
- historical backfill;
- provenance cross-checking.

But a user-initiated archive prepared minutes, hours, days, or weeks later is not equivalent to a continuous acquisition interface.

Bluesky is the strongest contrast because repository portability and incremental synchronization are part of the protocol architecture rather than separate emergency-export products.

### A free API can still be sovereignty-hostile

Reddit is the clearest example.

Its approved non-commercial Data API can be free and operationally generous, but current retention/deletion requirements conflict with Sociarium's durable provenance-preserving corpus.

YouTube has a related but different issue: the supported Data API is genuinely useful and free-quota-based, while storage/refresh rules constrain what kind of historical evidence corpus may be built from API data.

Therefore:

`free API != automatically Tier A/B for every Sociarium purpose`

### A rich private protocol does not imply implementation admission

X R2 proved this directly.

The first-party client exposes a broad, structured live protocol with stable IDs, pagination, relationship structure, private collection reads, and notification streams.

That made X **more technically legible**, not more cooperative.

Current X Terms/Automation Rules still materially constrain automated non-published access. Sociarium therefore preserves the protocol map while declining to convert it automatically into an adapter.

The same distinction will apply to Facebook, Reddit, Quora, and YouTube archaeology.

### Stable IDs are common; portable identity is not

All six surfaces expose useful stable identifiers somewhere in their stack.

Bluesky is different because the DID is intended to outlive a mutable handle and participates in a broader portable identity/repository architecture.

On the other surfaces, a stable numeric/string account ID is valuable for normalization but does not itself grant portability or independent authority.

## Acquisition-pattern clusters

### Sovereign protocol

**Bluesky**

Best current fit for direct local mirroring and user-controlled continuity.

Primary future acquisition split:

1. signed repo acquisition/export;
2. repo-event synchronization;
3. AppView-derived views;
4. authenticated private/app state.

### Supported but policy-constrained APIs

**YouTube** and parts of **Reddit**

These deserve careful implementation-admission analysis because ordinary supported APIs may be technically sufficient while policy governs what can be retained.

### Export-rich but live-access-poor

**Facebook**, **Quora**, and partly **X**

Their official portability systems are meaningful, but recurring archive requests should not become the operator's synchronization job.

### Technically rich adversarial web clients

**X**, **Facebook**, **Reddit**, **Quora**, and **YouTube**

All expose materially richer first-party application behavior than a simplistic reading of the public API surface would suggest.

Sociarium treats that as research evidence, not automatic permission to automate.

## Research leverage after X

The next useful deep-research target is **Reddit**, selected for research leverage rather than operator priority.

Why Reddit next:

- unlike Facebook/Quora, it still has a plausible supported free machine path;
- unlike Bluesky, its implementation-admission status is unresolved rather than already favorable;
- unlike X, the primary blocker may be policy/retention compatibility rather than price plus explicit private-interface automation restrictions;
- direct validation could materially change the project decision: official Data API adapter, export importer, documentation-only, or a split strategy.

The first Reddit deep pass should therefore answer:

1. which self-data Data API endpoints are actually approval-accessible now;
2. whether a personal/local archival use case is operationally approvable;
3. practical history/listing ceilings for authored, saved, vote, and subscription state;
4. exact current export format/completeness from a real package when available;
5. current first-party Shreddit read operations for the operator's own account;
6. how deletion/retention obligations interact with a personal provenance-preserving corpus.

## Implementation consequence

R3 does **not** unfreeze implementation globally.

It improves the I0 decision surface.

Current broad posture:

- Bluesky: implementation-admissible in principle;
- YouTube: implementation-admissible in principle for carefully scoped supported data;
- X: private protocol not admitted; paid official/API archive sources remain optional candidates;
- Reddit: unresolved, next deep research target;
- Facebook: research/export validation first;
- Quora: research/archive/private-protocol validation first.

The human principal may override these gates deliberately. Agents may not silently turn research findings into new adapter code.
