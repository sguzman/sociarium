# X archive import runbook (paused implementation path)

This document preserves the X archive-import implementation path developed before the Surface Atlas pivot.

**It is not the active project milestone. Do not resume this work merely because the code/runbook exists.**

Sociarium is currently in R0/R1 research mode. X archive import should be reconsidered only after the X surface dossier compares it against the official API, public web surface, and authorized first-party private protocol as acquisition sources.

X's first-party account archive is requested from the normal X account settings and is separate from the paid Developer API. X documents the archive as machine-readable HTML/JSON and states that it includes profile information and the account's entire post history, plus additional account data.

## 1. Request the archive from X

On X web:

1. Open **Settings and privacy**.
2. Choose **Your account**.
3. Choose **Download an archive of your data**.
4. Confirm the account as X requests.
5. Choose **Request archive**.

X may take time to prepare the archive. No Developer App, API key, API credit purchase, or paid subscription is required for this export path.

Do not upload the archive to a public repository. Even without credentials it can contain private/account-sensitive material beyond public Posts.

## 2. Keep the ZIP intact

Sociarium's target importer accepts either:

- the original downloaded ZIP; or
- an extracted archive directory.

Keeping the original ZIP is preferable for provenance because the exact source package can be hashed/recorded without depending on an extraction tool's behavior.

## 3. Initialize a dedicated corpus

```text
cargo run -p sociarium-cli --locked -- corpus init <CORPUS_ROOT>
```

The corpus must remain separate from the public Sociarium software checkout.

## 4. Configure the local profile

The archive path does not require `[surfaces.x] client_id`, OAuth callback settings, or credentials.

The target profile remains an ordinary X profile:

```toml
schema_version = 1

[[profiles]]
id = "x-main"
surface = "x"
handle = "sguzman"
ownership = "self_owned"
enabled = true
```

If the archive exposes the stable native X account ID, the first successful import binds `x-main` to it. Later imports must agree.

## 5. Import

Target CLI:

```text
cargo run -p sociarium-cli --locked -- \
  --config <CORPUS_CONFIG> \
  --corpus <CORPUS_ROOT> \
  import x-archive <ARCHIVE_ZIP_OR_DIR> --profile x-main
```

The import must not contact X.

Expected behavior:

- detect a supported archive layout;
- preserve selected original archive evidence;
- normalize the self-owned profile and authored Posts;
- reject a remote-account identity conflict;
- avoid flattening reposts into authored Posts;
- make repeated import idempotent at the visible/query layer;
- rebuild the disposable search index.

## 6. Verify local results

```text
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts list --profile x-main
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts search <TERM> --profile x-main
```

M0 closes when a real operator archive imports successfully on Windows and those local queries return real authored Posts without any paid X API usage.
