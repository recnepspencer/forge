# Fetch A Collection

## What This Feature Is

The normal route-first list lane for one collection snapshot.

## Why You Use It

- fetch a list with stable item identity
- keep list identity and item identity separate
- get family-owned patch and delivery helpers later if the route declares them

## Stable Entry Points

- `.items(...)`
- `.list(...)`
- `family.line(...)`
- `line.value()`

## Core Mental Model

The line is still one logical member.
The loaded value happens to be a list.

That is why you still call `family.line(...)`, but the family also needs
`itemIdentity(...)` so item-level patching can stay honest later.

## How It Executes

1. declare a route
2. declare item identity with `.items(...)`
3. finish the route with `.list(...)`
4. materialize one line

## Small Example

```ts
const tasks = api.url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });
```

## Real Example

```ts
const workspaceTasks = signals.api({
  baseUrl: "/api",
}).url("/workspaces/:workspaceId/tasks")
  .params<{ search?: string }>()
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: ({ workspaceId, params }) => [
      {
        id: `${workspaceId}:${params.search ?? "all"}:1`,
        title: "First",
      },
    ],
  });

const line = workspaceTasks.line({
  workspaceId: "demo",
  params: { search: "open" },
});
```

## How It Relates To Other Features

- Use [Fetch A Paged List](./fetch-a-paged-list.md) when later pages accumulate.
- Use [Submit Patches And Replacements](../updating/submit-patches-and-replacements.md)
  when you need to change a loaded list locally.
- Use [Collections And Delivery](../collections-and-delivery.md) when the route
  needs richer reconcile, aspect, or summary behavior.

## Inspection And Debugging

Check:

- `line.value()`
- `line.summary()`
- whether the chosen `itemIdentity(...)` really names one logical item

## Anti-Patterns

- using display text as item identity
- choosing `list` when page accumulation is part of the feature

## Current Limits

- this page covers plain list fetching
- richer collection reconciliation lives in the older flat collection docs for
  now

## Related Docs

- [Fetch A Paged List](./fetch-a-paged-list.md)
- [Collections And Delivery](../collections-and-delivery.md)
