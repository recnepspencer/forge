use crate::construction::admitted_scaffold::prepare_primitive_construction_admitted_result_input;
use crate::construction::artifact::build_canonical_primitive_construction_artifact;
use crate::construction::evidence::PrimitiveConstructionResultAssemblyReport;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_branch_preview_runtime_report,
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_rejection_locality_report,
    prepare_primitive_construction_replay_parity_report,
    primitive_construction_family_coverage_report, OrthotopeSpec, PrimitiveConstructionFamily,
    PrimitiveConstructionFamilyCoverageStatus, PrimitiveConstructionIntent,
    PrimitiveConstructionRequest, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec,
    SimplexSolidSpec, WireBodySpec,
};
use forge_query::facade::ForgeQueryAuthorityLane;
use topology::facade::{
    milestone_one_runtime_builder, topology_runtime, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyRuntimeAdapters,
};
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};

#[test]
fn admitted_phase_three_family_ladder_builds_generic_result_assembly_reports() {
    let requests = [
        PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)).into_request(),
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
        let result_input =
            prepare_primitive_construction_admitted_result_input(&request).expect("result input");
        let report =
            PrimitiveConstructionResultAssemblyReport::from_admitted_result_input(&result_input);

        assert_eq!(report.family(), request.family());
        assert_eq!(
            report.mutation_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(
            report.topology_query_admitted_handoff_digest(),
            result_input
                .topology_query_admitted_handoff()
                .admitted_handoff_digest()
        );
        assert_ne!(
            report.report_digest(),
            report.topology_query_admitted_handoff_digest()
        );
    }
}

#[test]
fn out_of_class_phase_three_requests_fail_typed_and_locally() {
    let wire_request = PrimitiveConstructionRequest::wire_body(2);
    let wire_error = prepare_primitive_construction_admitted_result_input(&wire_request)
        .expect_err("wire body should reject");
    let shell_request = PrimitiveConstructionRequest::shell_with_hole(6, Vec::new());
    let shell_error = prepare_primitive_construction_admitted_result_input(&shell_request)
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
    assert_ne!(
        report
            .row_for(PrimitiveConstructionFamily::RegularPrism)
            .expect("prism row")
            .row_digest(),
        report
            .row_for(PrimitiveConstructionFamily::WireBody)
            .expect("wire row")
            .row_digest()
    );
}

#[test]
fn canonical_artifact_surface_binds_result_input_and_birth_truth() {
    let intent = PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 });
    let request = intent.clone().into_request();
    let result_input =
        prepare_primitive_construction_admitted_result_input(&request).expect("result input");
    let artifact = build_canonical_primitive_construction_artifact(&result_input);

    assert_eq!(artifact.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(
        artifact.birth_truth_digest(),
        result_input
            .topology_query_admitted_handoff()
            .topology_query_handoff()
            .source_birth_digest()
    );
    assert_eq!(
        artifact.mutation_surface(),
        TopologyConstructionQueryMutationSurface::ComposeGraph
    );
    assert_eq!(
        artifact.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        artifact.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
    assert_ne!(artifact.artifact_digest(), artifact.birth_truth_digest());
}

#[test]
fn prepared_result_input_bundles_birth_mapping_and_artifact() {
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
    assert_eq!(
        result
            .evidence()
            .topology_query_handoff()
            .topology_query_envelope()
            .fact_rows()
            .len(),
        7
    );
    assert_ne!(
        result.result_digest(),
        result.canonical_artifact().artifact_digest()
    );
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
fn literal_world_collapsed_simplex_result_survives_full_kernel_result_input() {
    let intent = PrimitiveConstructionIntent::simplex_solid(
        SimplexSolidSpec::new(1.0e-200).with_auxiliary_altitude_component(1.0e-220),
    )
    .at([2.0f64.powi(548), -2.0f64.powi(548), 2.0f64.powi(548)]);
    let request = intent.clone().into_request();
    let result_input =
        prepare_primitive_construction_admitted_result_input(&request).expect("result input");
    let result = prepare_primitive_construction_result(intent)
        .expect("literal world-collapsed simplex result");

    assert_eq!(
        result_input.realization_report().strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        result_input.realization_report().report_digest(),
        result.realization_report().report_digest()
    );
    assert_eq!(
        result_input.realization_report().attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
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
        TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
    );
    assert_eq!(
        inspection.inspection_surface(),
        TopologyConstructionQueryInspectionSurface::InspectReceipt
    );
    assert_eq!(
        inspection.fact_provenance(),
        TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
    );
    assert_eq!(locality.accepted_count(), 1);
    assert_eq!(locality.rejected_count(), 1);
}
