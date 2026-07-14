# Handoffs

## What This Feature Is

This page covers the handoff presentation lane for route, modal, and external
handoff surfaces.

## Why You Use It

- inspect current handoff state from the controller
- keep open, return, dismiss, and close updates on a typed surface
- understand when handoff presentation is unavailable or superseded

## Stable Entry Points

- `form.handoff()`
- `form.presentation()`

## Core Mental Model

Handoffs are presentation events about moving the user into or out of another
surface. They are not the same thing as route authority or route-coupled form
state.

## How It Executes

1. handoff updates are reported
2. the runtime derives current handoff state and history
3. the broader presentation report can also surface the handoff lane

## Small Example

```ts
console.log(form.handoff().summary);
```

## Real Example

```ts
console.log(form.handoff().current);
console.log(form.handoff().history);
console.log(form.presentation().lanes.find((lane) => lane.lane === "handoff"));
```

## How It Relates To Other Features

- Read [External Lanes](./external-lanes.md) when the handoff target is an
  external surface.
- Read [Exit Posture](./exit-posture.md) when the question is about leaving the
  current surface instead.

## Inspection And Debugging

- `handoff().summary` is the fastest read
- `current` shows the current target, operation, and status
- `history` shows superseded or unavailable handoff updates over time

## Anti-Patterns

- treating handoff updates like ordinary validation or message state
- keeping a second modal or external-surface status store beside the form

## Current Limits

- route-authority continuity lives in the later route-coupling docs

## Related Docs

- [Exit Posture](./exit-posture.md)
- [External Lanes](./external-lanes.md)
