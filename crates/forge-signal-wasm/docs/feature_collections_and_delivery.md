# Collections And Delivery

Use this page when one route owns list-shaped data and you need first-class
patch or delivery helpers.

## What This Covers

- `.items(...)`
- `.reconcile(...)`
- `.aspect(...)`
- `.summary(...)`
- `.pageWindowSummary(...)`
- family-owned `patch`
- family-owned `delivery`

## Direct-Array Happy Path

```ts
const tasks = api.url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item) => item.title,
    (item, title: string) => ({ ...item, title }),
  )
  .list({
    load: ({ workspaceId }) => [
      { id: `${workspaceId}:1`, title: "First" },
    ],
  });

const line = tasks.line({ workspaceId: "demo" });

line.patch(
  tasks.patch.itemAspect({
    itemId: "demo:1",
    aspect: "title",
    value: "Updated",
  }),
);
```

## Reconcile Happy Path

Use `.reconcile(...)` when items live inside an envelope value instead of being
the value itself.

```ts
const catalog = api.url("/workspaces/:workspaceId/catalog")
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
    load: ({ workspaceId }) => ({
      items: [{ id: `${workspaceId}:1`, title: "First" }],
      total: 1,
    }),
  });
```

## Delivery Happy Path

If the family owns reconciliation truth, it also owns delivery helpers.

```ts
line.deliver(
  catalog.delivery.summary({
    packetId: "pkt-1",
    basisId: null,
    nextBasisId: "basis-1",
    summary: "total",
    value: 2,
  }),
);
```

## Where To Go Next

- route-first CRUD and advanced request shaping:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- line reads and diagnostics:
  [feature_line_inspection.md](./feature_line_inspection.md)
- lower-level reconciliation reference:
  [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
