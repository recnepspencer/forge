use crate::application::{
    WorthQueryDeclarationEntryOrchestrationChecked, WorthQueryDeclarationEntryOrchestrationProof,
    WorthQueryDeclarationEntryOrchestrationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};

use super::brief::WorthQueryRecoveryBrief;
use super::ordinary::worth_query_recovery_brief_from_ordinary_outcome;

pub fn worth_query_recovery_brief_from_declaration_entry_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationEntryOrchestrationChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    recovery_from_declaration_entry_outcome(&checked)
}

pub fn worth_query_recovery_brief_from_declaration_entry_proof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    proof: &WorthQueryDeclarationEntryOrchestrationProof<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    recovery_from_declaration_entry_outcome(proof.outcome())
}

fn recovery_from_declaration_entry_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &crate::application::WorthQueryDeclarationEntryOrchestrationOutcome<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    match outcome {
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Enveloped(_) => None,
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Deferred,
                crate::ordinary_outcome::WorthQueryOrdinaryNextStep::RetryLater,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Denied(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Denied,
                crate::ordinary_outcome::WorthQueryOrdinaryNextStep::InspectCheckedLane,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Stale(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Stale,
                crate::ordinary_outcome::WorthQueryOrdinaryNextStep::RefreshBasis,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::RebindRequired(
            value,
        ) => recovery_from_declaration_entry_terminal(
            value.reason(),
            crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::RebindRequired,
            crate::ordinary_outcome::WorthQueryOrdinaryNextStep::RebindContext,
            value.stop_stage(),
            value.retained_digest().map(str::to_owned),
            None,
        ),
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Failed(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Failed,
                crate::ordinary_outcome::WorthQueryOrdinaryNextStep::EscalateFailure,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::WorthQueryDeclarationEntryOrchestrationOutcome::Refused(value) => {
            let next_step = match value.refusal_class() {
                WorthQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation => {
                    crate::ordinary_outcome::WorthQueryOrdinaryNextStep::CheckSupport
                }
                WorthQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired => {
                    crate::ordinary_outcome::WorthQueryOrdinaryNextStep::NarrowInput
                }
                WorthQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired => {
                    crate::ordinary_outcome::WorthQueryOrdinaryNextStep::InspectProofLane
                }
                WorthQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired
                | WorthQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
                | WorthQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation => crate::ordinary_outcome::WorthQueryOrdinaryNextStep::UseExplicitHandoff,
            };
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Refused,
                next_step,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                Some(value.refusal_class()),
            )
        }
    }
}

fn recovery_from_declaration_entry_terminal(
    reason: &'static str,
    kind: crate::ordinary_outcome::WorthQueryOrdinaryPostureKind,
    next_step: crate::ordinary_outcome::WorthQueryOrdinaryNextStep,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    retained_digest: Option<String>,
    refusal_class: Option<WorthQueryDeclarationEntryOrchestrationRefusalClass>,
) -> Option<WorthQueryRecoveryBrief> {
    let posture = crate::ordinary_outcome::WorthQueryOrdinaryPosture::new(
        reason,
        kind,
        next_step,
        crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::orchestration(
            stop_stage,
            retained_digest,
            refusal_class,
        ),
    );
    let outcome = match kind {
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Deferred => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::Deferred(posture)
        }
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Denied => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::Denied(posture)
        }
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Stale => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::Stale(posture)
        }
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::RebindRequired => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::RebindRequired(posture)
        }
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Failed => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::Failed(posture)
        }
        crate::ordinary_outcome::WorthQueryOrdinaryPostureKind::Refused => {
            crate::ordinary_outcome::WorthQueryOrdinaryOutcome::<()>::Refused(posture)
        }
        _ => unreachable!("declaration entry recovery only builds terminal ordinary postures"),
    };
    worth_query_recovery_brief_from_ordinary_outcome(&outcome)
}
