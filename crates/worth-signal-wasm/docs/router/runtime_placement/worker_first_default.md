# Worker-First Default

Worth's default deployment posture is worker-first, and the public
`signals.router` namespace is available in both deployment modes.

```ts
const signals = await createSignals({ deployment: "workerFirst" });
const routes = signals.router.define({
  home: signals.router.route("/"),
});
```

That is the useful starting point. It does **not** mean browser APIs moved into
the worker, or that every JavaScript callback declared on a route is proven to
execute there.

## The Honest Boundary

The browser host observes `location`, `popstate`, clicks, and external
navigation. It packages those observations with the public router namespace:

```ts
const ingress = signals.router.browserHistory.pop(window.location.href);
const report = await routes.admitBrowserHistoryIngress(ingress);
story.record(report);
```

Worth admits that typed ingress against the route tree and returns a report.
The host remains responsible for installing listeners, rendering the result,
and performing `pushState`, `replaceState`, or external navigation.

## Compatibility Is Explicit

Use `mainThreadCompatibility` when the application intentionally needs that
deployment:

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
```

Construction should fail explicitly when the requested worker deployment is
unavailable. A silent fallback would make performance and placement impossible
to reason about.

Public router docs intentionally teach `signals.router` and resolved route-tree
methods, not raw worker bridge methods. The bridge is runtime infrastructure,
not the application authoring surface.

Next: [Host And Worker Boundary](./host_worker_boundary.md) and
[Worker Navigation Auditability](./worker_navigation_auditability.md).
