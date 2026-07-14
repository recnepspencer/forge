mod bridge_fixture;
mod core;
mod phase_six;
mod source_adapter;

use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    WorthQueryLowerRuntimeCapabilityEligibility, WorthQueryLowerRuntimeCapabilityRequest,
    WorthQueryLowerRuntimeRoutePlan, WorthQueryLowerRuntimeSeamKey,
};
use crate::runtime::WorthQueryAspectTouch;
use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};

#[derive(Clone)]
pub(super) struct RepresentativeArtifacts {
    pub seam_key: WorthQueryLowerRuntimeSeamKey,
    pub request: WorthQueryLowerRuntimeCapabilityRequest,
    pub eligibility: WorthQueryLowerRuntimeCapabilityEligibility,
    pub route_plan: Option<WorthQueryLowerRuntimeRoutePlan>,
    pub boundary_receipt: WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    pub envelope: WorthQueryLowerRuntimeBoundaryEnvelope,
    pub evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource,
}

pub(super) use super::evidence::WorthQueryLowerRuntimeRepresentativeEvidenceSource;
pub(super) use bridge_fixture::representative_bridge_authority_runtime;
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
pub(super) use source_adapter::RepresentativeSourceAdapter;

pub(super) fn title_value_touch() -> WorthQueryAspectTouch {
    representative_aspect_field_touch("title", "value")
}

pub(super) fn status_value_touch() -> WorthQueryAspectTouch {
    representative_aspect_field_touch("status", "value")
}

pub(super) fn priority_value_touch() -> WorthQueryAspectTouch {
    representative_aspect_field_touch("priority", "value")
}

fn representative_aspect_field_touch(
    aspect_label: &'static str,
    field_label: &'static str,
) -> WorthQueryAspectTouch {
    let aspect_key =
        AspectKey::new(aspect_label).expect("representative static aspect key should admit");
    let field_key =
        FieldKey::new(field_label).expect("representative static field key should admit");
    let field_path = CanonicalFieldPath::new([field_key])
        .expect("representative static field path should admit");
    WorthQueryAspectTouch::aspect_field_path(aspect_key, field_path)
}
