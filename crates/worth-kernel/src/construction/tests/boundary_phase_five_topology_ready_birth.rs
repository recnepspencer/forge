const ADMITTED_SCAFFOLD_ROOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/mod.rs"
));
const TOPOLOGY_READY_BIRTH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/topology_ready_birth.rs"
));

#[test]
fn phase_five_topology_ready_birth_boundary_localizes_post_birth_query_bridge() {
    assert!(ADMITTED_SCAFFOLD_ROOT.contains("prepare_primitive_construction_topology_ready_birth("));
    for forbidden in [
        "plan_primitive_construction_birth(",
        "TopologyPrimitiveConstructionQueryBirthSynopsis::new(",
        "topology_family_from_spatial_family(",
    ] {
        assert!(
            !ADMITTED_SCAFFOLD_ROOT.contains(forbidden),
            "admitted scaffold root should not own `{forbidden}` once topology-ready birth is localized",
        );
    }
    for required in [
        "TopologyPrimitiveConstructionQueryBirthSynopsis::new(",
        "topology_family_from_spatial_family(",
        "prepare_primitive_construction_query_admitted_handoff_from_synopsis(",
    ] {
        assert!(
            TOPOLOGY_READY_BIRTH.contains(required),
            "topology-ready birth seam should own `{required}`",
        );
    }
}
