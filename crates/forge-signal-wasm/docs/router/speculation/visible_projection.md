# Visible Projection

## What This Feature Is

Visible projection is the explicit runtime artifact that says whether a
speculative candidate is hidden, visible while pending, visible after commit,
or not visible at all.

## Why You Use It

- keep pending route visibility honest
- avoid guessing from branch status alone
- support products that preserve visible truth until commit

## Stable Entry Points

- `outcome.visibleProjection()`
- `visiblePosture` in `speculate(...)`

## Core Mental Model

Speculation has two separate truths:

- candidate route truth
- currently visible route truth

Visible projection is the contract between them. It explains whether candidate
truth is still suppressed, already visible, or has become visible only after
commit.

## How It Executes

1. read visible posture from the plan
2. combine it with current branch lifecycle state
3. emit one visible projection state

## Small Example

```ts
const pending = await routes.speculate("/users/u1", {
  visiblePosture: "preserve-visible-until-commit",
})?.evaluate();

console.log(pending?.visibleProjection().state);
```

## Real Example

```ts
const plan = routes.speculate("/users/u1", {
  visiblePosture: "allow-visible-flicker-before-commit",
});

const session = await plan?.open(history);
const pendingProjection = session?.outcome().visibleProjection();

console.log(pendingProjection?.state);
console.log(pendingProjection?.href);
```

## How It Relates To Other Features

- visible projection is a property of [Speculative Outcomes](./speculative_outcomes.md)
- route-resource continuity after ordinary navigation lives in
  [Route Resources](../resources/README.md), not here

## Inspection And Debugging

- `visibleProjection().posture`
- `visibleProjection().state`
- `visibleProjection().routeId`
- `visibleProjection().href`
- `visibleProjection().verification()`

## Anti-Patterns

- inferring visible state from `outcome.kind` alone
- assuming pending always means invisible
- using visible posture as if it changed branch commit semantics

## Current Limits

- the surface is focused on speculation-specific visibility
- redirects before branch open yield `candidateNotVisible`

## Related Docs

- [Speculative Outcomes](./speculative_outcomes.md)
- [Speculative Branch Plans](./speculative_branch_plans.md)
