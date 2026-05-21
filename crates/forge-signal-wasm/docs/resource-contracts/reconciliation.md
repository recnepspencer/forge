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

- collection and paged families

Recommended native declaration lane:

- `signals.api(...).url(...).items(...).list(...)`
- `signals.api(...).url(...).items(...).paged(...)`

Raw reconciliation escape hatch:

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
  resourcePatch,
} from "forge-signal-wasm";

const signals = await createSignals();

const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item) => item.title,
    (item, title: string) => ({ ...item, title }),
  )
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });
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
  resourcePatch,
} from "forge-signal-wasm";

const signals = await createSignals();

const feed = signals.api({}).url("/workspaces/:workspaceId/feed")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item) => item.title,
    (item, title: string) => ({ ...item, title }),
  )
  .pageWindowSummary(
    "visibleCount",
    (value: { id: string; title: string }[]) => value.length,
    (value, visibleCount: number) => value.slice(0, visibleCount),
  )
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
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

- API route docs explain the small direct-array and collection-owned builder
  lane before you drop to raw reconciliation helpers.
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

- [Route Authoring Reference](../api-reference/route-authoring.md)
- [Resource Family Authoring Reference](../api-reference/resource-family-authoring.md)
- [Delivery And Compatibility Contract](./delivery-and-compatibility.md)
- [Resource Line Reference](../api-reference/resource-line.md)
