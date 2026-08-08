//! Stage-typed construction of the external-effect causal ladder.

use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::super::WorthQueryAftermathDerivationFailure;
use super::identity_derivation::{
    attempt_identity, emission_identity, observation_identity, provider_commit_identity,
};
use super::{ExternalEffectPosture, ExternalEffectPostureKind, WorthQueryDispatchOutboxRecord};
use crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxObservation;

pub(super) struct CoCommittedApplicationEmission {
    provider_commit: ExternalEffectPosture,
    emission: ExternalEffectPosture,
    committed: WorthQueryCommittedDispatchOutboxObservation,
}

pub(super) struct DispatchAttemptEvent<'emission> {
    emission: &'emission CoCommittedApplicationEmission,
    attempt: ExternalEffectPosture,
}

pub(super) fn admit_co_committed_emission(
    runtime: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    committed: WorthQueryCommittedDispatchOutboxObservation,
) -> Result<
    (
        CoCommittedApplicationEmission,
        WorthQueryCanonicalWorkEvidence,
    ),
    WorthQueryAftermathDerivationFailure,
> {
    let provider = provider_commit_identity(runtime, &committed)?;
    let provider_posture =
        ExternalEffectPosture::root(ExternalEffectPostureKind::ProviderCommit, provider.identity);
    let emitted = emission_identity(
        provider_posture.identity(),
        committed.record().correlation(),
        committed.record().outcome_identity(),
    )?;
    let emission_posture = ExternalEffectPosture::successor(
        ExternalEffectPostureKind::EmittedApplicationCausality,
        emitted.identity,
        &provider_posture,
    );
    Ok((
        CoCommittedApplicationEmission {
            provider_commit: provider_posture,
            emission: emission_posture,
            committed,
        },
        provider.work.combine(emitted.work),
    ))
}

pub(super) fn begin_dispatch_attempt(
    emission: &CoCommittedApplicationEmission,
    attempt_ordinal: crate::domain_computation::primary_graph::WorthQueryExternalDispatchAttemptOrdinal,
) -> Result<
    (DispatchAttemptEvent<'_>, WorthQueryCanonicalWorkEvidence),
    WorthQueryAftermathDerivationFailure,
> {
    let derived = attempt_identity(
        emission.emission.identity(),
        emission.record().correlation(),
        attempt_ordinal.get(),
    )?;
    let attempt = ExternalEffectPosture::successor(
        ExternalEffectPostureKind::DispatchAttempt,
        derived.identity,
        &emission.emission,
    );
    Ok((DispatchAttemptEvent { emission, attempt }, derived.work))
}

pub(super) fn observe_completion(
    attempt: &DispatchAttemptEvent<'_>,
) -> Result<
    (ExternalEffectPosture, WorthQueryCanonicalWorkEvidence),
    WorthQueryAftermathDerivationFailure,
> {
    observe_attempt(
        attempt,
        ExternalEffectPostureKind::ExternalCompletion,
        "completed",
    )
}

pub(super) fn observe_acknowledgement(
    attempt: &DispatchAttemptEvent<'_>,
) -> Result<
    (ExternalEffectPosture, WorthQueryCanonicalWorkEvidence),
    WorthQueryAftermathDerivationFailure,
> {
    observe_attempt(
        attempt,
        ExternalEffectPostureKind::ExternalAcknowledgement,
        "acknowledged",
    )
}

fn observe_attempt(
    attempt: &DispatchAttemptEvent<'_>,
    kind: ExternalEffectPostureKind,
    observation: &'static str,
) -> Result<
    (ExternalEffectPosture, WorthQueryCanonicalWorkEvidence),
    WorthQueryAftermathDerivationFailure,
> {
    let derived = observation_identity(
        attempt.attempt.identity(),
        attempt.emission.record().correlation(),
        observation,
    )?;
    Ok((
        ExternalEffectPosture::successor(kind, derived.identity, &attempt.attempt),
        derived.work,
    ))
}

impl CoCommittedApplicationEmission {
    pub(super) const fn provider_commit(&self) -> &ExternalEffectPosture {
        &self.provider_commit
    }

    pub(super) const fn posture(&self) -> &ExternalEffectPosture {
        &self.emission
    }

    pub(super) const fn record(&self) -> &WorthQueryDispatchOutboxRecord {
        self.committed.record()
    }
}

impl DispatchAttemptEvent<'_> {
    pub(super) const fn provider_commit(&self) -> &ExternalEffectPosture {
        self.emission.provider_commit()
    }

    pub(super) const fn emission(&self) -> &ExternalEffectPosture {
        self.emission.posture()
    }

    pub(super) const fn attempt(&self) -> &ExternalEffectPosture {
        &self.attempt
    }
}
