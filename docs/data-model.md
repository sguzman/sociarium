# Data model

The core ontology must remain useful across surfaces without pretending all social systems are identical.

## Identity hierarchy

```text
Surface
  +-- RemoteProfile
        +-- ProfileSnapshot[]

Entity (optional)
  +-- explicit links to RemoteProfile[]
```

A `RemoteProfile` is a platform-scoped account identity. It is durably identified by the **surface plus that surface's stable remote profile identifier**.

A local `ProfileId` is Sociarium's operator-facing/configuration identity for one tracked remote profile scope. Once that local profile is bound to a remote profile ID on its surface, later observations under the same local profile must continue to refer to that same stable remote profile identity.

An `Entity` is a separate local assertion that one or more remote profiles represent the same person, organization, project, pseudonym, bot, or other actor. Remote-profile binding and cross-profile entity resolution are different concepts: M0 needs the first and explicitly defers the second.

## Initial normalized records

M0 starts with only the records necessary for the first vertical slice:

- `TrackedProfile` — local configuration for a profile to synchronize.
- `ProfileSnapshot` — mutable profile metadata observed at a time, including the stable remote profile ID that observation belongs to.
- `Post` — authored social content with stable remote identity and relationship references.
- `ObservationMeta` — acquisition/provenance metadata shared by normalized records.

Later records may include reactions, relationships, collections, media, threads, and explicit identity links.

## Stable versus mutable identifiers

Never use a handle as the durable key when a surface provides a stable profile identifier.

```text
(surface=x, remote_profile_id=12345) = stable identity
@old_handle -> observed profile state
@new_handle -> later observed profile state
```

The same rule applies to posts and other remote objects.

A handle can be useful as **first-enrollment intent** when the operator has not yet bound a numeric/stable remote ID, but once the stable remote ID is known, handle changes must not create a new profile identity or invalidate the binding.

## Local profile binding

The durable invariant is:

```text
(local ProfileId, SurfaceId) -> one stable RemoteId
```

`TrackedProfile.remote_id` may be absent before the first authorized acquisition because the operator should not have to manually discover opaque numeric IDs that Sociarium can obtain safely from the remote surface.

M0 must therefore distinguish these states:

- **unbound** — no explicit configured remote ID and no prior durable profile observation establishes one;
- **bound** — configuration or durable corpus evidence establishes one stable remote profile ID;
- **conflicted** — durable evidence for the same local profile/surface contains incompatible stable remote profile IDs.

A conflicted profile is a corpus-integrity error. Sociarium must not silently choose one ID, merge the identities, or continue appending new data under that local profile.

For an unbound X self-owned profile, the configured handle may act as an enrollment guard for the first authorized `/users/me` result. Once the resulting stable remote ID is durably established, future authorization/sync attempts are checked against that remote ID rather than requiring the handle to remain unchanged.

M0 reconstructs this binding directly from completed durable `ProfileSnapshot` history through `CorpusStore::profile_binding`. The result is explicitly one of:

```text
Unbound
Bound(RemoteId)
Conflicted(RemoteId[])
```

Before synchronization, CLI composition resolves that durable state. A bound ID is injected into the working `TrackedProfile` when configuration omitted it, so the surface adapter can reject a credential/account mismatch before fetching Posts. If configuration explicitly names a different remote ID than durable evidence, synchronization fails locally.

For the first unbound X enrollment, a configured handle is used only as a case-insensitive guard against authorizing the wrong account. Once the first `ProfileSnapshot.remote_id` is durable, the stable ID becomes authoritative and future handle changes remain ordinary mutable observations.

The persistence boundary independently enforces the same invariant before publishing a new acquisition. A conflicting `ProfileSnapshot.remote_id`, a batch containing multiple profile IDs, or already-conflicted durable history is rejected even if an adapter or composition layer is buggy.

The canonical binding therefore remains reconstructable from ordinary acquisition evidence. It is not stored only in SQLite, a mutable singleton state file, Git history, or credential metadata.

## Shared semantics versus extensions

Normalize fields only when the semantics are genuinely portable. Surface-specific data may live in typed adapter records before normalization and in an extension map or future surface-extension representation after normalization.

A Reddit score, X repost metric, Mastodon visibility mode, and Bluesky facet should not be flattened into a misleading generic field merely to make schemas look symmetrical.

A portable field should also use the **best complete value** supplied by the surface. For example, if X supplies full long-form authored text in `note_tweet.text` while `text` is only a shorter representation, generic `Post.text` should contain the complete authored text while the raw payload remains available for later richer interpretation.

## Provenance

Every durable normalized record should eventually answer:

- which surface produced this information?
- which remote object/profile did it concern?
- when was it observed?
- which raw evidence or acquisition batch supports it?
- which schema/normalizer version interpreted it?

M0 establishes the types and repository locations; later milestones strengthen content-addressed links and migration/versioning rules.
