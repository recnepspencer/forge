use super::projection::{
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictRichDerivedDiagnosticLocalization,
};
use crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket;

pub(crate) fn select_rich_localization(
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
) -> Option<WorthTouchedGraphConflictRichDerivedDiagnosticLocalization> {
    match artifact_policy {
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth => None,
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization => {
            Some(build_rich_localization(selected_route_packet))
        }
    }
}

fn build_rich_localization(
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
) -> WorthTouchedGraphConflictRichDerivedDiagnosticLocalization {
    let triggered_bridge_scopes = Vec::new();

    WorthTouchedGraphConflictRichDerivedDiagnosticLocalization::new(
        selected_route_packet.touched_closure_digest().to_string(),
        selected_route_packet
            .touched_semantic_family_key()
            .to_string(),
        selected_route_packet.selected_plan_digest().to_string(),
        selected_route_packet.touched_aspect_count(),
        selected_route_packet.touched_scope_count(),
        selected_route_packet
            .selected_row_family_identities()
            .to_vec(),
        triggered_bridge_scopes,
        selected_route_packet
            .compiled_product_reuse_route_packet_identity()
            .to_string(),
        selected_route_packet.topology_reuse_posture(),
        format!("{:?}", selected_route_packet.spatial_reuse_posture()),
        selected_route_packet
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        selected_route_packet
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        selected_route_packet
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        selected_route_packet.batch_admission_denial_witness_kind(),
        selected_route_packet
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        selected_route_packet.conflict_independence_denial_witness_kind(),
    )
}
