#[test]
fn spatial_facade_is_namespaced_and_no_longer_flat() {
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));
    let lib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

    assert!(!facade.contains("pub use crate::bindings::{"));
    assert!(!facade.contains("pub use crate::spatial_domain::{"));
    assert!(!facade.contains("pub use crate::test_support::SpatialFixtureWitnessCatalog"));
    assert!(facade.contains("pub mod anchor_binding;"));
    assert!(facade.contains("pub mod binding;"));
    assert!(facade.contains("pub mod bindings;"));
    assert!(facade.contains("pub mod continuation;"));
    assert!(facade.contains("pub mod inspection;"));
    assert!(facade.contains("pub mod neighborhood;"));
    assert!(facade.contains("pub mod planar_contracts;"));
    assert!(facade.contains("pub mod planar_m6_closeout;"));
    assert!(facade.contains("pub mod planar_overlap;"));
    assert!(facade.contains("pub mod planar_signed_area;"));
    assert!(facade.contains("pub mod planar_predicates;"));
    assert!(facade.contains("pub mod placement;"));
    assert!(facade.contains("pub mod projection;"));
    assert!(facade.contains("pub mod rebinding;"));
    assert!(facade.contains("pub mod recovery;"));
    assert!(facade.contains("pub mod support;"));
    assert!(facade.contains("pub mod tolerance;"));
    assert!(facade.contains("pub mod workload_inventory;"));
    assert!(facade.contains("pub mod workload_operators;"));
    assert!(!facade.contains("pub mod policy;"));
    assert!(!facade.contains("pub mod birth;"));
    assert!(!facade.contains("pub mod motion;"));
    assert!(!facade.contains("pub mod constraints;"));
    assert!(!facade.contains("pub mod arbitration;"));
    assert!(!facade.contains("pub mod witness_resolution;"));
    assert!(!lib.contains("mod spatial_domain;"));
    assert!(!lib.contains("pub mod test_support;"));
    assert!(lib.contains("pub mod certification;"));
    assert!(lib.contains("pub mod facade;"));
}
