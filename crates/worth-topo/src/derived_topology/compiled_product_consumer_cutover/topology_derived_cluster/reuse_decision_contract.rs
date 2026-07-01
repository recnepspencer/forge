use serde::{Deserialize, Serialize};

use crate::compiled_product_reuse_decision::{
    execute_topology_derived_reuse, TopologyDerivedReuseDecisionPosture,
    TopologyDerivedReuseExecutionInput, TopologyDerivedReuseMismatchLocus,
};
use crate::derived_topology::compiled_product_consumer_cutover::topology_derived_cluster::admitted_contract::DerivedEquivalenceContractReport;
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyUpdatePosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationPlannedDisposition {
    IncrementalUpdate,
    BoundedRebuild,
}

impl DerivedInvalidationPlannedDisposition {
    pub const fn from_update_posture(posture: DerivedTopologyUpdatePosture) -> Self {
        match posture {
            DerivedTopologyUpdatePosture::IncrementalEligible => Self::IncrementalUpdate,
            DerivedTopologyUpdatePosture::BoundedRebuildRequired => Self::BoundedRebuild,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncrementalUpdate => "incremental_update",
            Self::BoundedRebuild => "bounded_rebuild",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedParityComparisonReport {
    pub comparison_supported: bool,
    pub unsupported_comparison_reason: Option<String>,
    pub reuse_decision_posture: Option<TopologyDerivedReuseDecisionPosture>,
    pub rebuild_denial_identity_digest: Option<String>,
    pub mismatch_loci: Vec<TopologyDerivedReuseMismatchLocus>,
    pub compared_basis_dimension_count: usize,
    pub compared_derived_surface_digest_count: usize,
    pub authority_identity_match: bool,
    pub branch_identity_match: bool,
    pub invalidation_target_match: bool,
    pub materialized_topology_digest_match: bool,
    pub interpreted_topology_digest_match: bool,
    pub derived_validation_digest_match: bool,
    pub equivalent_derived_meaning: bool,
}

pub fn compare_derived_equivalence_contracts(
    lhs: &DerivedEquivalenceContractReport,
    rhs: &DerivedEquivalenceContractReport,
) -> DerivedParityComparisonReport {
    let lhs_input = TopologyDerivedReuseExecutionInput::lower(lhs);
    let rhs_input = TopologyDerivedReuseExecutionInput::lower(rhs);
    let resolution = execute_topology_derived_reuse(&lhs_input, &rhs_input);
    let decision = resolution.decision();

    DerivedParityComparisonReport {
        comparison_supported: decision.comparison_supported(),
        unsupported_comparison_reason: decision.unsupported_comparison_reason().map(str::to_string),
        reuse_decision_posture: Some(decision.posture()),
        rebuild_denial_identity_digest: decision
            .rebuild_denial()
            .map(|denial| denial.denial_identity_digest().to_string()),
        mismatch_loci: decision
            .rebuild_denial()
            .map(|denial| denial.mismatch_loci().to_vec())
            .unwrap_or_default(),
        compared_basis_dimension_count: decision.counters().compared_basis_dimension_count(),
        compared_derived_surface_digest_count: decision
            .counters()
            .compared_derived_surface_digest_count(),
        authority_identity_match: resolution.authority_identity_match(),
        branch_identity_match: resolution.branch_identity_match(),
        invalidation_target_match: resolution.invalidation_target_match(),
        materialized_topology_digest_match: resolution.materialized_topology_digest_match(),
        interpreted_topology_digest_match: resolution.interpreted_topology_digest_match(),
        derived_validation_digest_match: resolution.derived_validation_digest_match(),
        equivalent_derived_meaning: resolution.equivalent_derived_meaning(),
    }
}

pub const fn topology_cutover_planned_disposition_from_update_posture(
    posture: DerivedTopologyUpdatePosture,
) -> DerivedInvalidationPlannedDisposition {
    DerivedInvalidationPlannedDisposition::from_update_posture(posture)
}
