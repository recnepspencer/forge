# Response Topology Proof

Response topology proof lets diagnostics say where a change happened inside a
shaped response. Instead of "some list item changed," you can see "this map
entry changed," "this grouped item changed," or "this whole detail response
changed."

## What This Feature Is

`signals.resource.response.*(...)` declarations compile response lenses. Those
lenses keep enough runtime data to lower local patches and delivered patches into
resource effect loci such as `membership`, `entityStore`, `mapCollection`,
`groupedCollection`, `sparsePage`, `recursiveTree`, `detailResponse`,
`summaryResponse`, or `jsonItemAspect`.

## Quick Decision

- Use `array(...)` for a plain array response.
- Use `objectItems<T>()(...)` when items live under one object field.
- Use `collection<T>()(...)` for custom extract-and-replace collections.
- Use `map<T>()(...)` or `entityStore<T>()(...)` when the server shape is keyed.
- Use `grouped<T>()(...)`, `named<T>()(...)`, `sparse<T>()(...)`, or
  `tree<T>()(...)` when item location matters.
- Use `detail<T>()` or `summary<T>()` for whole-response effects.

## Why You Use It

- Your response is not just `readonly Item[]`.
- You need narrow item replacement inside a grouped, map, entity-store, sparse,
  named, tree, connection, or object-field response.
- You want diagnostics and merge plans to name the actual response shape.
- You need to know whether a patch was narrow or required broad traversal.

## Stable Entry Points

- `signals.resource.response.array(...)`
- `signals.resource.response.collection<T>()(...)`
- `signals.resource.response.objectItems<T>()(...)`
- `signals.resource.response.connection<T>()(...)`
- `signals.resource.response.entityStore<T>()(...)`
- `signals.resource.response.map<T>()(...)`
- `signals.resource.response.grouped<T>()(...)`
- `signals.resource.response.named<T>()(...)`
- `signals.resource.response.multiple<T>()(...)`
- `signals.resource.response.sparse<T>()(...)`
- `signals.resource.response.discriminated<T>()(...)`
- `signals.resource.response.tree<T>()(...)`
- `signals.resource.response.detail<T>()`
- `signals.resource.response.summary<T>()`
- `signals.resource.response.jsonPathAspects<T>()(...)`

Public raw collection shapes do not accept borrowed `responseLensProof`.
Topology proof is created by the response declaration.

## Core Mental Model

A response lens has three jobs:

1. read the visible resource items or value from the response shape
2. replace a narrow item, aspect, summary, or whole response without losing
   identity
3. emit data that the runtime can carry into effects, diagnostics, history,
   rollback, and merge

The topology data is attached to the resource line. Later effects copy the relevant
portion into `effect.locusProof`.

## How It Executes

Each topology chooses a different lookup and reconstruction contract:

- `array`: direct array item identity
- `collection`: custom item extraction and replacement
- `objectItems`: items stored in one object field
- `connection`: edge/node shape with direct node replacement
- `entityStore`: normalized object records with direct entity replacement
- `map`: `ReadonlyMap` entries with direct entry replacement
- `grouped`: record of group arrays
- `named` and `multiple`: record of named collection arrays
- `sparse`: page-keyed arrays
- `discriminated`: active variant arrays
- `tree`: recursive roots and node paths
- `detail`: whole detail response replacement
- `summary`: whole summary response replacement

If a topology hook returns a malformed shape, drops identity, moves an item to
the wrong key, or corrupts a required container, admission denies before effect
mutation.

## Small Example

```ts
const response = signals.resource.response.map<{
  tasks: ReadonlyMap<string, { id: string; title: string }>;
}>()({
  itemId: (task) => task.id,
  entries: (value) => value.tasks,
  replaceEntries: (value, tasks) => ({ ...value, tasks }),
  replaceEntry: (value, itemId, nextItem) => {
    const tasks = new Map(value.tasks);
    tasks.set(itemId, nextItem);
    return { ...value, tasks };
  },
});
```

The direct `replaceEntry(...)` hook is what lets an item effect stay narrow
instead of reconstructing the whole map-backed response.

## Real Example

```ts
const response = signals.resource.response.entityStore<{
  entities: Record<string, { id: string; title: string }>;
}>()({
  itemId: (task) => task.id,
  entities: (value) => value.entities,
  replaceEntities: (value, entities) => ({ ...value, entities }),
  replaceEntity: (value, itemId, nextItem) => ({
    ...value,
    entities: {
      ...value.entities,
      [itemId]: nextItem,
    },
  }),
  aspects: signals.resource.response.objectAspects()({
    title: "title",
  }),
});

const tasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks")
  .response(response)
  .list({
    load: () => ({
      entities: { "task:1": { id: "task:1", title: "First" } },
    }),
  });

const line = tasks.line({});
line.patch(tasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "title",
  value: "Draft",
}));

console.log(line.diagnostics().lastEffect.locusProof.topology);
console.log(line.diagnostics().lastEffect.locusProof.cost.lookup);
```

The effect reports `entityStore` topology proof, including cost data for
lookup, traversal, and reconstruction.

## How It Relates To Other Features

- [Branch-Native Resource Effects](../resources/branch-native-effects.md) uses
  topology data to issue better effect envelopes.
- [Effect Merge And Rebase](../resources/merge-and-rebase.md) uses topology
  topology data to bind effects to native branch merge regions.
- [JSON Path Effects](../resources/json-effects.md) adds nested JSON path proof
  inside item-aspect effects.
- [Collections And Delivery](../resources/collections-and-delivery.md) covers
  the simpler collection, aspect, summary, patch, and delivery lane.

## Inspection And Debugging

Read `line.diagnostics().lastEffect.locusProof` after a patch. The most useful
fields are topology, lens source, lowered locus, digest fields, and cost.

When a response-lens denial throws, read it as "the runtime refused to create an
effect." The declaration or hook did not preserve the response shape well enough
to safely patch it.

## Anti-Patterns

- Do not reuse proof from one response declaration on another response shape.
- Do not use raw reconciliation callbacks to claim response topology proof.
- Do not implement direct replacement hooks that drop item identity or return an
  item under the wrong key.
- Do not use a broad replacement when the topology has a narrow replacement
  hook for the same operation.

## Current Limits

- Detail and summary topologies currently prove whole-response effects.
  Field-level detail response lenses are outside this stable surface.
- Response-lens proof describes runtime-observed behavior, not arbitrary server
  schema meaning.
- A topology can still be denied if runtime value shape does not match the
  declared response lens.

## Related Docs

- [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- [Effect Envelope Contract](./effect-envelope.md)
- [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- [JSON Path Effects](../resources/json-effects.md)
