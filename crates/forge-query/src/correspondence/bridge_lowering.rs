use forge_runtime_bridge::facade::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageTopology, ReducedStructuralMatchSet,
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralMatchOutcomeClass,
};

use super::contracts::StructuralCandidateBudget;
use super::cost::{StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract};
use super::error::CorrespondenceEvaluationError;
use super::request::{LineageEvidenceInput, StructuralEvidenceInput};

pub(crate) fn lower_lineage_authority(
    authority: &BridgeHistoricalLineageAuthority,
) -> LineageEvidenceInput {
    match authority.topology() {
        BridgeHistoricalLineageTopology::SingleSuccessor => {
            let canonical_subject = authority
                .canonical_resolved_lineage_identities()
                .first()
                .map(|identity| {
                    identity
                        .bridge_admission_evidence()
                        .terminal_projection_for_reporting()
                        .to_string()
                })
                .unwrap_or_else(|| authority.lineage_digest().to_string());
            let authoritative_counterpart = authority
                .canonical_resolved_record_identities()
                .first()
                .map(|identity| {
                    identity
                        .bridge_admission_evidence()
                        .terminal_projection_for_reporting()
                        .to_string()
                })
                .unwrap_or_else(|| authority.lineage_digest().to_string());

            LineageEvidenceInput::AuthoritativeContinuity {
                canonical_subject,
                authoritative_counterpart,
            }
        }
        BridgeHistoricalLineageTopology::NoAuthoritativeSuccessor => {
            LineageEvidenceInput::UnsupportedTopology {
                topology: "no_authoritative_successor",
            }
        }
        BridgeHistoricalLineageTopology::UnsupportedWithoutSuccessor => {
            LineageEvidenceInput::UnsupportedTopology {
                topology: "unsupported_without_successor",
            }
        }
        BridgeHistoricalLineageTopology::MergeLikeSuccessor => {
            LineageEvidenceInput::UnsupportedTopology {
                topology: "merge_like_successor",
            }
        }
        BridgeHistoricalLineageTopology::SplitSuccessors => {
            LineageEvidenceInput::UnsupportedTopology {
                topology: "split_successors",
            }
        }
        BridgeHistoricalLineageTopology::AmbiguousSuccessor => {
            LineageEvidenceInput::UnsupportedTopology {
                topology: "ambiguous_successor",
            }
        }
    }
}

pub(crate) fn lower_reduced_structural_match_set(
    reduced: &ReducedStructuralMatchSet,
    discovery_plan: &StructuralCandidateDiscoveryPlan,
    budget: &StructuralCandidateBudget,
    ordering_contract: StructuralCandidateOrderingContract,
) -> Result<StructuralEvidenceInput, CorrespondenceEvaluationError> {
    if matches!(
        discovery_plan,
        StructuralCandidateDiscoveryPlan::RequiresBroadScanDenied
    ) {
        return Err(CorrespondenceEvaluationError::BroadStructuralScanRequired);
    }

    let declaration = reduced
        .planned_packet_set()
        .validated_declaration()
        .declaration();

    if matches!(
        declaration.candidate_scope(),
        StructuralCandidateSearchScope::ExplicitWidenedDebtScan
    ) {
        return Err(CorrespondenceEvaluationError::BroadStructuralScanRequired);
    }

    if !matches!(
        reduced.planned_packet_set().comparison_mode(),
        StructuralComparisonMode::AdvisoryRemap
    ) {
        return Err(CorrespondenceEvaluationError::UnsupportedStructuralFamily {
            family: "branch_comparison",
        });
    }

    let candidates = reduced
        .retained_candidates()
        .iter()
        .map(|candidate| candidate.as_ref().to_string())
        .collect::<Vec<_>>();

    if candidates.len() > budget.max_candidates() {
        return Err(CorrespondenceEvaluationError::StructuralBreadthExceeded {
            budget: budget.max_candidates(),
            actual: candidates.len(),
        });
    }

    match reduced.outcome_class() {
        StructuralMatchOutcomeClass::ExactAdvisoryMatch
        | StructuralMatchOutcomeClass::AdvisoryReuseCandidate
        | StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch
        | StructuralMatchOutcomeClass::RejectedNoStructuralMatch => {
            Ok(StructuralEvidenceInput::CandidateSet {
                candidates,
                ordering_contract,
            })
        }
        StructuralMatchOutcomeClass::RejectedIdentityAuthorityConflict => {
            Ok(StructuralEvidenceInput::UnsupportedFamily {
                family: "identity_authority_conflict",
            })
        }
        StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence => {
            let structural_counterpart = reduced
                .retained_candidates()
                .first()
                .map(|candidate| candidate.as_ref())
                .unwrap_or("lineage_structural_divergence");
            Ok(StructuralEvidenceInput::LineageConflict {
                structural_counterpart: structural_counterpart.to_string(),
            })
        }
        StructuralMatchOutcomeClass::BranchComparisonArtifact => {
            Err(CorrespondenceEvaluationError::UnsupportedStructuralFamily {
                family: "branch_comparison_artifact",
            })
        }
    }
}
