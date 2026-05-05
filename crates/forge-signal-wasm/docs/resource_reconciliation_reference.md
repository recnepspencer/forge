# Resource Reconciliation Reference

## What This Feature Is

Reconciliation is the part of the resource surface that lets collection and
paged lines accept smaller updates than "replace the whole list".

Use it when you want to update:

- one item in a list
- one field on one item
- one declared summary value

without pretending the runtime can safely rewrite anything it wants.

## Why You Use It

- update one visible item instead of replacing the whole list
- support websocket or push updates that only change part of the value
- keep narrow updates explicit and safe
- share the same update rules between local patches and delivered patches

## Stable Entry Points

Authoring helpers:

- `resourceCollectionShape(...)`
- `resourceItemAspects(...)`
- `resourceValueSummaries(...)`
- `resourceValueSummaries.pageWindow(...)`

Patch helpers:

- `resourcePatch.replace(...)`
- `resourcePatch.item(...)`
- `resourcePatch.itemAspect(...)`
- `resourcePatch.summary(...)`

Line methods:

- `line.patch(...)`
- `line.reconciliation()`

Reconciliation only applies to:

- `signals.resource.collection(...)`
- `signals.resource.paged(...)`

## Core Mental Model

Reconciliation is not a general mutation API.

It is a declaration of what the runtime can update honestly without lying about
the value.

There are four patch shapes:

- `replace`: replace the whole value
- `item`: replace one item
- `itemAspect`: replace one declared field on one item
- `summary`: replace one declared summary value

If the runtime cannot prove a narrow patch is safe, it should reject it.

## How It Executes

At declaration time you describe:

- how to read the items from the value
- how to write items back into the value
- which item fields can be patched directly
- which summary values can be patched directly

At runtime:

1. `line.reconciliation()` tells you what narrow patching the line supports
2. `line.patch(...)` applies one patch
3. the runtime either applies the patch or rejects it

Common rejection cases:

- the item is not visible
- the same item id appears twice in the visible value
- an item patch changes item identity
- a summary patch is not declared for this line

## Small Example

```ts
import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
} from "forge-signal-wasm";

const signals = createSignals();

const tasks = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (value: { items: Array<{ id: string; title: string }> }) =>
      value.items,
    replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (item: { id: string; title: string }) => item.title,
        write: (item, title: string) => ({ ...item, title }),
      },
    }),
  }),
  load: () => ({
    items: [{ id: "t1", title: "First" }],
  }),
});

const line = tasks.line({ workspaceId: "demo" });

line.patch(
  resourcePatch.itemAspect({
    itemId: "t1",
    aspect: "title",
    value: "Updated",
  }),
);
```

This is the most common reconciliation case: update one field on one visible
item.

## Real Example

```ts
import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourceValueSummaries,
} from "forge-signal-wasm";

const signals = createSignals();

const feed = signals.resource.paged({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (
      value: {
        items: Array<{ id: string; title: string }>;
        cursor: string | null;
        visibleCount: number;
      },
    ) => value.items,
    replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (item: { id: string; title: string }) => item.title,
        write: (item, title: string) => ({ ...item, title }),
      },
    }),
    summaries: resourceValueSummaries.pageWindow({
      visibleCount: {
        read: (value) => value.visibleCount,
        write: (value, visibleCount: number) => ({ ...value, visibleCount }),
      },
    }),
  }),
  accumulatePage: (existing, next) => ({
    items: [...existing.items, ...next.items],
    cursor: next.cursor,
    visibleCount: next.visibleCount,
  }),
  load: ({ workspaceId }) => ({
    items: [{ id: `${workspaceId}:1`, title: "First" }],
    cursor: null,
    visibleCount: 1,
  }),
});

const line = feed.line({ workspaceId: "demo" });

line.patch(
  resourcePatch.itemAspect({
    itemId: "demo:1",
    aspect: "title",
    value: "Retitled",
  }),
);

line.patch(
  resourcePatch.summary({
    summary: "visibleCount",
    value: 2,
  }),
);
```

Use this pattern when:

- the line is list-shaped
- items have stable identity
- a few narrow updates are much more common than full replacement

## How It Relates To Other Features

- Delivery uses the same patch shapes when pushed updates arrive later.
- Family authoring decides whether reconciliation exists at all.
- Line diagnostics and history explain what the last patch actually did.

## Inspection And Debugging

Check these first:

- `line.reconciliation()`
- `line.diagnostics().lastPatchKind`
- `line.diagnostics().lastPatchScope`
- `line.history().lifecycle`

That usually tells you both what the line supports and what actually happened.

## Anti-Patterns

- treating reconciliation like a general mutation system
- assuming every list can be narrow-patched just because it has items
- changing item identity in an `item(...)` patch

## Current Limits

- reconciliation only exists on collection and paged resources
- summary patching is explicit and can be narrower on paged lines than on plain
  collections

## Related Docs

- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
