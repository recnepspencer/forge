# Speculative Outcomes

## What This Feature Is

Speculative outcomes are the unified route-facing results produced by
speculative evaluation, active sessions, commits, discards, and pre-branch
redirect or denial.

## Why You Use It

- read one outcome shape across pending, committed, discarded, and redirected
  speculation
- inspect branch lifecycle results without dropping back to raw history details
- keep visible projection tied to the same outcome

## Stable Entry Points

- `plan.evaluate(...)`
- `session.outcome()`
- `commit.outcome()`
- `discard.outcome()`

## Core Mental Model

A speculative outcome is not just "pending or committed." It is the route truth
plus lifecycle explanation for where speculation is right now.

Use it when the UI needs to understand both route-facing semantics and branch
status.

## How It Executes

1. build or resume a speculative branch
2. resolve admission if needed
3. map the result into one speculative outcome kind
4. expose visible projection and lifecycle diagnostics beside it

## Small Example

```ts
const outcome = await plan.evaluate({ signedIn: true });

console.log(outcome.kind);
console.log(outcome.routeOutcome()?.kind);
```

## Real Example

```ts
const redirectOutcome = await routes.speculate("/users/u1")?.evaluate({
  signedIn: false,
});

console.log(redirectOutcome?.kind);
console.log(redirectOutcome?.diagnostics().branchDisposition);
console.log(redirectOutcome?.visibleProjection().state);
```

## How It Relates To Other Features

- speculative outcomes wrap ordinary [Route Outcomes](../admission/route_outcomes.md)
  when route admission already exists
- pending visibility belongs to [Visible Projection](./visible_projection.md)

## Inspection And Debugging

- `outcome.diagnostics()`
- `outcome.routeOutcome()`
- `outcome.visibleProjection()`
- `outcome.verification()`

## Anti-Patterns

- reading only `kind` and ignoring `diagnostics()`
- assuming all speculative outcomes have an admitted route outcome
- using route outcome kind alone to infer branch lifecycle

## Current Limits

- speculative outcomes are focused on route and branch semantics, not on
  general-purpose history introspection
- some outcomes have `routeOutcome() === null`, especially after commit or
  discard lifecycle-only steps

## Related Docs

- [Speculative Sessions](./speculative_sessions.md)
- [Visible Projection](./visible_projection.md)
- [Discard And Keep Pending](./discard_and_keep_pending.md)
