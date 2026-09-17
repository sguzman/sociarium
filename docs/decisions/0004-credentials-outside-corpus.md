# ADR 0004: Credentials are outside the corpus

- Status: accepted
- Date: 2026-09-17

## Context

Sociarium preserves durable, Git-versioned social observations. Remote adapters also need bearer credentials whose disclosure would grant authority over accounts. Treating those credentials as corpus data would make normal repository operations dangerous and would couple social history to one machine's authentication state.

## Decision

Credentials are operational secret state, not corpus state.

OAuth access tokens, refresh tokens, client secrets, PKCE verifiers, authorization codes, passwords, and session cookies must not be stored in raw evidence, normalized objects, derived indexes, or other Git-tracked corpus paths.

Adapters own their authentication protocol, but persistent secret storage is reached through a distinct credential boundary keyed by profile identity. M0 may keep tokens in memory while that store is implemented; it must not fall back to committing plaintext tokens.

## Consequences

- A corpus can be copied, inspected, or published without implicitly copying account authority.
- Restoring a corpus on a new machine requires re-establishing credentials separately.
- Diagnostics must redact secret values.
- Remote write capability can later be audited independently of local corpus readability.
