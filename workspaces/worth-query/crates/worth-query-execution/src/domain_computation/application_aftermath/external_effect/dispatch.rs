//! Transport orchestration for one freshly observed committed outbox row.

use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::super::WorthQueryAftermathDerivationFailure;
use super::causal_event::{admit_co_committed_emission, begin_dispatch_attempt};
use super::classification::{ExternalEffectClassification, ExternalRailTransportFault};
use super::correlation::ExternalEffectCorrelationIdentity;
use super::observation::classify_dispatch_observation;
use super::posture::ExternalEffectPosture;
use super::transport::{WorthQueryExternalDispatchRequest, WorthQueryExternalEffectTransport};

/// Observable kind of one dispatch result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExternalDispatchPostureKind {
    Completed,
    Acknowledged,
    Unresolved,
}

/// Opaque dispatch-result projection. Callers can inspect but cannot wrap an
/// earlier causal event in a forged completion or acknowledgement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExternalDispatchPosture {
    state: DispatchPostureState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DispatchPostureState {
    Completed(ExternalEffectPosture),
    Acknowledged(ExternalEffectPosture),
    Unresolved(ExternalEffectClassification),
}

impl WorthQueryExternalDispatchPosture {
    pub(super) const fn completed(observation: ExternalEffectPosture) -> Self {
        Self {
            state: DispatchPostureState::Completed(observation),
        }
    }

    pub(super) const fn acknowledged(observation: ExternalEffectPosture) -> Self {
        Self {
            state: DispatchPostureState::Acknowledged(observation),
        }
    }

    pub(super) const fn unresolved(classification: ExternalEffectClassification) -> Self {
        Self {
            state: DispatchPostureState::Unresolved(classification),
        }
    }

    pub const fn kind(&self) -> WorthQueryExternalDispatchPostureKind {
        match self.state {
            DispatchPostureState::Completed(_) => WorthQueryExternalDispatchPostureKind::Completed,
            DispatchPostureState::Acknowledged(_) => {
                WorthQueryExternalDispatchPostureKind::Acknowledged
            }
            DispatchPostureState::Unresolved(_) => {
                WorthQueryExternalDispatchPostureKind::Unresolved
            }
        }
    }

    pub const fn observation(&self) -> Option<&ExternalEffectPosture> {
        match &self.state {
            DispatchPostureState::Completed(observation)
            | DispatchPostureState::Acknowledged(observation) => Some(observation),
            DispatchPostureState::Unresolved(_) => None,
        }
    }

    pub const fn classification(&self) -> Option<&ExternalEffectClassification> {
        match &self.state {
            DispatchPostureState::Unresolved(classification) => Some(classification),
            DispatchPostureState::Completed(_) | DispatchPostureState::Acknowledged(_) => None,
        }
    }
}

/// Exact causal events established by one dispatch invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExternalEffectCausalLadder {
    provider_commit: ExternalEffectPosture,
    emission: ExternalEffectPosture,
    attempt: ExternalEffectPosture,
    observation: Option<ExternalEffectPosture>,
}

impl WorthQueryExternalEffectCausalLadder {
    pub const fn provider_commit(&self) -> &ExternalEffectPosture {
        &self.provider_commit
    }

    pub const fn emission(&self) -> &ExternalEffectPosture {
        &self.emission
    }

    pub const fn attempt(&self) -> &ExternalEffectPosture {
        &self.attempt
    }

    /// Present only when the external owner acknowledged or completed this
    /// exact attempt. Transport uncertainty does not fabricate owner evidence.
    pub const fn observation(&self) -> Option<&ExternalEffectPosture> {
        self.observation.as_ref()
    }
}

/// One completed production dispatch invocation and its exact canonical cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExternalEffectDispatch {
    correlation: ExternalEffectCorrelationIdentity,
    posture: WorthQueryExternalDispatchPosture,
    causal_ladder: WorthQueryExternalEffectCausalLadder,
    canonical_work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryExternalEffectDispatch {
    pub const fn correlation(&self) -> &ExternalEffectCorrelationIdentity {
        &self.correlation
    }

    pub const fn posture(&self) -> &WorthQueryExternalDispatchPosture {
        &self.posture
    }

    pub const fn causal_ladder(&self) -> &WorthQueryExternalEffectCausalLadder {
        &self.causal_ladder
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.canonical_work
    }

    pub const fn is_external_completion(&self) -> bool {
        matches!(
            self.posture.kind(),
            WorthQueryExternalDispatchPostureKind::Completed
        )
    }

    pub const fn fault(&self) -> Option<ExternalRailTransportFault> {
        match self.posture.classification() {
            Some(classification) => Some(classification.fault()),
            None => None,
        }
    }
}

/// Dispatches one authoritative committed outbox observation through the host port.
pub(in crate::domain_computation) fn dispatch_external_effect(
    transport: &dyn WorthQueryExternalEffectTransport,
    admitted: crate::domain_computation::primary_graph::WorthQueryAdmittedExternalDispatchAttempt,
) -> Result<WorthQueryExternalEffectDispatch, WorthQueryAftermathDerivationFailure> {
    let runtime = admitted.query_runtime();
    let attempt_ordinal = admitted.ordinal();
    let clock = admitted.clock();
    let committed = admitted.into_committed();
    let correlation = *committed.record().correlation();
    let (emission, emission_work) = admit_co_committed_emission(runtime, committed)?;
    let (attempt, attempt_work) = begin_dispatch_attempt(&emission, attempt_ordinal)?;
    let observed = transport.dispatch(WorthQueryExternalDispatchRequest::for_record(
        emission.record(),
    ));
    let classified = classify_dispatch_observation(observed, &attempt, Some(&clock))?;
    let causal_ladder = WorthQueryExternalEffectCausalLadder {
        provider_commit: attempt.provider_commit().clone(),
        emission: attempt.emission().clone(),
        attempt: attempt.attempt().clone(),
        observation: classified.observation.clone(),
    };
    Ok(WorthQueryExternalEffectDispatch {
        correlation,
        posture: classified.posture,
        causal_ladder,
        canonical_work: emission_work
            .combine(attempt_work)
            .combine(classified.canonical_work),
    })
}
