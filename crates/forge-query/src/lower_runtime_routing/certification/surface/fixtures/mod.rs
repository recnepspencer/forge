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
    representative_causal_bridge_materialization_row, representative_frontier_evidence_row,
    representative_projection_bridge_row, representative_projection_query_receipts_row,
    representative_projection_relational_row, representative_subscription_activation_row,
};
