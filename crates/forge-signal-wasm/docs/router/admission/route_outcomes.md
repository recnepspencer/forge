# Route Outcomes

## What This Feature Is

Route outcomes are the terminal result artifacts returned by route admission and
browser-boundary admission flows.

## Why You Use It

- branch on admitted versus non-admitted route truth explicitly
- inspect denial, redirect, or missing-route behavior without guessing

## Stable Entry Points

- `outcome.kind`
- `outcome.route()`
- `outcome.artifact()`
- `outcome.diagnostics()`
- `outcome.recovery()`
- `outcome.provenance()`

## Core Mental Model

The router does not flatten every non-success into one failure bucket. It
distinguishes:

- `admitted`
- `redirect`
- `notFound`
- `forbidden`
- `unavailable`
- `denied`

## How It Executes

1. admission evaluates the candidate
2. if admitted, the outcome carries route/layout truth
3. otherwise, the outcome carries a terminal admission artifact

## Small Example

```ts
const outcome = await routes.admit("/missing");

if (outcome.kind === "notFound") {
  console.log(outcome.artifact().reason);
}
```

## Real Example

```ts
const outcome = await routes.admit("/projects/p7", {
  actorRole: "viewer",
});

switch (outcome.kind) {
  case "admitted":
    console.log(outcome.route().routeId);
    break;
  case "redirect":
  case "forbidden":
  case "unavailable":
  case "denied":
  case "notFound":
    console.log(outcome.artifact().kind, outcome.diagnostics());
    break;
}
```

## How It Relates To Other Features

- use [Forbidden, Unavailable, And Denied](./forbidden_unavailable_denied.md)
  for the non-admitted posture distinctions
- use [Browser History Ingress](../history/browser_history_ingress.md) when the
  same outcomes need boundary provenance

## Inspection And Debugging

- `outcome.kind`
- `outcome.diagnostics()`
- `outcome.provenance()`
- `outcome.verification()`

## Anti-Patterns

- collapsing all non-admitted outcomes into one generic "blocked" path
- assuming a redirect is semantically the same as forbidden or unavailable

## Current Limits

- only admitted outcomes expose route/layout/outlet truth directly

## Related Docs

- [Admit](./admit.md)
- [Forbidden, Unavailable, And Denied](./forbidden_unavailable_denied.md)
