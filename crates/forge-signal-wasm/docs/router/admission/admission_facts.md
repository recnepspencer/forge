# Admission Facts

## What This Feature Is

Admission facts are the caller-supplied facts object passed into route
admission.

## Why You Use It

- provide route-specific policy context that is not a host/resource/graph
  admission source
- keep per-admission facts explicit instead of ambient

## Stable Entry Points

- `routes.admit(..., facts)`
- `routes.admitBrowserHistoryIngress(..., facts)`
- `routes.admitHydrationHandoff(..., facts)`
- `routes.applyBrowserHistoryWriteback(..., facts)`

## Core Mental Model

Facts are per-admission input, not durable router authority. They complement
declared admission sources when the caller knows something the route needs for
this one evaluation.

## How It Executes

1. the caller passes a plain facts object
2. prerequisites and recovery steps may read it
3. plan and outcome provenance record which fact keys were involved

## Small Example

```ts
const outcome = await routes.admit("/settings", {
  role: "admin",
});
```

## Real Example

```ts
const plan = routes.project("/projects/p7")?.admission({
  role: "viewer",
  workspace: "acme",
});

console.log(plan?.provenance().factsKeys);
```

## How It Relates To Other Features

- use [Prerequisites](./prerequisites.md) for declared source consumption
- use [Access Policy](./access_policy.md) for the overall admission model

## Inspection And Debugging

- `plan.provenance().factsKeys`
- `outcome.provenance()`

## Anti-Patterns

- treating facts as hidden global context
- stuffing durable resource truth into ad hoc facts instead of declared
  admission sources

## Current Limits

- facts are caller-owned and per-evaluation
- they do not replace host, graph, or resource admission sources

## Related Docs

- [Prerequisites](./prerequisites.md)
- [Access Policy](./access_policy.md)
