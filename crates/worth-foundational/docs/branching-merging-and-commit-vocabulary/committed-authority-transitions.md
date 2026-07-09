# Committed Authority Transitions

## What This Feature Is

This feature is the first proof-bearing authority lane in Milestone 5.

It turns a commit-eligible merge verdict into a committed-authority artifact
with explicit:

- transition class
- no-op cause
- parent basis
- parentage
- merge ancestry basis
- committed delta summary
- proof-bearing admission

The transition class is not one generic "commit happened" bucket. The shipped
classes are:

- `NoOp`
- `Commit`
- `MetadataOnlyCommit`
- `PromotionCommit`
- `ReplayRevalidatedCommit`

The shipped no-op causes are also part of the history contract:

- `AlreadyConverged`
- `BasisEquivalent`
- `StrategySuppressed`
- `ChangeDenied`
- `ReplayEquivalent`

Use this when you are no longer planning or describing and are now crossing an
actual authority boundary.

## Why You Use It

Use this surface when you need to:

- represent a real authority transition instead of branch-local or merge-local
  intent
- preserve no-op versus commit meaning
- preserve canonical parentage and merge ancestry
- carry a proof-bearing committed transition into later receipt or
  current-basis lanes

This is the transition equivalent of "the write actually happened."

## Stable Entry Points

- `foundational_committed_authority_admission()`
- `FoundationalCommittedAuthorityInput`
- `FoundationalCommittedAuthorityArtifact<T>`
- `FoundationalAuthorityTransitionClass`
- `FoundationalAuthorityTransitionOutcomeKind`
- `FoundationalNoOpCause`
- `FoundationalCommitParentBasis`
- `FoundationalCommitParentage`
- `FoundationalMergeAncestryBasis`
- `FoundationalCommitDeltaSummary`

The main call site is:

- `merge_verdict.commit_with(input, authority)`

## Core Mental Model

Committed authority is stronger than merge admission.

The merge lane says what should happen or what would happen.

Committed authority says the authority transition was actually admitted under a
proof-bearing lane.

That is why this feature uses `AuthorityWitness::from_authority_marker`,
`Proof::from_authority_witness`, and
`Artifact::with_proofs_and_current_basis`.

Parentage is also intentionally multi-parent capable. Milestone 5 does not
collapse ancestry into one primary parent plus ad hoc merge metadata.

## How It Executes

You need:

- a commit-eligible merge verdict
- a `FoundationalCommittedAuthorityInput`
- the committed-authority admission witness

The input must already be honest about:

- transition class
- parent basis
- parentage
- merge ancestry basis if applicable
- committed delta summary
- explicit no-op cause if the transition class is `NoOp`

Then `commit_with(...)` returns a proof-bearing committed-authority artifact.

## Small Example

```rust
use worth_foundational::foundational_committed_authority_admission;

let committed = merge_verdict.commit_with(
    committed_input,
    foundational_committed_authority_admission(),
)?;
```

## Real Example

Inspect committed authority as a real authority surface, not as a payload bag:

```rust
let committed = merge_verdict.commit_with(
    committed_input,
    foundational_committed_authority_admission(),
)?;

let transition_class = committed.transition_class();
let no_op_cause = committed.no_op_cause();
let parent_basis = committed.parent_basis();
let parentage = committed.parentage();
let merge_ancestry_basis = committed.merge_ancestry_basis();
let delta_summary = committed.committed_delta_summary();
let proofs = committed.proofs();

let _ = (
    transition_class,
    no_op_cause,
    parent_basis,
    parentage,
    merge_ancestry_basis,
    delta_summary,
    proofs,
);
```

## How It Relates To Other Features

- [Merge Planning And Verdicts](./merge-planning-and-verdicts.md) is the lane
  that produces commit-eligible verdicts.
- [Commit Receipts, Discard, And Transition Bundles](./commit-receipts-discard-and-transition-bundles.md)
  derives receipts and bundles from committed authority.
- [Transition Canonical Basis, Locators, And Current-Basis](./transition-canonical-basis-locators-and-current-basis.md)
  strengthens committed authority further for canonical/current-basis use.

## Inspection And Debugging

Check these first:

- `transition_class()`
- `transition_outcome_kind()`
- `no_op_cause()`
- `parent_basis()`
- `parentage()`
- `merge_ancestry_basis()`
- `committed_delta_summary()`
- `proofs()`

If a commit was rejected here, the usual causes are:

- the verdict kind was not commit-eligible
- the parent basis was not actually part of parentage
- a no-op transition was missing an explicit cause
- a committed transition tried to carry a no-op cause
- a no-op transition tried to carry committed deltas

## Anti-Patterns

- Do not treat advisory or accepted verdicts as committed authority before
  `commit_with(...)` succeeds.
- Do not hand-assemble parentage outside the typed input surface and then hope
  consumers agree with it.
- Do not encode no-op meaning as "empty delta summary" alone.
- Do not invent a second proof-bearing authority lane for commits.

## Current Limits

- This feature only admits committed authority. It does not issue receipts by
  itself.
- It does not implement storage or journal behavior. It only standardizes the
  transition boundary meaning.
- It does not flatten replay or promotion semantics into ordinary commits. If
  the distinction matters, the transition class must keep it visible.

## Related Docs

- [Commit Receipts, Discard, And Transition Bundles](./commit-receipts-discard-and-transition-bundles.md)
- [Transition Canonical Basis, Locators, And Current-Basis](./transition-canonical-basis-locators-and-current-basis.md)
