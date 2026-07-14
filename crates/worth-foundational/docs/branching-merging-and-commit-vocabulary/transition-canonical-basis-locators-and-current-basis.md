# Transition Canonical Basis, Locators, And Current-Basis

## What This Feature Is

This feature lowers Milestone 5 transition artifacts through the canonical
basis lane and then lets stronger transition artifacts move into current-basis
and trust-boundary/readmission behavior.

It is also where transition locators and profile reuse become part of the
shared transition surface.

## Why You Use It

Use this surface when you need to:

- prepare branch-local, merge, committed-authority, or receipt artifacts for
  canonical basis
- compare or export transition meaning on a canonical surface
- strengthen committed authority or receipts into current-basis artifacts
- bridge a trust boundary and explicitly readmit the artifact later

This is the lane that stops later consumers from guessing at transition basis
or silently preserving current-basis strength across boundaries.

## Stable Entry Points

Canonical preparation:

- `prepare_branch_candidate_for_canonical_basis(...)`
- `prepare_staged_branch_for_canonical_basis(...)`
- `prepare_merge_verdict_for_canonical_basis(...)`
- `prepare_committed_authority_for_canonical_basis(...)`
- `prepare_commit_receipt_for_canonical_basis(...)`
- `foundational_transition_canonical_basis_entries(...)`

Current-basis strengthening:

- `admit_current_basis_committed_authority(...)`
- `admit_current_basis_commit_receipt(...)`
- `bridge_current_basis_committed_authority_trust_boundary(...)`
- `bridge_current_basis_commit_receipt_trust_boundary(...)`
- `readmit_current_basis_committed_authority_after_boundary(...)`
- `readmit_current_basis_commit_receipt_after_boundary(...)`

Profile reuse:

- `attach_boundary_profiled_branch_candidate(...)`
- `attach_boundary_profiled_staged_branch(...)`
- `attach_support_profiled_merge_verdict(...)`
- `attach_proof_bearing_profiled_committed_authority(...)`
- `attach_proof_bearing_profiled_commit_receipt(...)`

Locator participation:

- `FoundationalTransitionLocator`
- `FoundationalBranchCandidateLocator`
- `FoundationalMergeConflictLocator`
- `FoundationalCommitParentageLocator`
- `FoundationalCommittedDeltaLocator`

## Core Mental Model

Canonical basis and current-basis solve different problems.

Canonical basis answers:

- what this transition means
- how to compare or digest it honestly
- where the meaningful transition loci live for blind consumers

Current-basis answers:

- whether this transition artifact still carries a live stronger basis claim
- whether that stronger claim survived a trust boundary

Do not confuse those layers. Canonical preparation is not current-basis
admission, and current-basis admission is not a free pass through boundaries.

## How It Executes

Canonical flow:

1. start from a transition artifact
2. prepare it for canonical basis using the right function
3. consume the ready artifact downstream

Current-basis flow:

1. start from committed authority or a commit receipt
2. admit it into current-basis with the milestone-owned authority witness
3. optionally bridge a trust boundary
4. explicitly readmit with the readmission authority later

Profile attachment is not optional garnish here. Milestone 5 explicitly reuses
the Milestone 3 attachment/materialization law instead of creating a
transition-local profile dialect.

## Small Example

```rust
use worth_foundational::{
    admit_current_basis_committed_authority,
    foundational_transition_current_basis_authority,
};

let current_basis = admit_current_basis_committed_authority(
    version,
    committed,
    foundational_transition_current_basis_authority(),
)?;
```

## Real Example

Use the trust-boundary and readmission path when the stronger transition leaves
the original trust boundary:

```rust
use worth_foundational::{
    bridge_current_basis_commit_receipt_trust_boundary,
    foundational_transition_current_basis_readmission_authority,
    readmit_current_basis_commit_receipt_after_boundary,
};

let bridged = bridge_current_basis_commit_receipt_trust_boundary(current_basis_receipt);

let readmitted = readmit_current_basis_commit_receipt_after_boundary(
    bridged,
    rebound_basis,
    foundational_transition_current_basis_readmission_authority(),
);

let receipt = readmitted.receipt();
let basis = readmitted.strong_basis();
let _ = (receipt, basis);
```

## How It Relates To Other Features

- [Committed Authority Transitions](./committed-authority-transitions.md) and
  [Commit Receipts, Discard, And Transition Bundles](./commit-receipts-discard-and-transition-bundles.md)
  provide the stronger transition artifacts this lane consumes.
- [Transition Production Readiness](./transition-production-readiness.md)
  freezes the exact assumptions and `worth-proof` API choices for this lane.

## Inspection And Debugging

Check these first:

- the canonical preparation denial or success path
- the emitted `FoundationalTransitionLocator` variant when the consumer needs
  branch-candidate, merge-conflict, parentage, or committed-delta structure
- `strong_basis()` on current-basis artifacts
- whether the artifact was bridged and not yet readmitted
- whether the right current-basis authority or readmission authority was used

If a consumer still sees plain committed authority or a plain receipt after a
boundary crossing, it has not been strengthened or re-admitted yet.

## Anti-Patterns

- Do not preserve current-basis strength implicitly across a trust boundary.
- Do not rebuild canonical basis locally in another crate.
- Do not attach profile meaning ad hoc when the Milestone 3 attachment helpers
  already exist.
- Do not use current-basis APIs on branch-local or plain merge surfaces.

## Current Limits

- This feature only strengthens committed authority and receipts into
  current-basis surfaces.
- It does not provide a generic current-basis lane for every transition noun.
- Branch-local and plain merge surfaces can canonicalize and profile, but they
  do not gain stronger current-basis behavior here.

## Related Docs

- [Committed Authority Transitions](./committed-authority-transitions.md)
- [Transition Production Readiness](./transition-production-readiness.md)
