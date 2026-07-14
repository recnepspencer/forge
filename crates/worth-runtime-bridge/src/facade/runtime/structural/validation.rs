use super::*;

pub(super) fn validate_candidate_kinds(
    contract: &AdmittedStructuralComparisonContract,
    candidates: &[StructuralMatchCandidate],
) -> Result<(), BridgeDeliveryError> {
    let comparison_mode = contract
        .validated_declaration()
        .declaration()
        .comparison_mode();

    for candidate in candidates {
        match (comparison_mode, candidate.candidate_kind()) {
            (
                StructuralComparisonMode::AdvisoryRemap,
                StructuralMatchCandidateKind::ExactAdvisoryMatch
                | StructuralMatchCandidateKind::AdvisoryReuseCandidate
                | StructuralMatchCandidateKind::IdentityAuthorityConflict
                | StructuralMatchCandidateKind::LineageStructuralDivergence,
            )
            | (
                StructuralComparisonMode::BranchComparison,
                StructuralMatchCandidateKind::BranchDiff,
            ) => {}
            (StructuralComparisonMode::AdvisoryRemap, StructuralMatchCandidateKind::BranchDiff) => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` is advisory remap but candidate `{}` was classified as a branch diff.",
                        contract.contract_identity().as_str(),
                        candidate.candidate_identity().as_str()
                    ),
                ))
            }
            (StructuralComparisonMode::BranchComparison, _) => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` is branch comparison but candidate `{}` was not classified as a branch diff.",
                        contract.contract_identity().as_str(),
                        candidate.candidate_identity().as_str()
                    ),
                ))
            }
        }
    }

    Ok(())
}
