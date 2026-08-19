# Requested, Admitted, And Materialized Profile Progression

## What This Feature Is

This feature gives profile meaning a real lifecycle. You start with a requested
profile, admit it as something the runtime is willing to honor, and then
materialize the final profile that a concrete target will actually carry.

## Why You Use It

- Use this when you need profile narrowing to be explicit instead of implicit.
- Use this when the requested profile and the delivered profile may resolve
  several independent families at one boundary.
- Use this when a runtime or artifact needs a proof-bearing record of how a
  profile changed across phases.

## Stable Entry Points

Common path:

- `profiles().set().request()`
- `profiles().progression().admit_same(...)`
- `profiles().progression().admit_as_with_resolutions(...)`
- `profiles().progression().materialize_same(...)`
- `profiles().progression().materialize_as_with_resolutions(...)`

Lower lane:

- `request_foundational_profile_set(...)`
- `admit_requested_foundational_profile_with_resolutions(...)`
- `materialize_admitted_foundational_profile_with_resolutions(...)`
- `FoundationalProfileResolutionLedger`
- `FoundationalProfileResolutionRecord`
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

If meaning changes across phases, the change must be validated and carried, not
silently rewritten. The canonical family-keyed resolution ledger records every
changed family and its relation. The older narrowing record is only a
descriptive compatibility projection for monotonic narrowing consumers; it is
not a second authority or stored transition representation.

## How It Executes

Progression enforces these rules:

1. requested and admitted profiles may not strengthen monotonic families
2. admission readiness cannot drift across progression
3. every changed family must have the matching canonical resolution relation
4. duplicate, omitted, unexpected, or wrongly-related records are denied

The same rules apply again from admitted to materialized. Objective and
observation-activation adjustments are orthogonal selections, so both may be
carried in one ledger without being misclassified as a single narrowing.

## Small Example

```rust
use worth_foundational::{
    profiles, DiagnosticRichnessProfile, FoundationalProfileResolutionLedger,
};
use worth_proof::TransitionOutcome;

let requested = profiles().set()
    // assign all families...
    .request()?;

let resolutions = FoundationalProfileResolutionLedger::empty();

let admitted = match profiles().progression().admit_as_with_resolutions(
    requested,
    admitted_profile,
    resolutions,
) {
    TransitionOutcome::Success(admitted) => admitted,
    other => return Err(format!("progression failed: {other:?}").into()),
};
```

This is the smallest honest example because it shows the one thing progression
exists to protect: explicit, family-complete resolution.

## Real Example

```rust
use worth_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    ExecutionObjectiveProfile, FoundationalProfileResolutionFamily,
    FoundationalProfileResolutionLedger, FoundationalProfileResolutionRecord,
    FoundationalProfileResolutionRelation, ObservationActivationProfile,
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
    .execution_objective(ExecutionObjectiveProfile::Throughput)
    .observation_activation(ObservationActivationProfile::Continuous)
    .request()?;

let admitted = match profiles().progression().admit_same(requested) {
        TransitionOutcome::Success(admitted) => admitted,
    other => return Err(format!("admission failed: {other:?}").into()),
};

let mut resolutions = FoundationalProfileResolutionLedger::empty();
resolutions.insert(FoundationalProfileResolutionRecord::new(
    FoundationalProfileResolutionFamily::DiagnosticRichness,
    FoundationalProfileResolutionRelation::Narrowing,
    "proof-bearing targets can keep the same truth with less descriptive detail",
))?;

let materialized = match profiles().progression().materialize_as_with_resolutions(
    admitted,
    profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::CertificationReady)
        .compatibility_posture(CompatibilityPostureProfile::CompatibilityRequired)
        .admission_readiness(AdmissionReadinessProfile::ProductionGateReady)
        .retention_delivery(RetentionDeliveryProfile::Durable)
        .certification_posture(CertificationPostureProfile::ProductionCertified)
        .execution_objective(ExecutionObjectiveProfile::Throughput)
        .observation_activation(ObservationActivationProfile::Continuous)
        .compose()?,
    resolutions,
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
- `requested_to_admitted_resolutions()` and
  `admitted_to_materialized_resolutions()` on the payloads
- `FoundationalProfileProgressionDenial` when progression fails
- `FoundationalProfileNarrowingKind` only when inspecting the descriptive
  compatibility projection

If progression fails, it usually means the caller changed too many families or
tried to strengthen meaning instead of narrowing it.

## Anti-Patterns

- Do not treat profiles as mutable "effective config."
- Do not hide a changed family in comments or call-site folklore.
- Do not let admission readiness drift between requested and admitted meaning.

## Current Limits

- Monotonic narrowing remains limited to one family per step; orthogonal
  objective and activation selections may resolve together.
- Progression records meaning changes. It does not attach payloads yet.

## Related Docs

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
