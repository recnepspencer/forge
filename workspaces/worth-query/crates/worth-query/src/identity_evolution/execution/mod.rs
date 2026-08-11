mod classification;
mod comparison;
mod lineage;

use super::{
    admission::{AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError},
    families::IdentityEvolutionOutcomeFamily,
    performance::IdentityEvolutionPredictionDriftOutcome,
    results::IdentityEvolutionResultBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionExecutionFamily {
    InstalledOperationComparison,
    DirectPredecessor,
    DirectSuccessor,
    DirectReplacement,
    DirectSplitSuccessors,
    DirectMergeSuccessor,
    GeneratedIdentity,
    RetiredIdentity,
    BranchLocalDirectEvolution,
    BranchToBranchComparison,
    CurrentToHistoricalComparison,
    HistoricalToHistoricalComparison,
    PreviewToAuthoritativeComparison,
}

impl IdentityEvolutionExecutionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InstalledOperationComparison => "installed_operation_comparison",
            Self::DirectPredecessor => "direct_predecessor",
            Self::DirectSuccessor => "direct_successor",
            Self::DirectReplacement => "direct_replacement",
            Self::DirectSplitSuccessors => "direct_split_successors",
            Self::DirectMergeSuccessor => "direct_merge_successor",
            Self::GeneratedIdentity => "generated_identity",
            Self::RetiredIdentity => "retired_identity",
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
        return lineage::execute(admitted_query, descriptor);
    }
    if let Some((basis_family, left_basis_digest, right_basis_digest, comparison)) =
        admitted_query.correspondence_identity_comparison()
    {
        return comparison::execute(
            admitted_query,
            basis_family,
            left_basis_digest,
            right_basis_digest,
            comparison,
        );
    }
    unreachable!("admitted identity evolution query must have one closed shape")
}

pub(super) fn result_bundle_is_denied(bundle: &IdentityEvolutionResultBundle) -> bool {
    matches!(
        bundle.outcome_family(),
        IdentityEvolutionOutcomeFamily::Denied
    )
}
