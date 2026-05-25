# Merge And Rebase

## What This Feature Is

This page covers the resource-aware merge planning and execution path for one
effect envelope.

## Why You Use It

Use it when you need to know:

- can this exact effect rebase onto another branch?
- are the conflicts real resource conflicts or native-only conflicts?
- is resource-topology mapping unavailable?

## Stable Entry Points

- `signals.resource.branch.planEffectMerge(...)`
- `signals.resource.branch.mergeEffect(...)`
- `line.diagnostics().lastEffect`

## Core Mental Model

Native branch merge proof is not enough on its own. Resource-aware merge binds
that native merge to one resource locus and tells you whether the resource patch
can be rebased honestly.

## How It Executes

`planEffectMerge(...)` returns:

- a planned effect merge
- a native merge-plan denial
- a resource-effect merge denial

When planned, the result tells you whether the resource effect is:

- `rebaseAvailable`
- `conflict`
- `mappingUnavailable`

## Small Example

```ts
const effect = line.diagnostics().lastEffect;

const preview = signals.resource.branch.planEffectMerge({
  merge: { source_branch_id: effect.optimistic.branchId, target_branch_id: 0 },
  effect,
});

console.log(preview.kind);
```

## Real Example

```ts
const effect = line.diagnostics().lastEffect;
const preview = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
  },
  effect,
});

if (preview.kind === "planned") {
  console.log(preview.resourceEffect.rebaseArtifact.kind);
  console.log(preview.resourceEffect.policyBinding.resourceGranularity);
}
```

## How It Relates To Other Features

- Use [Branch-Native Effects](./branch-native-effects.md) before this if you do
  not already have a real effect envelope.
- Use [Forms](../forms/README.md) when you want field- and section-level merge
  projection in a resource-backed form.

## Inspection And Debugging

Check:

- preview kind
- `resourceEffect.rebaseArtifact.kind`
- conflict count
- mapping-unavailable detail

## Anti-Patterns

- Do not run plain native merge preview and assume it is enough for resource
  rebasing.
- Do not treat `mappingUnavailable` as the same thing as `conflict`.

## Current Limits

Resource-aware merge still depends on native merge proof and resource-topology
mapping. When either boundary is unavailable, the result stays explicit.

## Related Docs

- [Effect Merge And Rebase](../merge-and-rebase.md)
- [Forms](../forms/handle-resource-drift-and-merge.md)
