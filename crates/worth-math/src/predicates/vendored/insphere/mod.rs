//! Vendored 3D insphere family.

mod adaptive;
mod evaluation;
mod exact;
mod fast;
mod slow;

pub(in crate::predicates) use adaptive::insphereadapt;
pub(in crate::predicates) use evaluation::insphere;
pub(in crate::predicates) use exact::insphere_exact;
pub(in crate::predicates) use fast::insphere_fast;
pub(in crate::predicates) use slow::insphere_slow;
