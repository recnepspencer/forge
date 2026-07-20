# Breadcrumb Declarations

Declare each breadcrumb beside the route that gives it meaning. The router can
then compose, carry, restore, and explain the trail without guessing from URL
segments.

```ts
const routes = signals.router.define({
  projects: signals.router.route("/projects", {
    breadcrumb: signals.router.breadcrumb({
      id: "projects",
      label: "Projects",
      target: "/projects",
    }),
  }),
  project: signals.router.route("/projects/:projectId", {
    breadcrumb: signals.router.breadcrumb({
      id: "project",
      label: ({ params }) => `Project ${params.projectId}`,
    }),
  }),
});
```

An id is stable identity. A label is presentation. A target is where the crumb
leads. Do not use the label as identity or split the pathname to manufacture a
trail.

## Declaring A Parent Strategy

A deep route can recompute, carry, or fall back to parent context:

```ts
const resultBreadcrumb = signals.router.breadcrumb({
  id: "search-result",
  label: ({ params }) => `Result ${params.resultId}`,
  parent: signals.router.breadcrumbParent({
    carry: true,
    fallback: signals.router.breadcrumbEntry({
      id: "search",
      label: "Search",
      target: "/search",
    }),
  }),
});
```

The happy path uses the declared route trail. Carry and fallback exist for
navigation paths where the current URL alone cannot reproduce useful ancestry.

## Read Provenance, Not Just Labels

Materialized entries report `status` and `sourceKind`: a crumb may be resolved,
recomputed, carried, restored, or fallback truth. `entry.provenance()` tells you
which kind you have and whether restore or replay is available.

```ts
const candidate = routes.project("/projects/p7");
const trail = candidate?.route().breadcrumbTrail();

for (const entry of trail?.entries ?? []) {
  console.log(entry.label, entry.provenance().sourceKind);
}
```

Breadcrumbs remain synchronous declarations. Fetch display names before
admission through resource truth, or carry/restore a previously materialized
label; do not hide network work inside the label callback.

Next: [Breadcrumb Parent Strategies](./breadcrumb_parent_strategies.md),
[Carried Provenance](./carried_provenance.md), and
[Restored Provenance](./restored_provenance.md).
