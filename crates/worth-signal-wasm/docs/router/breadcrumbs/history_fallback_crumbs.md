# History Fallback Crumbs

## What This Feature Is

History fallback crumbs are breadcrumb entries the browser-history story can
produce when richer declared route breadcrumb capability is unavailable.

## Why You Use It

- keep breadcrumb trails honest on thinner boundary reports
- retain replay and restore authority from history entries even without route
  breadcrumb declarations
- fail closed into a weaker explicit trail instead of crashing or pretending

## Stable Entry Points

- `story.breadcrumbTrail()`
- `entry.sourceKind === "historyFallback"`

## Core Mental Model

History fallback crumbs are not the preferred lane. They are the explicit
degradation path when the story can only rely on route-history entries.

That weaker trail is still useful because it preserves history-owned authority.

## How It Executes

1. record boundary events in a browser-history story
2. detect that richer breadcrumb capability is absent
3. derive crumb entries from route-history entries
4. preserve per-entry replay and restore authority

## Small Example

```ts
const trail = story.breadcrumbTrail();
console.log(trail.entries[0].sourceKind);
```

## Real Example

```ts
const inspection = story.inspection();

console.log(inspection.summary().historyFallbackBreadcrumbPresent);
console.log(
  inspection.breadcrumbTrail.entries.map((entry) => ({
    crumbId: entry.crumbId,
    sourceKind: entry.sourceKind,
    targetHref: entry.targetHref,
  })),
);
```

## How It Relates To Other Features

- richer declared lanes start in [Breadcrumb Declarations](./breadcrumb_declarations.md)
- restore and replay still flow through
  [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)

## Inspection And Debugging

- `entry.sourceKind`
- `entry.provenance().sourceKind`
- `inspection.summary().historyFallbackBreadcrumbPresent`

## Anti-Patterns

- treating history fallback crumbs as equivalent to declared route crumbs
- inferring that fallback means breadcrumbs are broken

## Current Limits

- history fallback is intentionally thinner than route declaration plus
  ancestry-strategy breadcrumbs
- the feature exists mainly at browser-history and worker-boundary seams

## Related Docs

- [Breadcrumb Entries](./breadcrumb_entries.md)
- [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)
