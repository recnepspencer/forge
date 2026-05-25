# Choose A Resource Shape

## What This Feature Is

This page helps you choose between the three main family shapes:

- detail
- collection
- paged

## Why You Use It

- pick the right family once instead of fighting it later
- keep line identity and item identity honest
- know when page accumulation is part of the feature

## Stable Entry Points

- `.detail(...)`
- `.list(...)`
- `.paged(...)`
- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`

## Core Mental Model

Use:

- `detail` for one logical member
- `collection` for one list snapshot with stable item identity
- `paged` for a list whose later pages accumulate into the same line

The biggest decision is not "what did the server return once?" It is "what
should this line keep meaning over time?"

## How It Executes

- `detail` lines do not own `itemIdentity(...)`
- `collection` lines require `itemIdentity(...)`
- `paged` lines require `itemIdentity(...)` and `accumulatePage(...)`

The runtime validates those shape rules when the family is declared, before any
line runs.

## Small Example

```ts
const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const tasks = api.url("/tasks")
  .items((item: { id: string }) => item.id)
  .list({
    load: () => [{ id: "t1" }],
  });
```

## Real Example

```ts
const taskPages = api.url("/workspaces/:workspaceId/tasks/search")
  .items((item: { id: string; title: string }) => item.id)
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: ({ workspaceId, params }) => [
      { id: `${workspaceId}:${params.query}:1`, title: "First" },
    ],
  });

const line = taskPages.line({
  workspaceId: "demo",
  params: { query: "open" },
});
```

## How It Relates To Other Features

- For route-first happy-path fetching, continue to
  [Fetch A Single Record](../fetching/fetch-a-single-record.md),
  [Fetch A Collection](../fetching/fetch-a-collection.md), or
  [Fetch A Paged List](../fetching/fetch-a-paged-list.md).
- For the raw lane and canonical identity details, see
  [Resource Family Authoring Reference](../../api-reference/resource-family-authoring.md).

## Inspection And Debugging

If the family shape is wrong, you usually notice it in one of three ways:

- the runtime denies the declaration before load begins
- item-level patch helpers do not exist where you expected them
- page accumulation logic feels bolted on instead of natural

## Anti-Patterns

- using `detail` for a list because it feels simpler
- using `paged` when you do not actually accumulate pages
- choosing `collection` without stable item identity

## Current Limits

- this page chooses the family shape
- request posture, policy, and effect profile choices live elsewhere

## Related Docs

- [Your First Resource](./your-first-resource.md)
- [Fetch A Collection](../fetching/fetch-a-collection.md)
- [Fetch A Paged List](../fetching/fetch-a-paged-list.md)
