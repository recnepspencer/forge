# History Inspection

## What This Feature Is

History inspection is the summary surface that unifies current entry, back
provenance, breadcrumb provenance, outlet composition, and coherence evidence.

## Why You Use It

- answer most retained-history questions from one read surface
- inspect restore/replay availability without walking every lower artifact

## Stable Entry Points

- `story.inspection()`
- `inspection.summary()`
- `inspection.currentOutletComposition()`
- `inspection.backOutletComposition()`
- `inspection.breadcrumbProvenance()`

## Core Mental Model

Inspection is the story-level inventory and summary surface. It is richer than
one entry or one breadcrumb, but narrower than the final closeout explanation
surface.

## How It Executes

1. story gathers current, back, and breadcrumb retained truth
2. inspection derives summary counts and availability posture
3. callers read the inspection instead of reconstructing the story manually

## Small Example

```ts
const inspection = story.inspection();

console.log(inspection.summary().currentEntryAvailable);
```

## Real Example

```ts
const inspection = story.inspection();

console.log(inspection.summary());
console.log(inspection.currentOutletComposition());
console.log(inspection.breadcrumbProvenance());
```

## How It Relates To Other Features

- use [Navigation Auditability](./navigation_auditability.md) when you need the
  route-first closeout explanation surface
- use [Route History Entries](./route_history_entries.md) when you need one
  specific retained entry instead of the aggregate read

## Inspection And Debugging

- `summary()`
- `verification()`
- current and back outlet composition reads
- breadcrumb provenance reads

## Anti-Patterns

- reconstructing retained-history counts from raw story lists in app code
- using inspection as if it were the final boundary-explanation artifact

## Current Limits

- inspection is history-centered; `auditability()` is the higher route-visibility
  explanation layer

## Related Docs

- [Navigation Auditability](./navigation_auditability.md)
- [Route History Entries](./route_history_entries.md)
