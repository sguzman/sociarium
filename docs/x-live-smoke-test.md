# X live smoke test

This runbook is the final M0 validation gate. It exercises the real Windows credential, OAuth, X API, durable corpus, checkpoint, index, and local query path against an authorized X account.

> **Current status:** repository-side M0 pre-live work is complete. This runbook is now the final M0 validation gate.

Do not put tokens, authorization codes, PKCE verifiers, client secrets, raw failed OAuth/API response bodies, or Windows Credential Manager exports into issues, commits, screenshots, or test evidence.

## Preconditions

- M0 pre-live issues #2–#7 are closed with their acceptance criteria satisfied.
- Create/configure the X app in the current X Developer Console at `https://console.x.com`.
- In the app's authentication settings, enable **OAuth 2.0** and select **Native App**. Native Apps are public clients: Sociarium uses PKCE and does not require or persist an X client secret.
- In the app's callback URL allowlist, register the configured loopback URL **exactly**, including path and any trailing slash. For local development, use `http://127.0.0.1:<port>/...`, not `localhost`.
- Copy only the OAuth 2.0 **Client ID** needed by Sociarium into the corpus configuration. API keys, OAuth 1.0a secrets, bearer tokens, and any confidential-client secret shown elsewhere in the Developer Console are not part of the M0 configuration path.
- Linux stable, native Windows, and Rust 1.85 locked CI are green on the exact `main` head being tested.
- Windows is the test host.
- The Sociarium **software source checkout** is on the current `main` branch.
- Rust/Cargo can build the workspace using the committed `Cargo.lock`.
- A separate operator-owned corpus directory/repository has been initialized through the #6 corpus boundary; do not use a real-data `corpus/` child inside the public Sociarium source checkout.
- An X Developer App exists with OAuth 2.0 enabled.
- The X developer project/account has whatever **current API credits, billing state, and endpoint entitlement** X requires for the user-post reads exercised by the smoke test. Do not encode a dollar price into this runbook; X commercial terms are external and mutable.
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
- For the first unbound `x-main` enrollment, configuration contains either the intended stable `remote_id` or the intended X handle. The handle is only an enrollment guard; the remote numeric ID observed from `/users/me` becomes the durable identity after the first successful acquisition.

## 0. Configure the X Developer App

The current X Developer Console flow is:

1. Sign in at `https://console.x.com`.
2. Create an App if one does not already exist for Sociarium.
3. Open the App's authentication settings and enable OAuth 2.0.
4. Select **Native App**. This is the public-client shape intended for desktop/mobile applications that cannot keep a client secret.
5. Register the exact callback URL that will appear in `sociarium.toml`, for example:

```text
http://127.0.0.1:49152/oauth/x/callback
```

6. Make sure the callback uses `127.0.0.1`, not `localhost`, and that path/trailing-slash spelling matches exactly.
7. Copy the OAuth 2.0 **Client ID** from the App's keys/tokens area. Sociarium does not need a client secret for this Native App flow.
8. Ensure the App can request the M0 scopes `tweet.read users.read offline.access`.
9. Confirm the developer account/project has whatever current X API billing/credit/endpoint entitlement is required for the user-post reads exercised by M0.

X currently documents OAuth 2.0 Authorization Code + PKCE for this flow. `offline.access` is required for a refresh token; without it, the access token is short-lived and the persistent refresh path cannot be validated.

Do not copy Developer Console credentials into an issue or commit. The Client ID is non-secret application identification; bearer/user tokens, API secrets, and any client secret are authority-bearing credentials and do not belong in the corpus.

## 1. Initialize/select the dedicated corpus

The durable corpus is not the Sociarium software repository.

Conceptually:

```text
C:\path\to\sociarium\          # software source checkout
C:\path\to\my-social-corpus\  # operator-owned durable corpus
```

Initialize the dedicated corpus before inserting real configuration or contacting X:

```text
cargo run -p sociarium-cli --locked -- corpus init <CORPUS_ROOT>
```

This creates the versioned corpus marker, durable layout, corpus-local Git ignore policy, and `<CORPUS_ROOT>/sociarium.toml` non-secret configuration template. The command does not initialize Git, commit, or push; Git remains optional history/transport. Re-running initialization on a compatible corpus is safe, while an unrelated non-empty target is rejected.

If the corpus is pushed to a remote Git host, private visibility is the conservative default. The corpus must contain no credentials, but future authorized data can still be private even when it is not credential material.

For the rest of this runbook:

```text
<CORPUS_ROOT>   = the initialized dedicated corpus directory
<CORPUS_CONFIG> = <CORPUS_ROOT>/sociarium.toml
```

The config may be tracked in the dedicated corpus if that is the initialized corpus policy; it still must not contain bearer credentials or client secrets.

## 2. Prepare local configuration

Use the corpus configuration and set the real X Client ID:

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

The Client ID is application identification, not a bearer token. Do not add a client secret or bearer credential to this file.

Before contacting X, run the no-network profile-aware preflight:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> --corpus <CORPUS_ROOT> doctor --profile x-main
```

It validates the configured/enabled profile, registered adapter, X OAuth application settings, loopback URI and real socket bindability, initialized dedicated corpus, reconstructable profile-binding state, an isolated non-secret native Credential Manager save/load/delete round trip, and whether the emergency `SOCIARIUM_X_ACCESS_TOKEN` override is present. It does **not** contact X.

Expected first-enrollment shape:

```text
PASS config: <CORPUS_CONFIG>
PASS profile: x-main (surface=x enabled=true)
PASS adapter: x
PASS x oauth config: client_id present, loopback redirect valid
PASS callback bind: 127.0.0.1:49152
PASS corpus: <CORPUS_ROOT> (initialized dedicated corpus)
PASS profile binding: unbound; enrollment guard handle=@sguzman present
PASS credential store: available
PASS credential roundtrip: save/load/delete
PASS emergency env override: absent
READY local preflight passed; next boundary is live X authorization
```

For an already-enrolled corpus, the profile-binding line instead reports the stable non-secret remote ID. A conflicted binding, configuration mismatch, occupied callback port, credential-store failure, uninitialized corpus, or emergency X token override fails non-zero before X is contacted.

## 3. Start from a known authorization state

Inspect credential state:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> auth status x-main
```

For the first deliberate smoke test, remove an old Sociarium credential if one exists so the interactive path is exercised:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> auth logout x-main
```

Also ensure `SOCIARIUM_X_ACCESS_TOKEN` is not set for this test. The environment override is useful for recovery/debugging, but using it would skip the native credential path that M0 needs to validate.

## 4. Exercise native OAuth2/PKCE login

Run:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> auth login x-main
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
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> auth status x-main
```

Expected: native credential `present`, a non-secret expiry timestamp is visible, and `refresh_token=true`.

## 5. Perform first real synchronization

Run against the dedicated corpus root:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> --corpus <CORPUS_ROOT> sync x-main
```

Expected:

- authenticated-user lookup succeeds;
- if a stable remote ID was already configured or reconstructed from corpus history, the authenticated X user must have that exact ID before Posts are fetched;
- if the profile is genuinely unbound, its configured handle must match the authenticated username case-insensitively before first enrollment is accepted;
- the first successful unbound acquisition durably establishes the stable remote ID through its `ProfileSnapshot`;
- a later handle rename for that same remote ID does not create a new local identity or invalidate the binding;
- the X user-post request uses the current remote wire contract, including `tweet.fields=created_at,referenced_tweets,note_tweet` or an equivalent field set preserving those semantics;
- the chosen M0 repost policy is applied deliberately rather than flattening an unsupported repost relation;
- when X returns `note_tweet.text`, normalized `Post.text` uses that full authored text instead of a shorter/truncated `text` representation;
- one or more acquisition pages are persisted under the dedicated corpus, not the software checkout;
- each successful acquisition contains raw evidence and normalized records;
- a durable X checkpoint is stored;
- the disposable search index is rebuilt after sync;
- no bearer token appears in the corpus.

The first traversal bootstraps the historical window X currently exposes for the user-post endpoint, not necessarily the account's complete lifetime history. X currently limits this timeline to roughly the most recent 3,200 Posts. The smoke test proves correct acquisition/preservation of the available window and forward incremental continuity; it does not prove a lifetime-complete historical export.

Record only non-secret terminal output and resulting corpus paths as smoke-test evidence. Remote failure diagnostics should be the sanitized/structured form established by issue #4, not pasted raw response bodies.

## 6. Verify local query behavior and fidelity

List recent posts:

```text
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts list --profile x-main
```

Search for a word known to occur in at least one acquired post:

```text
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> posts search YOUR_TERM --profile x-main
```

Expected: results are served from the local index and contain the normalized post identity/text/URL information without contacting X.

Where the account history provides suitable examples, verify both:

- a reply or quote Post retains `reply_to` or `quote_of` in normalized acquisition data;
- a Post longer than 280 characters is locally searchable/listable with its **full** authored text, matching `note_tweet.text` from preserved raw evidence rather than only the shorter `text` field.

If the smoke-test account simply has no suitable live example in the retrievable window, the fixture/unit coverage from issue #5 remains the deterministic acceptance evidence for that shape; do not manufacture or publish a Post solely to satisfy this read-only M0 test.

## 7. Verify incremental checkpoint behavior

Run sync again without deleting the corpus:

```text
cargo run -p sociarium-cli --locked -- --config <CORPUS_CONFIG> --corpus <CORPUS_ROOT> sync x-main
```

Expected: Sociarium starts from prior durable state. Before contacting X for Posts, the CLI reconstructs the stable remote-profile binding from completed `ProfileSnapshot` evidence and passes that ID to the X adapter. X receives the completed high-water `since_id` rather than treating the previous pagination token as the incremental checkpoint. Already durable history remains intact.

A credential swap or later login to a different X account must therefore fail the stable-ID check rather than append that account beneath `x-main`. Deterministic M0 tests cover this negative path; the live smoke run does not require the operator to maintain or deliberately authorize a second X account merely to prove it.

If there are no new posts, a successful no-new-data traversal is still a valid result; it must not erase the previous checkpoint, corpus, or profile binding.

## 8. Verify rebuildability

Delete/rebuild only the disposable search projection through Sociarium:

```text
cargo run -p sociarium-cli --locked -- --corpus <CORPUS_ROOT> index rebuild
```

Repeat the list/search commands. Expected: query results are reconstructed from durable acquisition files; the SQLite/FTS index is not the sole copy of social data.

Verify the corpus Git policy also behaves as intended: completed acquisitions are visible as canonical changes while indexes and any `.pending-*` staging directories are ignored/not canonical.

## 9. Optional refresh-path validation

The normal access token lifetime reported by X for this flow is short relative to a persistent corpus, while `offline.access` supplies a refresh token. The refresh code path is already unit-tested, but a later real sync after the access token reaches the refresh window should verify that Sociarium:

1. loads the profile-scoped stored envelope;
2. refreshes through X without interactive login;
3. preserves/rolls the refresh token correctly;
4. writes the refreshed envelope back to Windows Credential Manager;
5. completes sync normally.

This optional delayed check is useful evidence but is not required to block the initial M0 smoke test if steps 1–8 succeed and the issued credential contains a refresh token.

## Failure classification

Classify a failure before changing architecture:

- **corpus-boundary failure:** source/corpus paths are conflated, initialization policy is invalid, or canonical/disposable Git behavior is wrong;
- **configuration failure:** malformed/missing `[surfaces.x]` setting or profile;
- **local preflight failure:** callback port/binding, credential-store round trip, corpus validation, or environment state fails before X is contacted;
- **developer-app failure:** OAuth2 disabled, wrong app type, callback mismatch, or missing permitted scope;
- **authorization failure:** user denies access, state mismatch, expired authorization code, or token exchange rejection;
- **credential-store failure:** Windows credential backend cannot save/load/delete the envelope;
- **API billing/credit failure:** X accepts the request path/auth context but returns a payment/credit condition such as HTTP 402; verify current Developer Console billing/credits before changing Sociarium architecture;
- **API entitlement/rate failure:** X accepts authentication but rejects the requested endpoint because of current access policy or rate limits;
- **adapter/wire-contract failure:** X response shape, endpoint, query parameter, or field behavior no longer matches the adapter;
- **persistence failure:** acquisition data or cursor cannot be durably written;
- **index/query failure:** corpus persists correctly but projection rebuild/query fails.

Do not bypass a failure by moving tokens into configuration/corpus files, writing real data into the public source repo, or changing architecture to hide an X contract/billing error. Fix or satisfy the failing boundary.

## M0 completion evidence

M0 can close when the repository-side pre-live issues are resolved and the real smoke run demonstrates all of these together:

- a dedicated operator corpus is initialized and Git-safe;
- profile-aware local preflight passes without contacting X;
- native S256 PKCE login succeeds;
- Windows Credential Manager retains the profile-scoped credential;
- first X sync persists real raw + normalized data from the retrievable remote window into the dedicated corpus;
- the local profile has one reconstructable stable remote-ID binding and later syncs preserve it even if the handle changes;
- deterministic tests prove a conflicting credential/remote account cannot be committed under that local profile;
- supported reply/quote references are preserved when present;
- full long-form authored text is normalized from `note_tweet.text` when present (deterministic fixture evidence is acceptable if the live retrievable window contains no such Post);
- durable checkpoint is present;
- second sync starts from prior state;
- local index rebuild succeeds;
- local list/search returns acquired posts;
- canonical corpus files are Git-visible while indexes/pending state are disposable;
- no bearer secret is found in tracked/local corpus data or pasteable diagnostics.

Once recorded, update issue #1 with the smoke-test result and close M0.
