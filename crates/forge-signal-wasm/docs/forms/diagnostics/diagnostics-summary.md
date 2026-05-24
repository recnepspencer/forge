# Diagnostics Summary

## What This Feature Is

This page covers `form.diagnosticsSummary()`, the compact public read of the
form's current state.

## Why You Use It

- inspect dirty, patch, readiness, validation, actions, steps, and route
  authority from one place
- drive debugging UIs or developer tooling without materializing full history
- compare summary truth with `form.diagnostics()` when you need the full read

## Stable Entry Points

- `form.diagnosticsSummary()`
- `form.diagnostics()`

## Core Mental Model

`diagnosticsSummary()` is the lighter current-state read.
`diagnostics()` is the fuller report.

They agree on current truth, but the summary stays smaller and easier to carry.

## How It Executes

1. the runtime derives current-state summaries for each feature area
2. `diagnosticsSummary()` packages those summaries together
3. `diagnostics()` exposes the same current state plus the full adjacent reports

## Small Example

```ts
const summary = form.diagnosticsSummary();

console.log(summary.dirty);
console.log(summary.readiness);
console.log(summary.validation.summary);
```

## Real Example

```ts
const summary = form.diagnosticsSummary();

console.log({
  dirty: summary.dirty,
  patch: summary.patch,
  readiness: summary.readiness,
  routeAuthority: summary.routeAuthority,
  histories: summary.histories,
});
```

## How It Relates To Other Features

- Read [Diagnostics History](./diagnostics-history.md) for retained snapshots
  over time.
- Read [Continuity Audit](../route-coupling/continuity-audit.md) for the
  route-authority portion of the summary.

## Inspection And Debugging

- `summary.digest` changes when the summary changes
- `summary.histories` gives counts of retained history families
- `summary.routeAuthority` exposes the route-authority continuity audit without
  requiring the full `routeAuthority()` report

## Anti-Patterns

- using `diagnosticsSummary()` when you actually need per-operation history
- assuming the summary hides route-authority truth; it intentionally includes a
  compact route-authority section

## Current Limits

- the summary is current-state shaped, not a chronological log

## Related Docs

- [Diagnostics History](./diagnostics-history.md)
- [Verification Packages](../verification/verification-packages.md)
