use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject;
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts, PlanarLocalRebuildScope,
    PlanarRebindingContinuityEvidence,
};

use super::diagnostic_fixture::diagnostic_receipt_for;
use super::parity_fixture::{local_rebuild_parity_parts, LocalRebuildParityParts};
use super::runtime_handles::local_rebuild_handle;

#[test]
fn local_rebuild_rebinding_parity_preserves_movement_and_diagnostics() {
    let first = certified_digest_for("phase-19-movement-order", "rebinding-continuation:a");
    let replay_a = certified_digest_for("phase-19-movement-order", "rebinding-continuation:a");
    assert_eq!(
        replay_a, first,
        "equivalent retained replay and local rebuild evidence must converge"
    );

    let changed_rebinding =
        certified_digest_for("phase-19-movement-order", "rebinding-continuation:b");
    assert_ne!(
        changed_rebinding, first,
        "rebinding continuity remains part of the certified rebuild parity basis"
    );

    let mut diagnostic_changed_parts = local_rebuild_parity_parts("phase-19-diagnostic-binding");
    diagnostic_changed_parts.diagnostics = diagnostic_receipt_for(
        "phase-19-diagnostic-binding",
        PlanarDiagnosticSubject::binding_failure("orientation-flip-through-rebuild:known-step"),
    );
    let changed_diagnostic = certified_digest_for_parts(
        "phase-19-diagnostic-binding",
        diagnostic_changed_parts,
        "rebinding-continuation:diagnostic",
    );
    let unchanged_diagnostic = certified_digest_for(
        "phase-19-diagnostic-binding",
        "rebinding-continuation:diagnostic",
    );
    assert_ne!(
        changed_diagnostic, unchanged_diagnostic,
        "diagnostic localization remains part of the certified rebuild parity basis"
    );
}

fn certified_digest_for(world: &'static str, continuity: &'static str) -> String {
    let parts = local_rebuild_parity_parts(world);
    certified_digest_for_parts(world, parts, continuity)
}

fn certified_digest_for_parts(
    world: &'static str,
    parts: LocalRebuildParityParts,
    continuity: &'static str,
) -> String {
    let neighborhood_digest = parts.neighborhood.fact_digest().to_string();
    PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:movement-rotation",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        continuity,
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
    .expect("movement rebuild parity plan")
    .certify()
    .expect("movement rebuild parity receipt")
    .parity_digest()
    .to_string()
}
