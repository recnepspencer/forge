# Profile Families And Composed Profile Sets

## What This Feature Is

This feature lets you describe one complete policy stance for a boundary
surface. A profile set says how much diagnostic detail you want, what support
posture applies, whether compatibility lowering is allowed, what readiness is
required, what retention level exists, and whether the result is uncertified,
evidence-backed, or production-certified.

## Why You Use It

- Use this when a runtime needs one explicit policy bundle instead of six loose
  knobs.
- Use this when you need composition legality checked up front instead of
  discovering bad combinations later.
- Use this when downstream code should hold one typed profile set rather than a
  bag of optional fields or defaults.

## Stable Entry Points

Common path:

- `profiles().set()`
- `profiles().set().compose()`
- `profiles().set().request()`

Lower lane:

- `FoundationalProfileSetInput`
- `FoundationalProfileSet::new(...)`

Family vocabulary:

- `DiagnosticRichnessProfile`
- `SupportPostureProfile`
- `CompatibilityPostureProfile`
- `AdmissionReadinessProfile`
- `RetentionDeliveryProfile`
- `CertificationPostureProfile`

Good to know:

- `profiles_api::common_path` is the recommended grouped public lane.
- `profiles_api::lower_lane::composition` is the inspectable lower lane.

## Core Mental Model

A composed profile set is the first stable policy object in Milestone 3.

It is not:

- a mutable runtime settings bag
- a partially filled draft object
- a default profile that gets fixed up later

It is one complete, coherent meaning bundle. You must assign every family
explicitly, and some combinations are illegal by design. For example,
production-certified posture cannot claim support or retention that is too
weak for that strength.

## How It Executes

The front door works in three steps:

1. assign all six profile families
2. compose the set and validate cross-family legality
3. optionally request the set immediately so progression can begin

Composition fails closed when:

- a family is missing
- a family is assigned twice on the common path
- the composed meaning violates strength rules

## Small Example

```rust
use worth_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};

let requested = profiles()
    .set()
    .diagnostic_richness(DiagnosticRichnessProfile::Standard)
    .support_posture(SupportPostureProfile::SupportReady)
    .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
    .admission_readiness(AdmissionReadinessProfile::Admitted)
    .retention_delivery(RetentionDeliveryProfile::Retained)
    .certification_posture(CertificationPostureProfile::EvidenceBacked)
    .request()?;
```

This is the smallest honest example because it assigns every family and moves
straight into a requested artifact instead of pretending partial construction
is valid.

## Real Example

```rust
use worth_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};

let profile = profiles()
    .set()
    .diagnostic_richness(DiagnosticRichnessProfile::Forensic)
    .support_posture(SupportPostureProfile::CertificationReady)
    .compatibility_posture(CompatibilityPostureProfile::CompatibilityRequired)
    .admission_readiness(AdmissionReadinessProfile::ProductionGateReady)
    .retention_delivery(RetentionDeliveryProfile::Durable)
    .certification_posture(CertificationPostureProfile::ProductionCertified)
    .compose()?;
```

What is authoritative here is the composed `FoundationalProfileSet` itself.
Nothing downstream should guess at missing families or invent default posture.
If this composition succeeds, later progression and attachment code can trust
that the profile meaning is internally coherent.

## How It Relates To Other Features

- [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
  takes the composed set and moves it through the progression phases.
- [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
  explains how the chosen families affect visible descriptive surfaces.
- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
  depends on certification posture already being coherent here.

## Inspection And Debugging

Inspect these first:

- the composed `FoundationalProfileSet` getters when you want the exact family
  values
- `FoundationalProfileCompositionDenial` when composition fails
- `profiles_api::lower_lane::composition` when you need the lower-lane
  vocabulary directly

If composition fails, the fix is almost always a cross-family mismatch, not a
runtime bug.

## Anti-Patterns

- Do not build profile meaning from partial structs or maps.
- Do not rely on implicit defaults for any family.
- Do not claim evidence-backed or production-certified posture with weak
  support, weak readiness, or ephemeral retention.

## Current Limits

- This layer only composes meaning. It does not admit, attach, materialize, or
  certify anything by itself.
- The common path rejects duplicate family assignment instead of silently
  overriding earlier choices.

## Related Docs

- [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
- [Profile Production Readiness](./profile-production-readiness.md)
