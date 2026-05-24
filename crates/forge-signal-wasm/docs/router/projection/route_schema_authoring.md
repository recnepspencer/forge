# Route Schema Authoring

## What This Feature Is

This is the declaration lane for routes, layouts, and resolved router trees.

## Why You Use It

- declare route structure once
- attach search/hash, resources, breadcrumbs, admission, recovery, and forms
  authority at the route boundary
- give layouts and outlets one explicit composition model

## Stable Entry Points

- `signals.router.route(...)`
- `signals.router.layout(...)`
- `signals.router.define(...)`

## Core Mental Model

Route declarations are static schema. `signals.router.define(...)` resolves that
schema into a public tree of stable route references. Projection and admission
work from the resolved tree, not from the raw declaration object.

## How It Executes

1. declare routes and layouts
2. resolve them into a tree
3. use the resolved tree for build, match, project, admit, and history work

## Small Example

```ts
const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/items/:itemId"),
});
```

## Real Example

```ts
const routes = signals.router.define({
  app: signals.router.layout("/app", {
    outlet: "main",
    children: {
      dashboard: signals.router.route("/app/dashboard"),
      project: signals.router.route("/app/projects/:projectId", {
        search: {
          tab: signals.router.search.optional.string(),
        },
      }),
    },
  }),
});

console.log(routes.app.outletId);
console.log(routes.app.project.descriptor().routeId);
```

The declarations are authoritative. The resolved tree is the stable public
surface you use afterward.

## How It Relates To Other Features

- use [Path, Search, And Hash State](../authority/path_search_hash_state.md)
  for typed route input
- use [Projected Candidates](./projected_candidates.md) once you want to match
  authority against the tree

## Inspection And Debugging

- `routeRef.descriptor()`
- `layoutRef.outletId`
- resolved-tree property names for structural pathing

## Anti-Patterns

- using raw declaration objects as if they were resolved route references
- rebuilding route maps in app code after calling `define(...)`

## Current Limits

- declarations describe structure; they do not perform admission by themselves
- later docs cover prerequisites, recovery, resources, and breadcrumbs in
  detail

## Related Docs

- [Projected Candidates](./projected_candidates.md)
- [Layout Placement](./layout_placement.md)
- [Admit](../admission/admit.md)
