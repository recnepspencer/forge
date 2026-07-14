# Browser History Writeback

## What This Feature Is

Browser-history writeback is the explicit host envelope for graph-issued local
or external history mutation.

## Why You Use It

- keep graph-issued history writes explicit
- distinguish local route-truth writeback from external escape
- preserve coherence, restore, and breadcrumb provenance during writeback

## Stable Entry Points

- `signals.router.browserHistory.writeback.push(...)`
- `signals.router.browserHistory.writeback.replace(...)`
- `signals.router.browserHistory.writeback.external(...)`
- `routes.applyBrowserHistoryWriteback(...)`

## Core Mental Model

Writeback is the inverse boundary of ingress. It records what the runtime wants
the host history to do, then the router can certify whether local route truth
converged or escaped.

## How It Executes

1. create a writeback envelope
2. apply it through the resolved tree
3. receive a writeback report
4. optionally record it in a browser-history story

## Small Example

```ts
const writeback = signals.router.browserHistory.writeback.push("/projects/p7", {
  routeIdentity: "project",
});

const report = await routes.applyBrowserHistoryWriteback(writeback);
```

## Real Example

```ts
const report = await routes.applyBrowserHistoryWriteback(
  signals.router.browserHistory.writeback.external("https://example.com/docs"),
);

console.log(report.diagnostics().boundaryArtifact);
console.log(report.outcome());
```

## How It Relates To Other Features

- use [Browser History Ingress](./browser_history_ingress.md) for host-to-router
  movement
- use [Route History Entries](./route_history_entries.md) when a local admitted
  writeback should become retained route truth

## Inspection And Debugging

- `report.diagnostics()`
- `report.coherence()`
- `report.restoreBoundary()`
- `report.verification()`

## Anti-Patterns

- mutating browser history outside the writeback surface and expecting retained
  router story to stay honest
- using local writeback without a `routeIdentity`

## Current Limits

- external writeback is an honest escape boundary; it does not fabricate local
  route truth

## Related Docs

- [Browser History Ingress](./browser_history_ingress.md)
- [Route History Entries](./route_history_entries.md)
