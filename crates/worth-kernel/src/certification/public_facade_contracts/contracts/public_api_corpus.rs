use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy};
use worth_kernel::facade::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    prepare_primitive_construction_corpus_replay_siege,
    prepare_primitive_construction_family_boundary_report,
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundTopologyClass, PrimitiveConstructionCompoundWorkloadFamily,
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionFamily,
    PrimitiveConstructionFamilyBoundaryTransitionClass,
};

#[test]
fn kernel_public_facade_exports_corpus_replay_siege_certification_artifact() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-corpus-replay-siege".to_string(),
    )
    .expect("workspace");
    let report =
        prepare_primitive_construction_corpus_replay_siege(&mut workspace).expect("siege report");
    let rejected_wire = report
        .row_for(
            PrimitiveConstructionFamily::WireBody,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("rejected wire row");
    let tiny_pyramid = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("tiny pyramid row");

    assert_eq!(report.rows().len(), 36);
    assert_eq!(report.accepted_count(), 24);
    assert_eq!(report.rejected_count(), 12);
    assert_eq!(report.authoring_order_rows().len(), 4);
    assert!(report.authoring_order_parity_verified());
    assert_eq!(report.rejection_witness_rows().len(), 6);
    assert!(rejected_wire.birth_digest().is_none());
    assert_eq!(rejected_wire.construction_breadth(), 0);
    assert_eq!(
        tiny_pyramid.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        tiny_pyramid.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert!(!report.report_digest().is_empty());
}

#[test]
fn kernel_public_facade_exports_family_boundary_certification_artifact() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-family-boundaries".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_family_boundary_report(&mut workspace)
        .expect("family boundary report");
    let pyramid = report
        .row_for(PrimitiveConstructionFamily::RegularPyramid)
        .expect("pyramid row");
    let prism = report
        .row_for(PrimitiveConstructionFamily::RegularPrism)
        .expect("prism row");

    assert_eq!(report.rows().len(), 6);
    assert_eq!(
        prism.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::DirectStableToTypedRejection
    );
    assert_eq!(
        pyramid.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
    );
    assert_eq!(
        pyramid.admitted_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert!(!report.report_digest().is_empty());
}

#[test]
fn kernel_public_facade_exports_compound_motion_and_grazing_certification_artifacts() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-compound-corpus".to_string(),
    )
    .expect("workspace");
    let compound = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound siege");
    let motion = prepare_primitive_construction_compound_motion_parity_report(&mut workspace)
        .expect("motion parity");
    let grazing = prepare_primitive_construction_compound_grazing_boundary_report(&mut workspace)
        .expect("grazing boundary");
    let shell = compound
        .row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell row");
    let wire = compound
        .row_for("wire_open_origin_graze")
        .expect("wire row");

    assert!(compound.authoring_order_parity_verified());
    assert_eq!(
        shell.workload_family(),
        PrimitiveConstructionCompoundWorkloadFamily::SheetPatch
    );
    assert_eq!(
        shell.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenShell
    );
    assert_eq!(
        shell.motion_kind(),
        Some(PrimitiveConstructionCompoundMotionKind::Reorient)
    );
    assert_eq!(
        shell.grazing_kind(),
        Some(PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment)
    );
    assert_eq!(
        wire.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenWire
    );
    assert!(motion.parity_verified());
    assert!(grazing.parity_verified());
    assert!(!compound.report_digest().is_empty());
}
