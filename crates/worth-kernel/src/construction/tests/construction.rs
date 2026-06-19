use super::super::admitted_scaffold::{
    prepare_primitive_construction_admitted_artifact,
    prepare_primitive_construction_executed_admitted_artifact,
};
use super::super::artifact::build_canonical_primitive_construction_artifact;
use super::super::intent::PrimitiveConstructionIntent;
use super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionPhaseError, PRIMITIVE_CONSTRUCTION_FAMILIES,
};
use super::super::result::prepare_primitive_construction_result;
use super::super::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
use super::support::family_coverage::{
    primitive_construction_family_coverage_report, PrimitiveConstructionFamilyCoverageStatus,
};
use forge_query::facade::ForgeQueryGraphObligationSupportLane;
use topology::certification::milestone_one_runtime_builder;
use topology::facade::{
    TopologyConstructionQueryMutationSurface, TopologyPrimitiveConstructionBirthTopologyKind,
};
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};

#[test]
fn admitted_phase_three_family_ladder_builds_direct_prepared_result_truth() {
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
        let result =
            prepare_primitive_construction_result(request.clone()).expect("prepared result");

        assert_eq!(result.family(), request.family());
        assert_eq!(
            result.mutation_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(
            result.topology_query_handoff().handoff_digest(),
            result.topology_query_handoff().handoff_digest()
        );
        assert_ne!(
            result.result_digest(),
            result.topology_query_handoff().handoff_digest()
        );
    }
}

#[test]
fn out_of_class_phase_three_requests_fail_typed_and_locally() {
    let wire_request =
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }).into_request();
    let wire_error = prepare_primitive_construction_admitted_artifact(&wire_request)
        .expect_err("wire body should reject");
    let shell_request = PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
        outer_loop_edge_count: 6,
        hole_loop_edge_counts: Vec::new(),
    })
    .into_request();
    let shell_error = prepare_primitive_construction_admitted_artifact(&shell_request)
        .expect_err("shell-with-hole should reject");

    match wire_error {
        PrimitiveConstructionPhaseError::InvalidRequest { family, reason } => {
            assert_eq!(family, PrimitiveConstructionFamily::WireBody);
            assert_eq!(
                reason,
                "polygonal construction families require at least three edges"
            );
        }
        other => panic!("expected invalid wire_body request, got {other:?}"),
    }
    match shell_error {
        PrimitiveConstructionPhaseError::InvalidRequest { family, reason } => {
            assert_eq!(family, PrimitiveConstructionFamily::ShellWithHole);
            assert_eq!(
                reason,
                "shell-with-hole requires at least one inner hole loop"
            );
        }
        other => panic!("expected invalid shell_with_hole request, got {other:?}"),
    }
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
    assert_eq!(report.rows().len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
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
fn primitive_birth_family_directory_inventory_matches_compose_coverage() {
    let family_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/construction/phase_chain/admitted_scaffold/family_birth_input/families");
    let mut family_files = std::fs::read_dir(&family_directory)
        .expect("family birth input directory should be readable")
        .map(|entry| {
            entry
                .expect("family file entry should be readable")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .filter(|name| name.ends_with(".rs") && name != "mod.rs")
        .collect::<Vec<_>>();
    family_files.sort();

    let mut expected_family_files = PRIMITIVE_CONSTRUCTION_FAMILIES
        .iter()
        .map(|family| format!("{}.rs", family.as_str()))
        .collect::<Vec<_>>();
    expected_family_files.sort();

    assert_eq!(family_files, expected_family_files);
    assert_eq!(family_files.len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
}

#[test]
fn canonical_artifact_surface_binds_admitted_artifact_and_birth_truth() {
    let intent = PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 });
    let request = intent.clone().into_request();
    let admitted_artifact =
        prepare_primitive_construction_admitted_artifact(&request).expect("admitted artifact");
    let artifact = build_canonical_primitive_construction_artifact(&admitted_artifact);

    assert_eq!(artifact.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(
        artifact.birth_truth_digest(),
        admitted_artifact
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
fn kernel_admitted_artifact_executes_topology_birth_compose_with_runtime_evidence() {
    let runtime = milestone_one_runtime_builder()
        .expect("topology runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.primitive-birth-compose.execution",
    )
    .expect("topology workspace");
    let request =
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }).into_request();

    let artifact =
        prepare_primitive_construction_executed_admitted_artifact(&mut workspace, &request)
            .expect("executed admitted artifact");
    let evidence = artifact
        .topology_compose_evidence()
        .expect("kernel artifact should retain topology compose evidence");

    assert_eq!(artifact.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(
        evidence.mutation_surface(),
        TopologyConstructionQueryMutationSurface::ComposeGraph
    );
    assert_eq!(evidence.graph_obligation_selected_count(), 1);
    assert_eq!(
        evidence
            .materialization_coverage()
            .committed_topology_kinds(),
        &[TopologyPrimitiveConstructionBirthTopologyKind::Vertex]
    );
    assert!(evidence
        .materialization_coverage()
        .unmaterialized_topology_kinds()
        .contains(&TopologyPrimitiveConstructionBirthTopologyKind::Edge));
    let selected_obligation = evidence
        .selected_obligation_rows()
        .first()
        .expect("kernel evidence should retain selected obligation row");
    assert_eq!(
        selected_obligation.rule_name(),
        "primitive-construction-birth-compose.graph-obligation"
    );
    assert_eq!(
        selected_obligation.support_lane(),
        ForgeQueryGraphObligationSupportLane::GraphComposition
    );
    assert_eq!(
        evidence.source_admitted_handoff_digest(),
        artifact.admitted_handoff_digest()
    );
    assert!(!evidence.graph_obligation_envelope_digest().is_empty());
}

#[test]
fn prepared_result_bundles_phase_chain_artifact_and_birth_mapping() {
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
        result.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        result.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
    assert_eq!(
        result
            .topology_query_handoff()
            .topology_query_envelope()
            .fact_rows()
            .len(),
        12
    );
    assert_ne!(result.result_digest(), result.artifact_digest());
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
fn literal_world_collapsed_simplex_result_survives_full_kernel_admitted_artifact() {
    let intent = PrimitiveConstructionIntent::simplex_solid(
        SimplexSolidSpec::new(1.0e-200).with_auxiliary_altitude_component(1.0e-220),
    )
    .at([2.0f64.powi(548), -2.0f64.powi(548), 2.0f64.powi(548)]);
    let request = intent.clone().into_request();
    let admitted_artifact =
        prepare_primitive_construction_admitted_artifact(&request).expect("admitted artifact");
    let result = prepare_primitive_construction_result(intent)
        .expect("literal world-collapsed simplex result");

    assert_eq!(
        admitted_artifact.realization_strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        admitted_artifact.realization_digest(),
        result.realization_digest()
    );
    assert_eq!(
        admitted_artifact.attempted_realization_strategies(),
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
    assert_eq!(
        result.conditioning_witness().normalization_disposition(),
        admitted_artifact
            .conditioning_witness()
            .normalization_disposition()
    );
}
