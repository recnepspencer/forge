use topology::facade::{
    milestone_one_runtime_builder, topology_runtime, TopologyConstructionCertificationReadSurface,
    TopologyConstructionInspectionSurface, TopologyConstructionMutationSurface,
    TopologyRuntimeAdapters,
};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_kernel::facade::{
    authoring::construction::*,
    diagnostics::{family::*, query::*},
    outcome::{execution::*, prepared::*},
};

#[test]
fn kernel_public_facade_exports_query_backed_authoring_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api".to_string(),
    )
    .expect("workspace");
    let session = primitive_construction_authoring(&mut workspace).expect("authoring session");
    let report = session.authority_chain_report();
    assert_eq!(session.query_front_door(), "ForgeQueryWorkspace");
    assert_eq!(report.required_query_family_contracts().len(), 2);
    assert!(report.query_gap_rows().is_empty());
}

#[test]
fn kernel_public_facade_exports_phase_three_family_ladder() {
    let request = PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
        outer_loop_edge_count: 6,
        hole_loop_edge_counts: vec![3, 4],
    })
    .into_request();
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
    let artifact = build_canonical_primitive_construction_artifact(
        &request,
        &intent,
        &scaffold,
        &birth_plan,
        &lowering_plan,
        &execution,
        &certification,
    )
    .expect("artifact");
    let coverage = primitive_construction_family_coverage_report();

    assert_eq!(
        execution.family(),
        PrimitiveConstructionFamily::ShellWithHole
    );
    assert_eq!(scaffold.support_planes().len(), 1);
    assert_eq!(scaffold.topology_counts().loop_count(), 3);
    assert_eq!(
        lowering_plan.mutation_surface(),
        TopologyConstructionMutationSurface::ComposeGraph
    );
    assert_eq!(
        certification.read_surface(),
        TopologyConstructionCertificationReadSurface::ProjectionConsumptionFromInspectionReceipt
    );
    assert_eq!(
        certification.inspection_surface(),
        TopologyConstructionInspectionSurface::InspectReceipt
    );
    assert_eq!(artifact.birth_truth_digest(), birth_plan.birth_digest());
    assert_eq!(artifact.supported_loop_count(), 3);
    assert_eq!(
        artifact.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        coverage
            .row_for(PrimitiveConstructionFamily::WireBody)
            .expect("wire row")
            .status(),
        PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction
    );
}

#[test]
fn kernel_public_facade_exports_prepared_result_common_path() {
    let result = prepare_primitive_construction_result(PrimitiveConstructionIntent::wire_body(
        WireBodySpec { edge_count: 8 },
    ))
    .expect("prepared result");

    assert_eq!(result.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(result.topology_birth_class(), "planar_wire_body");
    assert_eq!(
        result.canonical_artifact().family(),
        PrimitiveConstructionFamily::WireBody
    );
    assert_eq!(result.evidence().birth_mapping_report().rows().len(), 7);
    assert_eq!(result.evidence().topology_fact_report().rows().len(), 7);
    assert_eq!(
        result.canonical_artifact().birth_completeness_digest(),
        result
            .evidence()
            .birth_completeness_report()
            .completeness_digest()
    );
    assert_eq!(
        result.canonical_artifact().topology_fact_digest(),
        result.evidence().topology_fact_report().report_digest()
    );
    assert_eq!(
        result.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        result.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_eq!(
        result.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
}

#[test]
fn kernel_public_facade_exports_branch_preview_runtime_report() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-branch-preview".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)),
    )
    .expect("branch preview report");

    assert_eq!(report.family(), PrimitiveConstructionFamily::SimplexSolid);
    assert!(report.authority_chain_report().query_gap_rows().is_empty());
    assert_eq!(
        report.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        report.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_eq!(
        report.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_ne!(
        report.outcome().outcome_digest(),
        report.preview_lane().admission_digest()
    );
    assert_ne!(
        report.outcome().outcome_digest(),
        report.branch_lane().admission_digest()
    );
    assert_ne!(
        report.preview_lane().admission_digest(),
        report.branch_lane().admission_digest()
    );
}

#[test]
fn kernel_public_facade_exports_escalated_realization_truth_for_tiny_pyramid() {
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
        result.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        result.stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        result
            .realization_report()
            .conditioning_witness()
            .support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
    );
    assert_eq!(
        result
            .realization_report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}
