# Update One Region, Field, Or Item

Use this page when you know a full replace would be too broad and you need the
right narrow lane instead.

## The Main Choices

- `field`
  One declared detail field changed.
- `region`
  One declared detail region changed.
- `jsonPath`
  One declared nested path changed.
- `item`
  One collection or paged item changed.
- `itemAspect`
  One derived view of one item changed.
- `summary`
  One collection-level or page-window-level value changed.

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

catalog.line({}).patch(
  catalog.patch.summary({
    summary: "total",
    value: 2,
  }),
);
```

## Rule Of Thumb

- pick the narrowest lane the declaration actually supports
- do not reach for `replace` when a smaller lane already names the truth
- do not invent a smaller lane the family never declared

## Related Docs

- [How Partial Resource Updates Work](./how-partial-resource-updates-work.md)
- [How Partial Updates Affect Caching And Delivery](./how-partial-updates-affect-caching-and-delivery.md)
