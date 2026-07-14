# JSON Path Effects

JSON path effects let an item aspect patch nested JSON data without replacing
the whole item by hand. The runtime still records a branch-native effect with
the path, rollback behavior, and traversal cost.

## What This Feature Is

`signals.resource.response.jsonPathAspects<T>()(...)` declares item aspects that
read and write nested JSON paths below a selected item field. Local patches and
delivered patches lower through the `jsonItemAspect` effect locus.

## Why You Use It

- A resource item contains a JSON object and you want to patch one nested value.
- You need the effect envelope to name the JSON path that changed.
- You want immutable reconstruction instead of in-place object mutation.
- You need explicit denial for unsafe path segments, missing required paths,
  accessors, non-JSON containers, or lossy rollback cases.

## Stable Entry Points

- `signals.resource.response.jsonPathAspects<TItem>()(...)`
- `signals.resource.response.objectItems<T>()(...)`
- `signals.resource.response.collection<T>()(...)`
- `signals.resource.response.grouped<T>()(...)`
- `signals.resource.response.map<T>()(...)`
- `signals.resource.response.entityStore<T>()(...)`
- family-owned `patch.itemAspect(...)`
- `line.diagnostics().lastEffect.patch.jsonPath`
- `line.history().rollbackLastEffect()`

JSON path aspects are response-owned. They are not accepted as caller-supplied
metadata on ordinary patch objects.

## Core Mental Model

A JSON path aspect is an item aspect with a path validator. The root item field
is selected by `field`, and the nested segments are selected by `path`.
The runtime reads descriptors instead of invoking getters, validates the path,
applies an immutable copy, and records path details on the effect envelope.

Required paths must already exist. Optional terminal paths can materialize only
the final object property. Intermediate containers and array indexes are always
required.

## How It Executes

Admission checks happen before mutation:

1. the root field must exist on the item type and runtime value
2. each segment must be safe
3. required object and array segments must exist
4. accessors are denied without invoking them
5. containers must be plain JSON objects, null-prototype dictionaries, arrays,
   primitives, or `null` as allowed by the selected path
6. class instances such as `Date`, `Map`, and custom prototypes are denied for
   JSON containers
7. the patch reconstructs immutable copies and records traversal and
   reconstruction counters

## Small Example

```ts
const response = signals.resource.response.objectItems<{
  tasks: Array<{
    id: string;
    metadata: { priority: number };
  }>;
}>()({
  field: "tasks",
  itemId: (task) => task.id,
  aspects: signals.resource.response.jsonPathAspects<{
    id: string;
    metadata: { priority: number };
  }>()({
    priority: { field: "metadata", path: ["priority"] },
  }),
});
```

This declares a stable `priority` aspect whose value lives at
`item.metadata.priority`.

## Real Example

```ts
const response = signals.resource.response.objectItems<{
  tasks: Array<{
    id: string;
    metadata: {
      labels?: { review?: string };
      checklist: Array<{ done: boolean }>;
    };
  }>;
}>()({
  field: "tasks",
  itemId: (task) => task.id,
  aspects: signals.resource.response.jsonPathAspects<{
    id: string;
    metadata: {
      labels?: { review?: string };
      checklist: Array<{ done: boolean }>;
    };
  }>()({
    reviewLabel: {
      field: "metadata",
      path: ["labels", "review"],
      presence: "optional",
    },
    firstDone: {
      field: "metadata",
      path: ["checklist", 0, "done"],
    },
  }),
});

line.patch(tasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "reviewLabel",
  value: "needs-design",
}));

const effect = line.diagnostics().lastEffect;

console.log(effect.locus.kind);
console.log(effect.patch.jsonPath?.presence);
console.log(effect.counters.jsonPathTraversalBreadth);
```

The optional path can materialize `review` when `labels` exists. It does not
materialize missing intermediate containers.

## How It Relates To Other Features

- [Branch-Native Resource Effects](./branch-native-effects.md) explains the
  effect profile and rollback behavior.
- [Effect Envelope Contract](../resource-contracts/effect-envelope.md) explains
  `patch.jsonPath` and cost counters.
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
  explains how the enclosing response topology is proved.
- [History And Restore](../resource-contracts/history-and-restore.md) explains
  compact inverse and exact branch rollback.

## Inspection And Debugging

Read `effect.patch.jsonPath` for path, aspect, presence policy, immutable-copy
policy, prototype reconstruction policy, and traversal cost. Read
`effect.counters.jsonPathTraversalBreadth` and
`effect.counters.jsonPathReconstructionBreadth` when performance matters.

If a JSON path write denies, check whether the path was required, whether an
intermediate container was missing, whether an accessor was present, or whether
the value was not plain JSON data.

## Anti-Patterns

- Do not use JSON path aspects to mutate class instances or rich domain objects.
- Do not rely on getters or setters. Accessor-backed paths deny without
  invocation.
- Do not expect optional paths to create intermediate containers.
- Do not claim compact inverse rollback for an absent optional terminal. The
  runtime avoids lossy inverse claims in that case.
- Do not pass `aspectLocus` metadata on patches. JSON path authority comes from
  the compiled response declaration.

## Current Limits

- Paths are existing-object and array-index oriented. They are not a full JSON
  Patch language.
- Optional presence applies only to the terminal object property.
- Prototype reconstruction is limited to plain-object or null-prototype copy
  behavior at the runtime boundary.

## Related Docs

- [Branch-Native Resource Effects](./branch-native-effects.md)
- [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- [History And Restore](../resource-contracts/history-and-restore.md)
