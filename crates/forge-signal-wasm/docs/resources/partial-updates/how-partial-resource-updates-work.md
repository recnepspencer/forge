# How Partial Resource Updates Work

Use this page when one field, region, item, derived item view, or summary
should update without broad replacement.

## Stable Entry Points

- `resourcePatch.field(...)`
- `resourcePatch.region(...)`
- `resourcePatch.jsonPath(...)`
- `resourcePatch.item(...)`
- `resourcePatch.itemAspect(...)`
- `resourcePatch.summary(...)`
- family-owned `patch.*`
- family-owned `delivery.*`

## Mental Model

A partial update is still real resource truth. It is just narrower than
`replace(...)`.

The runtime supports narrow lanes only when the family declared enough shape to
apply them honestly.

That is why:

- detail families can narrow through fields, regions, or JSON paths
- collection and paged families can narrow through items, derived item views,
  and summaries

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

## Why This Exists

Without narrow patches, the app would have to replace the full visible value
for every small change. Forge lets you stay precise instead.

## Related Docs

- [Automatic Derived Views](./automatic-derived-views.md)
- [Update One Region, Field, Or Item](./update-one-region-field-or-item.md)
