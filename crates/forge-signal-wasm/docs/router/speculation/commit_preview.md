# Commit Preview

## What This Feature Is

Commit preview is the explicit merge-preview artifact the router can require
before committing a speculative branch.

## Why You Use It

- inspect merge posture before commit
- keep commit expensive-looking when branch merge proof matters
- pass an explicit preview artifact into the final commit step

## Stable Entry Points

- `session.commitPreview(...)`
- `SpeculativeRouteBranchCommitPreview`
- `session.commit(preview, ...)`

## Core Mental Model

Preview is not a side note. When the plan says
`merge-preview-before-commit`, preview is part of the commit contract.

The router makes that visible instead of hiding it behind one `commit()` call.

## How It Executes

1. ask the history surface for a merge preview
2. package the preview with source and target branch ids
3. preserve the commit posture on the preview artifact
4. optionally feed the preview back into `commit(...)`

## Small Example

```ts
const preview = await session.commitPreview({
  conflict_policy_name: "prefer-source",
});
```

## Real Example

```ts
const dirtyExit = await session.dirtyExit(specialist);
const preview = await session.commitPreview({
  conflict_policy_name: "prefer-source",
});
const commit = await session.commit(preview, dirtyExit);

console.log(preview.sourceBranchId);
console.log(preview.targetBranchId);
console.log(commit.previewDisposition);
```

## How It Relates To Other Features

- preview belongs to a live [Speculative Session](./speculative_sessions.md)
- commit still respects [Dirty Exit](./dirty_exit.md)

## Inspection And Debugging

- `preview.preview`
- `preview.posture`
- `preview.sourceBranchId`
- `preview.targetBranchId`
- `preview.verification()`

## Anti-Patterns

- treating preview as optional when commit posture requires it
- fabricating preview-like objects in app code
- ignoring branch ids when debugging merge issues

## Current Limits

- preview only exists for live sessions
- preview options fail closed on unsupported fields
- direct-merge posture can commit without a preview artifact

## Related Docs

- [Speculative Sessions](./speculative_sessions.md)
- [Dirty Exit](./dirty_exit.md)
