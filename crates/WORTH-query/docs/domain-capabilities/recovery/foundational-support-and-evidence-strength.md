# Foundational Support And Evidence Strength

Recovery answers do not all have the same strength.

Query recovery now exposes two related ideas:

- `WorthQueryRecoveryEvidenceStrength`
- foundational support and diagnostic context on the explanation

## Evidence Strength

The common cases are:

- `OrdinaryProjection`
  - derived from the compact ordinary lane
- `CheckedRetained`
  - derived from a checked artifact with retained source-family context
- `ProofRetained`
  - derived from a proof-visible transcript with the richest retained context

Use this to decide how much confidence and detail your app should show by
default.

## Foundational Support Context

When recovery needs to explain degraded or freshness-limited posture, the
explanation can also carry foundational support context:

- `support_truth_kind()`
- `basis_disclosure()`
- `degraded_recovery_posture()`

This is how Query distinguishes things like:

- stale retained basis
- reduced retained basis
- replay-reconstructed support truth
- rebuild-required support posture

That context is descriptive. It helps your app explain the stop honestly
without pretending the stop is already repaired.
