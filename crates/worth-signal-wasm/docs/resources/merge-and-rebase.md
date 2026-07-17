# Effect Merge And Rebase

Use effect merge when an application branch merge must explain one resource
effect at its declared resource locus. Normal request confirmation already
merges and retires its effect-owned branch; application code does not call this
API for ordinary response closeout.

## Stable Entry Points

- `signals.resource.branch.planEffectMerge({ merge, effect })`
- `signals.resource.branch.mergeEffect({ merge, effect })`
- `signals.resource.branch.planMerge(...)` for native-only preview

## Mental Model

Native proof describes branch conflicts. The sealed effect envelope describes
which resource locus changed. The resource merge layer binds them and returns
`rebaseAvailable`, `conflict`, or `mappingUnavailable`.

Narrow resource reconciliation is resource-locus materialization. Native per-
aspect conflict proof does not perform a partial JavaScript object merge.

## Example

```ts
const summary = line.effects().get(effectId);
if (!summary) throw new Error("unknown effect");

const plan = await signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: summary.branchId,
    target_branch_id: targetBranchId,
  },
  effect: summary.envelope,
});

if (plan.kind === "planned") {
  console.log(plan.resourceEffect.rebaseArtifact.kind);
}
```

## Anti-Patterns

- Do not pass copied or hand-built envelopes.
- Do not erase `mappingUnavailable` or rename it as a conflict.
- Do not call native `planMerge(...)` and claim resource-locus proof.
- Do not use explicit merge APIs as a replacement for
  `line.effects().confirm(effectId)`.

## Related Docs

- [Merge And Rebase](./effects/merge-and-rebase.md)
- [Concurrent Optimistic Effects](./effects/concurrency-and-dependencies.md)
- [Branch-Native Resource Effects](./branch-native-effects.md)
- [Response Topology Proof](./verification/response-topology-proof.md)
