# Update One Item Without Replacing Everything

Use this page when one row or one piece of list metadata should update without
replacing the full visible collection.

## Stable Entry Points

- family-owned `patch.item(...)`
- family-owned `patch.itemAspect(...)`
- family-owned `patch.summary(...)`
- family-owned `delivery.item(...)`
- family-owned `delivery.itemAspect(...)`
- family-owned `delivery.summary(...)`

## Example

```ts
const tasks = api.url("/tasks")
  .items((item: { id: string; title: string; status: string }) => item.id)
  .aspect("status", (item) => item.status, (item, status: string) => ({
    ...item,
    status,
  }))
  .list({
    load: () => [{ id: "t1", title: "First", status: "open" }],
  });

const line = tasks.line({});

line.patch(
  tasks.patch.itemAspect({
    itemId: "t1",
    aspect: "status",
    value: "done",
  }),
);
```

## Other Narrow Lanes

- `item(...)`
  Replace one resident item value.
- `itemAspect(...)`
  Replace one declared item view.
- `summary(...)`
  Replace collection-level metadata.

These lanes exist because the family declared how to identify items and, when
needed, how to read and write the derived view.

## Related Docs

- [Derived Item Views And Summaries](./derived-item-views-and-summaries.md)
- [Collections And Delivery](../collections-and-delivery.md)
