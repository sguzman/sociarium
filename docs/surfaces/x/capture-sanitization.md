# X capture sanitization rules

This file defines the public/private boundary for R2/X evidence.

## Secret/authority classes: never public

Always redact/remove values for:

- `Cookie`;
- `Set-Cookie`;
- `Authorization`;
- `x-csrf-token`;
- OAuth access/refresh tokens;
- authorization codes;
- PKCE verifier/challenge material when security-sensitive;
- session IDs;
- device/session secrets;
- anti-abuse proof values;
- account recovery/security values.

Header/field **names** may be documented.

## Browser "sanitized HAR" is still private

Direct observation on 2026-09-18 with Microsoft Edge 153 showed that Edge's sanitized HAR export omitted ordinary `Cookie`, `Authorization`, and `Set-Cookie` material from the captured file but **retained non-empty `x-csrf-token` request-header values**.

Therefore:

- do not treat the browser's "sanitized" label as a publication guarantee;
- sanitized HAR files remain private evidence;
- never commit them directly;
- extract and review structural observations before publication;
- continue removing/redacting `x-csrf-token` and other authority-bearing values explicitly if creating any secondary sanitized artifact.

This is a dated observation of Edge 153 behavior, not a claim about every browser/exporter version.

## Private-content classes: default private

Do not publish raw values for:

- Direct Messages;
- bookmark contents;
- private Lists;
- protected-account content;
- private notification bodies;
- private email/phone/account settings;
- IP/security metadata.

## Usually safe structural evidence

After review, public docs may normally include:

- HTTP method;
- hostname/path pattern;
- GraphQL operation name;
- dated query ID;
- variable names;
- feature-flag names;
- response field names;
- cursor field names;
- status codes;
- generic error codes;
- already-public self-owned Post IDs;
- already-public self-owned user ID;
- already-public canonical Post/profile URLs;
- browser/version;
- capture hash.

## Stable IDs

A numeric X user ID or Post ID is not a credential.

However, only publish it when:

- it belongs to the operator or already-public content relevant to the observation; and
- publishing it adds real technical value.

Do not unnecessarily publish IDs of private third parties.

## Raw HAR rule

The public repo stores **no raw HAR files**, even after superficial redaction.

Reason:

HAR is easy to incompletely sanitize and may contain authority/private material in:

- headers;
- query strings;
- POST bodies;
- response bodies;
- redirect targets;
- cookies sections;
- embedded JSON.

Publish extracted structural observations instead.

## Corrections

If sensitive material is accidentally committed:

1. treat the value as compromised if it grants authority;
2. rotate/revoke it where applicable;
3. remove it from repository history, not merely the latest file;
4. document the incident without reproducing the secret.

Prevention is substantially better than repository-history cleanup.
