#[test]
fn spatial_refs_boundary_exports_refs_and_catalog_through_named_namespaces() {
    let facade = include_str!("../../../facade/mod.rs");
    let spatial_intent_mod = include_str!("../../../spatial_intent/mod.rs");

    assert!(facade.contains("pub mod refs;"));
    assert!(facade.contains("pub mod witness_catalog;"));
    assert!(facade.contains("pub mod witness_resolution;"));
    assert!(!spatial_intent_mod.contains("pub use refs::*;"));
    assert!(!facade.contains("SpatialAnchorMatchConstraintSpec, SpatialAnchorRef,"));
    assert!(!facade.contains("SpatialChosenIntentResolution, SpatialAnchorRef,"));
    assert!(!facade.contains("SpatialObservedRelationFact, SpatialDirectionWitnessRef,"));
    assert!(facade.contains("pub mod refs;"));
    assert!(facade.contains("pub mod witness_catalog;"));
}
