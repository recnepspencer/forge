# Entry Bootstrap

## What This Feature Is

This page covers the declared dependencies that can keep a form from being
ready on first entry.

## Why You Use It

- wait for source admission, draft restore, or layout measurement before
  declaring the form ready
- inspect which dependency is still blocking entry
- keep entry readiness explicit instead of implicit in UI code

## Stable Entry Points

- `presentation: { entry: { bootstrap: ... } }`
- `form.presentation()`

## Core Mental Model

Entry bootstrap is the "can this form present as ready yet?" read. It does not
replace semantic form truth; it layers presentation readiness on top of it.

## How It Executes

1. entry bootstrap dependencies are declared
2. the runtime reads source, draft, validation, host, input, and layout
   readiness
3. the entry lane reports `ready`, `pending`, or `unavailable`

## Small Example

```ts
const entry = form.presentation().lanes.find((lane) => lane.lane === "entry");
console.log(entry?.bootstrap);
```

## Real Example

```ts
const entry = form.presentation().lanes.find((lane) => lane.lane === "entry");

console.log(entry?.bootstrap?.posture);
console.log(entry?.bootstrap?.dependencies.blocking);
console.log(entry?.bootstrap?.layoutMeasurementPending);
```

## How It Relates To Other Features

- Read [Host Facts](../interaction/host-facts.md) when host availability is
  part of entry readiness.
- Read [Layout Measurement](../layout/layout-measurement.md) when layout
  measurement is one of the declared bootstrap dependencies.

## Inspection And Debugging

- the entry lane's `bootstrap` read is the main entrypoint
- `dependencies.blocking` tells you exactly what is still holding entry open
- `hostUnavailableFacts` and `inputUnavailableFields` show common missing
  prerequisites

## Anti-Patterns

- declaring the form visually ready before bootstrap dependencies are met
- burying entry gating in renderer-local booleans

## Current Limits

- route-level entry behavior belongs in router docs

## Related Docs

- [Visible Lifecycle](./visible-lifecycle.md)
- [Host Facts](../interaction/host-facts.md)
