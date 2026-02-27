//! DOMAIN: B-Rep (Boundary Representation)
//!
//! Home for true NURBS B-rep parametric curves, surfaces, and solid abstractions.

pub mod patch;
pub mod state;

pub use patch::BrepPatch;
pub use state::BrepState;
