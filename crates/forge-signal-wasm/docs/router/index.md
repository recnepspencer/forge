# Router Overview

## What This Feature Is

The router is the typed navigation surface for `forge-signal-wasm`. You declare
routes and layouts with `signals.router.*(...)`, resolve a route tree with
`signals.router.define(...)`, and then use that resolved tree to project, admit,
speculate, restore, and inspect navigation truth.

## Why You Use It

- declare route meaning once instead of rebuilding it from strings
- keep URL authority, route admission, and browser-history truth in one system
- give layouts, outlets, resources, breadcrumbs, and forms one shared route
  vocabulary
- inspect why a route is visible without spelunking private framework glue

## Stable Entry Points

- `signals.router.route(...)`
- `signals.router.layout(...)`
- `signals.router.define(...)`
- `routeRef.to(...)`
- `routes.simulateSequence([...])`
- `routes.project(...)`
- `routes.admit(...)`
- `signals.router.browserHistory.*(...)`

## Core Mental Model

Think in layers:

1. **Authority**: raw or canonical URL truth
2. **Projection**: "which route shape matches this authority?"
3. **Admission**: "is that route allowed, redirected, denied, or recovered?"
4. **History**: "what boundary event made this route visible?"

The router owns those layers. Your app code mostly holds:

- `RouteReference` values when it is building links
- `ProjectedRouteCandidate` values when it is previewing or warming a route
- `RouteOutcome` values when it is admitting or inspecting visible truth
- browser-history artifacts when it is crossing the host boundary explicitly

## How It Executes

For the ordinary lane:

1. declare routes and layouts
2. resolve them with `signals.router.define(...)`
3. build or match a `RouteLocation`
4. project with `routes.project(...)` or admit with `routes.admit(...)`
5. inspect route, outlet, breadcrumb, history, or boundary artifacts

For browser-driven navigation:

1. create an ingress or writeback envelope with `signals.router.browserHistory`
2. pass it through `routes.admitBrowserHistoryIngress(...)` or
   `routes.applyBrowserHistoryWriteback(...)`
3. record the report in `signals.router.browserHistory.story()`
4. read `inspection()` or `auditability()` when you need explanation surfaces

For ordered route rehearsal and playback:

1. build a sequence from typed route locations
2. call `routes.simulateSequence([...])`
3. run it once
4. inspect `result.steps`, `result.story`, `result.replay.*()`, and
   `result.diagnostics()`

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/items/:itemId"),
});

const detail = routes.detail.to({ params: { itemId: "i1" } });
const projected = routes.project(detail.href);

console.log(detail.href);
console.log(projected?.route().routeId);
```

This is the smallest honest example because it shows the core flow without
skipping the resolved-tree boundary.

## Real Example

```ts
const signals = await createSignals();

const routes = signals.router.define({
  app: signals.router.layout("/app", {
    outlet: "main",
    children: {
      dashboard: signals.router.route("/app/dashboard"),
      project: signals.router.route("/app/projects/:projectId", {
        breadcrumb: signals.router.breadcrumb({
          id: "project",
          label: ({ params }) => `Project ${params.projectId}`,
        }),
      }),
    },
  }),
});

const ingress = signals.router.browserHistory.load("/app/projects/p7", {
  routeIdentity: "project",
});

const report = await routes.admitBrowserHistoryIngress(ingress);
const story = signals.router.browserHistory.story(report);

console.log(report.outcome().kind);
console.log(story.current()?.breadcrumbTrail()?.entries);
console.log(story.auditability().summary());
```

For sequence rehearsal, the route tree now owns the ordinary playback lane too:

```ts
const scenario = routes.simulateSequence([
  routes.home.to(),
  routes.project.to({ params: { projectId: "p7" } }),
]);

const result = await scenario.run();

console.log(result.steps.map((step) => step.report.outcome().routeId));
console.log(result.replay.breadcrumbTrail().map((trail) => trail.entries));
console.log(result.story.current()?.routeId);
console.log(result.diagnostics().notAdmitted);
```

Authoritative truth is the browser-history ingress. The route outcome is
derived from that authority. The story retains the admitted route-history entry
and boundary explanation automatically. Sequence rehearsal uses the same
boundary truth, but without forcing the app to hand-author the ingress/admit/
record loop for every step.

## How It Relates To Other Features

- Read [Authority](./authority/README.md) when you need canonical URL truth or
  route equivalence.
- Read [Projection](./projection/README.md) when you are choosing routes before
  admission.
- Read [Admission](./admission/README.md) when prerequisites, recovery, or
  route outcomes matter.
- Read [History](./history/README.md) when host boundaries, replay, restore, or
  explanation surfaces matter.
- Read [Recovery](./recovery/README.md) when stale deep links need declared
  nearest-valid fallback.
- Read [Speculation](./speculation/README.md) when route truth should stay
  pending behind branch-native commit or discard.
- Read [Breadcrumbs](./breadcrumbs/README.md) when breadcrumb ancestry depends
  on declarations, carry, or restore provenance.
- Read [Restore And Replay](./restore/README.md) when exact snapshot return or
  route-history replay matters.
- Read [Route Resources](./resources/README.md) when routes own native resource
  lines, prefetch, or warmup.
- Read [Transitions](./transitions/README.md) when you need explicit route
  change, continuity, and visible-truth behavior.
- Read [Boundaries](./boundaries/README.md) when hydration or browser authority
  ownership matters.
- Read [Runtime Placement](./runtime_placement/README.md) when the host-worker
  boundary or worker fallback semantics matter.
- Read [Router And Forms](./forms/README.md) when route authority drives form
  continuity or route-coupled behavior.
- Read [Diagnostics And Proof](./diagnostics/README.md) when you need the
  explanation, provenance, and verification surfaces that sit above the other
  router features.

## Inspection And Debugging

The main inspection surfaces are:

- `canonicalArtifact.verification()`
- `candidate.verification()`
- `outcome.diagnostics()`
- `outcome.provenance()`
- `story.inspection()`
- `story.auditability()`

Use the cheapest surface that answers your question first. `auditability()` is
the "why is this route visible now?" lane.

## Anti-Patterns

- treating `signals.router` as a string helper layer
- rebuilding browser-history explanation in app code instead of recording
  boundary reports in a story
- assuming route authoring changes lower runtime truth ownership
- reading milestone plans to learn day-to-day usage instead of the product docs

## Current Limits

- these docs cover the stable router surface that is already implemented
- some later feature groups still need dedicated docs, even though the runtime
  support already exists
- compatibility posture exists, but the intended default product direction is
  still worker-first execution

## Related Docs

- [Router Docs Home](./README.md)
- [Authority](./authority/README.md)
- [Projection](./projection/README.md)
- [Admission](./admission/README.md)
- [History](./history/README.md)
- [Recovery](./recovery/README.md)
- [Speculation](./speculation/README.md)
- [Breadcrumbs](./breadcrumbs/README.md)
- [Restore And Replay](./restore/README.md)
- [Route Resources](./resources/README.md)
- [Transitions](./transitions/README.md)
- [Boundaries](./boundaries/README.md)
- [Runtime Placement](./runtime_placement/README.md)
- [Router And Forms](./forms/README.md)
- [Diagnostics And Proof](./diagnostics/README.md)
