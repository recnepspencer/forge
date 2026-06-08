//! Euler formula and genus invariant validators.
//!
//! DOMAIN: Classic and generalized Euler formula verification,
//! genus computation consistency, and per-component Euler checks.
//!
//! STRUCTURE:
//!   euler_formula.rs — Generalized Euler formula (V−E+F = 2−2G+R) + genus computation
//!   per_component_euler.rs — Euler formula validation per topological component

mod euler_formula;
mod per_component_euler;

pub(crate) use per_component_euler::validate_per_component_euler;
