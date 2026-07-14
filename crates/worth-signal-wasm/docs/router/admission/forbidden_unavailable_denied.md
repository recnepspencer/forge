# Forbidden, Unavailable, And Denied

## What This Feature Is

This page explains the non-admitted outcome kinds that are easy to flatten
together but should stay distinct.

## Why You Use It

- keep user-facing route behavior honest
- preserve why a route failed instead of collapsing everything into "no access"

## Stable Entry Points

- `outcome.kind`
- `outcome.artifact()`
- `outcome.diagnostics()`

## Core Mental Model

- `forbidden`: the route exists, but authority says no
- `unavailable`: the route exists, but required capability or prerequisite truth
  is unavailable right now
- `denied`: the route exists, but the route was blocked by an explicit denial
  artifact rather than a permission or availability posture

`redirect` and `notFound` are related but different outcome lanes.

## How It Executes

These kinds are produced by prerequisite or recovery decisions during admission.

## Small Example

```ts
if (outcome.kind === "forbidden") {
  console.log("Show a permission explanation");
}
```

## Real Example

```ts
switch (outcome.kind) {
  case "forbidden":
    console.log("User lacks authority");
    break;
  case "unavailable":
    console.log("Dependency or host truth is unavailable");
    break;
  case "denied":
    console.log("Explicit route denial artifact");
    break;
}
```

## How It Relates To Other Features

- use [Route Outcomes](./route_outcomes.md) for the whole outcome family
- use [Browser History Ingress](../history/browser_history_ingress.md) when
  these results need boundary provenance

## Inspection And Debugging

- `outcome.artifact().reason`
- `outcome.diagnostics().prerequisiteDecisions`

## Anti-Patterns

- mapping `unavailable` to `forbidden`
- assuming `denied` is only a permission error

## Current Limits

- exact artifact detail still depends on what the triggering prerequisite or
  recovery surface returned

## Related Docs

- [Route Outcomes](./route_outcomes.md)
- [Prerequisites](./prerequisites.md)
