# List Shapes And Item Identity

Use this page when you are deciding how to model a list and how the runtime
will recognize one item across patches, deliveries, and mutation responses.

## The Usual Shapes

- `.list(...)`
  One collection snapshot.
- `.paged(...)`
  A collection with page-window behavior.

The list lane becomes more useful once you give it stable item identity.

## Stable Entry Points

- `api.url(...).items(itemId)`
- `api.url(...).response(signals.resource.response.array(...)).list(...)`
- `api.url(...).response(signals.resource.response.objectItems<T>()(...)).list(...)`
- `api.url(...).response(signals.resource.response.collection<T>()(...)).list(...)`
- `api.url(...).response(...).paged(...)`

## Example

```ts
const tasks = api.url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });

console.log(tasks.line({}).value());
```

## Why Identity Matters

Stable item identity is what lets the runtime:

- patch one item
- deliver one item
- reconcile one response-owned item target
- track derived item views and summaries

If the family cannot name one stable item, it cannot honestly offer the narrow
item lanes.

## Related Docs

- [Update One Item Without Replacing Everything](./update-one-item-without-replacing-everything.md)
- [Collections And Delivery](../collections-and-delivery.md)
