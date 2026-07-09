# Commit Receipts, Discard, And Transition Bundles

## What This Feature Is

This feature covers the transition surfaces that come after committed
authority:

- commit receipts
- non-authoritative discard receipts
- transition provenance rows
- coordinated transition bundles

Use it when you need emitted transition evidence instead of just a committed
authority artifact.

## Why You Use It

Use this surface when you need to:

- issue a receipt from a committed authority artifact
- preserve a real receipt evidence floor
- emit a typed discard/closeout artifact for branch-local work
- emit a coordinated bundle of committed authority plus optional summary,
  merge report, and receipt

This feature is where Milestone 5 stops transitions from turning into thin
metadata envelopes or local result bags.

## Stable Entry Points

- `foundational_commit_receipt_issuance()`
- `FoundationalCommitReceiptArtifact`
- `FoundationalCommitReceiptIdentity`
- `FoundationalCommitId`
- `FoundationalBranchDiscardReceipt`
- `FoundationalTransitionProvenanceRow`
- `FoundationalTransitionBundleBuilder<T>`
- `FoundationalTransitionBundle<T>`

Main calls:

- `committed.issue_receipt(receipt_identity, commit_id, authority)`
- `committed.emit_transition_bundle()`
- `candidate.discard_with_zero_residue_proof()`

## Core Mental Model

Receipts are derived from committed authority.

They are not just another view of the payload. They carry attested transition
evidence:

- commit id
- branch id
- parent basis
- parentage
- transition class
- no-op cause when relevant
- delta evidence
- provenance rows

Discard receipts are different. They are explicit non-authoritative closeout
evidence for branch-local work that never crossed authority.

Report-only bundles are different too. They may carry transition provenance and
descriptive summary/report members, but they must not fake receipt attestation
fields when no receipt was actually issued.

## How It Executes

For commit receipts:

1. start from committed authority
2. choose receipt identity and commit id
3. provide the issuance authority witness
4. issue the receipt

For discard receipts:

1. start from a branch-local candidate
2. discard with zero-residue proof

For bundles:

1. start from committed authority
2. opt into summary, merge report, and/or receipt
3. materialize the bundle

## Small Example

```rust
use worth_foundational::{
    foundational_commit_receipt_issuance, FoundationalCommitId,
    FoundationalCommitReceiptIdentity,
};

let receipt = committed.issue_receipt(
    FoundationalCommitReceiptIdentity::new(17).expect("receipt identity"),
    FoundationalCommitId::new(41).expect("commit id"),
    foundational_commit_receipt_issuance(),
)?;
```

## Real Example

Use the bundle lane when a caller needs more than one coordinated output:

```rust
use worth_foundational::{
    foundational_commit_receipt_issuance, FoundationalCommitId,
    FoundationalCommitReceiptIdentity,
};

let bundle = committed
    .emit_transition_bundle()
    .with_summary()
    .with_merge_report()
    .with_receipt(
        FoundationalCommitReceiptIdentity::new(17).expect("receipt identity"),
        FoundationalCommitId::new(41).expect("commit id"),
        foundational_commit_receipt_issuance(),
    )
    .materialize()?;

let primary = bundle.primary();
let summary = bundle.summary();
let report = bundle.merge_report();
let receipt = bundle.receipt();
let cost = bundle.materialization_cost();

let _ = (primary, summary, report, receipt, cost);
```

## How It Relates To Other Features

- [Committed Authority Transitions](./committed-authority-transitions.md) is
  the only lane that can issue a real commit receipt.
- [Transition Canonical Basis, Locators, And Current-Basis](./transition-canonical-basis-locators-and-current-basis.md)
  canonicalizes committed authority and receipts for later strengthening.

## Inspection And Debugging

For receipts, inspect:

- `receipt_identity()`
- `commit_id()`
- `parent_basis()`
- `parentage()`
- `transition_class()`
- `delta_evidence()`
- `transition_provenance_rows()`
- `receipt_claim()`
- `issuance_cause()`

For discard receipts, inspect:

- `branch_id()`
- `fork_basis()`
- `closeout_cause()`
- `non_authoritative_residue_report()`

For bundles, inspect:

- `primary()`
- `summary()`
- `merge_report()`
- `receipt()`
- `materialization_cost()`

If something looks too weak, check whether you emitted only a merge report
without a receipt. Report-only bundles are descriptive and do not fake receipt
attestation.

If you are consuming provenance rows, remember that `commit_id()`,
`receipt_identity()`, and `issuance_cause()` are allowed to be absent on
report-only paths. Their absence is real evidence, not an omitted field to fill
in locally.

## Anti-Patterns

- Do not mint receipts from branch-local or merge-local state.
- Do not treat discard receipts as commit receipts.
- Do not collapse coordinated transition emission into a generic
  `TransitionResult { ... }` bag.
- Do not assume a merge report implies a receipt exists.

## Current Limits

- This feature emits transition evidence. It does not make the evidence
  current-basis or boundary-readmitted by itself.
- Summary and merge report members are optional and descriptive.
- Discard receipts remain non-authoritative even when they are well structured
  and zero-residue.

## Related Docs

- [Committed Authority Transitions](./committed-authority-transitions.md)
- [Transition Canonical Basis, Locators, And Current-Basis](./transition-canonical-basis-locators-and-current-basis.md)
