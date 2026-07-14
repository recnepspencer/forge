# Carried Provenance

## What This Feature Is

Carried provenance is the explicit breadcrumb trail you move forward through a
navigation boundary with `signals.router.carryBreadcrumbs(...)`.

## Why You Use It

- preserve dynamic ancestry that is not reconstructable from the deep link
- keep search-result or workspace context across in-app navigation
- make carried breadcrumb truth explicit instead of ambient

## Stable Entry Points

- `signals.router.carryBreadcrumbs(...)`
- `carry: true` in `breadcrumbParent(...)`
- browser-history ingress or writeback `carriedBreadcrumbs`

## Core Mental Model

Carried provenance is not canonical route truth. It is explicit navigation
provenance. A route consumes it only when that route opted into the carried
lane.

## How It Executes

1. materialize a breadcrumb trail
2. wrap it with `carryBreadcrumbs(...)`
3. send it through browser writeback or another boundary
4. let a later route opt into consuming it
5. materialize breadcrumb entries with `status: "carried"`

## Small Example

```ts
const carried = signals.router.carryBreadcrumbs(story.breadcrumbTrail());
```

## Real Example

```ts
const carriedBreadcrumbs = signals.router.carryBreadcrumbs(story.breadcrumbTrail());

const report = await routes.applyBrowserHistoryWriteback(
  signals.router.browserHistory.writeback.push("/search/results/r55", {
    routeIdentity: "result:r55",
    carriedBreadcrumbs,
  }),
);
```

## How It Relates To Other Features

- parent opt-in lives in [Breadcrumb Parent Strategies](./breadcrumb_parent_strategies.md)
- restore-backed carried crumbs are covered in
  [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)

## Inspection And Debugging

- `entry.status === "carried"`
- `entry.sourceKind === "carriedProvenance"`
- `entry.provenance().sourceKind`
- `report.carriedBreadcrumbs()`

## Anti-Patterns

- treating carried breadcrumbs as canonical route ancestry
- consuming carried provenance on routes that did not opt in
- auto-carrying every breadcrumb trail through every navigation

## Current Limits

- carried provenance is explicit and keyed only by the artifact you pass
- carried entries do not automatically gain restore authority unless they came
  from restore-backed history entries

## Related Docs

- [Breadcrumb Parent Strategies](./breadcrumb_parent_strategies.md)
- [Restored Provenance](./restored_provenance.md)
- [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)
