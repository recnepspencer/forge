#[test]
fn spatial_facade_is_namespaced_and_no_longer_flat() {
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));
    let lib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

    assert!(!facade.contains("pub use crate::bindings::{"));
    assert!(!facade.contains("pub use crate::spatial_intent::{"));
    assert!(!facade.contains("pub use crate::test_support::SpatialFixtureWitnessCatalog"));
    assert!(facade.contains("pub mod bindings;"));
    assert!(facade.contains("pub mod frames;"));
    assert!(facade.contains("pub mod placement;"));
    assert!(facade.contains("pub mod motion;"));
    assert!(facade.contains("pub mod constraints;"));
    assert!(facade.contains("pub mod lowering;"));
    assert!(facade.contains("pub mod arbitration;"));
    assert!(!lib.contains("pub mod test_support;"));
}
