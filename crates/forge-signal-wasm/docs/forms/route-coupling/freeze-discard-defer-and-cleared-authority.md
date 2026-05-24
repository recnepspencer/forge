# Freeze, Discard, Defer, And Cleared Authority

## What This Feature Is

This page explains the route-coupled authority postures people actually need to
reason about:

- `preserve`
- `freeze`
- `discard`
- `defer`
- `cleared`

## Why You Use It

- understand how route transitions affect draft truth
- choose the right UI copy and disabled state
- avoid treating different route outcomes as if they meant the same thing

## Stable Entry Points

- `form.routeAuthority().summary.handoff`
- `form.routeAuthority().summary.draftContinuity`
- `form.diagnosticsSummary().routeAuthority.continuityAudit`

## Core Mental Model

- `preserve`: keep the draft and keep route-coupled behavior admitted
- `freeze`: keep the draft but block further route-coupled mutation
- `discard`: replace draft truth from current source truth
- `defer`: wait for later admitted route truth before enabling route-coupled
  behavior
- `cleared`: route authority was removed, not just deferred

## How It Executes

1. the router emits a forms authority artifact
2. the form lowers that into handoff and draft continuity
3. steps, actions, readiness, and diagnostics respond to the resulting posture

## Small Example

```ts
const summary = form.routeAuthority().summary;

console.log(summary.handoff?.posture);
console.log(summary.handoff?.routeCoupledBehavior);
console.log(summary.draftContinuity?.draftResolution);
```

## Real Example

```ts
const continuity = form.verification().routeAuthorityContinuity;

console.log(continuity.handoffPosture);
console.log(continuity.routeCoupledBehavior);
console.log(continuity.draftDisposition);
console.log(continuity.draftResolution);
console.log(continuity.blockingReason);
```

## How It Relates To Other Features

- Read [Route-Coupled Steps And Actions](./route-coupled-steps-and-actions.md)
  for the immediate submit and step impact.
- Read [Route Authority Verification](../verification/route-authority-verification.md)
  for the verification-package view of the same continuity state.

## Inspection And Debugging

- `routeCoupledBehavior: "admitted"` means route-coupled work can run
- `routeCoupledBehavior: "deferred"` means the form is waiting on later route
  truth
- `routeCoupledBehavior: "cleared"` means the route authority is gone
- `blockingReason` in the continuity audit keeps route-authority blockers
  distinct from unrelated host blockers

## Anti-Patterns

- treating `cleared` as if it were just another deferred state
- assuming `freeze` and `preserve` are the same because both keep draft bytes
- reading route-coupled denial from generic action status without inspecting the
  continuity audit

## Current Limits

- route-coupled authority only governs behavior that was actually declared as
  route-coupled

## Related Docs

- [Route-Coupled Steps And Actions](./route-coupled-steps-and-actions.md)
- [Continuity Audit](./continuity-audit.md)
