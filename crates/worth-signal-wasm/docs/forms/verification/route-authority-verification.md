# Route Authority Verification

## What This Feature Is

This page covers the route-authority continuity check inside
`form.verification()`.

## Why You Use It

- certify preserve, freeze, discard, defer, and cleared behavior
- inspect route-coupled step/action counts through the proof package
- compare the verification audit to the diagnostics summary audit

## Stable Entry Points

- `form.verification().routeAuthorityContinuity`
- `form.verification().digests.routeAuthorityContinuityDigest`

## Core Mental Model

Route-authority verification is the verification-package twin of the
diagnostics continuity audit.

## How It Executes

1. route authority summary and route-coupled counts are derived
2. the continuity audit is materialized
3. verification retains the audit plus its digest

## Small Example

```ts
const continuity = form.verification().routeAuthorityContinuity;

console.log(continuity.handoffPosture);
console.log(continuity.blockingReason);
```

## Real Example

```ts
const verification = form.verification();

console.log({
  continuity: verification.routeAuthorityContinuity,
  digest: verification.digests.routeAuthorityContinuityDigest,
});
```

## How It Relates To Other Features

- Read [Continuity Audit](../route-coupling/continuity-audit.md) for the
  diagnostics-facing version.
- Read [Freeze, Discard, Defer, And Cleared Authority](../route-coupling/freeze-discard-defer-and-cleared-authority.md)
  for the posture meanings.

## Inspection And Debugging

- verify that the digest matches the continuity audit digest
- inspect route-coupled action denial and step availability through the proof
  package when the current UI is ambiguous

## Anti-Patterns

- reading generic action denial first when the issue is clearly route-coupled
- treating `cleared` as if it were the same thing as `deferred`

## Current Limits

- this proof lane only exists when the form actually participates in route
  authority

## Related Docs

- [Continuity Audit](../route-coupling/continuity-audit.md)
- [Freeze, Discard, Defer, And Cleared Authority](../route-coupling/freeze-discard-defer-and-cleared-authority.md)
