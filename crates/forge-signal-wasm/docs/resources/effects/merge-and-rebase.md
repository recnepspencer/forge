# Merge And Rebase

## What This Feature Is

Effect merge binds one runtime-issued resource effect to native branch merge
proof. Use it when you are explicitly merging application branches. Ordinary
request confirmation already closes its own effect branch; it does not require
application code to call `mergeEffect(...)`.

## Why You Use It

- preview one resource edit inside an application branch merge;
- distinguish a real conflict from missing resource-topology mapping;
- retain resource locus and native proof in the merge result.

## Stable Entry Points

- `signals.resource.branch.planEffectMerge({ merge, effect })`
- `signals.resource.branch.mergeEffect({ merge, effect })`
- `signals.resource.branch.planMerge(...)` for native-only branch preview

## Core Mental Model

Native merge proof describes branch state and conflict isolation. The effect
envelope describes the resource locus: the exact item, aspect, field, region,
JSON path, summary, insert, delete, or line replacement that changed.

The resource layer binds those two proofs. It does not perform a hidden partial
object merge. When a narrow effect is accepted, resource-locus materialization
reconstructs the declared locus while native proof decides whether that locus
conflicts.

## How It Executes

`planEffectMerge(...)` is read-only and returns `rebaseAvailable`, `conflict`,
or `mappingUnavailable` within its resource artifact. `mergeEffect(...)`
executes the accepted native merge and carries the corresponding resource
artifact.

## Small Example

```ts
const effect = line.effects().get(effectId);
if (!effect) throw new Error("unknown effect");

const plan = await signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.branchId,
    target_branch_id: targetBranchId,
  },
  effect: effect.envelope,
});
```

The targeted effect summary supplies both stable branch IDs and the sealed
envelope required by the merge API.

## Real Example

```ts
const result = await signals.resource.branch.mergeEffect({
  merge: {
    source_branch_id: envelope.optimistic.branchId,
    target_branch_id: releaseBranchId,
  },
  effect: envelope,
});

if (result.kind === "planned" || result.kind === "merged") {
  audit(result.resourceEffect.policyBinding);
  audit(result.resourceEffect.mergeArtifact ?? result.resourceEffect.rebaseArtifact);
}
```

## How It Relates To Other Features

- [Branch-Native Effects](./branch-native-effects.md) creates effect branches.
- [Concurrent Optimistic Effects](./concurrency-and-dependencies.md) covers
  request closeout and derived projection.
- [History And Restore](../../resource-contracts/history-and-restore.md) covers
  retained historical targets, not branch merging.

## Inspection And Debugging

Inspect result kind, conflicts, policy binding, resource locus, topology proof,
and mapping detail. Treat `mappingUnavailable` as a denial, not a conflict or
success.

## Anti-Patterns

- Do not call native `planMerge(...)` and claim resource-locus proof.
- Do not pass copied or hand-built effect envelopes.
- Do not describe native per-aspect proof as a partial object merge.
- Do not call explicit merge APIs for normal `confirm(effectId)` closeout.

## Current Limits

Rebase support depends on the selected effect profile, native merge proof, and
resource topology mapping. Unsupported boundaries remain typed denials.

## Related Docs

- [Concurrent Optimistic Effects](./concurrency-and-dependencies.md)
- [Effect Envelopes And Closeout](./effect-envelopes-and-closeout.md)
- [Response Topology Proof](../verification/response-topology-proof.md)
