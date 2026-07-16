# Effect Envelope Contract

The effect envelope is the debug record for one admitted resource effect. Read
it when you need to know what changed, why it was allowed, whether it was
optimistic, and whether rollback or merge support is available.

In other words, it is the runtime-issued effect envelope for a resource
operation that passed admission.

## Quick Use

Start here when a patch behaved differently than expected:

```ts
const effect = line.effects().get(effectId)?.envelope;

console.log(effect?.profile?.name);
console.log(effect?.optimistic.kind);
console.log(effect?.optimistic.rollback.kind);
console.log(effect?.locus.kind);
console.log(effect?.patch.jsonPath);
```

Use `lastEffect` when you only need the most recently admitted effect for
diagnostics. Use `line.effects().get(effectId)` when request identity matters.

## What This Feature Is

`ResourceEffectEnvelope` is the stable record behind
`line.diagnostics().lastEffect`, history verification packages, rollback
results, and resource effect merge planning. It is created by the runtime, not
by application code.

## Why You Use It

- Explain why a visible value changed.
- Decide whether an optimistic patch can be rolled back.
- Pass a real resource effect to native branch merge helpers.
- Inspect response topology, JSON path, delivery, authority, and cost data.
- Debug profile choices without reading lower runtime internals.

## Stable Entry Points

- `line.diagnostics().lastEffect`
- `line.effects().get(effectId)?.envelope`
- `line.history().verificationPackage().lifecycle.lastEffect`
- `line.history().rollbackEffect(effectId)`
- `line.history().rollbackLastEffect()`
- `signals.resource.branch.planEffectMerge({ merge, effect })`
- `signals.resource.branch.mergeEffect({ merge, effect })`
- `ResourceEffectEnvelope`

The public API accepts envelopes issued by the runtime. forged or tampered
objects are denied.

## Core Mental Model

The envelope is not the mutation itself. It is the evidence record for a
resource mutation that has already passed admission.

The fields you will usually read are:

- `provenance`: whether the effect came from a local patch, delivered patch,
  delivered replacement, delivery invalidation, or basis refresh.
- `profile`: the selected optimism, confirmation, rollback, rebase, and
  preimage behavior.
- `branchLifecycle`: how the runtime acquired or declined branch speculation.
- `optimistic`: whether the effect was applied, committed only, or unavailable.
- `delivery`: packet and basis evidence for delivery effects.
- `locus` and `locusProof`: the resource-level place that changed. Think "which
  item, aspect, summary, response, map entry, group, or JSON path did this
  touch?"
- `patch`: patch scope, changed value status, and JSON path proof.
- `authority`: the runtime token and envelope digest used to reject forged
  effects.
- `counters`: breadth and cost counters for planning, execution, rollback,
  response lenses, and JSON path traversal.

## How It Executes

The runtime issues the envelope after admission succeeds and after it knows the
selected branch behavior. Denials happen before an envelope exists. That matters:
if a JSON path is unsafe, a topology hook corrupts identity, or a profile cannot
support the requested behavior, the visible value and diagnostics stay
unchanged.

When an envelope exists, it becomes the shared input for inspection, rollback,
verification, merge planning, and merge execution.

## Small Example

```ts
const admission = await line.patch(tasks.patch.itemAspect({
  itemId: "task:1",
  aspect: "title",
  value: "Draft",
}));
if (!("effectId" in admission)) throw new Error("effect was not admitted");

const effect = line.effects().get(admission.effectId)?.envelope;
if (!effect) throw new Error("effect envelope is unavailable");

console.log(effect.effectId);
console.log(effect.provenance);
console.log(effect.profile?.name);
console.log(effect.optimistic.rollback.kind);
console.log(effect.locus.kind);
```

This example reads the envelope by its runtime-issued request identity.

## Real Example

```ts
const inspected = line.effects().get(effectId);
if (!inspected) throw new Error("effect is unknown");
const effect = inspected.envelope;

if (effect.optimistic.rollback.kind === "effectBranchRetirementAvailable") {
  console.log(effect.optimistic.rollback.branchId);
  console.log(effect.optimistic.rollback.dependencyBasisBranchId);
}

if (effect.patch.jsonPath) {
  console.log(effect.patch.jsonPath.path);
  console.log(effect.counters.jsonPathTraversalBreadth);
  console.log(effect.counters.jsonPathReconstructionBreadth);
}

const plan = signals.resource.branch.planEffectMerge({
  merge: {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: canonicalBranchId,
  },
  effect,
});

console.log(plan.kind);
```

Use the effect for targeted rejection and JSON path details. Use
`planEffectMerge(...)` only for an explicit native branch workflow with a
known canonical target. Ordinary request confirmation goes through
`line.effects().confirm(effectId, ...)`.

## How It Relates To Other Features

- [Branch-Native Resource Effects](../resources/branch-native-effects.md)
  explains how envelopes are produced.
- [Effect Merge And Rebase](../resources/merge-and-rebase.md) explains how
  merge plans consume envelopes.
- [Response Topology Proof](./response-topology-proof.md) explains
  `locusProof`.
- [JSON Path Effects](../resources/json-effects.md) explains
  `patch.jsonPath`.
- [History And Restore](./history-and-restore.md) explains rollback results.

## Inspection And Debugging

Use `effect.authority.envelopeDigest` to correlate records without comparing
whole objects. Use `effect.plan.planId` and
`effect.plan.causalSequence` when debugging repeated or delivered operations.
Use `effect.counters` to understand why a topology or JSON path operation was
cheap or broad.

If `line.effects().get(effectId)` is `null`, the identity is unknown to that
line. Terminal effect summaries remain inspectable after their branches retire.

## Anti-Patterns

- Do not serialize an envelope, edit it, and pass it back to merge APIs.
- Do not treat `profile` as a request. It is the selected behavior after
  admission.
- Do not infer rollback from visible value changes. Read
  `effect.optimistic.rollback`.
- Do not use `lastEffect` as authority when several effects are open.
- Do not assume `locusProof` exists for raw or unsupported loci. The envelope
  can still name a generic resource locus without response topology proof.

## Current Limits

- The envelope is a debug and inspection surface, not a persistence format for
  application databases.
- Some delivery and committed-only effects have `rollback.kind:
  "notApplicable"` because they did not apply speculative branch state.
- Some resource loci can produce native branch merge proof without a response
  topology digest. In that case `topology` and `compiledLensDigest` can be
  `null`.

## Related Docs

- [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- [History And Restore](./history-and-restore.md)
- [Response Topology Proof](./response-topology-proof.md)
- [JSON Path Effects](../resources/json-effects.md)
