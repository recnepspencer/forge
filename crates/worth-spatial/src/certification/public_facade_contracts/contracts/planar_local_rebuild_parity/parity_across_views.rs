use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts, PlanarLocalRebuildScope,
    PlanarRebindingContinuityEvidence,
};

use super::parity_fixture::local_rebuild_parity_parts;
use super::runtime_handles::local_rebuild_handle;

#[test]
fn local_planar_rebuild_and_rebinding_converge_for_equivalent_neighborhoods() {
    let world = "phase-19-equivalent-neighborhoods";
    let parts = local_rebuild_parity_parts(world);
    let neighborhood_digest = parts.neighborhood.fact_digest().to_string();
    let receipt = PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:equivalent-neighborhood",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "rebinding-continuation:equivalent-neighborhood",
        neighborhood_digest,
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    ))
    .expect("local rebuild parity plan")
    .certify()
    .expect("local rebuild parity receipt");

    assert_eq!(receipt.parity_rows().len(), 7);
    assert_eq!(receipt.counters().local_neighborhood_rows(), 1);
    assert_eq!(receipt.counters().rebinding_continuity_rows(), 1);
    assert_eq!(receipt.counters().parity_views_compared(), 7);
    assert_eq!(receipt.counters().source_receipts_consumed(), 8);
    assert!(receipt.declaration_digest().contains("sha"));
    assert!(!receipt.parity_digest().is_empty());
}
