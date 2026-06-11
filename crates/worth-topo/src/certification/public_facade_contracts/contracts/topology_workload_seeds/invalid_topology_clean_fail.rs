use std::collections::BTreeSet;

use topology::facade::{
    TopologySeed, TopologySeedCleanFailClass, TopologySeedCleanFailReasonCode,
    TopologySeedCleanFailStage, TopologySeedKind, TopologySeedTopologyPosture,
};

#[test]
fn topology_workload_seeds_fail_closed_for_invalid_topology() {
    let short_loop = TopologySeed::single_face_loop(2).build().unwrap_err();
    assert_eq!(short_loop.kind(), TopologySeedKind::SingleFaceLoop);
    assert_eq!(
        short_loop.stage(),
        TopologySeedCleanFailStage::ParameterAdmission
    );
    assert_eq!(
        short_loop.class(),
        TopologySeedCleanFailClass::UnsupportedSeedParameter
    );
    assert_eq!(
        short_loop.reason_code(),
        TopologySeedCleanFailReasonCode::SingleFaceLoopEdgeCountOutOfRange
    );
    assert_eq!(
        short_loop.topology_posture(),
        TopologySeedTopologyPosture::ClosedValid
    );
    assert_human_readable_reason(short_loop.reason());
    assert!(short_loop.query_receipts().is_some());
    assert!(short_loop.entity_identities().is_none());
    assert!(!short_loop.can_enter_spatial_binding());

    let too_large_shell = TopologySeed::multi_face_shell(65).build().unwrap_err();
    assert_eq!(too_large_shell.kind(), TopologySeedKind::MultiFaceShell);
    assert_eq!(
        too_large_shell.class(),
        TopologySeedCleanFailClass::UnsupportedSeedParameter
    );
    assert_eq!(
        too_large_shell.reason_code(),
        TopologySeedCleanFailReasonCode::MultiFaceShellFaceCountOutOfRange
    );
    assert_human_readable_reason(too_large_shell.reason());
    assert!(!too_large_shell.can_enter_spatial_binding());
}

#[test]
fn dirty_topology_seeds_report_human_readable_spatial_binding_denials() {
    let dirty_loop = TopologySeed::self_intersecting_loop().build().unwrap_err();
    assert_eq!(dirty_loop.kind(), TopologySeedKind::SelfIntersectingLoop);
    assert_eq!(
        dirty_loop.topology_posture(),
        TopologySeedTopologyPosture::Dirty
    );
    assert_eq!(
        dirty_loop.stage(),
        TopologySeedCleanFailStage::SpatialBindingAdmission
    );
    assert_eq!(
        dirty_loop.class(),
        TopologySeedCleanFailClass::DirtyTopology
    );
    assert_eq!(
        dirty_loop.reason_code(),
        TopologySeedCleanFailReasonCode::SelfIntersectingLoopRequiresSpatialPolicy
    );
    assert_human_readable_reason(dirty_loop.reason());
    assert!(!dirty_loop.clean_fail_identity().trim().is_empty());
    assert!(dirty_loop.entity_identities().is_some());
    assert!(dirty_loop.counters().unwrap().total_topology_entities() > 0);
    assert!(!dirty_loop.can_enter_spatial_binding());

    let non_manifold = TopologySeed::non_manifold_wire().build().unwrap_err();
    assert_eq!(non_manifold.kind(), TopologySeedKind::NonManifoldWire);
    assert_eq!(
        non_manifold.stage(),
        TopologySeedCleanFailStage::SpatialBindingAdmission
    );
    assert_eq!(
        non_manifold.reason_code(),
        TopologySeedCleanFailReasonCode::NonManifoldWireCannotBindAsGeometry
    );
    assert_human_readable_reason(non_manifold.reason());
    assert!(non_manifold.entity_identities().is_some());
    assert!(!non_manifold.can_enter_spatial_binding());

    let thin_wall = TopologySeed::thin_wall_local_basis().build().unwrap_err();
    assert_eq!(thin_wall.kind(), TopologySeedKind::ThinWallLocalBasis);
    assert_eq!(
        thin_wall.stage(),
        TopologySeedCleanFailStage::SpatialBindingAdmission
    );
    assert_eq!(
        thin_wall.reason_code(),
        TopologySeedCleanFailReasonCode::ThinWallLocalBasisCannotBindAsGeometry
    );
    assert_human_readable_reason(thin_wall.reason());
    assert!(thin_wall.entity_identities().is_some());
    assert!(!thin_wall.can_enter_spatial_binding());

    let orientation = TopologySeed::orientation_inconsistency()
        .build()
        .unwrap_err();
    assert_eq!(
        orientation.kind(),
        TopologySeedKind::OrientationInconsistency
    );
    assert_eq!(
        orientation.stage(),
        TopologySeedCleanFailStage::SpatialBindingAdmission
    );
    assert_eq!(
        orientation.reason_code(),
        TopologySeedCleanFailReasonCode::OrientationInconsistencyRequiresRepairPolicy
    );
    assert_human_readable_reason(orientation.reason());
    assert!(orientation.entity_identities().is_some());
    assert!(!orientation.can_enter_spatial_binding());

    let clean_fail_identities = [
        dirty_loop.clean_fail_identity(),
        non_manifold.clean_fail_identity(),
        thin_wall.clean_fail_identity(),
        orientation.clean_fail_identity(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        clean_fail_identities.len(),
        4,
        "dirty topology seeds must not reuse one clean-fail identity under multiple labels"
    );

    let topology_shapes = [
        topology_shape_signature(dirty_loop.counters().unwrap()),
        topology_shape_signature(non_manifold.counters().unwrap()),
        topology_shape_signature(thin_wall.counters().unwrap()),
        topology_shape_signature(orientation.counters().unwrap()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        topology_shapes.len(),
        4,
        "dirty topology seeds must carry distinct topology evidence, not one fixture relabeled four ways"
    );
}

#[test]
fn topology_seed_parameter_boundaries_are_explicitly_admitted_or_denied() {
    assert!(TopologySeed::single_face_loop(3).build().is_ok());
    assert!(TopologySeed::single_face_loop(64).build().is_ok());
    assert_eq!(
        TopologySeed::single_face_loop(0)
            .build()
            .unwrap_err()
            .reason_code(),
        TopologySeedCleanFailReasonCode::SingleFaceLoopEdgeCountOutOfRange
    );
    assert_eq!(
        TopologySeed::single_face_loop(65)
            .build()
            .unwrap_err()
            .reason_code(),
        TopologySeedCleanFailReasonCode::SingleFaceLoopEdgeCountOutOfRange
    );

    assert!(TopologySeed::multi_face_shell(4).build().is_ok());
    assert!(TopologySeed::multi_face_shell(64).build().is_ok());
    assert_eq!(
        TopologySeed::multi_face_shell(3)
            .build()
            .unwrap_err()
            .reason_code(),
        TopologySeedCleanFailReasonCode::MultiFaceShellFaceCountOutOfRange
    );
    assert_eq!(
        TopologySeed::multi_face_shell(65)
            .build()
            .unwrap_err()
            .reason_code(),
        TopologySeedCleanFailReasonCode::MultiFaceShellFaceCountOutOfRange
    );
}

#[test]
fn topology_seed_workload_declaration_denial_is_typed_and_non_consumable() {
    let denial = TopologySeed::cube()
        .with_declaration("  ")
        .build()
        .unwrap_err();

    assert_eq!(denial.kind(), TopologySeedKind::Cube);
    assert_eq!(
        denial.stage(),
        TopologySeedCleanFailStage::ParameterAdmission
    );
    assert_eq!(
        denial.class(),
        TopologySeedCleanFailClass::WorkloadDeclaration
    );
    assert_eq!(
        denial.reason_code(),
        TopologySeedCleanFailReasonCode::WorkloadDeclarationRejectedSeed
    );
    assert!(denial.query_receipts().is_none());
    assert!(denial.entity_identities().is_none());
    assert!(!denial.can_enter_spatial_binding());
}

fn assert_human_readable_reason(reason: &str) {
    assert!(reason.contains(' '));
    assert!(!reason.contains('_'));
    assert!(!reason.trim().is_empty());
}

fn topology_shape_signature(
    counters: topology::facade::TopologySeedCounters,
) -> (usize, usize, usize, usize, usize, usize) {
    (
        counters.shell_count(),
        counters.face_count(),
        counters.loop_count(),
        counters.wire_count(),
        counters.half_edge_count(),
        counters.vertex_count(),
    )
}
