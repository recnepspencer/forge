# Speculative Sessions

A speculative session binds a projected route to a real history branch. That
lets an application prepare route truth and then merge or discard it without
pretending the preview already became visible truth.

```ts
const plan = routes.speculate("/projects/p7", {
  branchName: "project-p7-preview",
  commitPosture: "merge-preview-before-commit",
  visiblePosture: "preserve-visible-until-commit",
});

if (plan) {
  const session = await plan.open(history);
  console.log(session.originBranch().id, session.branch().id);
}
```

`history` must implement `SpeculativeRouteBranchHistory`. The router does not
invent branch ids or emulate merges in JavaScript. In a full Worth runtime this
is the branch-capable history authority supplied by the application.

## Commit Honestly

A robust commit asks the specialist whether the branch is dirty, builds a merge
preview, and carries confirmation when it is required.

```ts
const dirtyExit = await session.dirtyExit(specialist);
const confirmation = dirtyExit.confirm();
const preview = await session.commitPreview();

if (!dirtyExit.confirmationRequired || confirmation) {
  const commit = await session.commit(preview, dirtyExit, confirmation);
  console.log(commit.outcome().kind);
}
```

The confirmation artifact proves that the application acknowledged dirty work;
it is not a boolean bypass. A committed or discarded session is terminal and
must not be reused.

## Discard Or Keep Pending

`session.discard()` follows the plan's discard posture. It either abandons the
speculative branch or returns a `pendingBranch()` that can later `resume(history)`.
Neither path advances visible route truth by accident.

Use `plan.evaluate(facts)` when you only need the speculative admission and
visibility result. Use `open(history)` when you need an actual branch lifecycle,
merge preview, commit, discard, or dirty-exit evaluation.

Next: [Speculative Branch Plans](./speculative_branch_plans.md),
[Dirty Exit](./dirty_exit.md), and [Commit Preview](./commit_preview.md).
