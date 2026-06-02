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
