# ADR 0002: Surfaces are adapters; profiles are synchronization scopes

Status: accepted

## Decision

The generic model contains no privileged X/Twitter concept. Each remote social system is represented by an adapter, and synchronization primitives operate on configured remote profiles.

A remote profile is not automatically equivalent to a real-world person or entity.

## Consequences

- multiple profiles may exist on one surface;
- one entity may explicitly link profiles across multiple surfaces;
- core APIs must not hide a global current-account singleton;
- stable remote IDs are retained separately from mutable handles.
