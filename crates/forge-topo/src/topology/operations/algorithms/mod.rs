//! Non-Euler topology algorithms.
//!
//! These operations use the Euler operator primitives but are not
//! themselves classical Euler operators. They are compound algorithms
//! built from the Euler operator primitives.
//!
//! DEPENDENCIES: `euler` (operators), `arena` (entity storage)

pub mod bridge_edge;
pub mod flip_edge;
pub mod triangulate;
pub mod extract_shell;
