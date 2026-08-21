# Throughput And Observation Activation

Milestone 10 separates two choices that are often collapsed into one runtime
tier:

- `ExecutionObjectiveProfile` says which already-correct execution strategy
  the runtime should prefer: `LatencyBounded`, `Balanced`, or `Throughput`.
- `ObservationActivationProfile` says whether optional observation is
  `Continuous` or `OnDemand`.

The axes are independent. Throughput is not a weaker correctness or durability
mode, and OnDemand is not permission to remove authoritative state, stable
identity, replay linkage, acknowledgements, recovery, or custody evidence.

## Composing the profile

Both families are required by the common profile front door:

```rust
use worth_foundational::{
    profiles, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    ExecutionObjectiveProfile, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};

let profile = profiles()
    .set()
    .diagnostic_richness(DiagnosticRichnessProfile::OperationalMinimal)
    .support_posture(SupportPostureProfile::SupportReady)
    .compatibility_posture(CompatibilityPostureProfile::NativeOnly)
    .admission_readiness(AdmissionReadinessProfile::Admitted)
    .retention_delivery(RetentionDeliveryProfile::Retained)
    .certification_posture(CertificationPostureProfile::Uncertified)
    .execution_objective(ExecutionObjectiveProfile::Throughput)
    .observation_activation(ObservationActivationProfile::OnDemand)
    .compose()?;
```

The composed profile has one canonical identity. Changing either new family
changes that identity and is represented by its own resolution record during
requested → admitted → materialized progression. A single generic narrowing
record cannot hide a simultaneous objective and activation change.
Resolution record reasons are descriptive text only; family, relation, and
profile-derived transition validation remain the authoritative meaning.

## Observation disposition

The profile describes the allowed posture; a materialization plan carries the
runtime disposition for a concrete operation. Use
`FoundationalObservationDisposition::Inactive` when no observation session was
admitted, `Continuous` for an always-active lane, or
`ExplicitlyActivated { scope, session, observed_epoch }` for a typed session.

An inactive selected surface is reported with
`FoundationalSurfaceAbsenceCause::ObservationNotActivated`. That is distinct
from budget denial, omitted richness, non-retention, non-reconstructability,
and certification weakness.

## Optional performance work

Milestone 8 claims can disclose these optional work classes explicitly:

- structural counter capture
- diagnostic fact capture
- descriptive lineage record maintenance
- provenance fact capture
- replay sidecar maintenance

Including one requires `FoundationalPerformanceObservationContext`, which binds
the canonical profile identity and an active disposition. This is descriptive
eligibility, not a performed receipt; the adopting runtime still owns session
admission, counters, locking, retention, and execution proof.

## Signal handoff

`worth-signal` accepts a `SignalRuntimePolicyRequest`, admits it, resolves it,
and installs an `InstalledSignalRuntimePolicy`. Its planner consumes the
installed policy's resolved objective rather than selecting a strategy directly
from `DiagnosticsTier`. `SignalRuntimePolicy::operational()` is the public
request preset for Throughput + OnDemand; it is not an unconditional
performance claim. Observation-session phases prove zero optional retained
work when OnDemand is inactive.

Store and other runtimes may use the same vocabulary while retaining ownership
of durability, persistence, recovery, and lifecycle policy.
