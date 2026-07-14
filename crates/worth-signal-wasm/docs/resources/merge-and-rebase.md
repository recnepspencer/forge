# Effect Merge And Rebase

Effect merge and rebase answers a practical question: "I have a resource write
on one branch. Can the runtime explain how that write merges into another
branch?"

## What This Feature Is

`signals.resource.branch.planEffectMerge(...)` and
`signals.resource.branch.mergeEffect(...)` take a runtime-issued resource effect
envelope and bind it to native branch merge evidence. They answer a narrower
question than plain branch merge: "Can this exact resource effect be mapped
into this branch merge?"

## Quick Decision

- Use `planMerge(...)` when you only care whether the branches can merge.
- Use `planEffectMerge(...)` when the UI, logs, or QA need to explain one
  resource effect inside that merge.
- Use `mergeEffect(...)` when you are ready to execute and want the merge result
  to carry the resource-effect details.
- Treat `mappingUnavailable` as a real result. It means the branch plan exists,
  but the runtime cannot safely map the resource effect onto it.

## Why You Use It

- Preview how a branch-native resource write will merge.
- Tell the difference between native branch conflicts and resource topology
  mapping failures.
- Keep response topology, JSON aspect, host-region, and policy details attached
  to the merge result.
- Deny forged or tampered effects before using them for merge decisions.

## Stable Entry Points

- `signals.resource.branch.planMerge({ ... })`
- `signals.resource.branch.planEffectMerge({ merge, effect })`
- `signals.resource.branch.mergeEffect({ merge, effect })`
- `ResourceEffectMergePlanResult`
- `ResourceEffectMergeExecutionResult`

Use `planMerge(...)` for a branch-level preview. Use `planEffectMerge(...)` when
the preview must be explained in terms of a particular resource effect. Use
`mergeEffect(...)` when you are ready to execute the branch merge and keep the
resource effect details in the result.

## Core Mental Model

Native branch merge knows about branch nodes and conflicts. Resource effect
merge adds the resource information back on top:

1. The native merge plan reports source branch, target branch, selected merge
   behavior, breadth, and native conflicts.
2. The effect envelope reports the family, line, resource place, topology,
   profile, and authority for the write.
3. The merge layer binds that resource place to the native conflict isolation
   policy.
4. The result reports `rebaseAvailable`, `conflict`, or
   `mappingUnavailable`.

`mappingUnavailable` is not a crash and not a silent success. It means native
branch evidence exists, but the runtime cannot safely map the resource effect
topology into that native merge plan.

## How It Executes

`planEffectMerge(...)` is read-only. It returns the native branch plan plus a
`resourceEffect.rebaseArtifact`.

`mergeEffect(...)` executes the native merge and returns a
`resourceEffect.mergeArtifact`.

Both APIs require:

- a runtime-issued effect envelope
- a merge request accepted by native branch planning
- an effect profile whose `rebase` setting is `nativeMergePlan`
- compatible conflict isolation policy for the resource place that changed

## Small Example

```ts
const effect = line.diagnostics().lastEffect;

const plan = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
  },
  effect,
});

console.log(plan.kind);
console.log(plan.resourceEffect?.rebaseArtifact.kind);
```

This is the smallest useful example because it uses the effect envelope produced
by the line and keeps merge interpretation tied to that exact effect.

## Real Example

```ts
const effect = groupedLine.diagnostics().lastEffect;

const plan = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
    conflict_isolation_policy_name: "signal.conflict-isolation.per-node",
  },
  effect,
});

if (plan.kind === "planned") {
  const artifact = plan.resourceEffect.rebaseArtifact;

  if (artifact.kind === "rebaseAvailable") {
    console.log("effect can be rebased");
  }

  if (artifact.kind === "conflict") {
    console.log(artifact.conflicts);
  }

  if (artifact.kind === "mappingUnavailable") {
    console.log(artifact.resource.topology);
    console.log(artifact.native.records);
  }
}
```

For grouped, named, sparse, tree, map, entity-store, connection, detail, summary,
and JSON-path effects, the result can include topology and host-region details.
That is what lets UI or tooling explain the conflict as "this grouped task
title changed" instead of only showing native branch node ids.

## How It Relates To Other Features

- [Branch-Native Resource Effects](./branch-native-effects.md) creates the
  effect envelope that merge APIs consume.
- [Effect Envelope Contract](../resource-contracts/effect-envelope.md) explains
  the effect fields that appear inside `resourceEffect`.
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
  explains why host-region evidence can appear in the policy binding.
- [JSON Path Effects](./json-effects.md) explains JSON effect loci.
- [History And Restore](../resource-contracts/history-and-restore.md) is about
  rollback and restore, not branch merge.

## Inspection And Debugging

Read these fields first:

- `result.kind`
- `result.conflicts`
- `result.resourceEffect.policyBinding`
- `result.resourceEffect.rebaseArtifact.kind`
- `result.resourceEffect.mergeArtifact.kind`
- `result.resourceEffect.proof.effectLocusDigest`
- `result.resourceEffect.proof.compiledLensDigest`

The `policyBinding` names whether the effect was bound as a host region,
resource aspect, resource item, or resource line. Host-region bindings include
response topology lookup, traversal, and reconstruction details.

## Anti-Patterns

- Do not call `planMerge(...)` and assume that is enough to explain a resource
  effect. It previews the native branch plan only.
- Do not pass a copied, edited, or hand-built effect envelope. Effect merge APIs
  deny forged or tampered envelopes.
- Do not erase `mappingUnavailable`. It is actionable information for users and
  QA.
- Do not weaken conflict isolation policy to force a merge. The resource effect
  needs compatible native isolation.

## Current Limits

- Rebase support is profile-dependent. Effects whose profile says `rebase:
  "unavailable"` cannot be planned as resource effect merges.
- Some raw resource effects have no response topology proof. They can still
  merge at generic resource granularity, but topology-specific host-region
  details are absent.
- `mergeEffect(...)` does not invent conflict resolution. It executes the native
  merge behavior and reports resource details for the result.

## Related Docs

- [Branch-Native Resource Effects](./branch-native-effects.md)
- [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- [JSON Path Effects](./json-effects.md)
