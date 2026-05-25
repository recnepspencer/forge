# Diagnostics History

## What This Feature Is

This page covers `form.diagnosticsHistory()`, the retained timeline of summary
state changes.

## Why You Use It

- inspect how summary state changed over time
- diff route-authority continuity transitions
- connect retained summary snapshots to verification digests

## Stable Entry Points

- `form.diagnosticsHistory()`
- `form.diagnosticsSummary()`

## Core Mental Model

Diagnostics history retains summary-shaped snapshots only when diagnostics truth
materially changes. It is not a wall-clock polling log.

## How It Executes

1. current diagnostics summary changes
2. a retained diagnostics history artifact is added
3. the artifact records the relevant digest set and selected route-authority
   fields

## Small Example

```ts
const history = form.diagnosticsHistory();

console.log(history.at(-1)?.summaryDigest);
console.log(history.at(-1)?.routeAuthorityTransitionKind);
```

## Real Example

```ts
for (const artifact of form.diagnosticsHistory()) {
  console.log({
    observedAtMs: artifact.observedAtMs,
    summaryDigest: artifact.summaryDigest,
    routeAuthorityTransitionKind: artifact.routeAuthorityTransitionKind,
    routeAuthorityHandoffPosture: artifact.routeAuthorityHandoffPosture,
  });
}
```

## How It Relates To Other Features

- Read [Diagnostics Summary](./diagnostics-summary.md) for the current-state
  version of the same information.
- Read [Source Compatibility History](./source-compatibility-history.md) when
  you need the lower-level compatibility timeline.

## Inspection And Debugging

- use `summaryDigest` to line up a history entry with a current or retained
  summary
- use the route-authority fields on the artifact when you need continuity
  transition details without separately replaying route authority history

## Anti-Patterns

- assuming every observation creates a new history entry
- treating `observedAtMs` as part of semantic identity

## Current Limits

- diagnostics history stores summary-shaped change truth, not every lower-level
  artifact family in full detail

## Related Docs

- [Diagnostics Summary](./diagnostics-summary.md)
- [Source Compatibility History](./source-compatibility-history.md)
