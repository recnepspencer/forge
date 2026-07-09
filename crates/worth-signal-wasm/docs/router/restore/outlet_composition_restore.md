# Outlet Composition Restore

## What This Feature Is

Outlet composition restore is the route-history feature that preserves admitted
layout and outlet structure across restore-backed back or breadcrumb return.

## Why You Use It

- keep nested layout truth honest after restore
- inspect multi-outlet route composition from route-history entries
- avoid reducing restore to route id alone

## Stable Entry Points

- `entry.outletComposition()`
- `inspection.currentOutletComposition()`
- `inspection.backOutletComposition()`

## Core Mental Model

Route restore is not only "which route id came back." It also includes the
admitted layout and outlet composition that made that route visible.

## How It Executes

1. admit a route with layouts and outlets
2. record history with a restore boundary
3. retain outlet composition on the route-history entry
4. expose that composition through history inspection and restore result

## Small Example

```ts
const composition = story.backProvenance().previous?.outletComposition();
console.log(composition?.summary());
```

## Real Example

```ts
const inspection = story.inspection();

console.log(inspection.currentOutletComposition()?.summary());
console.log(inspection.backOutletComposition()?.summary());
```

## How It Relates To Other Features

- layout authoring starts in [Layout Placement](../projection/layout_placement.md)
- restore boundaries are covered in [Restore Boundaries](./restore_boundaries.md)

## Inspection And Debugging

- `composition.summary()`
- `composition.verification()`
- `inspection.summary().currentOutletCompositionAvailable`
- `inspection.summary().backOutletCompositionAvailable`

## Anti-Patterns

- treating route restore as if layout and outlet structure were irrelevant
- rebuilding outlet composition from route ids after the fact

## Current Limits

- thinner worker-boundary fallback reports can fail closed to `null`
- outlet composition is a history inspection feature, not a generic route
  declaration helper

## Related Docs

- [Restore Boundaries](./restore_boundaries.md)
- [History Inspection](../history/history_inspection.md)
