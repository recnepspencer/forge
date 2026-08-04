//! Vendored 2D incircle family.

mod adaptive;
mod direct;
mod evaluation;

pub(in crate::predicates) use adaptive::incircleadapt;
pub(in crate::predicates) use direct::{incircle_exact, incircle_fast, incircle_slow};
pub(in crate::predicates) use evaluation::incircle;
