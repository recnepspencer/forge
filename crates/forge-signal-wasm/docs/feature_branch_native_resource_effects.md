# Branch-Native Resource Effects

Use branch-native optimistic effects when local writes should be optimistic
facts in the Signals branch history, not a separate UI cache.

The normal path is:

1. choose `signals.resource.effects.branchNative()`
2. declare a response topology with `signals.resource.response.*(...)`
3. patch through the family-owned helpers
4. read lifecycle, diagnostics, history, and merge evidence from the line

## Optimistic Resource Effects

```ts
const tasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks")
  .items((task: { id: string }) => task.id)
  .aspect("title", (task) => task.title, (task, title: string) => ({
    ...task,
    title,
  }))
  .list({
    load: () => [{ id: "task:1", title: "First" }],
  });

const line = tasks.line({});

line.patch(tasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "title",
  value: "Draft",
}));

console.log(line.diagnostics().lastEffect.optimistic.rollback.kind);
console.log(line.history().verificationPackage().lifecycle.lastEffect.profile);
```

The resource runtime records the selected branch and rollback posture in the
effect envelope. UI code consumes those facts from diagnostics or history; the
resource runtime does not call UI callbacks.

## Response-Lens Topology Declarations

For response-shaped resources, use `signals.resource.response.*(...)` before the
semantic finalizer. The proof stays attached to the line and to later effects.

```ts
const grouped = signals.resource.response.grouped()({
  itemId: (task: { id: string; group: string; title: string }) => task.id,
  groupId: (task: { id: string; group: string; title: string }) => task.group,
  groupForItem: () => "todo",
  groups: (value: {
    groups: Record<string, { id: string; group: string; title: string }[]>;
  }) => value.groups,
  replaceGroups: (value, groups) => ({ ...value, groups }),
  replaceGroupItem: (value, groupId, itemId, nextItem) => ({
    ...value,
    groups: {
      ...value.groups,
      [groupId]: value.groups[groupId].map((item) =>
        item.id === itemId ? nextItem : item),
    },
  }),
  aspects: signals.resource.response.objectAspects()({
    title: "title",
  }),
});
```

Grouped, named, map-backed, sparse-page, tree, connection, entity-store,
discriminated-tuple, detail, and summary topologies all expose proof metadata so
patches can say what resource locus changed.

## JSON Effects

Use JSON path aspects when the patch target is nested data inside an item. The
effect locus becomes `jsonItemAspect`, and the proof names the JSON path.

```ts
const jsonResponse = signals.resource.response.objectItems()({
  field: "tasks",
  itemId: (task: { id: string }) => task.id,
  aspects: signals.resource.response.jsonPathAspects()({
    priority: { field: "metadata", path: ["priority"] },
  }),
});
```

Unsafe path segments and missing required paths deny before an effect is
created.

## Advanced Topology Effects

Advanced topology effects keep traversal and reconstruction evidence. For a
grouped response, an item patch records a single-group traversal; a broad
replacement records a whole-response traversal. Merge planning then carries that
resource locus into native branch proof.

```ts
const plan = signals.resource.branch.planMerge({
  source_branch_id: line.diagnostics().lastEffect.optimistic.branchId,
  target_branch_id: 0,
});

console.log(plan.kind);
```

## UI Lifecycle Event Consumption

Treat lifecycle entries as data.

```ts
const events = line.history().lifecycle.map((entry) => ({
  operation: entry.lastOperation,
  outcome: entry.lastOutcome,
  effect: entry.lastEffect?.effectId ?? null,
}));
```

Render, log, or schedule from those typed facts outside the resource runtime.
Do not place UI callbacks inside resource declarations.
