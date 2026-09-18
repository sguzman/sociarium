# Sociarium project charter

## Purpose

Sociarium studies and preserves the relationship between people and the social systems that hold their data.

Its software may eventually acquire, normalize, preserve, search, and analyze that data locally. But the project begins one layer earlier:

> **How does each social surface actually permit an authorized user to access, preserve, and continuously observe their own data?**

That question is a first-class technical research problem.

Sociarium therefore treats documentation, digital forensics, protocol archaeology, and comparative access analysis as primary project outputs rather than preliminary chores before "real" coding.

## Documentation outranks implementation

A working adapter without a trustworthy dossier is incomplete knowledge.

A trustworthy dossier with no adapter can be a complete and valuable result.

Implementation is downstream of understanding:

```text
surface inventory
    ↓
forensic/access research
    ↓
dated evidence
    ↓
access classification
    ↓
acquisition candidates
    ↓
implementation admission
    ↓
code, if warranted
```

During an active research freeze, adding surface-specific code is not forward progress unless the human principal explicitly changes the phase.

## Surface sovereignty doctrine

A platform's official developer API is only one representation of the platform.

A surface may expose data through:

- documented APIs;
- account exports;
- public web representations;
- authenticated first-party web/mobile protocols;
- live channels;
- other observable first-party mechanisms.

Sociarium distinguishes the **surface** from every **acquisition source** used to observe it.

No platform is owed an adapter, and no sanctioned interface is automatically accepted as the only technically meaningful interface.

## Access relationships

Sociarium uses four dated access tiers:

- **Tier A — Sovereign-friendly**
- **Tier B — Workable**
- **Tier C — Adversarial**
- **Tier D — Inaccessible**

Tier C is intentionally strong language.

It means the sanctioned interfaces materially obstruct useful access to data already available to the authorized user. Sociarium does not normalize repeated manual submission, artificial export rituals, or paid self-data access as an acceptable long-term architecture. Instead, the first-party client becomes a legitimate object of protocol research.

Tier D marks a real stopping boundary: useful acquisition presently appears to require crossing meaningful authorization or security controls, or otherwise taking on constraints the project will not normalize.

## Evidence before assertion

For material claims, Sociarium distinguishes:

1. documented fact;
2. direct observation;
3. inference;
4. unknown.

Undocumented protocol observations are dated and contextual. The project preserves change history instead of rewriting yesterday's observation into today's supposed timeless truth.

A dossier should make it possible to answer:

- what was observed?
- when?
- using which client/context?
- what evidence supports it?
- which parts are inference?
- what could make it stale?

## The human principal is not an integration servant

The project must not casually externalize platform friction onto the human principal.

Recurring manual exports, repeated developer-console rituals, arbitrary token handling, and other platform-imposed labor are themselves access costs and should be documented as such.

If a platform makes useful acquisition degrading or impractical, Sociarium should classify and investigate that relationship rather than silently turning the operator into a permanent maintenance mechanism.

## Healthy abundance

Sociarium should become a rich atlas of social-system access behavior:

- many surfaces;
- many acquisition mechanisms;
- many data classes;
- historical observations;
- protocol changes;
- costs;
- friction;
- stable identifiers;
- portability mechanisms;
- adversarial findings;
- explicit unknowns.

The project is successful when this knowledge becomes durable, comparable, and useful, regardless of how many adapters are ultimately written.

## Implementation admission

Before new surface-specific implementation normally begins, there should be:

- an inventory entry;
- a current dossier;
- an access matrix;
- a dated tier assessment;
- identified acquisition candidates;
- known cost/auth/history constraints;
- an explicit reason to implement one candidate rather than another.

The human principal may override this deliberately. Agents may not bypass it to preserve coding momentum.

## Security boundary

Adversarial research does not erase authorization boundaries.

Sociarium may study the behavior of first-party clients used by an authorized account and document undocumented protocols. It does not treat credential theft, impersonation, or defeating meaningful access controls as ordinary acquisition techniques.

Sensitive raw captures remain outside the public repository unless safely sanitized.

## Current phase

The current phase is **R0 — Surface Inventory**, followed by **R1 — baseline dossiers**.

New surface-specific implementation is frozen until the human principal has enumerated the surfaces worth investigating and the research program has established enough evidence to make implementation decisions deliberately.
