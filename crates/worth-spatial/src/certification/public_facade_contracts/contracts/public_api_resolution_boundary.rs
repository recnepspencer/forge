#[test]
fn spatial_resolution_boundary_no_longer_teaches_witness_materialization_seams() {
    let resolution_mod = include_str!("../../../witness_resolution/mod.rs");
    let lib = include_str!("../../../lib.rs");
    let facade = include_str!("../../../facade/mod.rs");

    assert!(!resolution_mod.contains("mod materialization;"));
    assert!(!resolution_mod.contains("mod materialization_support;"));
    assert!(!resolution_mod.contains("mod materialization_vocab;"));
    assert!(!lib.contains("materialize_spatial_point_witness_support_report"));
    assert!(!lib.contains("materialize_spatial_direction_witness_support_report"));
    assert!(!facade.contains("materialize_spatial_point_witness_support_report"));
    assert!(!facade.contains("materialize_spatial_direction_witness_support_report"));
    assert!(!facade.contains("SpatialWitnessMaterializationProfilePlan"));
}

#[test]
fn spatial_resolution_boundary_separates_public_helper_entry_from_resolution_kernel() {
    let resolution_mod = include_str!("../../../witness_resolution/mod.rs");
    let helper_entry = include_str!("../../../witness_resolution/witness_helper_entry.rs");
    let resolution = include_str!("../../../witness_resolution/resolution.rs");
    let lib = include_str!("../../../lib.rs");
    let facade = include_str!("../../../facade/mod.rs");
    let certification_file = include_str!("../../../certification.rs");

    assert!(resolution_mod.contains("mod witness_helper_entry;"));
    assert!(resolution_mod.contains("pub(crate) mod witness_resolution {"));
    assert!(helper_entry.contains("pub(crate) fn resolve_spatial_point_witness("));
    assert!(helper_entry.contains("pub(crate) fn resolve_spatial_direction_witness("));
    assert!(!facade.contains("pub mod witness_resolution;"));
    assert!(facade.contains("pub mod anchor_selection;"));
    assert!(!certification_file.contains("pub mod support;"));
    assert!(!lib.contains("mod spatial_domain;"));
    assert!(!facade.contains("AdmittedSpatialFrameRef, ResolvedSpatialDirectionWitness,"));
    assert!(!facade.contains("SpatialFrameError, SpatialWitnessFailureClass,"));
    assert!(resolution.contains("pub(crate) fn resolve_admitted_spatial_point_witness_request("));
    assert!(
        resolution.contains("pub(crate) fn resolve_admitted_spatial_direction_witness_request(")
    );
    assert!(!resolution.contains("pub fn resolve_spatial_point_witness("));
    assert!(!resolution.contains("pub fn resolve_spatial_direction_witness("));
}
