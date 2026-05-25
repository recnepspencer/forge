# Worker-First Default

## What This Feature Is

Worker-first default is the intended deployment posture where the router runtime
executes in a worker unless you intentionally choose a compatibility posture.

## Why You Use It

- keep the router on the product's default runtime lane
- understand which boundary semantics are normal, not optional
- know when you are relying on fallback rather than full route capability

## Stable Entry Points

- router deployment choice at runtime creation
- worker bridge router boundary surfaces

## Core Mental Model

Worker-first is the default runtime lane, not a separate router product. These
docs exist because default placement still has explicit boundary semantics.

## How It Executes

1. route truth lives in the worker runtime
2. host-owned browser and capability events cross a bridge
3. the worker mirrors or reconstructs the public router surfaces it can support

## Small Example

```ts
const signals = await createSignals({ deployment: "workerFirst" });
```

## Real Example

```ts
const bridge = createWorkerRuntimeBridge();

const report = await bridge.admitBrowserHistoryIngress({
  navigationKind: "popstate",
  rawLocation: "/search?q=forge",
  routeIdentity: "searchRoute:forge",
});
```

## How It Relates To Other Features

- the actual bridge surface is covered in [Host And Worker Boundary](./host_worker_boundary.md)
- reduced route-history fallback is covered in [Worker History Fallback](./worker_history_fallback.md)

## Inspection And Debugging

- worker bridge browser-history reports
- worker browser-history story
- worker inspection and auditability summaries

## Anti-Patterns

- thinking worker-first means there is no boundary to document
- treating compatibility posture as the default mental model

## Current Limits

- some worker surfaces can be reduced compared with the main runtime and fail
  closed explicitly

## Related Docs

- [Host And Worker Boundary](./host_worker_boundary.md)
- [Worker Navigation Auditability](./worker_navigation_auditability.md)
