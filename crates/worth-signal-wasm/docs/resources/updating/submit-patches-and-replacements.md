# Submit Patches And Replacements

## What This Feature Is

The local runtime update lane for a loaded resource line.

This is different from server-backed `.create(...)` / `.update(...)` /
`.remove(...)`. Here, the value is already loaded and you want to change the
line through its family-owned patch helpers.

## Why You Use It

- replace the full visible value
- patch one item, field, region, JSON path, aspect, or summary when the family
  supports it
- keep local updates typed and family-owned

## Stable Entry Points

- `family.patch.replace(...)`
- `family.patch.itemAspect(...)`
- `family.patch.field(...)`
- `family.patch.region(...)`
- `family.patch.summary(...)`
- `line.patch(...)`

## Core Mental Model

The family owns what kinds of narrow changes are honest.

That is why patches come from `family.patch.*(...)`, not from ad hoc objects at
every call site.

## How It Executes

1. materialize a line
2. build a patch from the family helper
3. apply it with `line.patch(...)`
4. inspect the line status and diagnostics after the patch

## Small Example

```ts
const line = tasks.line({});

line.patch(
  tasks.patch.replace([
    { id: "t1", title: "Updated" },
  ]),
);
```

## Real Example

```ts
const tasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect("title", (item) => item.title, (item, title: string) => ({
    ...item,
    title,
  }))
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });

const line = tasks.line({});

line.patch(
  tasks.patch.itemAspect({
    itemId: "t1",
    aspect: "title",
    value: "Updated",
  }),
);
```

## How It Relates To Other Features

- Use [Write A Resource](./write-a-resource.md) for server-backed write calls.
- Use [Collections And Delivery](../collections-and-delivery.md) for the deeper
  reconcile and delivery feature lane.

## Inspection And Debugging

Use:

- `line.summary()`
- `line.diagnostics().lastEffect`
- `line.reconciliation()`

Those tell you whether the family actually admits the patch shape you are
trying to use.

## Anti-Patterns

- hand-building patch objects instead of using `family.patch.*(...)`
- expecting narrow item or field patches on a family that only admits broad
  replacement

## Current Limits

- this page covers local runtime patches
- server push delivery stays in the older collection / delivery docs for now

## Related Docs

- [Write A Resource](./write-a-resource.md)
- [Collections And Delivery](../collections-and-delivery.md)
