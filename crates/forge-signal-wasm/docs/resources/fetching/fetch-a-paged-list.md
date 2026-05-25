# Fetch A Paged List

## What This Feature Is

The route-first paged lane for list values that accumulate later pages into the
same line.

## Why You Use It

- page accumulation is part of the feature
- you want item identity and page accumulation declared once
- you want the line to keep meaning "this paged result" instead of "one page"

## Stable Entry Points

- `.items(...)`
- `.paged(...)`
- `accumulatePage(...)`
- `family.line(...)`

## Core Mental Model

Use `paged` when later pages should fold into one ongoing line value.

If every request is really just one independent list snapshot, use `list`
instead.

## How It Executes

1. declare item identity
2. finish the route with `.paged(...)`
3. declare `accumulatePage(existing, next)`
4. materialize a line

## Small Example

```ts
const taskPages = api.url("/tasks/search")
  .items((item: { id: string }) => item.id)
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: ({ params }) => [{ id: `page:${params.query}:1` }],
  });
```

## Real Example

```ts
const catalog = signals.api({ baseUrl: "/api" })
  .url("/workspaces/:workspaceId/catalog/search")
  .params<{ query: string }>()
  .items((item: { id: string; title: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string; title: string }> }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .paged({
    accumulatePage: (existing, next) => ({
      ...next,
      items: [...existing.items, ...next.items],
    }),
    load: ({ workspaceId, params }) => ({
      items: [{ id: `${workspaceId}:${params.query}:1`, title: "First" }],
      cursor: null,
    }),
  });
```

## How It Relates To Other Features

- Use [Fetch A Collection](./fetch-a-collection.md) when pages do not accumulate.
- Use [Collections And Delivery](../collections-and-delivery.md) for deeper
  item-aspect and summary helper lanes.

## Inspection And Debugging

The first thing to sanity-check is `accumulatePage(...)`.

If the runtime shape feels wrong after later pages, the accumulation rule is
usually the first culprit.

## Anti-Patterns

- choosing `paged` with no real accumulation story
- writing an accumulation rule that drops the previous visible items by accident

## Current Limits

- this page stays on the normal paged lane
- sparse page topologies and advanced topology proof live in the older effect
  docs for now

## Related Docs

- [Fetch A Collection](./fetch-a-collection.md)
- [Collections And Delivery](../collections-and-delivery.md)
