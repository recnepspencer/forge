use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::grouped_authoring::{
    ordinary_outcome_from_grouped_orchestration_checked, WorthQueryGroupedOrchestrationChecked,
    WorthQueryGroupedOrchestrationTranscript,
};
use crate::recovery_boundary::ordinary::worth_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryEvidenceStrength,
    WorthQueryRecoveryGroupedMemberContext, WorthQueryRecoverySourceFamily,
    WorthQueryRecoveryStopFamily,
};

pub fn worth_query_recovery_brief_from_grouped_orchestration_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryGroupedOrchestrationChecked<D, I>,
) -> Option<crate::recovery_boundary::WorthQueryRecoveryBrief> {
    let grouped_member_context = match &checked {
        WorthQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            Some(WorthQueryRecoveryGroupedMemberContext::new(
                stop.member_index(),
                stop.member_role(),
                stop.member_aspect_record().clone(),
            ))
        }
        _ => None,
    };
    worth_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_grouped_orchestration_checked(checked),
    )
    .map(|brief| {
        let mut explanation = brief
            .explanation()
            .clone()
            .with_source_family(WorthQueryRecoverySourceFamily::GroupedNeighborhood)
            .with_evidence_strength(WorthQueryRecoveryEvidenceStrength::CheckedRetained);
        if let Some(context) = grouped_member_context {
            explanation = explanation
                .with_aspect_posture(WorthQueryRecoveryAspectPosture::RetainedContractAndCoverage)
                .with_grouped_member_context(context);
        }
        brief
            .with_stop_family(WorthQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration)
            .with_explanation(explanation)
    })
}

pub fn worth_query_recovery_brief_from_grouped_orchestration_proof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    proof: WorthQueryGroupedOrchestrationTranscript<D, I>,
) -> Option<crate::recovery_boundary::WorthQueryRecoveryBrief> {
    worth_query_recovery_brief_from_grouped_orchestration_checked(proof.into_checked()).map(
        |brief| {
            let explanation = brief
                .explanation()
                .clone()
                .with_evidence_strength(WorthQueryRecoveryEvidenceStrength::ProofRetained);
            brief.with_explanation(explanation)
        },
    )
}
