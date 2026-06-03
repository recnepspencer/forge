#[test]
fn spatial_resolution_boundary_no_longer_teaches_witness_materialization_seams() {
    let resolution_mod = include_str!("../../../spatial_intent/resolution/mod.rs");
    let spatial_intent_mod = include_str!("../../../spatial_intent/mod.rs");
    let facade = include_str!("../../../facade/mod.rs");

    assert!(!resolution_mod.contains("mod materialization;"));
    assert!(!resolution_mod.contains("mod materialization_support;"));
    assert!(!resolution_mod.contains("mod materialization_vocab;"));
    assert!(!spatial_intent_mod.contains("materialize_spatial_point_witness_support_report"));
    assert!(!spatial_intent_mod.contains("materialize_spatial_direction_witness_support_report"));
    assert!(!facade.contains("materialize_spatial_point_witness_support_report"));
    assert!(!facade.contains("materialize_spatial_direction_witness_support_report"));
    assert!(!facade.contains("SpatialWitnessMaterializationProfilePlan"));
}

#[test]
fn spatial_resolution_boundary_separates_public_helper_entry_from_resolution_kernel() {
    let resolution_mod = include_str!("../../../spatial_intent/resolution/mod.rs");
    let helper_entry = include_str!("../../../spatial_intent/resolution/witness_helper_entry.rs");
    let resolution = include_str!("../../../spatial_intent/resolution/resolution.rs");
    let spatial_intent_mod = include_str!("../../../spatial_intent/mod.rs");
    let facade = include_str!("../../../facade/mod.rs");

    assert!(resolution_mod.contains("mod witness_helper_entry;"));
    assert!(resolution_mod.contains("pub(crate) mod witness_resolution {"));
    assert!(helper_entry.contains("pub fn resolve_spatial_point_witness("));
    assert!(helper_entry.contains("pub fn resolve_spatial_direction_witness("));
    assert!(facade.contains("pub mod witness_resolution;"));
    assert!(facade.contains("pub mod witness_resolution;"));
    assert!(!spatial_intent_mod.contains("pub use resolution::*;"));
    assert!(!spatial_intent_mod.contains("resolve_spatial_point_witness,"));
    assert!(!spatial_intent_mod.contains("resolve_spatial_direction_witness,"));
    assert!(!facade.contains("AdmittedSpatialFrameRef, ResolvedSpatialDirectionWitness,"));
    assert!(!facade.contains("SpatialFrameError, SpatialWitnessFailureClass,"));
    assert!(resolution.contains("pub(crate) fn resolve_admitted_spatial_point_witness_request("));
    assert!(
        resolution.contains("pub(crate) fn resolve_admitted_spatial_direction_witness_request(")
    );
    assert!(!resolution.contains("pub fn resolve_spatial_point_witness("));
    assert!(!resolution.contains("pub fn resolve_spatial_direction_witness("));
}
