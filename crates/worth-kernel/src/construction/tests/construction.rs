use crate::construction::{
    build_canonical_primitive_construction_artifact, lower_scaffold_to_topology,
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_branch_preview_runtime_report,
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_rejection_locality_report,
    prepare_primitive_construction_replay_parity_report, prepare_primitive_construction_result,
    primitive_construction_family_coverage_report, OrthotopeSpec,
    PreparedPrimitiveConstructionExecution, PrimitiveConstructionFamily,
    PrimitiveConstructionFamilyCoverageStatus, PrimitiveConstructionIntent,
    PrimitiveConstructionPhaseChainReport, PrimitiveConstructionRequest, RegularPrismSpec,
    RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec, WireBodySpec,
};
use forge_query::facade::ForgeQueryAuthorityLane;
use topology::facade::{
    milestone_one_runtime_builder, topology_runtime, TopologyConstructionCertificationReadSurface,
    TopologyConstructionFactProvenance, TopologyConstructionInspectionSurface,
    TopologyConstructionMutationSurface, TopologyRuntimeAdapters,
};
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};

#[test]
fn admitted_phase_three_family_ladder_builds_generic_phase_chain_reports() {
    let requests = [
        PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 1.0 }).into_request(),
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        })
        .into_request(),
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        })
        .into_request(),
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        })
        .into_request(),
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }).into_request(),
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        })
        .into_request(),
    ];

    for request in requests {
        let intent = request.clone().admit().expect("admitted intent");
        let scaffold = intent.build_scaffold().expect("scaffold");
        let (birth_plan, lowering_plan) = lower_scaffold_to_topology(&scaffold).expect("lowering");
        let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
            &request,
            &intent,
            &scaffold,
            &birth_plan,
            &lowering_plan,
        )
        .expect("execution");
        let certification = execution.plan_topology_certification();
        let report = PrimitiveConstructionPhaseChainReport::from_phase_chain(
            &request,
            &intent,
            &scaffold,
            &birth_plan,
            &lowering_plan,
            &execution,
            &certification,
        );

        assert_eq!(report.family(), request.family());
        assert_eq!(
            report.mutation_surface(),
            TopologyConstructionMutationSurface::ComposeGraph
        );
        assert!(!report.execution_digest().is_empty());
        assert!(!report.certification_digest().is_empty());
        assert!(!report.report_digest().is_empty());
    }
}

#[test]
fn out_of_class_phase_three_requests_fail_typed_and_locally() {
    let wire_error = PrimitiveConstructionRequest::wire_body(2)
        .admit()
        .expect_err("wire body should reject");
    let shell_error = PrimitiveConstructionRequest::shell_with_hole(6, Vec::new())
        .admit()
        .expect_err("shell-with-hole should reject");

    assert!(wire_error.to_string().contains("invalid wire_body request"));
    assert!(shell_error
        .to_string()
        .contains("invalid shell_with_hole request"));
}

#[test]
fn family_coverage_report_marks_all_phase_three_rows_explicitly() {
    let report = primitive_construction_family_coverage_report();

    assert_eq!(
        report
            .row_for(PrimitiveConstructionFamily::RegularPrism)
            .expect("prism row")
            .status(),
        PrimitiveConstructionFamilyCoverageStatus::AdmittedClosedSolid
    );
    assert_eq!(
        report
            .row_for(PrimitiveConstructionFamily::WireBody)
            .expect("wire row")
            .status(),
        PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction
    );
    assert_eq!(report.rows().len(), PrimitiveConstructionFamily::ALL.len());
    assert!(!report.report_digest().is_empty());
}

#[test]
fn canonical_artifact_surface_binds_phase_chain_and_birth_truth() {
    let intent = PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 });
    let request = intent.clone().into_request();
    let admitted = request.clone().admit().expect("admitted intent");
    let scaffold = admitted.build_scaffold().expect("scaffold");
    let (birth_plan, lowering_plan) = lower_scaffold_to_topology(&scaffold).expect("lowering");
    let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
        &request,
        &admitted,
        &scaffold,
        &birth_plan,
        &lowering_plan,
    )
    .expect("execution");
    let certification = execution.plan_topology_certification();
    let artifact = build_canonical_primitive_construction_artifact(
        &request,
        &admitted,
        &scaffold,
        &birth_plan,
        &lowering_plan,
        &execution,
        &certification,
    )
    .expect("artifact");

    assert_eq!(artifact.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(artifact.birth_truth_digest(), birth_plan.birth_digest());
    assert_eq!(
        artifact.mutation_surface(),
        TopologyConstructionMutationSurface::ComposeGraph
    );
    assert_eq!(
        artifact.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        artifact.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
    assert!(!artifact.artifact_digest().is_empty());
}

#[test]
fn prepared_result_common_path_bundles_birth_mapping_and_artifact() {
    let result = prepare_primitive_construction_result(
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        }),
    )
    .expect("result");

    assert_eq!(result.family(), PrimitiveConstructionFamily::ShellWithHole);
    assert_eq!(result.topology_birth_class(), "planar_shell_with_hole_body");
    assert_eq!(
        result.canonical_artifact().family(),
        PrimitiveConstructionFamily::ShellWithHole
    );
    assert_eq!(
        result.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        result.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
    assert_eq!(result.evidence().birth_mapping_report().rows().len(), 7);
    assert_eq!(result.evidence().topology_fact_report().rows().len(), 7);
    assert!(!result.result_digest().is_empty());
}

#[test]
fn tiny_pyramid_result_preserves_escalated_realization_truth() {
    let result = prepare_primitive_construction_result(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    )
    .expect("tiny pyramid result");

    assert_eq!(
        result.realization_strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        result.stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
}

#[test]
fn branch_preview_runtime_report_opens_preview_and_branch_sessions() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.branch-preview".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    )
    .expect("runtime basis report");

    assert!(report.authority_chain_report().query_gap_rows().is_empty());
    assert_eq!(
        report.preview_lane().authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        report.branch_lane().authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert!(!report.report_digest().is_empty());
}

#[test]
fn replay_and_branch_local_parity_reports_cover_accepted_and_rejected_workflows() {
    let replay = prepare_primitive_construction_replay_parity_report(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        }),
    );
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five.branch-local".to_string(),
    )
    .expect("workspace");
    let branch = prepare_primitive_construction_branch_local_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 2,
            hole_loop_edge_counts: vec![3],
        }),
    )
    .expect("branch parity");

    assert!(replay.parity_verified());
    assert!(branch.parity_verified());
    assert_eq!(replay.family(), PrimitiveConstructionFamily::RegularPyramid);
    assert_eq!(branch.family(), PrimitiveConstructionFamily::ShellWithHole);
}

#[test]
fn query_and_diagnostic_reports_cover_phase_five_runtime_and_rejection_surfaces() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five.query-and-diagnostics".to_string(),
    )
    .expect("workspace");
    let basis = prepare_primitive_construction_query_basis_preview_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("basis report");
    let inspection = prepare_primitive_construction_query_inspection_parity_report(
        &mut workspace,
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        }),
    )
    .expect("inspection report");
    let locality = prepare_primitive_construction_rejection_locality_report(vec![
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
    ]);

    assert!(basis.parity_verified());
    assert!(basis.query_gap_free());
    assert_eq!(
        inspection.read_surface(),
        TopologyConstructionCertificationReadSurface::ProjectionConsumptionFromInspectionReceipt
    );
    assert_eq!(
        inspection.inspection_surface(),
        TopologyConstructionInspectionSurface::InspectReceipt
    );
    assert_eq!(
        inspection.fact_provenance(),
        TopologyConstructionFactProvenance::EquivalentProjectionConsumptionFacts
    );
    assert_eq!(locality.accepted_count(), 1);
    assert_eq!(locality.rejected_count(), 1);
}
