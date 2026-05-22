use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_kernel::facade::{authoring::construction::*, certification::corpus::*};

fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

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
    let exhausted_simplex = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
        )
        .expect("exhausted simplex row");

    assert_eq!(report.rows().len(), 38);
    assert_eq!(report.accepted_count(), 24);
    assert_eq!(report.rejected_count(), 14);
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
    assert_eq!(exhausted_simplex.realization_strategy(), None);
    assert_eq!(
        exhausted_simplex.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        exhausted_simplex.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhausted_simplex.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_ne!(report.report_digest(), exhausted_simplex.row_digest());
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
    let simplex = report
        .row_for(PrimitiveConstructionFamily::SimplexSolid)
        .expect("simplex row");
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
        simplex.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
    );
    assert_eq!(
        simplex.admitted_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
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
    assert_ne!(report.report_digest(), simplex.row_digest());
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
        .row_for("wire_open_endpoint_graze")
        .expect("wire row");
    let mixed = compound
        .row_for("mixed_topology_class_batch")
        .expect("mixed row");
    let simplex = compound
        .row_for("simplex_world_collapsed_admitted_local_or_exact")
        .expect("simplex row");
    let closeout =
        prepare_primitive_construction_compound_milestone_closeout_report(&mut workspace)
            .expect("closeout");
    let motion_ids = sorted_ids(
        motion
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );
    let grazing_ids = sorted_ids(
        grazing
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );
    let closeout_ids = sorted_ids(closeout.required_scenarios().iter().cloned());

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
    assert_eq!(
        mixed.workload_family(),
        PrimitiveConstructionCompoundWorkloadFamily::MixedTopologyClassBatch
    );
    assert_eq!(
        simplex.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert!(motion.parity_verified());
    assert_eq!(
        motion_ids,
        vec![
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "wire_open_endpoint_graze".to_string(),
            "wire_open_motion_relocation".to_string(),
        ]
    );
    for motion_row in motion.rows() {
        assert_eq!(
            motion_row.motion_digest(),
            compound
                .row_for(motion_row.scenario_id())
                .expect("compound motion row")
                .motion_digest()
                .expect("compound motion digest")
        );
    }
    assert!(grazing.parity_verified());
    assert_eq!(
        grazing_ids,
        vec![
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "wire_open_endpoint_graze".to_string(),
        ]
    );
    for grazing_row in grazing.rows() {
        assert_eq!(
            grazing_row.grazing_digest(),
            compound
                .row_for(grazing_row.scenario_id())
                .expect("compound grazing row")
                .grazing_digest()
                .expect("compound grazing digest")
        );
    }
    assert!(closeout.closeout_gate_verified());
    assert_eq!(
        closeout_ids,
        vec![
            "mixed_topology_class_batch".to_string(),
            "orthotope_boundary_neighbor_rejected".to_string(),
            "orthotope_direct_stable".to_string(),
            "pyramid_direct_stable_comparison".to_string(),
            "pyramid_semantic_exhaustion".to_string(),
            "pyramid_threshold_admitted_exact_support".to_string(),
            "pyramid_threshold_rejected_neighbor".to_string(),
            "regular_prism_boundary_neighbor_rejected".to_string(),
            "regular_prism_direct_stable".to_string(),
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "simplex_world_collapsed_admitted_local_or_exact".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
            "simplex_world_collapsed_threshold_rejected".to_string(),
            "wire_open_endpoint_graze".to_string(),
            "wire_open_motion_relocation".to_string(),
        ]
    );
    assert!(closeout
        .required_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell closeout row")
        .inspection_digest()
        .is_some());
    assert!(closeout
        .required_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell closeout row")
        .projection_consumption_digest()
        .is_some());
    assert_eq!(
        closeout
            .required_row_for("pyramid_semantic_exhaustion")
            .expect("required pyramid exhaustion row")
            .exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_ne!(compound.report_digest(), shell.row_digest());
}
