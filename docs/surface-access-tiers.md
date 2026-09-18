# Surface access tiers

Sociarium classifies **the access relationship**, not the social value or political quality of a platform.

The tier is a summary of how a surface permits the operator to access and preserve their own data. Detailed per-data-class matrices remain authoritative.

## Tier A — Sovereign-friendly

The surface exposes strong user-controlled acquisition paths.

Typical characteristics:

- useful documented interfaces are free or meaningfully accessible;
- stable profile/object identifiers exist;
- incremental synchronization is supported;
- historical access is reasonably complete or portable;
- exports are usable and documented;
- authentication can be scoped without surrendering unrelated account secrets;
- local preservation is operationally straightforward.

Tier A does not require perfection. It means the surface substantially cooperates with user-controlled preservation.

## Tier B — Workable

The surface is imperfect but can be integrated without making recurring degradation, payment, or protocol archaeology the normal operating model.

Examples of limitations:

- incomplete history;
- conservative rate limits;
- missing data classes;
- awkward but tolerable authentication;
- export gaps;
- documented APIs that require compromises but remain practically usable.

Tier B implementation should document those limitations rather than normalize them into the core ontology.

## Tier C — Adversarial

The sanctioned interfaces materially obstruct useful programmatic access to the operator's own data.

Possible signals include:

- charging specifically for practically useful self-data reads;
- deliberately crippled or asynchronous portability as the only free path;
- useful first-party functionality exposed to the web/mobile client but withheld from documented developer interfaces;
- artificial history windows;
- severe recurring manual intervention;
- unstable or selectively undocumented interfaces;
- data classes visible to the user but not meaningfully exportable.

**Adversarial does not mean Sociarium gives up or asks the operator to keep groveling.**

It triggers protocol archaeology. The first-party application becomes evidence. Sociarium investigates what the shipped client actually does, maps the private protocol, records auth boundaries and drift, and determines what acquisition strategies are technically and operationally defensible.

An official export can still be useful in Tier C, especially as a historical bootstrap. It is not automatically accepted as the long-term synchronization model.

## Tier D — Inaccessible

Useful acquisition is not presently achievable without crossing a meaningful authorization/security boundary or accepting constraints that make an implementation unjustifiable.

Examples may include situations where the only apparent path would require:

- bypassing access controls;
- obtaining credentials the operator is not entitled to possess;
- impersonating another user;
- defeating a security boundary rather than reproducing an authorized client interaction.

Tier D is not a permanent moral judgment. It is a dated technical classification. New exports, protocols, APIs, policy changes, or observations can move a surface between tiers.

## Classification rules

1. **Date every classification.** Access conditions change.
2. **Cite evidence.** A tier without a dossier is provisional.
3. **Do not average away important differences.** A surface can be Tier B overall while bookmarks are Tier C.
4. **Cost matters.** Paid access to one's own data is a first-class access constraint, not a footnote.
5. **Manual friction matters.** Requiring recurring export requests may be acceptable for recovery/bootstrap and still unacceptable for continuous acquisition.
6. **The official API is one acquisition source, not the definition of the surface.**
7. **Private protocol research is not security bypass.** Record the authorization boundary explicitly.
8. **No site is owed implementation effort.** A bad tier can cause Sociarium to document rather than integrate.
