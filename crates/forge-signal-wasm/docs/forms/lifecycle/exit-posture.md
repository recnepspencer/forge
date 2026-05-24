# Exit Posture

## What This Feature Is

This page covers the controller's exit state for route, modal, and external
surfaces.

## Why You Use It

- inspect whether exit is clean, dirty, waiting on actions, or unavailable
- explain why leaving the form requires confirmation
- keep exit state on the controller instead of scattering it across UI code

## Stable Entry Points

- `form.exit()`
- `form.presentation()`

## Core Mental Model

Exit posture is about leaving the current surface safely. It reflects dirty
state, pending actions, and source availability without collapsing them into
one generic "are you sure?" flag.

## How It Executes

1. exit updates are reported
2. the runtime derives current exit state and history
3. summary exposes guard kind, confirmation needs, and pending actions

## Small Example

```ts
console.log(form.exit().summary);
```

## Real Example

```ts
console.log(form.exit().current);
console.log(form.exit().summary.guardKind);
console.log(form.exit().summary.requiresConfirmation);
```

## How It Relates To Other Features

- Read [Visible Lifecycle](./visible-lifecycle.md) for the broader presentation
  picture around exit.
- Read [Dirty State](../changes/dirty-state.md) when the guard is dirty-state
  driven.

## Inspection And Debugging

- `summary.guardKind` is the main read
- `pendingActions` explains action-driven exit delay
- `history` shows how exit state changed over time

## Anti-Patterns

- modeling exit as a single local modal flag
- hiding pending-action exit guards inside submit handlers

## Current Limits

- route-level exit recovery belongs in router docs

## Related Docs

- [Visible Lifecycle](./visible-lifecycle.md)
- [Dirty State](../changes/dirty-state.md)
