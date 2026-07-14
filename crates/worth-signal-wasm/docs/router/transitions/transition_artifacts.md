# Transition Artifacts

## What This Feature Is

Transition artifacts are the route-facing results returned by
`routes.transition(...)`.

## Why You Use It

- inspect what target route the transition produced
- explain why visible route truth changed the way it did
- keep continuity policy and transition source explicit

## Stable Entry Points

- `routes.transition(currentOutcome, target, options?)`
- `RouteTransitionArtifact`

## Core Mental Model

A transition artifact is the explanation surface for one route change. It tells
you:

- where the transition started
- what target route outcome it produced
- why visible truth changed the way it did

## How It Executes

1. start from an admitted current route outcome
2. admit or reuse the target route truth
3. apply continuity and visibility policy
4. return one transition artifact with diagnostics

## Small Example

```ts
const home = await routes.admit("/");
const transition = home.kind === "admitted"
  ? await routes.transition(home, "/about")
  : null;
```

## Real Example

```ts
const home = await routes.admit("/");

if (home.kind === "admitted") {
  const transition = await routes.transition(home, "/private", {
    facts: { auth: "anonymous" },
  });

  console.log(transition.target().kind);
  console.log(transition.diagnostics().requestedSource);
  console.log(transition.diagnostics().visibleChangeSource);
}
```

## How It Relates To Other Features

- prefetched targets are covered in [Prefetch Admission](./prefetch_admission.md)
- resource-backed pending continuity is covered in
  [Continuity Preservation](./continuity_preservation.md)

## Inspection And Debugging

- `transition.target()`
- `transition.diagnostics()`
- `transition.verification()`

## Anti-Patterns

- treating the target route outcome as enough without reading transition
  diagnostics
- rebuilding visibility policy outside the router

## Current Limits

- transitions require a current admitted route outcome as the starting point
- this surface is about route transitions, not general UI animation

## Related Docs

- [Pending Visibility](./pending_visibility.md)
- [Prefetch Admission](./prefetch_admission.md)
