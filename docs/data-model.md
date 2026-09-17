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

A `RemoteProfile` is a platform-scoped account identity. An `Entity` is a local assertion that one or more profiles represent the same person, organization, project, pseudonym, bot, or other actor.

## Initial normalized records

M0 starts with only the records necessary for the first vertical slice:

- `TrackedProfile` — local configuration for a profile to synchronize.
- `ProfileSnapshot` — mutable profile metadata observed at a time.
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

## Shared semantics versus extensions

Normalize fields only when the semantics are genuinely portable. Surface-specific data may live in typed adapter records before normalization and in an extension map or future surface-extension representation after normalization.

A Reddit score, X repost metric, Mastodon visibility mode, and Bluesky facet should not be flattened into a misleading generic field merely to make schemas look symmetrical.

## Provenance

Every durable normalized record should eventually answer:

- which surface produced this information?
- which remote object/profile did it concern?
- when was it observed?
- which raw evidence or acquisition batch supports it?
- which schema/normalizer version interpreted it?

M0 establishes the types and repository locations; later milestones strengthen content-addressed links and migration/versioning rules.
