# X live smoke test

This runbook is the final M0 validation gate. It exercises the real Windows credential, OAuth, X API, durable corpus, checkpoint, index, and local query path against an authorized X account.

> **Current status:** do not execute the deliberate live smoke test while M0 pre-live issues #2–#5 remain open. Issue #5 is a confirmed X wire-contract blocker in the current timeline request. This runbook describes the intended final procedure after those repository-side fixes are integrated and CI is green.

Do not put tokens, authorization codes, PKCE verifiers, client secrets, raw failed OAuth/API response bodies, or Windows Credential Manager exports into issues, commits, screenshots, or test evidence.

## Preconditions

- M0 pre-live issues #2–#5 are closed with their acceptance criteria satisfied.
- Linux stable, native Windows, and Rust 1.85 locked CI are green on the exact `main` head being tested.
- Windows is the test host.
- The repository is on the current `main` branch.
- Rust/Cargo can build the workspace using the committed `Cargo.lock`.
- An X Developer App exists with OAuth 2.0 enabled.
- Use an X **Native App** / public-client shape for Sociarium. The local application does not require or persist an X client secret.
- The app's callback URL exactly matches the configured loopback redirect, for example:

```text
http://127.0.0.1:49152/oauth/x/callback
```

- The app permits the scopes requested by M0:

```text
tweet.read
users.read
offline.access
```

`offline.access` is required for a refresh token so later syncs can refresh without another interactive authorization.

## 1. Prepare local configuration

Copy `sociarium.example.toml` to the repository-root `sociarium.toml` if needed and set the real X Client ID. The root `sociarium.toml` is ignored by default.

```toml
schema_version = 1

[surfaces.x]
client_id = "YOUR_X_CLIENT_ID"
redirect_uri = "http://127.0.0.1:49152/oauth/x/callback"

[[profiles]]
id = "x-main"
surface = "x"
handle = "sguzman"
ownership = "self_owned"
enabled = true
```

The Client ID is application identification, not a bearer token. Do not add a client secret or bearer credential to this file merely because it is ignored by Git.

Validate before contacting X:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml config check
cargo run -p sociarium-cli --locked -- --config sociarium.toml profiles list
cargo run -p sociarium-cli --locked -- doctor
```

Expected today: configuration validates, `x-main` is enabled, and Windows reports the native credential store as available.

Issue #2 will replace/augment these coarse checks with a profile-aware no-network preflight that also verifies OAuth configuration, loopback binding, an isolated native credential-store round trip, corpus initialization, and emergency-env state. Once #2 is implemented, use that profile-aware preflight as the authoritative first smoke-test command.

## 2. Start from a known authorization state

Inspect credential state:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth status x-main
```

For the first deliberate smoke test, remove an old Sociarium credential if one exists so the interactive path is exercised:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth logout x-main
```

Also ensure `SOCIARIUM_X_ACCESS_TOKEN` is not set for this test. The environment override is useful for recovery/debugging, but using it would skip the native credential path that M0 needs to validate.

## 3. Exercise native OAuth2/PKCE login

Run:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth login x-main
```

Sociarium should:

1. bind only the configured loopback callback;
2. print an X authorization URL containing an S256 PKCE challenge and random state;
3. wait for the callback for a bounded period;
4. tolerate unrelated loopback/browser requests without consuming the authorization session;
5. let the user authorize the requested read/offline scopes in X;
6. reject the configured callback if its OAuth `state` does not match;
7. exchange the short-lived authorization code immediately;
8. store the resulting token envelope under the profile-scoped Windows credential key;
9. report success without printing token values.

After authorization:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml auth status x-main
```

Expected: native credential `present`, a non-secret expiry timestamp is visible, and `refresh_token=true`.

## 4. Perform first real synchronization

Use a clean/local corpus path for the smoke test if practical:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml --corpus corpus sync x-main
```

Expected:

- authenticated-user lookup succeeds;
- the configured remote ID is checked if one is configured;
- the X user-post request uses the current remote wire contract (`tweet.fields`, including the relationship fields required by M0's normalized semantics);
- the chosen M0 repost policy is applied deliberately rather than flattening an unsupported repost relation;
- one or more acquisition pages are persisted;
- each successful acquisition contains raw evidence and normalized records;
- a durable X checkpoint is stored;
- the disposable search index is rebuilt after sync;
- no bearer token appears in the corpus.

The first traversal bootstraps the historical window X currently exposes for the user-post endpoint, not necessarily the account's complete lifetime history. X currently limits this timeline to roughly the most recent 3,200 Posts. The smoke test proves correct acquisition/preservation of the available window and forward incremental continuity; it does not prove a lifetime-complete historical export.

Record only non-secret terminal output and resulting corpus paths as smoke-test evidence. Remote failure diagnostics should be the sanitized/structured form established by issue #4, not pasted raw response bodies.

## 5. Verify local query behavior

List recent posts:

```text
cargo run -p sociarium-cli --locked -- --corpus corpus posts list --profile x-main
```

Search for a word known to occur in at least one acquired post:

```text
cargo run -p sociarium-cli --locked -- --corpus corpus posts search YOUR_TERM --profile x-main
```

Expected: results are served from the local index and contain the normalized post identity/text/URL information without contacting X.

If the acquired sample includes a reply or quote Post, inspect at least one normalized acquisition to confirm the requested remote relationship field survives into `reply_to` or `quote_of` where applicable.

## 6. Verify incremental checkpoint behavior

Run sync again without deleting the corpus:

```text
cargo run -p sociarium-cli --locked -- --config sociarium.toml --corpus corpus sync x-main
```

Expected: Sociarium starts from prior durable state. X receives the completed high-water `since_id` rather than treating the previous pagination token as the incremental checkpoint. Already durable history remains intact.

If there are no new posts, a successful no-new-data traversal is still a valid result; it must not erase the previous checkpoint or corpus.

## 7. Verify rebuildability

Delete/rebuild only the disposable search projection through Sociarium:

```text
cargo run -p sociarium-cli --locked -- --corpus corpus index rebuild
```

Repeat the list/search commands. Expected: query results are reconstructed from durable acquisition files; the SQLite/FTS index is not the sole copy of social data.

## 8. Optional refresh-path validation

The normal access token lifetime reported by X for this flow is short relative to a persistent corpus, while `offline.access` supplies a refresh token. The refresh code path is already unit-tested, but a later real sync after the access token reaches the refresh window should verify that Sociarium:

1. loads the profile-scoped stored envelope;
2. refreshes through X without interactive login;
3. preserves/rolls the refresh token correctly;
4. writes the refreshed envelope back to Windows Credential Manager;
5. completes sync normally.

This optional delayed check is useful evidence but is not required to block the initial M0 smoke test if steps 1–7 succeed and the issued credential contains a refresh token.

## Failure classification

Classify a failure before changing architecture:

- **configuration failure:** malformed/missing `[surfaces.x]` setting or profile;
- **local preflight failure:** callback port/binding, credential-store round trip, corpus initialization, or environment state fails before X is contacted;
- **developer-app failure:** OAuth2 disabled, wrong app type, callback mismatch, or missing permitted scope;
- **authorization failure:** user denies access, state mismatch, expired authorization code, or token exchange rejection;
- **credential-store failure:** Windows credential backend cannot save/load/delete the envelope;
- **API entitlement/rate failure:** X accepts authentication but rejects the requested endpoint because of current API access/rate policy;
- **adapter/wire-contract failure:** X response shape, endpoint, query parameter, or field behavior no longer matches the adapter;
- **persistence failure:** acquisition data or cursor cannot be durably written;
- **index/query failure:** corpus persists correctly but projection rebuild/query fails.

Do not bypass a failure by moving tokens into `sociarium.toml` or the corpus. Fix the failing boundary.

## M0 completion evidence

M0 can close when the repository-side pre-live issues are resolved and the real smoke run demonstrates all of these together:

- profile-aware local preflight passes without contacting X;
- native S256 PKCE login succeeds;
- Windows Credential Manager retains the profile-scoped credential;
- first X sync persists real raw + normalized data from the retrievable remote window;
- supported reply/quote references are preserved when present;
- durable checkpoint is present;
- second sync starts from prior state;
- local index rebuild succeeds;
- local list/search returns acquired posts;
- no bearer secret is found in tracked/local corpus data or pasteable diagnostics.

Once recorded, update issue #1 with the smoke-test result and close M0.
