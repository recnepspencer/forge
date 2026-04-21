use crate::identity::ResultDigest;

use super::{
    admission::{AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError},
    families::{
        IdentityEvolutionAmbiguityReason, IdentityEvolutionDenialReason,
        IdentityEvolutionIdentityBreakReason, IdentityEvolutionOutcomeFamily,
        LineageTraversalFamily,
    },
    metadata::{
        BranchLocalityClass, IdentityEvolutionComplexityReport, IdentityEvolutionMetadata,
        PromotionOrMergeAuthorityState,
    },
    performance::IdentityEvolutionPredictionDriftOutcome,
    request::{IdentityComparisonIntent, IdentityEvolutionComparisonBasisFamily},
    results::{
        AdvisoryIdentityCandidateSet, IdentityEvolutionAmbiguityBundle,
        IdentityEvolutionDeniedBundle, IdentityEvolutionIdentityBreakBundle,
        IdentityEvolutionResultBundle, PluralIdentitySuccessorSet,
        SingularIdentityContinuityResult,
    },
    synthetic::IdentityEvolutionSyntheticScenario,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionExecutionFamily {
    DirectPredecessor,
    DirectSuccessor,
    DirectReplacement,
    DirectSplitSuccessors,
    DirectMergeSuccessor,
    BranchLocalDirectEvolution,
    BranchToBranchComparison,
    CurrentToHistoricalComparison,
    HistoricalToHistoricalComparison,
    PreviewToAuthoritativeComparison,
}

impl IdentityEvolutionExecutionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectPredecessor => "direct_predecessor",
            Self::DirectSuccessor => "direct_successor",
            Self::DirectReplacement => "direct_replacement",
            Self::DirectSplitSuccessors => "direct_split_successors",
            Self::DirectMergeSuccessor => "direct_merge_successor",
            Self::BranchLocalDirectEvolution => "branch_local_direct_evolution",
            Self::BranchToBranchComparison => "branch_to_branch_comparison",
            Self::CurrentToHistoricalComparison => "current_to_historical_comparison",
            Self::HistoricalToHistoricalComparison => "historical_to_historical_comparison",
            Self::PreviewToAuthoritativeComparison => "preview_to_authoritative_comparison",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IdentityEvolutionExecutionCounters {
    declared_lineage_complexity_contract_count: usize,
    declared_correspondence_complexity_contract_count: usize,
    lineage_anchor_lookup_count: usize,
    lineage_step_count: usize,
    predicted_lineage_width: usize,
    realized_lineage_width: usize,
    lineage_width_drift_count: usize,
    split_successor_fanout_width: usize,
    branch_local_boundary_check_count: usize,
    branch_local_divergence_count: usize,
    promotion_or_merge_authority_proof_check_count: usize,
    identity_break_count: usize,
    unsupported_lineage_denial_count: usize,
    broad_lineage_scan_denial_count: usize,
    correspondence_candidate_count: usize,
    ambiguous_correspondence_count: usize,
    advisory_as_authoritative_denial_count: usize,
    branch_crossing_denial_count: usize,
    lineage_to_correspondence_fallback_count: usize,
    identity_evolution_metadata_attachment_count: usize,
    identity_evolution_replay_parity_count: usize,
    executor_rediscovery_count: usize,
    identity_evolution_basis_rediscovery_count: usize,
    complexity_contract_violation_denial_count: usize,
    complexity_status_debt_count: usize,
}

impl IdentityEvolutionExecutionCounters {
    pub fn declared_lineage_complexity_contract_count(&self) -> usize {
        self.declared_lineage_complexity_contract_count
    }
    pub fn declared_correspondence_complexity_contract_count(&self) -> usize {
        self.declared_correspondence_complexity_contract_count
    }
    pub fn lineage_anchor_lookup_count(&self) -> usize {
        self.lineage_anchor_lookup_count
    }
    pub fn lineage_step_count(&self) -> usize {
        self.lineage_step_count
    }
    pub fn predicted_lineage_width(&self) -> usize {
        self.predicted_lineage_width
    }
    pub fn realized_lineage_width(&self) -> usize {
        self.realized_lineage_width
    }
    pub fn lineage_width_drift_count(&self) -> usize {
        self.lineage_width_drift_count
    }
    pub fn split_successor_fanout_width(&self) -> usize {
        self.split_successor_fanout_width
    }
    pub fn branch_local_boundary_check_count(&self) -> usize {
        self.branch_local_boundary_check_count
    }
    pub fn branch_local_divergence_count(&self) -> usize {
        self.branch_local_divergence_count
    }
    pub fn promotion_or_merge_authority_proof_check_count(&self) -> usize {
        self.promotion_or_merge_authority_proof_check_count
    }
    pub fn identity_break_count(&self) -> usize {
        self.identity_break_count
    }
    pub fn unsupported_lineage_denial_count(&self) -> usize {
        self.unsupported_lineage_denial_count
    }
    pub fn broad_lineage_scan_denial_count(&self) -> usize {
        self.broad_lineage_scan_denial_count
    }
    pub fn correspondence_candidate_count(&self) -> usize {
        self.correspondence_candidate_count
    }
    pub fn ambiguous_correspondence_count(&self) -> usize {
        self.ambiguous_correspondence_count
    }
    pub fn advisory_as_authoritative_denial_count(&self) -> usize {
        self.advisory_as_authoritative_denial_count
    }
    pub fn branch_crossing_denial_count(&self) -> usize {
        self.branch_crossing_denial_count
    }
    pub fn lineage_to_correspondence_fallback_count(&self) -> usize {
        self.lineage_to_correspondence_fallback_count
    }
    pub fn identity_evolution_metadata_attachment_count(&self) -> usize {
        self.identity_evolution_metadata_attachment_count
    }
    pub fn identity_evolution_replay_parity_count(&self) -> usize {
        self.identity_evolution_replay_parity_count
    }
    pub fn executor_rediscovery_count(&self) -> usize {
        self.executor_rediscovery_count
    }
    pub fn identity_evolution_basis_rediscovery_count(&self) -> usize {
        self.identity_evolution_basis_rediscovery_count
    }
    pub fn complexity_contract_violation_denial_count(&self) -> usize {
        self.complexity_contract_violation_denial_count
    }
    pub fn complexity_status_debt_count(&self) -> usize {
        self.complexity_status_debt_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionExecutionArtifact {
    query_digest: String,
    basis_digest: String,
    lineage_digest: String,
    result_digest: String,
    family: IdentityEvolutionExecutionFamily,
    prediction_drift_outcome: IdentityEvolutionPredictionDriftOutcome,
    result_bundle: IdentityEvolutionResultBundle,
    counters: IdentityEvolutionExecutionCounters,
}

impl IdentityEvolutionExecutionArtifact {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub fn lineage_digest(&self) -> &str {
        &self.lineage_digest
    }
    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
    pub fn family(&self) -> &IdentityEvolutionExecutionFamily {
        &self.family
    }
    pub fn prediction_drift_outcome(&self) -> IdentityEvolutionPredictionDriftOutcome {
        self.prediction_drift_outcome
    }
    pub fn result_bundle(&self) -> &IdentityEvolutionResultBundle {
        &self.result_bundle
    }
    pub fn counters(&self) -> &IdentityEvolutionExecutionCounters {
        &self.counters
    }

    pub(crate) fn new(
        query_digest: String,
        basis_digest: String,
        lineage_digest: String,
        result_digest: String,
        family: IdentityEvolutionExecutionFamily,
        prediction_drift_outcome: IdentityEvolutionPredictionDriftOutcome,
        result_bundle: IdentityEvolutionResultBundle,
        counters: IdentityEvolutionExecutionCounters,
    ) -> Self {
        Self {
            query_digest,
            basis_digest,
            lineage_digest,
            result_digest,
            family,
            prediction_drift_outcome,
            result_bundle,
            counters,
        }
    }
}

pub fn execute_admitted_identity_evolution_query(
    admitted_query: &AdmittedIdentityEvolutionQuery,
) -> Result<IdentityEvolutionExecutionArtifact, IdentityEvolutionAdmissionError> {
    if let Some(descriptor) = admitted_query.traversal_descriptor() {
        return execute_lineage(admitted_query, descriptor);
    }
    if let Some((basis_family, left_basis_digest, right_basis_digest, comparison)) =
        admitted_query.correspondence_identity_comparison()
    {
        return execute_comparison(
            admitted_query,
            basis_family,
            left_basis_digest,
            right_basis_digest,
            comparison,
        );
    }
    unreachable!("admitted identity evolution query must have one closed shape")
}

fn execute_lineage(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    descriptor: &super::request::LineageTraversalDescriptor,
) -> Result<IdentityEvolutionExecutionArtifact, IdentityEvolutionAdmissionError> {
    let family = execution_family_for_lineage(descriptor.family());
    let lineage_digest = descriptor.family().digest();
    let complexity_report = IdentityEvolutionComplexityReport::from_contract(
        admitted_query.complexity_contract().clone(),
    );
    let metadata = IdentityEvolutionMetadata::from_parts(
        admitted_query.query_context().query_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
        lineage_digest.clone(),
        outcome_family_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        admitted_query.query_context().basis_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
        branch_locality_class_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        authority_state_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        complexity_report,
    );

    let mut counters = IdentityEvolutionExecutionCounters {
        declared_lineage_complexity_contract_count: 1,
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
                != super::contracts::IdentityEvolutionComplexityStatus::Verified,
        ),
        ..IdentityEvolutionExecutionCounters::default()
    };

    let scenario = admitted_query.synthetic_scenario();
    let (result_bundle, prediction_drift_outcome) = match (descriptor.family(), scenario) {
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::BroadLineageScanDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.broad_lineage_scan_denial_count = 1;
            counters.realized_lineage_width = 0;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::BroadLineageScanRequired,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.realized_lineage_width = 0;
            counters.complexity_contract_violation_denial_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::ComplexityContractViolationDenied,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::LineageToCorrespondenceFallbackDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.realized_lineage_width = 0;
            counters.unsupported_lineage_denial_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::LineageToCorrespondenceFallbackForbidden,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectPredecessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("predecessor:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectSuccessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("successor:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectReplacement, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("replacement:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectSplitSuccessors, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 2;
            counters.predicted_lineage_width = 1;
            counters.realized_lineage_width = 2;
            counters.lineage_width_drift_count = 1;
            counters.split_successor_fanout_width = 2;
            (
                IdentityEvolutionResultBundle::plural_identity_successor_set(
                    PluralIdentitySuccessorSet::new(
                        metadata,
                        vec![
                            format!("split-a:{}", descriptor.anchor_identity()),
                            format!("split-b:{}", descriptor.anchor_identity()),
                        ],
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WidthDriftDetected,
            )
        }
        (LineageTraversalFamily::DirectMergeSuccessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            counters.promotion_or_merge_authority_proof_check_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("merge-successor:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::BranchLocalDirectEvolution,
            IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::BranchCrossingLineageWithoutAdmittedBasisPairing,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::BranchLocalDirectEvolution,
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
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
        }
        (LineageTraversalFamily::BranchLocalDirectEvolution, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
            counters.lineage_step_count = 1;
            counters.branch_local_divergence_count =
                usize::from(scenario == IdentityEvolutionSyntheticScenario::BranchLocalDivergence);
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("branch-local:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
    };

    if matches!(
        result_bundle.outcome_family(),
        IdentityEvolutionOutcomeFamily::Denied
    ) && descriptor.family() == LineageTraversalFamily::BranchLocalDirectEvolution
        && scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied
    {
        counters.unsupported_lineage_denial_count = 1;
        counters.branch_crossing_denial_count = 1;
        counters.realized_lineage_width = 0;
    }

    let result_digest = execution_result_digest(
        admitted_query,
        family.as_str(),
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
        family,
        prediction_drift_outcome,
        result_bundle,
        counters,
    ))
}

fn execute_comparison(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_digest: &crate::identity::BasisDigest,
    right_basis_digest: &crate::identity::BasisDigest,
    comparison: &super::request::CorrespondenceIdentityComparison,
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
    let metadata = IdentityEvolutionMetadata::from_parts(
        admitted_query.query_context().query_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
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
                != super::contracts::IdentityEvolutionComplexityStatus::Verified,
        ),
        ..IdentityEvolutionExecutionCounters::default()
    };

    let (result_bundle, prediction_drift_outcome) = if scenario
        == IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence
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
    } else if scenario == IdentityEvolutionSyntheticScenario::IdentityBreak {
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

    let result_digest = execution_result_digest(
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

fn execution_family_for_lineage(
    family: LineageTraversalFamily,
) -> IdentityEvolutionExecutionFamily {
    match family {
        LineageTraversalFamily::DirectPredecessor => {
            IdentityEvolutionExecutionFamily::DirectPredecessor
        }
        LineageTraversalFamily::DirectSuccessor => {
            IdentityEvolutionExecutionFamily::DirectSuccessor
        }
        LineageTraversalFamily::DirectReplacement => {
            IdentityEvolutionExecutionFamily::DirectReplacement
        }
        LineageTraversalFamily::DirectSplitSuccessors => {
            IdentityEvolutionExecutionFamily::DirectSplitSuccessors
        }
        LineageTraversalFamily::DirectMergeSuccessor => {
            IdentityEvolutionExecutionFamily::DirectMergeSuccessor
        }
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            IdentityEvolutionExecutionFamily::BranchLocalDirectEvolution
        }
    }
}

fn execution_family_for_comparison(
    family: IdentityEvolutionComparisonBasisFamily,
) -> IdentityEvolutionExecutionFamily {
    match family {
        IdentityEvolutionComparisonBasisFamily::BranchToBranch => {
            IdentityEvolutionExecutionFamily::BranchToBranchComparison
        }
        IdentityEvolutionComparisonBasisFamily::CurrentToHistorical => {
            IdentityEvolutionExecutionFamily::CurrentToHistoricalComparison
        }
        IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical => {
            IdentityEvolutionExecutionFamily::HistoricalToHistoricalComparison
        }
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative => {
            IdentityEvolutionExecutionFamily::PreviewToAuthoritativeComparison
        }
    }
}

fn branch_locality_class_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> BranchLocalityClass {
    match family {
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            if scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied {
                BranchLocalityClass::CrossBranchDenied
            } else {
                BranchLocalityClass::BranchLocalOnly
            }
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectSplitSuccessors
        | LineageTraversalFamily::DirectMergeSuccessor => {
            BranchLocalityClass::CrossBranchAuthoritative
        }
    }
}

fn authority_state_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> PromotionOrMergeAuthorityState {
    match family {
        LineageTraversalFamily::DirectMergeSuccessor => {
            PromotionOrMergeAuthorityState::AuthorityWitnessed
        }
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            if scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied {
                PromotionOrMergeAuthorityState::RequiredButUnavailable
            } else {
                PromotionOrMergeAuthorityState::NotRequired
            }
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectSplitSuccessors => {
            PromotionOrMergeAuthorityState::NotRequired
        }
    }
}

fn outcome_family_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionOutcomeFamily {
    match family {
        LineageTraversalFamily::DirectPredecessor
            if matches!(
                scenario,
                IdentityEvolutionSyntheticScenario::BroadLineageScanDenied
                    | IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied
                    | IdentityEvolutionSyntheticScenario::LineageToCorrespondenceFallbackDenied
            ) =>
        {
            IdentityEvolutionOutcomeFamily::Denied
        }
        LineageTraversalFamily::DirectSplitSuccessors => {
            IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet
        }
        LineageTraversalFamily::BranchLocalDirectEvolution
            if matches!(
                scenario,
                IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied
            ) =>
        {
            IdentityEvolutionOutcomeFamily::Denied
        }
        LineageTraversalFamily::BranchLocalDirectEvolution
            if scenario == IdentityEvolutionSyntheticScenario::IdentityBreak =>
        {
            IdentityEvolutionOutcomeFamily::IdentityBreak
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectMergeSuccessor
        | LineageTraversalFamily::BranchLocalDirectEvolution => {
            IdentityEvolutionOutcomeFamily::SingularIdentityContinuity
        }
    }
}

fn comparison_outcome_family(
    comparison: &super::request::CorrespondenceIdentityComparison,
    locality: BranchLocalityClass,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionOutcomeFamily {
    if scenario == IdentityEvolutionSyntheticScenario::IdentityBreak {
        IdentityEvolutionOutcomeFamily::IdentityBreak
    } else if scenario == IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence {
        IdentityEvolutionOutcomeFamily::Ambiguity
    } else if comparison.intent() == IdentityComparisonIntent::AdvisoryCandidateSet {
        IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet
    } else if locality == BranchLocalityClass::CrossBranchAuthoritative
        && scenario != IdentityEvolutionSyntheticScenario::IdentityBreak
    {
        IdentityEvolutionOutcomeFamily::SingularIdentityContinuity
    } else {
        IdentityEvolutionOutcomeFamily::Denied
    }
}

fn branch_locality_class_for_comparison(
    scenario: IdentityEvolutionSyntheticScenario,
) -> BranchLocalityClass {
    match scenario {
        IdentityEvolutionSyntheticScenario::BranchLocalComparison => {
            BranchLocalityClass::BranchLocalOnly
        }
        IdentityEvolutionSyntheticScenario::AdvisoryAsAuthoritativeDenied => {
            BranchLocalityClass::CrossBranchDenied
        }
        _ => BranchLocalityClass::CrossBranchAuthoritative,
    }
}

fn execution_result_digest(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    execution_family: &str,
    metadata_digest: &str,
    outcome_family: &str,
) -> ResultDigest {
    ResultDigest::from_parts(&[
        format!(
            "query_digest:{}",
            admitted_query.query_context().query_digest().as_str()
        ),
        format!(
            "basis_digest:{}",
            admitted_query.query_context().basis_digest().as_str()
        ),
        format!("execution_family:{execution_family}"),
        format!("metadata_digest:{metadata_digest}"),
        format!("outcome_family:{outcome_family}"),
    ])
}
