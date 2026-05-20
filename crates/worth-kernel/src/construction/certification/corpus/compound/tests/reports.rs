use super::super::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::PrimitiveRealizationStrategy;

#[test]
fn compound_adversarial_siege_report_carries_shell_wire_and_motion_truth() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-adversarial-siege".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound report");
    let shell = report
        .row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell row");
    let wire = report.row_for("wire_open_origin_graze").expect("wire row");
    let pyramid = report
        .row_for("regular_pyramid_threshold_exact_support")
        .expect("pyramid row");

    assert!(report.authoring_order_parity_verified());
    assert_eq!(report.authoring_order_rows().len(), 3);
    assert_eq!(
        shell.workload_family(),
        PrimitiveConstructionCompoundWorkloadFamily::SheetPatch
    );
    assert_eq!(
        shell.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenShell
    );
    assert_eq!(
        shell.row_class(),
        PrimitiveConstructionCompoundRowClass::MotionHostileReorientation
    );
    assert_eq!(
        shell.motion_kind(),
        Some(PrimitiveConstructionCompoundMotionKind::Reorient)
    );
    assert_eq!(
        shell.grazing_kind(),
        Some(PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment)
    );
    assert!(shell.inspection_digest().is_some());
    assert_eq!(
        wire.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenWire
    );
    assert_eq!(
        wire.motion_kind(),
        Some(PrimitiveConstructionCompoundMotionKind::Offset)
    );
    assert_eq!(
        wire.grazing_kind(),
        Some(PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance)
    );
    assert_eq!(
        pyramid.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert!(!report.report_digest().is_empty());
}

#[test]
fn compound_motion_and_grazing_reports_summarize_their_specialized_rows() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-motion-grazing".to_string(),
    )
    .expect("workspace");
    let motion = prepare_primitive_construction_compound_motion_parity_report(&mut workspace)
        .expect("motion report");
    let grazing = prepare_primitive_construction_compound_grazing_boundary_report(&mut workspace)
        .expect("grazing report");

    assert!(motion.parity_verified());
    assert_eq!(motion.rows().len(), 3);
    assert!(grazing.parity_verified());
    assert_eq!(grazing.rows().len(), 2);
    assert!(!motion.report_digest().is_empty());
    assert!(!grazing.report_digest().is_empty());
}
