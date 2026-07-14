# Handle Resource Drift And Merge

## What This Feature Is

This page covers remote source changes and resource-merge previews as they show
up inside a resource-backed form.

## Why You Use It

Use it when you need to answer:

- did the remote resource change while the user had a local draft?
- was the local draft preserved, rebased, blocked, or conflicted?
- can the form preview merge conflicts at field or section granularity?

## Stable Entry Points

- `form.resourceDrift()`
- `form.resourceMerge()`
- `form.previewResourceMerge(...)`

## Core Mental Model

Drift answers "what happened when the source changed?"
Merge answers "what would happen if I rebase this resource effect?"

They are related, but not the same.

## How It Executes

`form.resourceDrift()` tracks observed source changes and whether draft truth was:

- `preserved`
- `rebased`
- `blocked`
- `conflict`

`form.resourceMerge()` tracks merge preview results:

- `ready`
- `conflict`
- `unavailable`

## Small Example

```ts
const drift = form.resourceDrift();
console.log(drift.summary.status);
```

## Real Example

```ts
const preview = form.previewResourceMerge({
  source_branch_id: effect.optimistic.branchId,
  target_branch_id: 0,
});

console.log(preview.status);
console.log(preview.projectedFields);
console.log(form.resourceMerge().summary.conflictCount);
console.log(form.resourceDrift().summary.status);
```

## How It Relates To Other Features

- Use [Merge And Rebase](../effects/merge-and-rebase.md) for the resource-side
  effect merge lane.
- Use the form-side [Resource Drift](../../forms/resource-backed/resource-drift.md)
  and [Resource Merge](../../forms/resource-backed/resource-merge.md) pages for
  deeper form-first detail.

## Inspection And Debugging

Inspect:

- `form.resourceDrift().current`
- `form.resourceMerge().current`
- projected fields and sections
- readiness blockers and visible messages

## Anti-Patterns

- Do not call every remote change a conflict.
- Do not assume merge-preview conflict means source-compatibility drift, or vice
  versa.

## Current Limits

Merge preview depends on a current resource effect and real native merge proof.
Without those, the result stays explicitly unavailable.

## Related Docs

- [Resource Drift](../../forms/resource-backed/resource-drift.md)
- [Resource Merge](../../forms/resource-backed/resource-merge.md)
