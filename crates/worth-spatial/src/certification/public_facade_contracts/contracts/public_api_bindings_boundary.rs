#[test]
fn spatial_bindings_boundary_no_longer_teaches_parallel_report_ecologies() {
    let bindings_mod = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings/mod.rs"));
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));

    assert!(!bindings_mod.contains("mod authority;"));
    assert!(!bindings_mod.contains("mod primitive_birth_completeness;"));
    assert!(!bindings_mod.contains("mod primitive_birth_mapping;"));
    assert!(!bindings_mod.contains("mod primitive_birth_rejection;"));
    assert!(!facade.contains("construction_birth_authority"));
    assert!(!facade.contains("certify_primitive_construction_birth_completeness"));
    assert!(!facade.contains("build_primitive_construction_birth_mapping_report"));
    assert!(!facade.contains("impossible_primitive_construction_birth_attachment"));
    assert!(!facade.contains("SpatialConstructionBirthAuthority"));
    assert!(!facade.contains("SpatialConstructionBirthCompletenessReport"));
    assert!(!facade.contains("SpatialConstructionBirthMappingReport"));
    assert!(!facade.contains("SpatialConstructionBirthRejectionRow"));
    assert!(!facade.contains("PrimitiveConstructionBirthContractCounts"));
    assert!(facade.contains("pub mod bindings;"));
}
