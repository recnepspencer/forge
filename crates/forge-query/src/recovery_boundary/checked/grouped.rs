use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::grouped_authoring::{
    ordinary_outcome_from_grouped_orchestration_checked, ForgeQueryGroupedOrchestrationChecked,
    ForgeQueryGroupedOrchestrationTranscript,
};
use crate::recovery_boundary::ordinary::forge_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryEvidenceStrength,
    ForgeQueryRecoveryGroupedMemberContext, ForgeQueryRecoverySourceFamily,
    ForgeQueryRecoveryStopFamily,
};

pub fn forge_query_recovery_brief_from_grouped_orchestration_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryGroupedOrchestrationChecked<D, I>,
) -> Option<crate::recovery_boundary::ForgeQueryRecoveryBrief> {
    let grouped_member_context = match &checked {
        ForgeQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            Some(ForgeQueryRecoveryGroupedMemberContext::new(
                stop.member_index(),
                stop.member_role(),
                stop.member_aspect_record().clone(),
            ))
        }
        _ => None,
    };
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_grouped_orchestration_checked(checked),
    )
    .map(|brief| {
        let mut explanation = brief
            .explanation()
            .clone()
            .with_source_family(ForgeQueryRecoverySourceFamily::GroupedNeighborhood)
            .with_evidence_strength(ForgeQueryRecoveryEvidenceStrength::CheckedRetained);
        if let Some(context) = grouped_member_context {
            explanation = explanation
                .with_aspect_posture(ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage)
                .with_grouped_member_context(context);
        }
        brief
            .with_stop_family(ForgeQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration)
            .with_explanation(explanation)
    })
}

pub fn forge_query_recovery_brief_from_grouped_orchestration_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryGroupedOrchestrationTranscript<D, I>,
) -> Option<crate::recovery_boundary::ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_grouped_orchestration_checked(proof.into_checked()).map(
        |brief| {
            let explanation = brief
                .explanation()
                .clone()
                .with_evidence_strength(ForgeQueryRecoveryEvidenceStrength::ProofRetained);
            brief.with_explanation(explanation)
        },
    )
}
