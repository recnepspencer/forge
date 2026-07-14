# Restored Provenance

## What This Feature Is

Restored provenance is the breadcrumb trail you explicitly recover from
restore-backed history entries with `signals.router.restoreBreadcrumbs(...)`.

## Why You Use It

- preserve history-backed ancestry across restore flows
- outrank ordinary carried provenance when restore truth exists
- keep replay and restore availability attached to breadcrumb context

## Stable Entry Points

- `signals.router.restoreBreadcrumbs(...)`
- browser ingress or writeback `restoredBreadcrumbs`
- `entry.status === "restored"`

## Core Mental Model

Restored provenance is stronger than carried provenance because it is backed by
restore-capable history entries. The router treats it as a distinct truth class
instead of flattening it back into carry.

## How It Executes

1. collect restore-backed breadcrumb entries
2. wrap them with `restoreBreadcrumbs(...)`
3. send them through a boundary event
4. materialize restored entries on the destination trail

## Small Example

```ts
const restored = signals.router.restoreBreadcrumbs(
  story.breadcrumbTrail().entries,
);
```

## Real Example

```ts
const restoredBreadcrumbs = signals.router.restoreBreadcrumbs(
  story.breadcrumbTrail().entries,
);

const report = await routes.applyBrowserHistoryWriteback(
  signals.router.browserHistory.writeback.push("/search/results/r1", {
    routeIdentity: "resultRoute",
    restoredBreadcrumbs,
  }),
);
```

## How It Relates To Other Features

- restore capability comes from [Restore Boundaries](../restore/restore_boundaries.md)
- replay parity is covered in [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)

## Inspection And Debugging

- `entry.status === "restored"`
- `entry.sourceKind === "restoredProvenance"`
- `entry.provenance().restoreAvailability`
- `entry.provenance().replayAvailability`

## Anti-Patterns

- trying to restore breadcrumbs from entries that are not restore-backed
- flattening restored and carried provenance into one analytics bucket

## Current Limits

- `restoreBreadcrumbs(...)` fails closed when entries are not restore-backed
- restored provenance exists mainly at the history and browser-boundary lanes

## Related Docs

- [Carried Provenance](./carried_provenance.md)
- [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)
- [Restore Boundaries](../restore/restore_boundaries.md)
