# Requested, Admitted, And Materialized Profile Progression

## What This Feature Is

This feature gives profile meaning a real lifecycle. You start with a requested
profile, admit it as something the runtime is willing to honor, and then
materialize the final profile that a concrete target will actually carry.

## Why You Use It

- Use this when you need profile narrowing to be explicit instead of implicit.
- Use this when the requested profile and the delivered profile might differ by
  one controlled family.
- Use this when a runtime or artifact needs a proof-bearing record of how a
  profile changed across phases.

## Stable Entry Points

Common path:

- `profiles().set().request()`
- `profiles().progression().admit_same(...)`
- `profiles().progression().admit_as(...)`
- `profiles().progression().materialize_same(...)`
- `profiles().progression().materialize_as(...)`

Lower lane:

- `request_foundational_profile_set(...)`
- `admit_requested_foundational_profile(...)`
- `materialize_admitted_foundational_profile(...)`
- `FoundationalProfileNarrowingRecord`
- `FoundationalProfileNarrowingKind`

Good to know:

- `profiles_api::common_path` is the recommended grouped public lane.
- `profiles_api::lower_lane::progression` is the inspectable lower lane.

## Core Mental Model

There are three phases:

- requested: what the caller wants
- admitted: what the runtime is willing to accept
- materialized: what a real target will carry

If meaning changes across phases, the change must be a narrowing, not a silent
rewrite. A narrowing record names both the kind of reduction and the reason.

## How It Executes

Progression enforces these rules:

1. requested and admitted profiles may differ in at most one family
2. that difference must be a narrowing, never a strengthening
3. admission readiness cannot drift across progression
4. if narrowing happened, the caller must provide a matching
   `FoundationalProfileNarrowingRecord`

The same rules apply again from admitted to materialized.

## Small Example

```rust
use worth_foundational::{
    profiles, DiagnosticRichnessProfile, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord,
};
use worth_proof::TransitionOutcome;

let requested = profiles().set()
    // assign all families...
    .request()?;

let admitted = match profiles().progression().admit_as(
    requested,
    admitted_profile,
    Some(FoundationalProfileNarrowingRecord::new(
        FoundationalProfileNarrowingKind::RichnessReduced,
        "support consumers do not require forensic detail",
    )),
) {
    TransitionOutcome::Success(admitted) => admitted,
    other => return Err(format!("progression failed: {other:?}").into()),
};
```

This is the smallest honest example because it shows the one thing progression
exists to protect: explicit narrowing.

## Real Example

```rust
use worth_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

let requested = profiles()
    .set()
    .diagnostic_richness(DiagnosticRichnessProfile::Forensic)
    .support_posture(SupportPostureProfile::CertificationReady)
    .compatibility_posture(CompatibilityPostureProfile::CompatibilityRequired)
    .admission_readiness(AdmissionReadinessProfile::ProductionGateReady)
    .retention_delivery(RetentionDeliveryProfile::Durable)
    .certification_posture(CertificationPostureProfile::ProductionCertified)
    .request()?;

let admitted = match profiles().progression().admit_same(requested) {
    TransitionOutcome::Success(admitted) => admitted,
    other => return Err(format!("admission failed: {other:?}").into()),
};

let materialized = match profiles().progression().materialize_as(
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
        "proof-bearing targets can keep the same truth with less descriptive detail",
    )),
) {
    TransitionOutcome::Success(materialized) => materialized,
    other => return Err(format!("materialization failed: {other:?}").into()),
};
```

What is authoritative here is the progression artifact at each phase. The code
does not mutate one profile in place. It records what was requested, what was
admitted, and what was finally materialized.

## How It Relates To Other Features

- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
  explains what gets requested in the first place.
- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
  uses admitted meaning to produce target-scoped artifacts.
- [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
  uses materialized meaning to decide surface availability.

## Inspection And Debugging

Inspect these first:

- `profiles_api::lower_lane::progression` when you need the exact staged
  progression vocabulary
- `requested_to_admitted_narrowing()` and
  `admitted_to_materialized_narrowing()` on the payloads
- `FoundationalProfileProgressionDenial` when progression fails
- `FoundationalProfileNarrowingKind` when the wrong family seems to be changing

If progression fails, it usually means the caller changed too many families or
tried to strengthen meaning instead of narrowing it.

## Anti-Patterns

- Do not treat profiles as mutable "effective config."
- Do not hide narrowing in comments or call-site folklore.
- Do not let admission readiness drift between requested and admitted meaning.

## Current Limits

- Only one family may narrow at each progression step.
- Progression records meaning changes. It does not attach payloads yet.

## Related Docs

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
