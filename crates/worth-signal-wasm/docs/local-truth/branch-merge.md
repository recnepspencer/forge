# Branch Merge And Manual Resolution

## What This Feature Is

Local truth gives a standalone browser application an in-memory place to keep
branch values and decide merges. Use it when independent edits must compose,
overlapping edits must be reviewed, and the result must remain explainable
after derived Signal work is rebuilt.

You hold a `LocalTruthAuthority<T>` returned by `signals.localTruth(...)`.
The authority owns application values. Signal receives committed values and
recomputes derived work; it does not decide the merge.

## Why You Use It

- Two screens or workflows edit different fields of the same object and should
  compose without overwriting one another.
- Two edits overlap and a reviewer must choose source, target, or a separately
  authored custom value.
- A regulated workflow needs the exact basis, alternatives, decision, commit,
  and derived-state posture for inspection.

This feature is process-local. Use a server or the Worth Query and Relational
platform when values need durable, shared, or cross-process authority.

## Stable Entry Points

- `localTruthSchema(...)` declares the top-level fields that are mergeable.
- `signals.localTruth(...)` creates one authority and binds its entities to
  Signal inputs.
- `localTruth.branch(...)` reads a branch and its current basis.
- `localTruth.commit(...)` changes declared aspects on one branch.
- `localTruth.previewMerge(...)` classifies each selected aspect.
- `localTruth.createResolutionBranch(...)` opens the only lane for a custom
  conflict value.
- `localTruth.resolveMerge(...)` admits runtime-issued alternative IDs and
  publishes one merge commit.
- `localTruth.inspect(...)` reads values, heads, decisions, and counters.
- `localTruth.history(...)` reads the retained commit segment for one branch.
- `localTruth.historicalSnapshot(...)` reads the sealed values at one retained
  ancestor commit without changing the branch head.
- `localTruth.derivation(...)`, `destroyDerivation(...)`, and
  `rebuildDerivation(...)` inspect and test the disposable Signal projection.

The lower-level native Signal branch merge API remains available for derived
execution state. It is not an application-value merge API.

## Core Mental Model

The standalone direction is:

```text
TypeScript Local Truth -> Signal derivation
```

Local truth owns values, branch history, merge policy, reviews, and decisions.
A **basis** is the exact branch head a request expects. If that head changes,
the request is stale and publishes nothing.

Each declared aspect maps one semantic ID, such as `teeth`, to one top-level
field. The declaration supplies validation and equivalence posture. Merge
planning never guesses aspects from an arbitrary object diff.

Every admitted mutation or merge produces one immutable `LocalTruthCommit`.
Branch heads, inspection, Signal transactions, and UI views derive from that
commit.

## How It Executes

1. The authority validates the request, schema, branch, and expected basis.
2. It plans exact aspect operations without changing state.
3. It reconstructs and seals the complete next snapshot and commit.
4. It publishes the snapshot, commit, head, history, and lineage in one
   synchronous authority-local move.
5. Signal consumes the commit through an exact aspect-aware transaction.
6. If Signal delivery fails, truth stays committed and derivation becomes
   `RebuildRequired`.

Merge preview compares each selected entity/aspect locus independently.
Disjoint work becomes `AdoptSource` or `PreserveTarget`. Equivalent values are
recognized by the declared comparator. Real overlap becomes
`ResolutionRequired`.

## Small Example

```ts
import { createSignals, localTruthSchema } from "worth-signals-wasm";

const signals = await createSignals();
const gearSchema = localTruthSchema({
  id: "gear",
  aspects: [
    { id: "teeth", field: "teeth", valueType: "number",
      equivalence: { kind: "exact" }, costClass: "constant" },
    { id: "label", field: "label", valueType: "string",
      equivalence: { kind: "exact" }, costClass: "constant" },
  ],
});

const initial = { teeth: 18, label: "Drive gear" };
const gear = signals.input(initial, { producesAspects: [0, 1] });
const gearTruth = signals.localTruth({
  authorityId: "gear-editor",
  schema: gearSchema,
  initialEntities: { gear: initial },
  bindings: [{
    entityId: "gear",
    input: gear,
    aspectMap: { teeth: 0, label: 1 },
  }],
});

const main = await gearTruth.branch();
if (main.posture !== "success") throw new Error(main.message);

await gearTruth.commit({
  requestId: crypto.randomUUID(),
  branchId: main.value.id,
  expectedBasis: main.value.basis,
  operations: [{ entityId: "gear", aspectId: "teeth", value: 20 }],
});
```

The bound Signal input must start with the same value as the local-truth
entity. `aspectMap` translates semantic truth aspects to native numeric Signal
aspects for exact invalidation.

## Real Example

This example resolves one overlapping edit. The UI may choose an alternative
ID, but it never submits a merged gear object.

```ts
const preview = await gearTruth.previewMerge({
  sourceBranchId: source.id,
  targetBranchId: target.id,
  expectedSourceBasis: source.basis,
  expectedTargetBasis: target.basis,
  scope: { entityIds: ["gear"], aspectIds: ["teeth"] },
});

if (preview.posture !== "reviewRequired") {
  throw new Error("Expected a tooth-count conflict");
}

const conflict = preview.review.conflicts[0];
const resolution = await gearTruth.createResolutionBranch({
  reviewId: preview.review.id,
  conflictId: conflict.id,
  name: "Engineering resolution",
});
if (resolution.posture !== "success") throw new Error(resolution.message);

const resolutionBranch = resolution.value.branch;
await gearTruth.commit({
  requestId: crypto.randomUUID(),
  branchId: resolutionBranch.id,
  expectedBasis: resolutionBranch.basis,
  operations: [{ entityId: "gear", aspectId: "teeth", value: 21 }],
});

const custom = await gearTruth.resolutionAlternative({
  reviewId: preview.review.id,
  conflictId: conflict.id,
  resolutionBranchId: resolutionBranch.id,
});
if (custom.posture !== "success") throw new Error(custom.message);

await gearTruth.resolveMerge({
  requestId: crypto.randomUUID(),
  reviewId: preview.review.id,
  selections: [{
    reviewId: preview.review.id,
    conflictId: conflict.id,
    alternativeId: custom.value.id,
  }],
});
```

The custom value is an ordinary schema-validated commit on a dedicated
resolution branch. The final submission contains only IDs issued by the
authority. If source, target, schema, policy, review, or resolution head has
changed, resolution is denied before publication.
After a successful merge, every resolution branch admitted for that review is
retired, including unselected candidates, and its disposable Signal projection
is destroyed. The merge receipt lists those retired branch IDs.

## How It Relates To Other Features

- Pair local truth with ordinary inputs, computed signals, and outputs when a
  standalone app needs branch-aware application state.
- Resource optimistic effects remain one branch per pending effect. Confirmed
  resource observations use the same explicit local-truth boundary, while
  effect envelopes remain speculative intent.
- Use native Signal history for derived execution inspection and replay. Do
  not promote a Signal receipt into a local-truth basis.
- Use Query -> Relational -> Bridge -> Signal for durable platform workflows.

## Inspection And Debugging

`inspect()` returns immutable branch heads, values, the decision log, exact
counters, and a digest. `derivation(branchId)` reports `Current`,
`CommittedDerivationPending`, `RebuildRequired`, `Unavailable`, or `Failed`.

If derivation is stale, call `rebuildDerivation(branchId)`. Rebuild uses the
committed truth snapshot and does not create a second truth commit.

`checkpoint(branchId)` seals that branch's receipt and fork ancestry, exact
values, locus heads, lineage, compacted commit identities, and segment digest.
The runtime validates these fields before rebuilding disposable indexes. Once every active branch is checkpointed at its
current head, the authority discards the covered in-memory commit and snapshot
segments. `history(branchId)` then returns the checkpoint plus only the bounded
post-checkpoint segment. A later checkpoint binds the prior checkpoint digest.

`historicalSnapshot({ branchId, commitId })` first proves that the commit is in
the selected branch's retained ancestry. It then returns the authority-sealed
snapshot values and the exact number of commits visited. A sibling commit is
denied, and reading a historical snapshot never changes truth or Signal
derivation. Individual pre-checkpoint commits are intentionally unavailable
after compaction; the checkpoint head remains inspectable.

## Anti-Patterns

- Do not spread source and target objects together in React or Three.js.
- Do not infer merge aspects from changed object keys.
- Do not submit a raw custom JSON value in a resolution selection.
- Do not treat a Signal snapshot, branch receipt, or projection digest as an
  application truth basis.
- Do not retry a stale review by editing its IDs. Request a new preview.

## Current Limits

- Local truth is in-memory and process-local. It is not durable or shared.
- V1 supports declared top-level fields on plain-object entity values.
- Nested paths, collections, deletion topology, and identity migration are
  typed unsupported until they have dedicated materializers.
- Actor, reason, and correlation metadata are host assertions, not
  authenticated identity.
- Native extended profiles currently expose 32 numeric Signal aspects. Every
  projected truth aspect needs a valid native binding in the active profile.

## Related Docs

- [Standalone And Platform Authority Boundaries](./authority-boundaries.md)
- [Aspects](../app-surface/aspects.md)
- [Diagnostics And History](../app-surface/diagnostics-and-history.md)
- [Concurrent Optimistic Effects](../resources/effects/concurrency-and-dependencies.md)
