# Speculative Sessions

## What This Feature Is

A speculative session is the live branch-bound runtime surface created by
opening a speculative branch plan against a history implementation.

## Why You Use It

- run branch-backed commit or discard flows
- keep pending navigation bound to a real speculative branch id
- require explicit dirty-exit and preview artifacts before commit

## Stable Entry Points

- `plan.open(history)`
- `SpeculativeRouteBranchSession`
- `pendingBranch.resume(history)`

## Core Mental Model

The session is the active form of speculation. It owns:

- the origin branch
- the speculative branch
- the current pending outcome
- commit, discard, and dirty-exit operations

Once a session commits or discards, it becomes terminal.

## How It Executes

1. validate the history surface
2. read the current branch
3. create a speculative branch
4. switch into that branch
5. expose session lifecycle and pending outcome
6. allow commit, discard, or dirty-exit evaluation

## Small Example

```ts
const session = await plan.open(history);

console.log(session.originBranch().id);
console.log(session.branch().id);
console.log(session.outcome().kind);
```

## Real Example

```ts
const history = createHistoryStub();
const session = await routes.speculate("/users/u1")?.open(history);

console.log(session?.lifecycle());

const dirtyExit = await session?.dirtyExit(specialist);
const preview = await session?.commitPreview();
const commit = await session?.commit(preview, dirtyExit);

console.log(commit?.outcome().kind);
```

## How It Relates To Other Features

- session outcomes still surface [Route Outcomes](../admission/route_outcomes.md)
  when admission succeeds or redirects before open
- retained pending work is explained in
  [Discard And Keep Pending](./discard_and_keep_pending.md)

## Inspection And Debugging

- `session.lifecycle()`
- `session.originBranch()`
- `session.branch()`
- `session.outcome()`
- `session.verification()`

## Anti-Patterns

- reusing a session after commit or discard
- assuming branch ids can be invented by app code
- bypassing `dirtyExit(...)` before commit

## Current Limits

- sessions require a history surface with the full branch API
- commit and discard are intentionally explicit and artifact-driven
- a resumed pending branch is still a new active session

## Related Docs

- [Speculative Branch Plans](./speculative_branch_plans.md)
- [Commit Preview](./commit_preview.md)
- [Dirty Exit](./dirty_exit.md)
