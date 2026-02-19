//! Generators sub-module — random solid and Boolean pair generators.
//!
//! DOMAIN: Test infrastructure — deterministic random polyhedra generation.
//! DEPENDENCIES: `forge-geom` (BSP), `forge-kernel` (mesh builder, boolean schema)
//!
//! ## Contents
//!
//! - `Xorshift64` — Deterministic PRNG
//! - `planar` — Random convex solids, cubes, and Boolean pairs

mod planar;

pub use planar::{
    Xorshift64,
    random_convex_solid,
    build_cube_at,
    random_cube,
    random_cube_pair,
    random_convex_pair,
};
