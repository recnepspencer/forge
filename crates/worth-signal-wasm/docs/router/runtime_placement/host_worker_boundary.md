# Host And Worker Boundary

The host owns browser side effects. The Worth runtime owns typed route meaning.
The boundary between them is a small set of public ingress and writeback
artifacts.

## Browser To Router

```ts
async function admitCurrentLocation(navigationKind: "load" | "pop") {
  const ingress = navigationKind === "load"
    ? signals.router.browserHistory.load(window.location.href)
    : signals.router.browserHistory.pop(window.location.href);

  return routes.admitBrowserHistoryIngress(ingress, admissionFacts);
}
```

The report says whether browser authority converged with admitted route truth,
drifted from it, or did not admit. Recording that report advances the router
story only when the report honestly contains route truth.

## Router To Browser

```ts
const target = routes.settings.to();
const writeback = signals.router.browserHistory.writeback.push(target, {
  routeIdentity: target.routeId,
});
const report = await routes.applyBrowserHistoryWriteback(writeback);

story.record(report);

if (report.outcome()?.kind === "admitted") {
  window.history.pushState(null, "", writeback.targetHref);
}
```

`applyBrowserHistoryWriteback` validates and explains the intended writeback.
It does not call the browser. This split is intentional: browser mutation is a
host capability and route admission remains inspectable before it happens.

## What Crosses The Boundary

Ingress and writeback may carry route identity, coherence, runtime continuity,
breadcrumbs, or an exact restore boundary. Missing evidence stays missing;
worker-first placement does not grant richer authority than the envelope
actually carries.

Do not call raw bridge APIs from application code or build a second worker-only
router story. Use the same public router artifacts in both deployments.

Next: [Browser Authority Coherence](../boundaries/browser_authority_coherence.md),
[Browser History Story](../history/browser_history_story.md), and
[Worker History Fallback](./worker_history_fallback.md).
