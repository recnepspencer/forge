# Resource Settlement

## What This Feature Is

This page covers the settlement report for a resource-backed form.

## Why You Use It

- inspect whether a resource-backed change is pending, confirmed, failed, or
  absent
- understand retry posture and confirmation state
- keep settlement separate from dirty and validation state

## Stable Entry Points

- `form.resourceSource()?.settlement`

## Core Mental Model

Settlement is the runtime's read of what happened to the resource-backed write
or refresh flow. It is about confirmation and failure, not about whether the
user changed the draft.

## How It Executes

1. resource activity happens
2. the runtime derives settlement state from the line
3. the settlement read exposes confirmation, failure, and retry posture

## Small Example

```ts
console.log(form.resourceSource()?.settlement);
```

## Real Example

```ts
const settlement = form.resourceSource()?.settlement;

console.log(settlement?.kind);
console.log(settlement?.confirmationKind);
console.log(settlement?.retryRecommended);
console.log(settlement?.detail);
```

## How It Relates To Other Features

- Read [Mutation-Response Readback](./mutation-response-readback.md) when the
  settled write also has detailed mutation-response evidence.
- Read [Resource Drift](./resource-drift.md) when the source changed during or
  after settlement.

## Inspection And Debugging

- `kind` is the first read
- `retryRecommended` and `retryOperation` show the next operational move
- `visibleSelectionKind` helps explain what source truth the form is currently
  showing

## Anti-Patterns

- treating settlement like validation or action history
- deciding retry posture from UI state alone

## Current Limits

- exact resource transport semantics live in the resource docs

## Related Docs

- [Mutation-Response Readback](./mutation-response-readback.md)
- [Resource Drift](./resource-drift.md)
