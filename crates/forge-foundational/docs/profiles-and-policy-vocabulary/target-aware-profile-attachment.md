# Target-Aware Profile Attachment

## What This Feature Is

This feature attaches materialized profile meaning to a real target payload.
The target matters: a boundary artifact, a support artifact, and a proof-bearing
artifact do not accept the same policy claims.

## Why You Use It

- Use this when a payload needs profile meaning carried with it as a typed
  artifact.
- Use this when support and proof-bearing targets need stricter legality than
  boundary targets.
- Use this when you want target kind to be part of the type, not hidden in a
  string or enum field outside the artifact.

## Stable Entry Points

Common path:

- `profiles().attach().to_boundary_artifact(...)`
- `profiles().attach().to_support_artifact(...)`
- `profiles().attach().to_proof_bearing_artifact(...)`

Lower lane:

- `attach_boundary_profiled_artifact(...)`
- `attach_support_profiled_artifact(...)`
- `attach_proof_bearing_profiled_artifact(...)`
- `BoundaryProfiledArtifact<_>`
- `SupportProfiledArtifact<_>`
- `ProofBearingProfiledArtifact<_>`

Good to know:

- `profiles_api::common_path` is the recommended grouped public lane.
- `profiles_api::lower_lane::attachment` is the inspectable lower lane.

## Core Mental Model

Attachment is a legal boundary crossing.

You are not "wrapping a payload with metadata." You are saying:

- this payload is now carrying materialized profile meaning
- this target kind allows that meaning
- any narrowing between admitted and materialized meaning was explicit

Different targets reject different claims. For example, support artifacts
cannot carry internal-only support posture, and proof-bearing artifacts require
admitted readiness.

## How It Executes

Attachment does three jobs together:

1. materialize the admitted profile for the target
2. enforce target-specific denial rules
3. return a typed profiled artifact whose target kind is fixed

The returned artifact keeps:

- the payload
- the materialized profile
- the target kind

## Small Example

```rust
use forge_foundational::profiles;
use forge_proof::TransitionOutcome;

let attached = match profiles().attach().to_support_artifact(
    admitted_profile,
    support_profile,
    narrowing,
    payload,
) {
    TransitionOutcome::Success(profiled) => profiled,
    other => return Err(format!("attachment failed: {other:?}").into()),
};
```

This is the smallest honest example because attachment is always target-aware.
There is no one generic "attach anything anywhere" helper.

## Real Example

```rust
use forge_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

let proof_bearing = match profiles().attach().to_proof_bearing_artifact(
    admitted,
    profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::CertificationReady)
        .compatibility_posture(CompatibilityPostureProfile::CompatibilityRequired)
        .admission_readiness(AdmissionReadinessProfile::ProductionGateReady)
        .retention_delivery(RetentionDeliveryProfile::Durable)
        .certification_posture(CertificationPostureProfile::ProductionCertified)
        .compose()?,
    Some(FoundationalProfileNarrowingRecord::new(
        FoundationalProfileNarrowingKind::RichnessReduced,
        "proof-bearing consumers keep the same authority with less descriptive richness",
    )),
    domain_payload,
) {
    TransitionOutcome::Success(profiled) => profiled,
    other => return Err(format!("proof-bearing attachment failed: {other:?}").into()),
};
```

What is authoritative here is the profiled artifact returned by the transition.
The payload is still the domain payload, but now it carries a target-scoped
materialized profile and target kind that downstream code can inspect safely.

## How It Relates To Other Features

- [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
  produces the admitted meaning attachment starts from.
- [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
  uses the attached profile to plan target-scoped descriptive surfaces.
- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
  starts from proof-bearing profiled artifacts, not generic payloads.

## Inspection And Debugging

Inspect these first:

- `profiles_api::lower_lane::attachment` when you need the raw attachment and
  profiled-artifact vocabulary
- `artifact.payload().profile()` for the materialized profile
- `artifact.payload().target_kind()` for the fixed target kind
- `FoundationalProfileAttachmentDenial` for target legality failures

If a support or proof-bearing attachment fails, the denial is usually about
support posture or admitted readiness, not the payload itself.

## Anti-Patterns

- Do not treat boundary, support, and proof-bearing targets as interchangeable.
- Do not attach payloads first and ask legality questions later.
- Do not smuggle plain payloads into APIs that require profiled artifacts.

## Current Limits

- Attachment is still profile-scoped. It does not certify stronger proof on its
  own.
- Boundary attachments are more permissive than support or proof-bearing
  attachments, by design.

## Related Docs

- [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
