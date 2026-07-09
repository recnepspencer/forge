# Breadcrumb Declarations

## What This Feature Is

Breadcrumb declarations are the route- and layout-owned breadcrumb definitions
you attach with `signals.router.breadcrumb(...)`.

## Why You Use It

- declare breadcrumb meaning beside route meaning
- avoid rebuilding crumbs from URL segments
- give routes one stable crumb id, label, and target posture

## Stable Entry Points

- `signals.router.breadcrumb(...)`
- `signals.router.breadcrumbEntry(...)`
- `route(..., { breadcrumb: ... })`

## Core Mental Model

Breadcrumbs are route truth, not decorative string arrays. Each route owns its
own crumb contribution. The router composes the trail later.

## How It Executes

1. declare a crumb id and label
2. optionally declare a target
3. attach the declaration to a route
4. let projection, admission, or history materialize real breadcrumb entries

## Small Example

```ts
const projectRoute = signals.router.route("/projects/:projectId", {
  breadcrumb: signals.router.breadcrumb({
    id: "project",
    label: ({ params }) => `Project ${params.projectId}`,
  }),
});
```

## Real Example

```ts
const resultRoute = signals.router.route("/search/results/:resultId", {
  breadcrumb: signals.router.breadcrumb({
    id: "result",
    label: ({ params }) => `Result ${params.resultId}`,
    parent: signals.router.breadcrumbParent({
      fallback: signals.router.breadcrumbEntry({
        id: "search-results",
        label: "Search Results",
        target: "/search",
      }),
    }),
  }),
});
```

## How It Relates To Other Features

- parent strategy is covered in [Breadcrumb Parent Strategies](./breadcrumb_parent_strategies.md)
- restore and replay authority is covered in
  [Breadcrumb Replay And Restore](./breadcrumb_replay_restore.md)

## Inspection And Debugging

- `projectedRoute.breadcrumb()`
- `projectedRoute.breadcrumbTrail()`
- `admittedRoute.breadcrumb()`
- `admittedRoute.breadcrumbTrail()`

## Anti-Patterns

- deriving crumbs by splitting path segments
- using labels as crumb identity
- forcing child routes to rebuild the entire trail

## Current Limits

- breadcrumb labels are synchronous strings on the public route breadcrumb
  declaration surface
- advanced ancestry comes from parent strategies and provenance, not from one
  callback that invents the whole trail

## Related Docs

- [Breadcrumb Entries](./breadcrumb_entries.md)
- [Breadcrumb Parent Strategies](./breadcrumb_parent_strategies.md)
