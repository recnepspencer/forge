#[test]
fn spatial_refs_boundary_exports_authored_reference_vocabulary_through_one_namespace() {
    let facade = include_str!("../../../facade/mod.rs");
    let lib = include_str!("../../../lib.rs");
    let refs = include_str!("../../../facade/refs.rs");

    assert!(facade.contains("pub mod refs;"));
    assert!(!facade.contains("pub mod witness_catalog;"));
    assert!(!facade.contains("pub mod witness_resolution;"));
    assert!(!lib.contains("mod spatial_domain;"));
    assert!(!facade.contains("SpatialAnchorMatchConstraintSpec, SpatialAnchorRef,"));
    assert!(!facade.contains("SpatialChosenIntentResolution, SpatialAnchorRef,"));
    assert!(!facade.contains("SpatialObservedRelationFact, SpatialDirectionWitnessRef,"));
    assert!(refs.contains("EmptySpatialWitnessCatalog"));
    assert!(refs.contains("SpatialWitnessCatalog"));
    assert!(refs.contains("SpatialAnchorRef"));
    assert!(refs.contains("SpatialFrameRef"));
}
