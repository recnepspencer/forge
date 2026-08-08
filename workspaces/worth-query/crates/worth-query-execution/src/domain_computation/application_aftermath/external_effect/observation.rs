//! Classification of transport results against one exact dispatch attempt.

use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::super::WorthQueryAftermathDerivationFailure;
use super::causal_event::{observe_acknowledgement, observe_completion, DispatchAttemptEvent};
use super::classification::{classify_transport_fault, ExternalRailTransportFault};
use super::dispatch::WorthQueryExternalDispatchPosture;
use super::posture::ExternalEffectPosture;
use super::transport::WorthQueryExternalTransportOutcome;

pub(super) struct ClassifiedDispatchObservation {
    pub posture: WorthQueryExternalDispatchPosture,
    pub observation: Option<ExternalEffectPosture>,
    pub canonical_work: WorthQueryCanonicalWorkEvidence,
}

pub(super) fn classify_dispatch_observation(
    observed: WorthQueryExternalTransportOutcome,
    attempt: &DispatchAttemptEvent<'_>,
    clock: Option<&crate::domain_computation::runtime_time::WorthQueryRuntimeClock>,
) -> Result<ClassifiedDispatchObservation, WorthQueryAftermathDerivationFailure> {
    match observed {
        WorthQueryExternalTransportOutcome::Completed => owner_observation(
            observe_completion(attempt)?,
            WorthQueryExternalDispatchPosture::completed,
        ),
        WorthQueryExternalTransportOutcome::Acknowledged => owner_observation(
            observe_acknowledgement(attempt)?,
            WorthQueryExternalDispatchPosture::acknowledged,
        ),
        other => {
            let fault = match other {
                WorthQueryExternalTransportOutcome::DuplicateAcknowledgement => {
                    ExternalRailTransportFault::DuplicatedAcknowledgement
                }
                WorthQueryExternalTransportOutcome::Rejected => {
                    ExternalRailTransportFault::PayloadRejected
                }
                WorthQueryExternalTransportOutcome::UnsupportedProtocolVersion(unsupported) => {
                    ExternalRailTransportFault::UnsupportedProtocolVersion(unsupported)
                }
                WorthQueryExternalTransportOutcome::TimedOut => ExternalRailTransportFault::Timeout,
                WorthQueryExternalTransportOutcome::Disconnected => {
                    ExternalRailTransportFault::Disconnect
                }
                WorthQueryExternalTransportOutcome::LostResponse => {
                    ExternalRailTransportFault::LostResponse
                }
                WorthQueryExternalTransportOutcome::Completed
                | WorthQueryExternalTransportOutcome::Acknowledged => unreachable!(),
            };
            Ok(ClassifiedDispatchObservation {
                posture: WorthQueryExternalDispatchPosture::unresolved(classify_transport_fault(
                    fault, attempt, clock,
                )),
                observation: None,
                canonical_work: WorthQueryCanonicalWorkEvidence::zero(),
            })
        }
    }
}

fn owner_observation(
    observed: (ExternalEffectPosture, WorthQueryCanonicalWorkEvidence),
    wrap: fn(ExternalEffectPosture) -> WorthQueryExternalDispatchPosture,
) -> Result<ClassifiedDispatchObservation, WorthQueryAftermathDerivationFailure> {
    let (observation, canonical_work) = observed;
    Ok(ClassifiedDispatchObservation {
        posture: wrap(observation.clone()),
        observation: Some(observation),
        canonical_work,
    })
}
