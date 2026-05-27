use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationProof,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::brief::ForgeQueryRecoveryBrief;
use super::ordinary::forge_query_recovery_brief_from_ordinary_outcome;

pub fn forge_query_recovery_brief_from_declaration_entry_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationEntryOrchestrationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    recovery_from_declaration_entry_outcome(&checked)
}

pub fn forge_query_recovery_brief_from_declaration_entry_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: &ForgeQueryDeclarationEntryOrchestrationProof<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    recovery_from_declaration_entry_outcome(proof.outcome())
}

fn recovery_from_declaration_entry_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    match outcome {
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(_) => None,
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Deferred,
                crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::RetryLater,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Denied,
                crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Stale(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Stale,
                crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::RefreshBasis,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::RebindRequired(
            value,
        ) => recovery_from_declaration_entry_terminal(
            value.reason(),
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::RebindRequired,
            crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::RebindContext,
            value.stop_stage(),
            value.retained_digest().map(str::to_owned),
            None,
        ),
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(value) => {
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Failed,
                crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::EscalateFailure,
                value.stop_stage(),
                value.retained_digest().map(str::to_owned),
                None,
            )
        }
        crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(value) => {
            let next_step = match value.refusal_class() {
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation => {
                    crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::CheckSupport
                }
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired => {
                    crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::NarrowInput
                }
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired => {
                    crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::InspectProofLane
                }
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired
                | ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
                | ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation => crate::ordinary_outcome::ForgeQueryOrdinaryNextStep::UseExplicitHandoff,
            };
            recovery_from_declaration_entry_terminal(
                value.reason(),
                crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Refused,
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
    kind: crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind,
    next_step: crate::ordinary_outcome::ForgeQueryOrdinaryNextStep,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    retained_digest: Option<String>,
    refusal_class: Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass>,
) -> Option<ForgeQueryRecoveryBrief> {
    let posture = crate::ordinary_outcome::ForgeQueryOrdinaryPosture::new(
        reason,
        kind,
        next_step,
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::orchestration(
            stop_stage,
            retained_digest,
            refusal_class,
        ),
    );
    let outcome = match kind {
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Deferred => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::Deferred(posture)
        }
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Denied => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::Denied(posture)
        }
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Stale => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::Stale(posture)
        }
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::RebindRequired => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::RebindRequired(posture)
        }
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Failed => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::Failed(posture)
        }
        crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Refused => {
            crate::ordinary_outcome::ForgeQueryOrdinaryOutcome::<()>::Refused(posture)
        }
        _ => unreachable!("declaration entry recovery only builds terminal ordinary postures"),
    };
    forge_query_recovery_brief_from_ordinary_outcome(&outcome)
}
