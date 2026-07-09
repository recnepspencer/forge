# Restore Boundaries

## What This Feature Is

Restore boundaries are exact snapshot artifacts that let route history or
breadcrumb return restore graph-owned route truth later.

## Why You Use It

- keep route return stronger than plain href navigation
- bind restore capability to an explicit snapshot boundary
- make restore availability inspectable

## Stable Entry Points

- `signals.router.restoreBoundary(signals.history().snapshot())`
- browser ingress or writeback `restoreBoundary`
- `entry.restoreBoundary()`

## Core Mental Model

The restore boundary is the authority-bearing artifact. It proves what exact
snapshot the router can restore later.

If a route-history or breadcrumb entry has no restore boundary, restore must
fail closed.

## How It Executes

1. capture a snapshot from history
2. wrap it with `signals.router.restoreBoundary(...)`
3. send it through browser ingress or writeback
4. store it on route-history or breadcrumb entries
5. use it later with `restore(...)`

## Small Example

```ts
const boundary = signals.router.restoreBoundary(
  signals.history().snapshot(),
);
```

## Real Example

```ts
const boundary = signals.router.restoreBoundary(signals.history().snapshot());

const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.load("/", {
    routeIdentity: "homeRoute",
    restoreBoundary: boundary,
  }),
);
```

## How It Relates To Other Features

- history-owned restore is used by [Back Provenance](../history/back_provenance.md)
- breadcrumb return is covered in
  [Breadcrumb Replay And Restore](../breadcrumbs/breadcrumb_replay_restore.md)

## Inspection And Debugging

- `boundary.guarantees()`
- `boundary.verification()`
- `entry.restoreBoundary()`

## Anti-Patterns

- treating `href` as enough to reproduce restore-backed route truth
- inventing restore-capable entries without a real boundary

## Current Limits

- restore boundaries depend on history snapshot support
- the router does not claim general restore outside the explicit snapshot
  boundary

## Related Docs

- [Restore Guarantees](./restore_guarantees.md)
- [Replay History](./replay_history.md)
