# Route Resource Declarations

Route resources connect a route to an existing Worth resource family. They do
not create a router cache or a second source of server truth.

```ts
const projectDetail = signals.router.resourceLine(projectFamily, {
  params: ({ params }) => ({ projectId: params.projectId }),
  prefetch: "intent",
});

const routes = signals.router.define({
  project: signals.router.route("/projects/:projectId", {
    resources: { detail: projectDetail },
  }),
});
```

`params` maps canonical route input into the resource family's normal `line`
parameters. `prefetch` declares the expected trigger: `hover`, `focus`,
`viewport`, or `intent`.

## Projection Can Warm; Admission Can Use

Before admission, the projected route exposes preview capabilities:

```ts
const candidate = routes.project("/projects/p7");

if (candidate) {
  const prefetched = candidate.route().resource("detail").prefetch("intent");

  try {
    console.log(prefetched.current().status);
  } finally {
    prefetched.free();
  }
}
```

After admission, the capability exposes the native resource line:

```ts
const outcome = await routes.admit("/projects/p7");

if (outcome.kind === "admitted") {
  const detail = outcome.route().resource("detail");
  console.log(detail.line(), detail.current().freshness);
}
```

The family owns fetch, cache, freshness, invalidation, and line identity. The
router owns the mapping from route truth to that line and records how it was
prefetched or used during admission and transition.

Prefetch artifacts own a lifetime. Call `free()` or use explicit disposal when
the preview is no longer needed. Do not keep a parallel application cache just
because the resource was reached through a route.

Next: [Projected Resource Capabilities](./projected_resource_capabilities.md),
[Admitted Resource Capabilities](./admitted_resource_capabilities.md), and
[Resource Warmup](./resource_warmup.md).
