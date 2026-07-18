# Browser History Story

A browser-history story retains the router reports your host records. From that
one stream it derives current route truth, back provenance, breadcrumbs,
inspection, and auditability.

```ts
const ingress = signals.router.browserHistory.load(window.location.href);
const report = await routes.admitBrowserHistoryIngress(ingress);
const story = signals.router.browserHistory.story(report);

console.log(story.current()?.href);
```

## The Host Still Owns The Browser

Worth creates typed ingress and writeback artifacts. It does not install a
`popstate` listener or call `history.pushState` for you.

In API vocabulary, the host packages **raw location authority** and the route
tree performs **browser-history admission** before the story records a report.

```ts
async function recordPopstate(href: string) {
  const ingress = signals.router.browserHistory.pop(href);
  const report = await routes.admitBrowserHistoryIngress(ingress);
  return story.record(report);
}
```

For an application-initiated navigation, create and admit the writeback first,
then let the host perform the browser mutation only when that is appropriate:

```ts
const writeback = signals.router.browserHistory.writeback.push(
  routes.projectDetail.to({ params: { projectId: "p7" } }),
  { routeIdentity: routes.projectDetail.descriptor().routeId },
);
const writebackReport = await routes.applyBrowserHistoryWriteback(writeback);

story.record(writebackReport);
const targetOutcome = writebackReport.outcome();

if (targetOutcome?.kind === "admitted") {
  window.history.pushState(null, "", writeback.targetHref);
}
```

## Reports Versus Retained Truth

Every report becomes a boundary event. Only admitted, converged or explicitly
drifted local route truth advances the retained route entry. A failed admission
or external escape remains visible as an event without pretending it became the
current app route.

Useful reads:

- `latestBoundaryEvent()` — the last thing seen at the boundary
- `currentRouteTruthEvent()` — the event that last advanced route truth
- `current()` and `back()` — retained admitted entries
- `inspection()` — what the story currently retains
- `auditability()` — why the current route is visible

The story is not the browser's native stack and is not durable storage. It only
knows the reports this application instance recorded.

Next: [Browser History Ingress](./browser_history_ingress.md),
[History Inspection](./history_inspection.md), and
[Navigation Auditability](./navigation_auditability.md).
