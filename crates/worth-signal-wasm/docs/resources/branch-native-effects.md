# Branch-Native Resource Effects

Branch-native optimistic effects make a resource write part of the Signals
branch history. A local patch can be optimistic, inspectable, merge-aware, and
rollback-aware without building a second UI cache beside the runtime.

## What This Feature Is

Branch-native effects are the API path for resource writes that should
participate in branch lifecycle, diagnostics, history, merge planning, and
rollback. The runtime issues an effect envelope for each admitted write, keeps
that envelope on the line diagnostics and history surfaces, and uses the active
Signals branch as the optimistic execution boundary when the selected profile
allows it.

Response-lens topology declarations, JSON effects, and UI lifecycle event
consumption are part of the same API path when they affect how a resource
effect is proved or observed.

## Quick Decision

- Use `branchNative()` for optimistic app edits that should be visible
  immediately and still participate in rollback and merge.
- Use `serverCanonical()` when the server response should decide final truth.
- Use `pessimistic()` when local optimistic writes are not appropriate.
- Use `nonReversible()` or `sensitive()` when retaining rollback preimages would
  be wrong for the data.
- Use response topology docs when the response is grouped, keyed, paged, tree,
  map-backed, entity-store-backed, detail, or summary-shaped.

## Why You Use It

- You want a local resource patch to update the visible line immediately.
- You need to tell whether rollback is exact, compact-inverse, unavailable, or
  not applicable.
- You want merge and rebase details for the resource place that changed.
- You need topology-aware proof for grouped, map, entity-store, tree, sparse,
  named, connection, detail, summary, or JSON-path effects.

## Stable Entry Points

- `signals.api({ effects: signals.resource.effects.branchNative() })`
- `signals.resource.effects.branchNative()`
- `signals.resource.effects.serverCanonical()`
- `signals.resource.effects.pessimistic()`
- `signals.resource.effects.deliveryAuthoritative()`
- `signals.resource.effects.nonReversible()`
- `signals.resource.effects.sensitive()`
- `signals.resource.effects.custom(...)`
- `signals.resource.response.*(...)`
- `line.patch(...)`
- `line.deliver(...)`
- `line.diagnostics().lastEffect`
- `line.history().verificationPackage().lifecycle.lastEffect`
- `line.history().rollbackLastEffect()`
- `signals.resource.branch.planEffectMerge(...)`
- `signals.resource.branch.mergeEffect(...)`

`signals.resource.branch.planMerge(...)` is still the plain branch-native merge
preview. Use `planEffectMerge(...)` when you need the plan bound to a concrete
resource effect envelope.

## Core Mental Model

The resource line owns the visible value. The Signals branch owns optimistic
branch state. The effect envelope is the debug record that connects a resource
operation to both.

A branch-native write follows this shape:

1. The family patch or delivery helper validates the requested resource place.
2. The effect profile decides optimism, confirmation, rollback, rebase, and
   preimage behavior.
3. The runtime either applies the change speculatively on the current branch or
   records it as committed-only.
4. The runtime stores an effect envelope on diagnostics and history.
5. Later rollback, merge, verification, and UI lifecycle reads consume that
   envelope as data.

The runtime does not call UI callbacks. UI code reads lifecycle and diagnostics
facts from the line and decides what to render or schedule.

## How It Executes

`branchNative()` is the default optimistic profile for resource effects. It
enables branch speculation, exact server confirmation, branch restore or compact
inverse rollback, native merge planning, and compact preimage capture when the
patch shape supports it.

Other built-in profiles deliberately narrow that behavior:

- `serverCanonical()` keeps server confirmation authoritative.
- `pessimistic()` disables optimistic branch speculation.
- `deliveryAuthoritative()` treats delivery as the authority source.
- `nonReversible()` refuses rollback claims.
- `sensitive()` avoids retained user-value preimages.
- `custom(...)` is for product surfaces that need explicit named behavior.

Do not select a profile because it sounds safer. Select the one whose retained
state and rollback promises are actually true for the endpoint.

## Small Example

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

const effect = line.diagnostics().lastEffect;

console.log(effect?.profile?.name);
console.log(effect?.optimistic.rollback.kind);
console.log(line.history().verificationPackage().lifecycle.lastEffect);
```

This is the smallest useful example because it declares the effect profile,
patches through a family-owned helper, and reads the issued effect from the line
instead of inventing side-channel state.

## Real Example

```ts
const groupedResponse = signals.resource.response.grouped<{
  groups: Record<string, { id: string; group: string; title: string }[]>;
}>()({
  itemId: (task) => task.id,
  groupId: (task) => task.group,
  groupForItem: () => "todo",
  groups: (value) => value.groups,
  replaceGroups: (value, groups) => ({ ...value, groups }),
  replaceGroupItem: (value, groupId, itemId, nextItem) => ({
    ...value,
    groups: {
      ...value.groups,
      [groupId]: value.groups[groupId].map((item) =>
        item.id === itemId ? nextItem : item),
    },
  }),
  aspects: signals.resource.response.jsonPathAspects<{
    id: string;
    group: string;
    title: string;
    metadata: { priority: number };
  }>()({
    priority: { field: "metadata", path: ["priority"] },
  }),
});

const groupedTasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/grouped-tasks")
  .response(groupedResponse)
  .list({
    load: () => ({
      groups: {
        todo: [{
          id: "task:1",
          group: "todo",
          title: "First",
          metadata: { priority: 1 },
        }],
      },
    }),
  });

const line = groupedTasks.line({});

line.patch(groupedTasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "priority",
  value: 2,
}));

const effect = line.diagnostics().lastEffect;
const mergePlan = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
  },
  effect,
});

console.log(effect.locus.kind);
console.log(effect.locusProof?.topology);
console.log(effect.patch.jsonPath?.path);
console.log(mergePlan.kind);
```

The response topology proves where `task:1` lives, the JSON path aspect proves
which nested value changed, and the merge plan binds native branch data back to
that resource place.

## How It Relates To Other Features

- Use [Effect Envelope Contract](../resource-contracts/effect-envelope.md) when
  you need the exact proof fields on `lastEffect`.
- Use [Effect Merge And Rebase](./merge-and-rebase.md) when a branch preview
  or merge has to be interpreted at resource-locus granularity.
- Use [Rollback And Restore](../resource-contracts/history-and-restore.md)
  when you need to explain exact restore, compact inverse rollback, or
  unavailable rollback.
- Use [Response Topology Proof](../resource-contracts/response-topology-proof.md)
  when the response shape is not a direct array.
- Use [JSON Path Effects](./json-effects.md) when a patch targets nested JSON
  inside an item.
- Use [Collections And Delivery](./collections-and-delivery.md) for ordinary
  `items(...)`, `reconcile(...)`, patch helpers, delivery helpers, aspects, and
  summaries.

## Inspection And Debugging

Start with `line.diagnostics().lastEffect`. It tells you the issued profile,
provenance, branch lifecycle, optimistic behavior, rollback kind, resource place,
response topology proof, JSON path proof, delivery digest, authority digest, and
cost counters.

Then use `line.history().verificationPackage()` when you need retained proof
across history, or `line.history().rollbackLastEffect()` when you need to test
the rollback behavior the effect claims.

For merge and rebase, use `signals.resource.branch.planEffectMerge(...)` before
executing. A plain branch plan can say the native branch merge is possible; an
effect merge plan says whether the specific resource effect can be mapped to the
native branch data.

## Anti-Patterns

- Do not mutate a resource value directly and then synthesize your own effect.
  `planEffectMerge(...)` rejects forged or tampered effect envelopes.
- Do not attach response-lens proof to raw `resourceCollectionShape(...)`
  declarations. Proof is sealed by `signals.resource.response.*(...)`.
- Do not promise rollback if the chosen profile says `rollback: "unavailable"`.
- Do not use JSON path aspects as arbitrary JavaScript object mutation. Unsafe
  path segments, accessors, missing required paths, non-JSON containers, and
  class instances deny before side effects.
- Do not treat delivery effects as local speculative truth unless the profile
  and envelope say that is the selected behavior.

## Current Limits

- Detail and summary response lenses support whole-response effects; detail
  field-level response lenses are still outside the stable surface.
- Resource effect merge depends on native branch merge evidence. If topology
  mapping is unavailable, the result is a typed `mappingUnavailable` artifact,
  not a best-effort merge.
- Compact inverse rollback is only claimed when the runtime can retain an exact
  compact preimage. Sensitive or lossy paths decline that claim.
- UI lifecycle event consumption is read-only. The runtime intentionally does
  not own callbacks, timers, or UI scheduling.

## Related Docs

- [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- [Effect Merge And Rebase](./merge-and-rebase.md)
- [History And Restore](../resource-contracts/history-and-restore.md)
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- [JSON Path Effects](./json-effects.md)
- [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)
- [Collections And Delivery](./collections-and-delivery.md)
