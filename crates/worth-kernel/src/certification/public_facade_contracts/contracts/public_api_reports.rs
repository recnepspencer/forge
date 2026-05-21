use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};
use worth_kernel::facade::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_conditioning_witness_report,
    prepare_primitive_construction_outcome,
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_boundary_gap_register,
    prepare_primitive_construction_query_existing_truth_binding_report,
    prepare_primitive_construction_query_graph_composition_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_no_local_runtime_workaround_audit,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_report_bundle,
    prepare_primitive_construction_realization_strategy_report,
    prepare_primitive_construction_rejection_locality_report,
    prepare_primitive_construction_replay_parity_report,
    prepare_primitive_construction_stability_class_report, OrthotopeSpec,
    PrimitiveConstructionFamily, PrimitiveConstructionIntent,
    PrimitiveConstructionQueryBoundaryGapStatus, PrimitiveConstructionQueryBoundaryUsagePosture,
    PrimitiveConstructionRealizationExhaustionStatus, RegularPrismSpec, RegularPyramidSpec,
    ShellWithHoleSpec, WireBodySpec,
};

#[test]
fn kernel_public_facade_exports_outcome_and_parity_reports() {
    let replay = prepare_primitive_construction_replay_parity_report(
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        }),
    );
    let rejected = prepare_primitive_construction_outcome(PrimitiveConstructionIntent::wire_body(
        WireBodySpec { edge_count: 2 },
    ));
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-branch-local".to_string(),
    )
    .expect("workspace");
    let branch = prepare_primitive_construction_branch_local_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
    )
    .expect("branch local parity report");

    assert_eq!(replay.family(), PrimitiveConstructionFamily::RegularPrism);
    assert!(replay.parity_verified());
    assert_eq!(rejected.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(branch.family(), PrimitiveConstructionFamily::WireBody);
    assert!(branch.parity_verified());
}

#[test]
fn kernel_public_facade_exports_query_and_rejection_reports() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-query-reports".to_string(),
    )
    .expect("workspace");
    let basis = prepare_primitive_construction_query_basis_preview_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("basis parity report");
    let inspection = prepare_primitive_construction_query_inspection_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        }),
    )
    .expect("inspection parity report");
    let graph = prepare_primitive_construction_query_graph_composition_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    )
    .expect("graph parity report");
    let existing_truth = prepare_primitive_construction_query_existing_truth_binding_report(
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
    );
    let receipt = prepare_primitive_construction_query_projection_consumption_receipt_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("projection consumption receipt report");
    let gap_register = prepare_primitive_construction_query_boundary_gap_register(&mut workspace)
        .expect("query gap register");
    let no_workaround = prepare_primitive_construction_query_no_local_runtime_workaround_audit();
    let locality = prepare_primitive_construction_rejection_locality_report(vec![
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
    ]);

    assert_eq!(basis.family(), PrimitiveConstructionFamily::RegularPyramid);
    assert!(basis.parity_verified());
    assert!(basis.query_gap_free());
    assert_eq!(
        basis.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        basis.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_eq!(
        basis.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        inspection.family(),
        PrimitiveConstructionFamily::ShellWithHole
    );
    assert!(inspection.parity_verified());
    assert_eq!(
        inspection.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        inspection.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_ne!(
        inspection.query_contract_digest(),
        inspection.report_digest()
    );
    assert_eq!(graph.family(), PrimitiveConstructionFamily::Orthotope);
    assert!(graph.parity_verified());
    assert_ne!(graph.query_contract_digest(), graph.report_digest());
    assert_eq!(
        existing_truth.family(),
        PrimitiveConstructionFamily::WireBody
    );
    assert_eq!(existing_truth.forbidden_pattern_count(), 0);
    assert_eq!(receipt.family(), PrimitiveConstructionFamily::RegularPrism);
    assert!(receipt.parity_verified());
    assert_eq!(
        receipt.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        receipt.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_ne!(receipt.query_contract_digest(), receipt.report_digest());
    assert_eq!(gap_register.rows().len(), 6);
    assert!(gap_register.unresolved_gap_count() >= 1);
    assert_eq!(
        gap_register
            .rows()
            .iter()
            .find(|row| row.family() == forge_query::facade::ForgeQueryRuntimeFacadeFamily::Write)
            .expect("write row")
            .usage_posture(),
        PrimitiveConstructionQueryBoundaryUsagePosture::RequiredNow
    );
    assert_eq!(
        gap_register
            .rows()
            .iter()
            .find(|row| row.family() == forge_query::facade::ForgeQueryRuntimeFacadeFamily::Temporal)
            .expect("temporal row")
            .gap_status(),
        PrimitiveConstructionQueryBoundaryGapStatus::DeferredUnsupportedNeighbor
    );
    assert_eq!(no_workaround.violation_count(), 0);
    assert_eq!(locality.accepted_count(), 1);
    assert_eq!(locality.rejected_count(), 1);
}

#[test]
fn kernel_public_facade_exports_realization_truth_reports() {
    let strategy = prepare_primitive_construction_realization_strategy_report(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    );
    let bundle = prepare_primitive_construction_realization_report_bundle(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    );
    let witness = prepare_primitive_construction_conditioning_witness_report(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    );
    let stability = prepare_primitive_construction_stability_class_report(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0,
            height: 1.0,
        })
        .at([1.0e308, 1.0e308, 1.0e308]),
    );
    let exhaustion = prepare_primitive_construction_realization_exhaustion_report(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0,
            height: 1.0,
        })
        .at([1.0e308, 1.0e308, 1.0e308]),
    );

    assert_eq!(
        strategy.family(),
        PrimitiveConstructionFamily::RegularPyramid
    );
    assert_eq!(
        strategy.selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        strategy.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        bundle.exhaustion_report().status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
    );
    assert!(witness.conditioning_witness().is_some());
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
    );
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_eq!(
        stability.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
}

#[test]
fn kernel_public_facade_exports_lower_layer_realization_exhaustion_witness_suite() {
    let report = prepare_primitive_construction_realization_exhaustion_witness_report();
    let pyramid = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse)
        .expect("pyramid witness row");
    let simplex = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse)
        .expect("simplex witness row");
    let squeezed_simplex = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        .expect("squeezed simplex witness row");

    assert_eq!(report.rows().len(), 3);
    assert_eq!(
        pyramid.family(),
        PrimitiveConstructionFamily::RegularPyramid
    );
    assert_eq!(
        pyramid.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        pyramid.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(simplex.family(), PrimitiveConstructionFamily::SimplexSolid);
    assert_eq!(
        simplex.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        simplex.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        squeezed_simplex.family(),
        PrimitiveConstructionFamily::SimplexSolid
    );
    assert_eq!(
        squeezed_simplex.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        squeezed_simplex.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_ne!(report.report_digest(), pyramid.row_digest());
    assert_ne!(report.report_digest(), simplex.row_digest());
    assert_ne!(report.report_digest(), squeezed_simplex.row_digest());
}
