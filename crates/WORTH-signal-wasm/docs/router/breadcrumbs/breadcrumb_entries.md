# Breadcrumb Entries

## What This Feature Is

Breadcrumb entries are the concrete breadcrumb artifacts produced from route
declarations, recomputation, carried provenance, restored provenance, fallback,
or history fallback.

## Why You Use It

- render visible crumb labels and targets
- inspect whether a crumb is resolved, carried, restored, or fallback
- keep breadcrumb UI tied to explicit route authority

## Stable Entry Points

- `RouteBreadcrumbEntry`
- `RouteBreadcrumbTrail`
- `entry.provenance()`

## Core Mental Model

A breadcrumb entry is already materialized route-facing truth. It is not the
same thing as a breadcrumb declaration.

It tells you:

- what crumb is visible
- where it points
- why it exists

## How It Executes

1. materialize route-owned crumb declarations
2. resolve parent strategy if present
3. assign entry status and source kind
4. retain target and provenance on each entry

## Small Example

```ts
const trail = outcome.route().breadcrumbTrail();

console.log(trail?.entries[0].label);
console.log(trail?.entries[0].status);
```

## Real Example

```ts
const trail = story.breadcrumbTrail();

console.log(trail.entries.map((entry) => ({
  crumbId: entry.crumbId,
  label: entry.label,
  status: entry.status,
  sourceKind: entry.sourceKind,
  targetHref: entry.targetHref,
})));
```

## How It Relates To Other Features

- provenance detail is deeper in [Carried Provenance](./carried_provenance.md)
  and [Restored Provenance](./restored_provenance.md)
- history story adds restore and replay authority in
  [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)

## Inspection And Debugging

- `entry.crumbId`
- `entry.label`
- `entry.status`
- `entry.sourceKind`
- `entry.targetKind`
- `entry.targetHref`
- `entry.provenance()`

## Anti-Patterns

- assuming all entries are route-declared
- rebuilding target URLs from label text
- losing `sourceKind` when debugging odd trail behavior

## Current Limits

- plain `RouteBreadcrumbEntry` exposes target and provenance, but restore and
  replay helpers become richer once the entry is wrapped by history story

## Related Docs

- [Breadcrumb Declarations](./breadcrumb_declarations.md)
- [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)
