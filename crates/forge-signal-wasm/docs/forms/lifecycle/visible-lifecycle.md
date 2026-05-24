# Visible Lifecycle

## What This Feature Is

This page covers the visible lifecycle lanes reported by `form.presentation()`.

## Why You Use It

- inspect whether a presentation lane is pending, busy, settling, ready,
  failed, or unavailable
- keep visible state separate from semantic form state
- understand acknowledgement-dependent lanes like action or exit presentation

## Stable Entry Points

- `form.presentation()`
- `form.presentationLifecycle(laneId)`

## Core Mental Model

Visible lifecycle is about what the user can currently see or acknowledge, not
what the form means semantically. A form can be semantically up to date while a
layout or message lane is still settling.

## How It Executes

1. presentation policies are declared
2. the runtime derives per-lane lifecycle state
3. acknowledgement and dependency state are tracked where required

## Small Example

```ts
console.log(form.presentation().summary);
```

## Real Example

```ts
const actionLane = form.presentation().lanes.find((lane) => lane.lane === "action");

console.log(actionLane?.status);
console.log(actionLane?.acknowledgement);
console.log(actionLane?.dependencies);
```

## How It Relates To Other Features

- Read [Entry Bootstrap](./entry-bootstrap.md) for the entry-specific case.
- Read [Exit Posture](./exit-posture.md) when the visible lifecycle question is
  about leaving the surface.

## Inspection And Debugging

- `presentation().lanes` shows the current lane set
- `summary` shows counts by status
- `acknowledgements` shows how many lanes are waiting for acknowledgement

## Anti-Patterns

- treating visible lifecycle status as if it were validation or readiness
- assuming every lane settles immediately with semantic updates

## Current Limits

- route-level lifecycle coordination belongs in router docs

## Related Docs

- [Entry Bootstrap](./entry-bootstrap.md)
- [Exit Posture](./exit-posture.md)
