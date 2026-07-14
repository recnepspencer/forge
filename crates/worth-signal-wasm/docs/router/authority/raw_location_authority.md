# Raw Location Authority

## What This Feature Is

Raw location authority is the explicit pre-canonical URL artifact for local
href-shaped input.

## Why You Use It

- preserve browser-shaped input before route matching
- create explicit browser-history ingress or writeback targets
- hand route projection a host-like location artifact instead of a bare string

## Stable Entry Points

- `signals.router.raw(...)`
- `signals.router.browserHistory.load(...)`
- `signals.router.browserHistory.push(...)`
- `signals.router.browserHistory.replace(...)`
- `signals.router.browserHistory.pop(...)`
- `signals.router.browserHistory.manual(...)`
- `signals.router.browserHistory.external(...)`

## Core Mental Model

Raw location authority is still authoritative, but it has not been fully
normalized into route truth yet. It is the right object when the host or caller
is saying "this is the location we saw or want to write," not "this is already
the canonical route story."

## How It Executes

1. capture a local path or href
2. attach navigation-type context when relevant
3. defer route normalization until projection, admission, or history processing

## Small Example

```ts
const raw = signals.router.raw("/projects/p7?tab=files", {
  navigationType: "push",
});

console.log(raw.href);
console.log(raw.navigationType);
```

## Real Example

```ts
const ingress = signals.router.browserHistory.pop("/projects/p7?tab=files", {
  routeIdentity: "project",
});

const report = await routes.admitBrowserHistoryIngress(ingress);

console.log(report.rawLocationHref);
console.log(report.diagnostics().boundaryArtifact);
```

The authoritative input is the raw location envelope. The route outcome is a
derived artifact produced by explicit browser-history admission.

## How It Relates To Other Features

- use [Canonical URL Authority](./canonical_url_authority.md) when you want
  normalized truth immediately
- use [Browser History Ingress](../history/browser_history_ingress.md) when the
  raw location is crossing the host boundary

## Inspection And Debugging

- `raw.href`
- `raw.navigationType`
- `raw.verification().rawLocationDigest`

## Anti-Patterns

- passing plain strings around deep call stacks when a raw location artifact is
  the actual contract
- pretending a browser-history event is admitted route truth before it runs
  through the router

## Current Limits

- raw location authority is local-router input, not full external URL
  generality
- route identity and outcome still require projection or admission

## Related Docs

- [Canonical URL Authority](./canonical_url_authority.md)
- [Browser History Ingress](../history/browser_history_ingress.md)
