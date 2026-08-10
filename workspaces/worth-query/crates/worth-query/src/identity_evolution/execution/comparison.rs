use crate::identity::ResultDigest;

use super::super::{
    admission::{AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError},
    contracts::IdentityEvolutionComplexityStatus,
    families::{
        IdentityEvolutionAmbiguityReason, IdentityEvolutionDenialReason,
        IdentityEvolutionIdentityBreakReason,
    },
    metadata::{
        BranchLocalityClass, IdentityEvolutionComplexityReport, IdentityEvolutionMetadata,
        PromotionOrMergeAuthorityState,
    },
    performance::IdentityEvolutionPredictionDriftOutcome,
    request::{
        CorrespondenceIdentityComparison, IdentityComparisonIntent,
        IdentityEvolutionComparisonBasisFamily,
    },
    results::{
        AdvisoryIdentityCandidateSet, IdentityEvolutionAmbiguityBundle,
        IdentityEvolutionDeniedBundle, IdentityEvolutionIdentityBreakBundle,
        IdentityEvolutionResultBundle, SingularIdentityContinuityResult,
    },
    synthetic::IdentityEvolutionSyntheticScenario,
};
use super::classification::{
    branch_locality_class_for_comparison, comparison_outcome_family,
    execution_family_for_comparison,
};
use super::{IdentityEvolutionExecutionArtifact, IdentityEvolutionExecutionCounters};

pub(super) fn execute(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_digest: &crate::identity::BasisDigest,
    right_basis_digest: &crate::identity::BasisDigest,
    comparison: &CorrespondenceIdentityComparison,
) -> Result<IdentityEvolutionExecutionArtifact, IdentityEvolutionAdmissionError> {
    let execution_family = execution_family_for_comparison(basis_family);
    let lineage_digest = ResultDigest::from_parts(&[
        format!("comparison_basis_family:{}", basis_family.as_str()),
        format!("left_basis_digest:{}", left_basis_digest.as_str()),
        format!("right_basis_digest:{}", right_basis_digest.as_str()),
        format!("comparison_intent:{}", comparison.intent().as_str()),
    ]);
    let scenario = admitted_query.synthetic_scenario();
    let branch_locality_class = branch_locality_class_for_comparison(scenario);
    let authority_state = if branch_locality_class == BranchLocalityClass::CrossBranchDenied {
        PromotionOrMergeAuthorityState::RequiredButUnavailable
    } else {
        PromotionOrMergeAuthorityState::AuthorityWitnessed
    };
    let comparison_outcome_family =
        comparison_outcome_family(comparison, branch_locality_class, scenario);
    let complexity_report = IdentityEvolutionComplexityReport::from_contract(
        admitted_query.complexity_contract().clone(),
    );
    let metadata = IdentityEvolutionMetadata::from_authority_parts(
        admitted_query.query_context().query_authority().clone(),
        admitted_query.query_context().basis_proof().clone(),
        crate::identity::LineageDigest::from_parts(&[format!(
            "comparison_lineage_digest:{}",
            lineage_digest.as_str()
        )]),
        comparison_outcome_family,
        left_basis_digest.clone(),
        left_basis_digest.clone(),
        right_basis_digest.clone(),
        branch_locality_class,
        authority_state,
        complexity_report,
    );

    let mut counters = IdentityEvolutionExecutionCounters {
        declared_correspondence_complexity_contract_count: 1,
        predicted_lineage_width: 1,
        realized_lineage_width: 1,
        identity_evolution_metadata_attachment_count: 1,
        identity_evolution_replay_parity_count: 1,
        identity_evolution_basis_rediscovery_count: 0,
        executor_rediscovery_count: 0,
        complexity_status_debt_count: usize::from(
            admitted_query
                .complexity_contract()
                .verified_or_debt_status()
                != IdentityEvolutionComplexityStatus::Verified,
        ),
        ..IdentityEvolutionExecutionCounters::default()
    };

    let (result_bundle, prediction_drift_outcome) = if comparison.intent()
        == IdentityComparisonIntent::AmbiguousCandidateSet
        || scenario == IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence
    {
        counters.correspondence_candidate_count = 2;
        counters.ambiguous_correspondence_count = 1;
        (
            IdentityEvolutionResultBundle::ambiguity(IdentityEvolutionAmbiguityBundle::new(
                metadata,
                IdentityEvolutionAmbiguityReason::AmbiguousCorrespondenceCandidates,
            )),
            IdentityEvolutionPredictionDriftOutcome::WidthDriftDetected,
        )
    } else if comparison.intent() == IdentityComparisonIntent::AdvisoryCandidateSet {
        counters.correspondence_candidate_count = 2;
        (
            IdentityEvolutionResultBundle::advisory_identity_candidate_set(
                AdvisoryIdentityCandidateSet::new(
                    metadata,
                    vec![
                        format!("candidate:{}", comparison.left_identity()),
                        format!("candidate:{}", comparison.right_identity()),
                    ],
                ),
            ),
            IdentityEvolutionPredictionDriftOutcome::WithinBudget,
        )
    } else if comparison.intent() == IdentityComparisonIntent::ExplicitContinuityBreak
        || scenario == IdentityEvolutionSyntheticScenario::IdentityBreak
    {
        counters.identity_break_count = 1;
        counters.realized_lineage_width = 0;
        (
            IdentityEvolutionResultBundle::identity_break(
                IdentityEvolutionIdentityBreakBundle::new(
                    metadata,
                    IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak,
                ),
            ),
            IdentityEvolutionPredictionDriftOutcome::WithinBudget,
        )
    } else if scenario == IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied {
        counters.complexity_contract_violation_denial_count = 1;
        counters.realized_lineage_width = 0;
        (
            IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                metadata,
                IdentityEvolutionDenialReason::ComplexityContractViolationDenied,
            )),
            IdentityEvolutionPredictionDriftOutcome::WithinBudget,
        )
    } else if comparison.intent() == IdentityComparisonIntent::AuthoritativeContinuityRequired
        && (branch_locality_class != BranchLocalityClass::CrossBranchAuthoritative
            || scenario == IdentityEvolutionSyntheticScenario::AdvisoryAsAuthoritativeDenied)
    {
        counters.advisory_as_authoritative_denial_count = 1;
        counters.branch_crossing_denial_count =
            usize::from(branch_locality_class == BranchLocalityClass::CrossBranchDenied);
        counters.realized_lineage_width = 0;
        (
            IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                metadata,
                IdentityEvolutionDenialReason::AuthoritativeContinuityRequiresAuthorityEvidence,
            )),
            IdentityEvolutionPredictionDriftOutcome::WithinBudget,
        )
    } else {
        counters.correspondence_candidate_count = 1;
        counters.promotion_or_merge_authority_proof_check_count = 1;
        (
            IdentityEvolutionResultBundle::singular_identity_continuity(
                SingularIdentityContinuityResult::new(
                    metadata,
                    format!(
                        "comparison-authoritative:{}:{}",
                        comparison.left_identity(),
                        comparison.right_identity()
                    ),
                ),
            ),
            IdentityEvolutionPredictionDriftOutcome::WithinBudget,
        )
    };

    let result_digest = super::classification::execution_result_digest(
        admitted_query,
        execution_family.as_str(),
        result_bundle.metadata().metadata_digest().as_str(),
        result_bundle.outcome_family().as_str(),
    );
    Ok(IdentityEvolutionExecutionArtifact::new(
        admitted_query
            .query_context()
            .query_digest()
            .as_str()
            .to_string(),
        admitted_query
            .query_context()
            .basis_digest()
            .as_str()
            .to_string(),
        lineage_digest.as_str().to_string(),
        result_digest.as_str().to_string(),
        execution_family,
        prediction_drift_outcome,
        result_bundle,
        counters,
    ))
}
