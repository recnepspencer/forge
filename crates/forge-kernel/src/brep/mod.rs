//! DOMAIN: B-Rep (Boundary Representation)
//!
//! Home for true NURBS B-rep parametric curves, surfaces, and solid abstractions.

pub mod state;
pub mod patch;

pub use state::BrepState;
pub use patch::BrepPatch;
