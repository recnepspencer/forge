#[test]
fn phase_five_public_construction_boundary_no_longer_teaches_stepwise_local_pipeline() {
    let facade = include_str!("../facade.rs");
    let public_api =
        include_str!("../certification/public_facade_contracts/contracts/public_api.rs");
    let compile_fail_contracts =
        include_str!("../certification/public_facade_contracts/compile_fail_contracts.rs");
    let construction_mod = include_str!("mod.rs");
    let query_native_boundary = include_str!("query_native_boundary.rs");
    let query_native_admitted_handoff = include_str!("query_native_boundary/admitted_handoff.rs");
    let query_native_envelope = include_str!("query_native_boundary/envelope.rs");
    let query_native_receipt = include_str!("query_native_boundary/receipt.rs");
    let query_native_admission = include_str!("query_native_boundary/admission.rs");

    for forbidden in [
        "topology_construction_authority",
        "TopologyConstructionAuthority",
        "lower_primitive_construction_birth_plan",
        "TopologyConstructionLoweringPlan",
        "prepare_primitive_construction_execution",
        "TopologyConstructionExecutionPlan",
        "prepare_primitive_construction_certification",
        "TopologyConstructionCertificationPlan",
        "build_topology_construction_fact_report",
        "TopologyConstructionFactReport",
    ] {
        assert!(
            !facade.contains(forbidden),
            "phase 5 remains incomplete while the root facade still teaches the local stepwise construction surface `{forbidden}` as public topology API",
        );
        assert!(
            !public_api.contains(forbidden),
            "phase 5 remains incomplete while public API certification still proves the local stepwise construction surface `{forbidden}`",
        );
    }

    for forbidden in [
        "mod authority;",
        "mod lowering;",
        "mod execution;",
        "mod certification;",
        "mod facts;",
    ] {
        assert!(
            !construction_mod.contains(forbidden),
            "phase 5 remains incomplete while construction still keeps the old stepwise module lane `{forbidden}` alive in live code",
        );
    }

    assert!(
        query_native_admission.contains("prepare_primitive_construction_query_receipt"),
        "phase 5 remains incomplete while live code lacks the topology-named construction receipt seam",
    );
    assert!(
        query_native_admission.contains("prepare_primitive_construction_query_envelope"),
        "phase 5 remains incomplete while live code lacks the topology-named construction envelope seam",
    );
    assert!(
        query_native_admission.contains("prepare_primitive_construction_query_handoff"),
        "phase 5 remains incomplete while live code lacks the topology-named construction handoff seam",
    );
    assert!(
        query_native_boundary.contains("prepare_primitive_construction_query_admitted_handoff"),
        "phase 5 remains incomplete while live code lacks the topology-named admitted construction handoff seam",
    );
    assert!(
        query_native_boundary.contains(
            "prepare_primitive_construction_query_admitted_handoff_from_synopsis"
        ),
        "phase 5 remains incomplete while topology still forces kernel to assemble receipt-envelope-handoff sequencing locally instead of exposing a synopsis-owned admitted handoff seam",
    );
    assert!(
        query_native_admitted_handoff.contains("pub fn topology_query_envelope("),
        "phase 5 remains incomplete while the admitted construction handoff does not retain the topology envelope seam directly",
    );
    assert!(
        query_native_boundary.contains("TopologyPrimitiveConstructionQueryBirthSynopsis"),
        "phase 5 remains incomplete while the construction boundary still lacks a topology-native birth synopsis seam",
    );
    assert!(
        !query_native_boundary.contains("worth_spatial::"),
        "phase 5 remains incomplete while the live topology construction boundary still imports worth-spatial directly",
    );
    assert!(
        !query_native_boundary.contains("worth_geom::"),
        "phase 5 remains incomplete while the live topology construction boundary still imports worth-geom directly",
    );
    assert!(
        query_native_receipt.contains("TopologyConstructionQueryMutationSurface::ComposeGraph"),
        "phase 5 remains incomplete while the construction receipt no longer retains compose-graph mutation posture on the topology boundary",
    );
    for required in [
        "TopologyPrimitiveConstructionQueryAdmittedHandoff",
        "TopologyPrimitiveConstructionQueryEnvelope",
        "TopologyPrimitiveConstructionQueryHandoff",
        "TopologyPrimitiveConstructionQueryReceipt",
        "TopologyConstructionQueryFactRow",
        "TopologyConstructionQueryMutationSurface",
        "TopologyConstructionQueryReadSurface",
    ] {
        assert!(
            query_native_boundary.contains(required)
                || query_native_envelope.contains(required)
                || query_native_receipt.contains(required),
            "phase 5 internal replacement seam must expose `{required}` as part of the consolidated Query-aware construction receipt and envelope boundary",
        );
    }
    for forbidden in [
        "TopologyPrimitiveConstructionQueryBoundary",
        "prepare_primitive_construction_query_boundary",
        "TopologyConstructionQueryBoundaryError",
    ] {
        assert!(
            !facade.contains(forbidden),
            "phase 5 receipt slice remains incomplete while the root facade still exports the replaced construction boundary symbol `{forbidden}`",
        );
        assert!(
            !public_api.contains(forbidden),
            "phase 5 receipt slice remains incomplete while public API certification still proves the replaced construction boundary symbol `{forbidden}`",
        );
    }

    for required in [
        "public_topology_construction_authority_not_exported",
        "public_topology_construction_stepwise_lowering_not_exported",
        "public_topology_construction_stepwise_execution_not_exported",
        "public_topology_construction_stepwise_certification_not_exported",
        "public_topology_construction_fact_report_not_exported",
        "public_topology_construction_boundary_not_exported",
        "public_topology_construction_boundary_preparation_not_exported",
    ] {
        assert!(
            compile_fail_contracts.contains(required),
            "phase 5 opening proof must keep `{required}` in the compile-fail contract set",
        );
    }
}

#[test]
fn phase_five_internal_construction_boundary_deletes_stepwise_bucket_files() {
    for removed in [
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/construction/authority.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/construction/lowering.rs"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/construction/execution.rs"),
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/certification.rs"
        ),
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/construction/facts.rs"),
    ] {
        assert!(
            !std::path::Path::new(removed).exists(),
            "phase 5 remains incomplete while removed stepwise construction bucket `{removed}` still exists in live code",
        );
    }
}

#[test]
fn primitive_construction_birth_compose_requires_declared_touched_basis_before_query_write() {
    let facade = include_str!("../facade.rs");
    let construction_mod = include_str!("mod.rs");
    let query_native_boundary = include_str!("query_native_boundary.rs");
    let compose_mod = include_str!("query_native_boundary/compose_execution/mod.rs");
    let execution = include_str!("query_native_boundary/compose_execution/execution.rs");
    let family_programs =
        include_str!("query_native_boundary/compose_execution/family_programs.rs");
    let program = include_str!("query_native_boundary/compose_execution/program.rs");
    let touched_basis = include_str!("query_native_boundary/compose_execution/touched_basis.rs");

    assert!(
        !facade.contains("execute_primitive_construction_birth_compose"),
        "primitive construction birth compose execution must not remain an ordinary public facade entrypoint",
    );
    assert!(
        !construction_mod.contains("execute_primitive_construction_birth_compose"),
        "construction root must not re-export the graph-birth compose execution helper as a competing authority surface",
    );
    assert!(
        !query_native_boundary.contains("execute_primitive_construction_birth_compose"),
        "query-native construction boundary must not re-export graph-birth compose execution outside the compose module",
    );
    assert!(
        compose_mod.contains("#[cfg(test)]\npub(crate) use execution::execute_primitive_construction_birth_compose"),
        "compose execution helper should be scoped to certification tests, not production facade authority",
    );
    assert!(
        execution.contains("pub(crate) fn execute_primitive_construction_birth_compose"),
        "primitive construction graph-birth compose execution must be crate-private",
    );
    assert!(
        execution.contains(
            "declared_touched_basis: TopologyPrimitiveConstructionBirthDeclaredTouchedBasis"
        ),
        "primitive construction graph-birth compose execution must require the declared touched-basis product",
    );

    assert!(
        !program.contains("pub(crate) fn execute("),
        "primitive construction compose program must not expose a crate-visible raw Query graph write method",
    );
    assert!(
        family_programs.contains("pub(super) fn build_primitive_construction_birth_compose_program"),
        "primitive construction compose program builder should be visible only to the guarded compose execution module",
    );
    assert!(
        execution.contains("program.execute_declared_touched_basis_checked("),
        "primitive construction compose execution must reach Query graph writes through the proof-bearing program method",
    );
    assert!(
        program.contains("admitted_handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff"),
        "the program graph-write entry must carry the admitted handoff being checked",
    );
    assert!(
        program.contains(
            "declared_touched_basis: &TopologyPrimitiveConstructionBirthDeclaredTouchedBasis"
        ),
        "the program graph-write entry must require the declared touched-basis product",
    );
    assert!(
        program.contains("fn execute_checked_graph_write(")
            && !program.contains("pub(crate) fn execute_checked_graph_write(")
            && !program.contains("pub(super) fn execute_checked_graph_write("),
        "the raw compose_graph implementation must remain private to the program module",
    );
    let guard = program
        .find("declared_touched_basis.require_matches_handoff(admitted_handoff)?")
        .expect("compose program must check declared touched-basis coverage");
    let write = program
        .find("workspace.compose_graph(")
        .expect("compose program must still call Query graph composition");
    assert!(
        guard < write,
        "declared touched-basis coverage must be checked inside the program before Query graph composition executes",
    );
    assert!(
        touched_basis.contains("topology_touched_graph_basis_from_mutation_sequence("),
        "construction birth intent may feed canonical touched-basis lowering but must not stand in as the basis",
    );
    assert!(
        touched_basis.contains("TopologyDeclaredTouchedGraphBasisProof::from_basis_with_touch_descriptor"),
        "construction birth touched-basis lowering must seal the basis behind descriptor-backed proof",
    );
    assert!(
        touched_basis.contains("TopologyDeclaredMutationSequenceBuilder"),
        "construction birth touched-basis lowering should use the shared topology mutation sequence vocabulary",
    );
    assert!(
        touched_basis.contains("TopologyTouchedOperatingWorld::mainline()"),
        "construction birth touched-basis lowering must declare the operating world used for Query execution",
    );
}
