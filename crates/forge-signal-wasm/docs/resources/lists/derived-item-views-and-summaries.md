# Derived Item Views And Summaries

Use this page when you want collection-level summaries or narrower per-item
views that update automatically with the list.

## What This Covers

Forge supports declared derived list views such as:

- item aspects
- list summaries
- paged page-window summaries

You do not need to learn the internal vocabulary first. The practical question
is just: do you want to update one derived item view or one list-level summary
without replacing the whole list?

## Stable Entry Points

- `.aspect(name, read, write)`
- `.summary(name, read, write)`
- `.pageWindowSummary(name, read, write)`
- `signals.resource.response.objectAspects<T>()(...)`
- `signals.resource.response.jsonPathAspects<T>()(...)`

## Example

```ts
const catalog = api.url("/catalog")
  .items((item: { id: string; title: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string; title: string }>; total: number }) =>
      value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .summary(
    "total",
    (value) => value.total,
    (value, total: number) => ({ ...value, total }),
  )
  .list({
    load: () => ({
      items: [{ id: "c1", title: "First" }],
      total: 1,
    }),
  });
```

## Why This Matters

This is the user-facing home for the "derived view" story:

- one row can expose named derived item views
- one collection can expose named summary views
- narrow updates can keep those views current automatically

## Related Docs

- [Update One Item Without Replacing Everything](./update-one-item-without-replacing-everything.md)
- [Collections And Delivery](../collections-and-delivery.md)
