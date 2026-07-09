# Speculative Branch Plans

## What This Feature Is

A speculative branch plan is an explicit navigation plan for a route that
should stay pending until history branch work commits or is discarded.

## Why You Use It

- preview route admission before changing visible truth
- bind speculative navigation to branch-native history work
- make commit, discard, and visibility posture explicit up front

## Stable Entry Points

- `routes.speculate(...)`
- `candidate.speculate(...)`
- `SpeculativeRouteBranchPlan`

## Core Mental Model

The plan is the cheap declaration surface. It says:

- which candidate route is being speculated
- how a branch should open
- whether commit needs preview
- whether discard abandons or keeps the branch pending
- whether visible truth stays preserved before commit

The plan is not yet an active session.

## How It Executes

1. project a candidate route
2. compile a speculative branch plan
3. optionally evaluate the plan against admission facts
4. optionally open the plan against history to get a live session

## Small Example

```ts
const plan = routes.speculate("/users/u1", {
  commitPosture: "merge-preview-before-commit",
  discardPosture: "discard-speculative-branch",
  visiblePosture: "preserve-visible-until-commit",
});
```

## Real Example

```ts
const candidate = routes.project("/users/u1");
const plan = candidate?.speculate({
  branchName: "speculative-user-detail",
  commitPosture: "merge-preview-before-commit",
  discardPosture: "keep-branch-pending",
  visiblePosture: "preserve-visible-until-commit",
});

console.log(plan?.branching());
console.log(plan?.diagnostics());

const outcome = await plan?.evaluate({ signedIn: true });
console.log(outcome?.kind);
console.log(outcome?.diagnostics());
```

## How It Relates To Other Features

- plans start from [Projected Candidates](../projection/projected_candidates.md)
- admitted route truth still comes from [Route Outcomes](../admission/route_outcomes.md)
- visible pending behavior is explained by [Visible Projection](./visible_projection.md)

## Inspection And Debugging

- `plan.branching()`
- `plan.diagnostics()`
- `plan.verification()`
- `plan.evaluate(...)`

## Anti-Patterns

- treating a plan as if it were already a live speculative session
- hiding commit and discard posture behind app-specific defaults
- using speculation when plain direct navigation is enough

## Current Limits

- plans do not open history branches until `open(...)`
- invalid lifecycle posture fails closed
- speculative routing is intentionally more explicit and more expensive-looking
  than ordinary navigation

## Related Docs

- [Speculative Sessions](./speculative_sessions.md)
- [Speculative Outcomes](./speculative_outcomes.md)
- [Visible Projection](./visible_projection.md)
