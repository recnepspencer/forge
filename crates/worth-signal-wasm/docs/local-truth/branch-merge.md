# Branch Merge And Manual Resolution

## The Frame

The happy path is short: preview two branches, inspect the aspect-level plan,
then publish it. Disjoint changes compose automatically. Equivalent changes
collapse. Overlapping changes become an explicit review instead of a silent
last-write-wins decision.

The same path extends to manual source, target, or custom resolution without
moving authority into UI state. It remains in-memory and process-local. Use
Query and Relational when the merge itself must be durable, shared, or governed
across processes.

## Mental Model

```text
TypeScript Local Truth -> Signal derivation

base snapshot
  |-- source: teeth = 20
  `-- target: thickness = 0.62
             |
          preview by declared aspect
             |
          resolve and publish one target merge commit
```

Local Truth owns branch values, ancestry, reviews, decisions, and commits.
Signal receives the committed target snapshot plus exact invalidation aspects.
Native Signal branches are derived execution branches; they do not decide an
application-value merge.

## Aspects Make The Merge Honest

A schema declares semantic loci such as `teeth` and `thickness`, maps each to
one top-level field, validates its value type, and states equivalence:

```ts
const gearSchema = localTruthSchema<Gear>({
  id: "gear",
  version: 1,
  aspects: [
    { id: "teeth", field: "teeth", valueType: "number",
      equivalence: { kind: "exact" }, costClass: "constant" },
    { id: "thickness", field: "thickness", valueType: "number",
      equivalence: { kind: "numberEpsilon", epsilon: 0.001 },
      costClass: "constant" },
  ],
});
```

Merge planning compares each selected entity/aspect locus against the
structural ancestor. It does not diff arbitrary JavaScript objects or infer
business meaning from field names.

## Preview, Then Publish

```ts
const source = await truth.branch(sourceBranchId);
const target = await truth.branch(targetBranchId);
if (source.posture !== "success" || target.posture !== "success") {
  throw new Error("Both branches must be current");
}

const preview = await truth.previewMerge({
  sourceBranchId,
  targetBranchId,
  expectedSourceBasis: source.value.basis,
  expectedTargetBasis: target.value.basis,
  policy: { overlap: "review" },
});
```

Each locus is classified as:

- `Unchanged`: neither side changed from the ancestor;
- `AdoptSource`: only source changed;
- `PreserveTarget`: only target changed;
- `Equivalent`: both changed to equivalent values under the schema comparator;
- `ResolutionRequired`: both changed incompatibly;
- `UnsupportedStructure`: the runtime cannot honestly plan that structure.

Preview never mutates either branch. Even a conflict-free or policy-resolved
preview must be published with `resolveMerge(...)`:

```ts
const review = preview.posture === "reviewRequired"
  ? preview.review
  : preview.posture === "success"
    ? preview.value
    : (() => { throw new Error(preview.message); })();

const merged = await truth.resolveMerge({
  requestId: crypto.randomUUID(),
  reviewId: review.id,
  selections: [],
});
if (merged.posture !== "success") throw new Error(merged.message);
```

`resolveMerge` publishes one merge commit on the target branch. The source
branch remains intact.

## Automatic Policy Is Still A Decision

`policy.overlap` accepts `review`, `preferSource`, or `preferTarget`. A prefer
policy can classify an overlap without asking for a manual selection, but it
does not skip preview or publish by itself. The review still captures the
bases, classifications, policy result, and eventual merge decision.

Use a prefer policy only when it is a real domain rule. Do not choose it merely
to make a demo avoid conflicts.

## Manual Source Or Target Resolution

A `ResolutionRequired` conflict contains runtime-issued alternatives. Submit
their IDs; never send an arbitrary value directly to `resolveMerge`:

```ts
const conflict = review.conflicts[0];
const sourceChoice = conflict.alternatives.find(
  alternative => alternative.choice === "source",
);
if (!sourceChoice) throw new Error("Source alternative unavailable");

await truth.resolveMerge({
  requestId: crypto.randomUUID(),
  reviewId: review.id,
  selections: [{
    reviewId: review.id,
    conflictId: conflict.id,
    alternativeId: sourceChoice.id,
  }],
});
```

Every unresolved conflict needs exactly one admitted selection. Alternatives
from another review, conflict, authority, or stale basis are denied.

## Author A Custom Resolution

Custom values travel through a narrow resolution branch so they receive the
same schema validation, basis checking, immutable commit, and inspection as
ordinary edits:

```ts
const resolution = await truth.createResolutionBranch({
  reviewId: review.id,
  conflictId: conflict.id,
  name: "reviewed-thickness",
});
if (resolution.posture !== "success") throw new Error(resolution.message);

const authored = await truth.commit({
  requestId: crypto.randomUUID(),
  branchId: resolution.value.branch.id,
  expectedBasis: resolution.value.branch.basis,
  operations: [{
    entityId: resolution.value.entityId,
    aspectId: resolution.value.aspectId,
    value: 0.6,
  }],
});
if (authored.posture !== "success") throw new Error(authored.message);

const custom = await truth.resolutionAlternative({
  reviewId: review.id,
  conflictId: conflict.id,
  resolutionBranchId: resolution.value.branch.id,
});
if (custom.posture !== "success") throw new Error(custom.message);

await truth.resolveMerge({
  requestId: crypto.randomUUID(),
  reviewId: review.id,
  selections: [{
    reviewId: review.id,
    conflictId: conflict.id,
    alternativeId: custom.value.id,
  }],
});
```

A resolution branch admits exactly one commit at its allowed locus. A
successful merge retires all resolution branches and projections attached to
that review, including alternatives that were not selected.

## Staleness And Failure

A review seals the source and target bases, schema identity, structural
ancestor, classifications, and alternatives. If either head advances before
resolution, the review is stale and publishes nothing. Preview again from
fresh branch receipts.

Merge publication is atomic at the Local Truth boundary. If downstream Signal
projection fails afterward, the target merge commit remains authoritative and
its derivation reports `RebuildRequired`. See
[History, Compaction, And Rebuild](./history-and-rebuild.md).

## Practical Boundaries

- Merge scope selects declared entity and aspect IDs, not arbitrary paths.
- The current schema covers declared top-level fields on plain objects.
- `numberEpsilon` changes equivalence, not validation or rounding.
- Reviews and decision logs are inspectable process memory, not durable audit.
- There is no network collaboration, authenticated reviewer identity, MVCC,
  or cross-process locking in this package.

## Related Docs

- [Branches And Snapshots](./branches-and-snapshots.md)
- [History, Compaction, And Rebuild](./history-and-rebuild.md)
- [Authority Boundaries](./authority-boundaries.md)
- [Local Truth API Reference](./api-reference.md)
