# Proof-Bearing Profile Certification And Readmission

## What This Feature Is

This feature is the stronger lane for profile-bearing artifacts. It turns a
proof-bearing profiled artifact into an evidence-backed or production-certified
artifact, and it makes trust-boundary crossing and readmission explicit.

## Why You Use It

- Use this when a profiled artifact needs stronger proof than plain attachment
  gives you.
- Use this when crossing a trust boundary must preserve shape but not current
  proof authority.
- Use this when production-certified posture should be enforced mechanically
  instead of by convention.

## Stable Entry Points

Common path:

- `profiles().certification().evidence_backed(...)`
- `profiles().certification().production_certified(...)`
- `profiles().certification().bridge_evidence_backed(...)`
- `profiles().certification().readmit_evidence_backed(...)`
- `profiles().certification().bridge_production_certified(...)`
- `profiles().certification().readmit_production_certified(...)`

Lower lane:

- `certify_evidence_backed_proof_bearing_artifact(...)`
- `certify_production_certified_proof_bearing_artifact(...)`
- `bridge_*_trust_boundary(...)`
- `readmit_*_after_boundary(...)`

Good to know:

- `profiles_api::lower_lane::certification` is the inspectable lower lane.
- `profiles_api::stronger_lane::readiness` is the stronger grouped readiness
  lane that freezes this capability as shipped.

## Core Mental Model

Certification strengthening is not "flip a posture enum and keep going."

It is a proof-bearing transition:

- evidence-backed strengthening requires evidence-backed or stronger profile
  meaning
- production-certified strengthening requires production-certified profile
  meaning
- boundary crossing drops current authority and requires explicit readmission

## How It Executes

The normal flow is:

1. attach a proof-bearing profiled artifact
2. strengthen it into evidence-backed form if the materialized profile allows
   that claim
3. optionally strengthen again into production-certified form
4. bridge the artifact across a trust boundary when needed
5. readmit it with the milestone-owned authority on the other side

## Small Example

```rust
use worth_foundational::profiles;
use worth_proof::TransitionOutcome;

let evidence_backed = match profiles().certification().evidence_backed(proof_bearing) {
    TransitionOutcome::Success(certified) => certified,
    other => return Err(format!("strengthening failed: {other:?}").into()),
};
```

This is the smallest honest example because the stronger lane begins only after
you already hold a proof-bearing profiled artifact.

## Real Example

```rust
use worth_foundational::profiles;
use worth_proof::TransitionOutcome;

let evidence_backed = match profiles().certification().evidence_backed(proof_bearing) {
    TransitionOutcome::Success(certified) => certified,
    other => return Err(format!("evidence-backed strengthening failed: {other:?}").into()),
};

let production = match profiles().certification().production_certified(evidence_backed) {
    TransitionOutcome::Success(certified) => certified,
    other => return Err(format!("production certification failed: {other:?}").into()),
};

let bridged = profiles().certification().bridge_production_certified(production);
let readmitted = profiles().certification().readmit_production_certified(bridged);
```

What is authoritative here is the stronger artifact at each step. After
bridging, the shape survives, but the current proof must be re-established with
the readmission authority before you can treat it as current again.

## How It Relates To Other Features

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
  is the gate into this lane.
- [Profile Production Readiness](./profile-production-readiness.md)
  freezes the stronger certification surfaces and forbidden shortcuts.
- [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
  explains the descriptive surfaces the stronger artifact may still carry.

## Inspection And Debugging

Inspect these first:

- `profiles_api::lower_lane::certification` when you need the exact stronger
  certification and readmission vocabulary
- the materialized profile on the profiled artifact before strengthening
- `FoundationalProfileCertificationDenial` when strengthening fails
- whether the artifact is boundary-bridged or readmitted when current proof
  seems unexpectedly unavailable

If production-certified strengthening fails, the usual cause is that the
materialized profile is not actually production-certified.

## Anti-Patterns

- Do not use support or boundary artifacts where proof-bearing artifacts are
  required.
- Do not assume evidence-backed and production-certified artifacts are
  interchangeable.
- Do not treat trust-boundary bridging as proof-preserving.

## Current Limits

- This lane only strengthens proof-bearing profiled artifacts.
- Boundary crossing always requires explicit readmission before current proof is
  considered live again.

## Related Docs

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
- [Profile Production Readiness](./profile-production-readiness.md)
