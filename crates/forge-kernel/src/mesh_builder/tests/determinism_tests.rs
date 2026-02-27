//! Determinism tests — same inputs must produce identical topology hash.

use forge_topo::hashing::compute_arena_topology_hash;
use crate::mesh_builder::{make_cube, make_tetrahedron, make_dodecahedron};
use super::test_config;

#[test]
fn cube_deterministic_hash() {
    let cfg = test_config();
    let hashes: Vec<u128> = (0..10).map(|_| {
        compute_arena_topology_hash(make_cube([0.0; 3], 2.0, &cfg).unwrap().topology().arena())
    }).collect();
    for (i, h) in hashes.iter().enumerate() {
        assert_eq!(*h, hashes[0], "Run {i}: hash {h:#x} != first {:#x}", hashes[0]);
    }
}

#[test]
fn tetrahedron_deterministic_hash() {
    let cfg = test_config();
    let hashes: Vec<u128> = (0..10).map(|_| {
        compute_arena_topology_hash(make_tetrahedron([0.0; 3], 1.0, &cfg).unwrap().topology().arena())
    }).collect();
    for (i, h) in hashes.iter().enumerate() {
        assert_eq!(*h, hashes[0], "Run {i}: hash {h:#x} != first {:#x}", hashes[0]);
    }
}

#[test]
fn dodecahedron_deterministic_hash() {
    let cfg = test_config();
    let hashes: Vec<u128> = (0..10).map(|_| {
        compute_arena_topology_hash(make_dodecahedron([0.0; 3], 1.0, &cfg).unwrap().topology().arena())
    }).collect();
    for (i, h) in hashes.iter().enumerate() {
        assert_eq!(*h, hashes[0], "Run {i}: hash {h:#x} != first {:#x}", hashes[0]);
    }
}
