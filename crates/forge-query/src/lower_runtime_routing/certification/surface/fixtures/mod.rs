mod core;
mod phase_six;

use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeCapabilityRequest,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};

#[derive(Clone)]
pub(super) struct RepresentativeArtifacts {
    pub seam_key: ForgeQueryLowerRuntimeSeamKey,
    pub request: ForgeQueryLowerRuntimeCapabilityRequest,
    pub eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
    pub route_plan: Option<ForgeQueryLowerRuntimeRoutePlan>,
    pub boundary_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    pub envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
    pub evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
}

pub(super) use super::evidence::ForgeQueryLowerRuntimeRepresentativeEvidenceSource;
pub(super) use core::{
    hostile_parity_divergence_digest, normalized_parity_digest,
    representative_live_view_schema_row, representative_live_view_source_row,
    representative_preview_basis_row, representative_signal_invalidation_row,
    representative_write_authority_row, synthetic_inventory_row,
};
pub(super) use phase_six::{
    representative_basis_subscription_readmission_row,
    representative_basis_truth_view_readmission_row,
    representative_causal_bridge_materialization_row, representative_compose_read_row,
    representative_compose_read_with_invariant_pack_row,
    representative_effect_bridge_writeback_row, representative_effect_relational_merge_row,
    representative_effect_relational_mutation_row,
    representative_execute_read_family_in_basis_context_row,
    representative_execute_read_family_row, representative_frontier_evidence_row,
    representative_historical_bridge_lowering_row, representative_intent_runtime_execution_row,
    representative_projection_bridge_row, representative_projection_query_receipts_row,
    representative_projection_relational_row, representative_public_live_view_declaration_row,
    representative_runtime_basis_context_read_graph_row,
    representative_runtime_current_read_graph_row, representative_runtime_intent_authority_row,
    representative_runtime_live_installation_orchestration_row,
    representative_subscription_activation_row, representative_subscription_continuity_row,
};
