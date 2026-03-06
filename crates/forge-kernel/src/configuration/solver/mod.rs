//! Solver vertical slice.
//!
//! DOMAIN: Configuration for iterative numeric solvers — section data,
//! default constants, and overrides.

mod solver_override;
mod solver_section;

pub use solver_override::SolverOverride;
pub use solver_section::SolverSection;
