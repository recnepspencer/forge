# When To Declare Derived Views Explicitly

Use this page when you are deciding whether the built-in object-field lane is
enough or whether your resource needs a more explicit derived-view declaration.

## Use The Simple Lane When

- the item view is just one plain object field
- the write can replace that field directly

That is what `objectAspects<T>()(...)` is for.

## Use The Explicit Lane When

- the derived view lives at a nested path
- the write needs custom reconstruction logic
- the response shape is not a plain object field shape

That is where `.aspect(...)`, `.reconcile(...)`, or `jsonPathAspects<T>()(...)`
become the honest lane.

## Example

```ts
const tasks = api.url("/tasks")
  .items((item: { id: string; tags: string[] }) => item.id)
  .aspect(
    "primaryTag",
    (item) => item.tags[0] ?? null,
    (item, tag: string | null) => ({
      ...item,
      tags: tag == null ? [] : [tag, ...item.tags.slice(1)],
    }),
  )
  .list({
    load: () => [{ id: "t1", tags: ["important"] }],
  });
```

## Related Docs

- [Automatic Derived Views](./automatic-derived-views.md)
- [Collections And Delivery](../collections-and-delivery.md)
