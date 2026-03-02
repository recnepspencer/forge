//! Solver vertical slice.
//!
//! DOMAIN: Configuration for iterative numeric solvers — section data,
//! default constants, and overrides.

mod solver_section;
mod solver_override;

pub use solver_section::SolverSection;
pub use solver_override::SolverOverride;
