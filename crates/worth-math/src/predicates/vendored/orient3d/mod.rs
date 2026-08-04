//! Vendored 3D orientation family.

mod adaptive;
mod direct;
mod evaluation;

pub(in crate::predicates) use adaptive::orient3dadapt;
pub(in crate::predicates) use direct::{orient3d_exact, orient3d_fast, orient3d_slow};
pub(in crate::predicates) use evaluation::orient3d;
