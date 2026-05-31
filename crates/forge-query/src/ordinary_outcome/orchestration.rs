use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationTerminalError, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};

pub(crate) fn ordinary_outcome_from_orchestration_terminal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    terminal: ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>,
) -> ForgeQueryOrdinaryOutcome<crate::application::ForgeQueryDeclarationEnvelope<D, I>> {
    match terminal {
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(value) => {
            ForgeQueryOrdinaryOutcome::Deferred(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Deferred,
                ForgeQueryOrdinaryNextStep::RetryLater,
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    None,
                ),
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    None,
                ),
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(value) => {
            ForgeQueryOrdinaryOutcome::Stale(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    None,
                ),
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(value) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    None,
                ),
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(value) => {
            ForgeQueryOrdinaryOutcome::Failed(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::EscalateFailure,
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    None,
                ),
            ))
        }
        ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(value) => {
            ForgeQueryOrdinaryOutcome::Refused(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Refused,
                refusal_next_step(value.refusal_class()),
                ForgeQueryOrdinaryCheckedTopology::orchestration(
                    value.stop_stage(),
                    value.retained_digest().map(str::to_owned),
                    Some(value.refusal_class()),
                ),
            ))
        }
    }
}

fn refusal_next_step(
    class: ForgeQueryDeclarationEntryOrchestrationRefusalClass,
) -> ForgeQueryOrdinaryNextStep {
    match class {
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation => {
            ForgeQueryOrdinaryNextStep::CheckSupport
        }
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired => {
            ForgeQueryOrdinaryNextStep::NarrowInput
        }
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired => {
            ForgeQueryOrdinaryNextStep::InspectProofLane
        }
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired => {
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff
        }
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault => {
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff
        }
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation => {
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff
        }
    }
}
