# Discard And Keep Pending

## What This Feature Is

Discard posture controls whether abandoning a speculative session ends the
branch immediately or leaves the branch pending so a later session can resume
it.

## Why You Use It

- choose between terminal discard and resumable pending work
- keep branch lifecycle explicit for review-heavy or interrupted flows
- explain why a discard still leaves candidate route truth pending

## Stable Entry Points

- `discardPosture` in `speculate(...)`
- `session.discard()`
- `discard.pendingBranch()`
- `pendingBranch.resume(history)`

## Core Mental Model

Discard does not always mean "throw everything away." The posture decides
whether discard:

- abandons the branch
- or keeps the speculative branch pending for later resumption

That choice is part of the plan, not an accidental UI behavior.

## How It Executes

1. open a speculative session
2. call `discard()`
3. emit a discard artifact and outcome
4. optionally expose a pending branch artifact for later resume

## Small Example

```ts
const plan = routes.speculate("/users/u1", {
  discardPosture: "keep-branch-pending",
});
```

## Real Example

```ts
const session = await routes.speculate("/users/u1", {
  discardPosture: "keep-branch-pending",
})?.open(history);

const discard = await session?.discard();
const pendingBranch = discard?.pendingBranch();
const resumed = await pendingBranch?.resume(history);
```

## How It Relates To Other Features

- discard outcomes are one form of [Speculative Outcomes](./speculative_outcomes.md)
- visible pending state is still explained by [Visible Projection](./visible_projection.md)

## Inspection And Debugging

- `discard.disposition`
- `discard.outcome().diagnostics()`
- `discard.pendingBranch()`
- `pendingBranch.verification()`

## Anti-Patterns

- assuming discard is always terminal
- losing the pending branch artifact when the product expects resumption
- letting UI guess whether a branch can be resumed

## Current Limits

- pending-branch resume still requires the same history surface family
- discarded sessions stay terminal even if the branch remains pending elsewhere

## Related Docs

- [Speculative Sessions](./speculative_sessions.md)
- [Speculative Outcomes](./speculative_outcomes.md)
