use topology::derived_read_diagnostic_input::support::TopologyDerivedReadDiagnosticSelectedRouteAuthority;

use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;

impl From<&WorthTouchedGraphConflictSelectedRoutePacket>
    for TopologyDerivedReadDiagnosticSelectedRouteAuthority
{
    fn from(packet: &WorthTouchedGraphConflictSelectedRoutePacket) -> Self {
        TopologyDerivedReadDiagnosticSelectedRouteAuthority::from_selected_route_identities(
            packet.selected_route_identity_digest(),
            packet.selected_family_identity(),
            packet.selected_product_identity_digest(),
            packet.selected_equivalence_policy_identity_digest(),
            packet.selected_compatibility_basis_identity_digest(),
            packet.selected_reuse_basis_identity_digest(),
            Some(
                packet
                    .compiled_product_reuse_route_packet_identity()
                    .to_string(),
            ),
            Some(packet.topology_reuse_posture()),
            Some(format!("{:?}", packet.spatial_reuse_posture())),
            packet
                .spatial_reuse_decision_identity_digest()
                .map(str::to_string),
            packet
                .spatial_rebuild_denial_identity_digest()
                .map(str::to_string),
            Some(packet.batch_admission_route_packet_identity().to_string()),
            packet
                .batch_admission_denial_witness_identity()
                .map(str::to_string),
            packet.batch_admission_denial_witness_kind(),
            Some(
                packet
                    .conflict_independence_route_packet_identity()
                    .to_string(),
            ),
            packet
                .conflict_independence_denial_witness_identity()
                .map(str::to_string),
            packet.conflict_independence_denial_witness_kind(),
        )
    }
}
