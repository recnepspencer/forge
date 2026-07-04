use topology::facade::TopologyQueryBackedConsumerCutover;
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

use crate::workload_composition::planner_owned_routing::{
    CompiledProductReusePlannerRoutePacket, ReplayUndoPlannerRoutePacket,
    WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictSelectedRoutePacket,
};

use super::path::{
    RepresentativeSelectedRouteParityPathError, RepresentativeSelectedRouteParityPathErrorKind,
};

pub(crate) fn validate_representative_path_sources(
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    public_facade: &WorthTouchedGraphConflictPublicFacade,
    query_cutover: &TopologyQueryBackedConsumerCutover,
    evidence_route: &EvidenceLookupRoutePacket,
    replay_route: &ReplayUndoPlannerRoutePacket,
    reuse_route: &CompiledProductReusePlannerRoutePacket,
) -> Result<(), RepresentativeSelectedRouteParityPathError> {
    let query_row = query_cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .expect("loop-cycle query-backed read row should exist");
    let public_proof = public_facade.public_proof();
    let diagnostics = public_facade.derived_diagnostics();
    let seed = public_proof.milestone_fifteen_seed();
    let proof_chain = public_proof.proof_chain_digest();

    require(
        selected_route_packet.selected_route_identity_digest(),
        public_proof.selected_route_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedRouteIdentity,
        "public proof selected-route identity",
    )?;
    require(
        selected_route_packet.selected_route_identity_digest(),
        diagnostics.selected_route_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedRouteIdentity,
        "diagnostic selected-route identity",
    )?;
    require(
        selected_route_packet.selected_family_identity(),
        public_proof.selected_family_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedFamilyIdentity,
        "public proof selected-family identity",
    )?;
    require(
        selected_route_packet.selected_family_identity(),
        diagnostics.selected_family_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedFamilyIdentity,
        "diagnostic selected-family identity",
    )?;
    require(
        selected_route_packet.selected_family_identity(),
        query_row
            .selected_equivalence_family_identity()
            .expect("loop-cycle row should carry selected family"),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedFamilyIdentity,
        "topology query row selected-family identity",
    )?;
    require(
        selected_route_packet.spatial_selected_family_identity(),
        evidence_route.selected_equivalence_family_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedFamilyIdentity,
        "spatial evidence selected-family identity",
    )?;
    require(
        selected_route_packet.selected_family_identity(),
        reuse_route.selected_family_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedFamilyIdentity,
        "reuse route selected-family identity",
    )?;
    require(
        selected_route_packet.selected_product_identity_digest(),
        public_proof.selected_product_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedProductIdentity,
        "public proof selected-product identity",
    )?;
    require(
        selected_route_packet.selected_product_identity_digest(),
        diagnostics.selected_product_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedProductIdentity,
        "diagnostic selected-product identity",
    )?;
    require(
        selected_route_packet.selected_product_identity_digest(),
        query_row
            .compiled_product_identity_digest()
            .expect("loop-cycle row should carry compiled product"),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedProductIdentity,
        "topology query row compiled-product identity",
    )?;
    require(
        selected_route_packet.spatial_selected_product_identity_digest(),
        evidence_route.compiled_product_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedProductIdentity,
        "spatial evidence compiled-product identity",
    )?;
    require(
        selected_route_packet.spatial_selected_product_identity_digest(),
        reuse_route.selected_product_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedProductIdentity,
        "reuse route compiled-product identity",
    )?;
    if selected_route_packet.selected_witness_identity_digest()
        != public_proof.selected_witness_identity_digest()
        || selected_route_packet.selected_witness_identity_digest()
            != diagnostics.selected_witness_identity_digest()
    {
        return Err(RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedWitnessIdentity,
            "selected-route witness identity must agree with public proof and derived diagnostics",
        ));
    }
    require(
        selected_route_packet.selected_reuse_basis_identity_digest(),
        query_row
            .selected_reuse_basis_identity_digest()
            .expect("loop-cycle row should carry selected reuse basis"),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity,
        "topology query selected-reuse-basis identity",
    )?;
    require(
        selected_route_packet.selected_reuse_basis_identity_digest(),
        seed.topology_query_selected_reuse_basis_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity,
        "milestone-fifteen seed selected-reuse-basis identity",
    )?;
    require(
        selected_route_packet.selected_reuse_basis_identity_digest(),
        reuse_route.selected_reuse_basis_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity,
        "reuse route selected-reuse-basis identity",
    )?;
    require(
        selected_route_packet.topology_query_backed_consumer_cutover_digest(),
        query_cutover.closeout_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "topology query cutover digest",
    )?;
    require(
        selected_route_packet.topology_query_public_read_family_row_digest(),
        query_row.row_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "topology query family row digest",
    )?;
    require(
        selected_route_packet.topology_query_handle_identity_digest(),
        query_cutover.handle_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "topology query handle identity",
    )?;
    require(
        selected_route_packet.topology_query_operating_context_identity_digest(),
        query_cutover.operating_context_identity_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "topology query operating context identity",
    )?;
    require(
        selected_route_packet.topology_query_support_snapshot_digest(),
        query_cutover.support_snapshot_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "topology query support snapshot digest",
    )?;
    require(
        selected_route_packet.evidence_lookup_query_support_digest(),
        evidence_route.query_support_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "spatial evidence query support digest",
    )?;
    require(
        selected_route_packet.evidence_lookup_query_boundary_support_digest(),
        seed.evidence_lookup_query_boundary_support_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedQueryPosture,
        "evidence boundary support digest",
    )?;
    require(
        selected_route_packet.compiled_product_reuse_route_packet_identity(),
        reuse_route.packet_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity,
        "compiled-product reuse route identity",
    )?;
    require(
        selected_route_packet.replay_undo_route_packet_identity(),
        replay_route.route_packet_identity(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedReuseIdentity,
        "replay route identity",
    )?;
    require(
        public_proof.residue_chain().residue_digest(),
        seed.residue_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedResidueDigest,
        "milestone-fifteen seed residue digest",
    )?;
    require(
        public_proof.source_firewall_digest(),
        seed.source_firewall_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSourceFirewallDigest,
        "milestone-fifteen seed source-firewall digest",
    )?;
    require(
        public_proof.source_firewall_digest(),
        selected_route_packet.source_firewall_digest(),
        RepresentativeSelectedRouteParityPathErrorKind::MismatchedSourceFirewallDigest,
        "selected-route packet source-firewall digest",
    )?;
    if proof_chain.is_empty() {
        return Err(RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::MismatchedSelectedRouteIdentity,
            "public proof inspection must carry a non-empty proof-chain digest",
        ));
    }
    Ok(())
}

fn require(
    left: &str,
    right: &str,
    kind: RepresentativeSelectedRouteParityPathErrorKind,
    label: &str,
) -> Result<(), RepresentativeSelectedRouteParityPathError> {
    if left == right {
        return Ok(());
    }
    Err(RepresentativeSelectedRouteParityPathError::new(
        kind,
        format!("representative selected-route parity path requires matching {label}"),
    ))
}
