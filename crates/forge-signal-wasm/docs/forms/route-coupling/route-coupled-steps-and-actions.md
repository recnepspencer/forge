# Route-Coupled Steps And Actions

## What This Feature Is

This page covers the visible behavior changes on steps and actions when route
authority changes.

## Why You Use It

- understand why route-coupled steps are unavailable
- understand why route-coupled actions are denied even when the form is
  otherwise healthy
- keep route-coupled denial separate from unrelated host or validation denial

## Stable Entry Points

- `form.steps()`
- `form.actions()`
- `form.diagnosticsSummary().routeAuthority.continuityAudit`
- `form.verification().routeAuthorityContinuity`

## Core Mental Model

Route authority is not just metadata. It can directly change whether a
route-coupled step or action is active, unavailable, accepted, or denied.

## How It Executes

1. the form receives route authority
2. route-coupled step and action posture is recomputed
3. diagnostics and verification summarize the route-coupled counts

## Small Example

```ts
const audit = form.diagnosticsSummary().routeAuthority.continuityAudit;

console.log(audit.routeCoupledSteps);
console.log(audit.routeCoupledActions);
```

## Real Example

```ts
const verification = form.verification().routeAuthorityContinuity;

console.log(verification.routeCoupledSteps.unavailable);
console.log(verification.routeCoupledActions.denied);
console.log(verification.blockingReason);
```

## How It Relates To Other Features

- Read [Freeze, Discard, Defer, And Cleared Authority](./freeze-discard-defer-and-cleared-authority.md)
  for the posture meanings behind these counts.
- Read [Action Verification](../verification/action-verification.md) for the
  wider action proof package.

## Inspection And Debugging

- route-coupled step counts live on the continuity audit
- route-coupled action counts live on the continuity audit
- the blocking reason is intentionally route-authority-scoped, not a generic
  first blocker from anywhere in the form

## Anti-Patterns

- inferring route-coupled denial from host blockers alone
- assuming a denied route-coupled action means every action is denied

## Current Limits

- only route-coupled steps and actions participate in this audit

## Related Docs

- [Continuity Audit](./continuity-audit.md)
- [Action Verification](../verification/action-verification.md)
