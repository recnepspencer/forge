# Draft Continuity

## What This Feature Is

This page covers the form-side explanation of what happened to the draft when
route authority changed.

## Why You Use It

- tell preserve from freeze
- tell discard from defer
- inspect whether draft bytes actually changed across the handoff

## Stable Entry Points

- `form.routeAuthority().summary.draftContinuity`
- `form.routeAuthority().history`
- `form.diagnosticsSummary().routeAuthority.draftContinuity`

## Core Mental Model

Route authority tells the form how to behave. Draft continuity explains what
that did to the draft.

Important reads:

- `posture`
- `authorityChange`
- `draftChanged`
- `draftResolution`

## How It Executes

1. route authority changes
2. the form applies the continuity posture
3. the draft continuity artifact records what actually happened

## Small Example

```ts
const continuity = form.routeAuthority().summary.draftContinuity;

console.log(continuity?.posture);
console.log(continuity?.draftResolution);
```

## Real Example

```ts
const summary = form.diagnosticsSummary().routeAuthority;

console.log(summary.handoff?.posture);
console.log(summary.draftContinuity?.posture);
console.log(summary.draftContinuity?.draftChanged);
console.log(summary.draftContinuity?.draftResolution);
```

## How It Relates To Other Features

- Read [Freeze, Discard, Defer, And Cleared Authority](./freeze-discard-defer-and-cleared-authority.md)
  for the meaning of each continuity lane.
- Read [State History](../diagnostics/state-history.md) if you need the
  lower-level draft-write history beside the route-authority summary.

## Inspection And Debugging

- `preservedValue` means the route handoff kept the draft value
- `preservedFrozenValue` means the draft stayed but editing became frozen
- `replacedFromSource` means the draft was replaced from current source truth
- `awaitingAdmittedTruth` means route-coupled behavior is deferred
- `authorityCleared` means the route authority was explicitly cleared

## Anti-Patterns

- assuming `draftChanged: false` means nothing important happened
- conflating `defer` with `cleared`

## Current Limits

- draft continuity explains the route authority transition, not every ordinary
  local edit

## Related Docs

- [Route Authority Handoff](./route-authority-handoff.md)
- [Freeze, Discard, Defer, And Cleared Authority](./freeze-discard-defer-and-cleared-authority.md)
