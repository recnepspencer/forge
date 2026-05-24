# Projection Verification

## What This Feature Is

Projection verification is the digest package emitted by projected candidates
and related route-authority artifacts.

## Why You Use It

- compare projected truth in hostile tests
- carry proof-bearing identity into higher-level diagnostics
- verify that different paths converged on the same projection result

## Stable Entry Points

- `canonical.verification()`
- `routeRef.verification()`
- `candidate.verification()`

## Core Mental Model

Verification packages are proof surfaces, not primary runtime state. They exist
so tests and diagnostics can compare route truth without re-deriving it from
private internals.

## How It Executes

1. projection computes canonical and structural facts
2. those facts are sealed into stable verification packages
3. tests and diagnostics read the package instead of reconstructing truth

## Small Example

```ts
const projected = routes.project("/projects/p7");

console.log(projected?.verification());
```

## Real Example

```ts
const routes = signals.router.define({
  projectRoute: signals.router.route("/projects/:projectId"),
});

const fromHref = routes.project("/projects/p7");
const fromLocation = routes.project(
  routes.projectRoute.to({ params: { projectId: "p7" } }).href,
);

console.log(
  fromHref?.verification().projectedCandidateDigest ===
    fromLocation?.verification().projectedCandidateDigest,
);
```

## How It Relates To Other Features

- use [History Inspection](../history/history_inspection.md) when you need
  retained story-level verification
- use [Navigation Auditability](../history/navigation_auditability.md) for the
  final closeout explanation surface

## Inspection And Debugging

- projected candidate digests
- canonical route verification packages
- route-reference verification packages

## Anti-Patterns

- treating verification packages as an everyday business-data read surface
- reconstructing projection facts in tests that already have a digest package

## Current Limits

- verification tells you what was proven, not how to admit or render the route

## Related Docs

- [Projected Candidates](./projected_candidates.md)
- [History Inspection](../history/history_inspection.md)
