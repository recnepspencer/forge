# State History

## What This Feature Is

This page covers `form.stateHistory()`, the retained history of raw input and
draft writes.

## Why You Use It

- inspect how the draft changed field by field
- see whether a retained entry came from raw input or draft writes
- line up draft changes with dirty, patch, and readiness digests

## Stable Entry Points

- `form.stateHistory()`
- `form.diagnostics().stateHistory`

## Core Mental Model

State history is lower-level than diagnostics history. It focuses on writes,
not on summary snapshots.

## How It Executes

1. a raw input or draft write happens
2. the runtime retains a `formStateHistory` artifact
3. the artifact records field, operation, digest transitions, and write source

## Small Example

```ts
for (const artifact of form.stateHistory()) {
  console.log(artifact.field, artifact.entryKind, artifact.operation);
}
```

## Real Example

```ts
const latest = form.stateHistory().at(-1);

console.log({
  field: latest?.field,
  source: latest?.source,
  previousDraftDigest: latest?.previousDraftDigest,
  nextDraftDigest: latest?.nextDraftDigest,
  patchPlanDigest: latest?.patchPlanDigest,
  readinessDigest: latest?.readinessDigest,
});
```

## How It Relates To Other Features

- Read [Dirty State](../changes/dirty-state.md) for the semantic dirty layer
  derived from these writes.
- Read [Diagnostics History](./diagnostics-history.md) for the higher-level
  summary timeline.

## Inspection And Debugging

- `entryKind` distinguishes raw input from draft writes
- `patchPlanDigest` and `readinessDigest` show the larger effect of the write
- `source` and `reason` help explain why the write happened

## Anti-Patterns

- treating state history as if it were the same thing as diagnostics history
- assuming every state history entry means submit became available

## Current Limits

- state history explains writes; it does not replace full validation,
  availability, or action histories

## Related Docs

- [Dirty State](../changes/dirty-state.md)
- [Diagnostics History](./diagnostics-history.md)
