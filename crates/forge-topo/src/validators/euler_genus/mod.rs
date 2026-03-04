//! Euler formula and genus invariant validators.
//!
//! DOMAIN: Classic and generalized Euler formula verification,
//! genus computation consistency, and per-component Euler checks.
//!
//! STRUCTURE:
//!   euler_formula.rs — Generalized Euler formula (V−E+F = 2−2G+R) + genus computation

mod euler_formula;

pub(crate) use euler_formula::validate_euler;
pub(crate) use euler_formula::compute_shell_genus;
