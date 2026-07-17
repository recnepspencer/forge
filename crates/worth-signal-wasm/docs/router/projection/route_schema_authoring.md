# Route Schema Authoring

Start small: declare paths once, call `define`, and use the resulting route
references everywhere you need links or matching. The same schema can later
carry layouts, admission, resources, breadcrumbs, and forms continuity without
changing the basic route-authoring model.

## Declare, Then Resolve

`signals.router.route(...)` and `signals.router.layout(...)` create declaration
objects. `signals.router.define(...)` resolves the whole declaration tree into
the public router surface.

```ts
const appRoute = signals.router.route("/app");

const routes = signals.router.define({
  home: signals.router.route("/"),
  app: signals.router.layout(appRoute, { outlet: "main" }, {
    dashboard: signals.router.route("/app/dashboard"),
    project: signals.router.route("/app/projects/:projectId", {
      search: {
        tab: signals.router.search.optional.string(),
      },
    }),
  }),
});
```

The third argument to `layout` is the child tree. It is not an option named
`children`. After `define`, `routes.app.project` is a typed route reference and
`routes.app.outletId` is the declared outlet.

## Use Route References, Not String Templates

```ts
const target = routes.app.project.to({
  params: { projectId: "p7" },
  search: { tab: "files" },
});

target.href;       // /app/projects/p7?tab=files
target.canonical();
target.plan();
```

Route references own parameter substitution and canonical URL construction.
That keeps links, matching, and navigation policy on one route identity.

## Keep Application Modules Boring

For a real application, put access rules, route declarations, and navigation
coordination in separate modules:

```text
routing/
  access.ts       admission sources and prerequisites
  routes.ts       route, layout, breadcrumb, and resource declarations
  session.ts      browser ingress, transitions, and story retention
```

The router supports inline callbacks, but business policy is easier to test and
reuse when it has a name and a home.

## What `define` Does Not Do

- It does not admit a route.
- It does not render an outlet.
- It does not call `window.history`.
- It does not fetch a resource merely because one is declared.

Continue with [Projected Candidates](./projected_candidates.md) to match a URL,
then [Admit](../admission/admit.md) to decide whether it may become route truth.
