# Resource Merge

## What This Feature Is

This page covers merge preview state for resource-backed forms.

## Why You Use It

- inspect whether a resource-backed merge is ready, conflicted, or unavailable
- preview projected fields, sections, blockers, and messages
- keep merge reasoning on a typed surface instead of renderer-local state

## Stable Entry Points

- `form.resourceMerge()`
- `form.previewResourceMerge(request)`
- `form.clearResourceMerge(...)`

## Core Mental Model

Resource merge is a preview lane. It tells you what the runtime currently knows
about a merge request before anything is finalized.

## How It Executes

1. a merge preview request is reported
2. the runtime derives merge status, conflicts, and projected impact
3. the report is retained until cleared or replaced

## Small Example

```ts
console.log(form.resourceMerge().summary);
```

## Real Example

```ts
const preview = form.previewResourceMerge(request);

console.log(preview.status);
console.log(preview.projectedFields);
console.log(form.resourceMerge().history);
```

## How It Relates To Other Features

- Read [Resource Drift](./resource-drift.md) when the merge is responding to
  an observed source drift.
- Read [Resource Reset](./resource-reset.md) for rollback/reset moves after
  merge trouble.

## Inspection And Debugging

- `summary.status` and `summary.conflictCount` are the main reads
- `current.projectedFields` and `current.projectedSections` show affected areas
- `current.blockers` and `current.messages` explain why the merge is blocked or
  unavailable

## Anti-Patterns

- treating merge preview as if it were already applied state
- hiding merge conflicts in app-local booleans

## Current Limits

- deeper branch merge semantics live in the resource docs

## Related Docs

- [Resource Drift](./resource-drift.md)
- [Resource Reset](./resource-reset.md)
