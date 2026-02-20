//! DOMAIN: Side-car geometry storage for topology entities.
//!
//! Stores per-face planes and per-vertex positions alongside the
//! `TopologyArena`. The topology layer stores structure only (Architecture
//! Rule 2.3); this module bridges topology handles to geometric data.
//!
//! INVARIANTS:
//! - Every face in the topology should have a corresponding plane entry
//! - Every vertex in the topology should have a corresponding position entry
//! - Implements `GeometrySource` (from `forge-math`) for use by `forge-geom` solvers
//!
//! DEPENDENCIES: `forge-math` (GeometrySource), `forge-core` (KernelError),
//!               `forge-topo` (handles), `forge-geom` (Plane)

mod schema;
mod eval;
#[cfg(test)]
mod tests;

pub use schema::GeometryStore;
pub use schema::ExactPosition;
pub use eval::build_position_lookup;
