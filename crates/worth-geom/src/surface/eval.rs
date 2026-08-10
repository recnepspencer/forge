//! Surface evaluation and analytic surface-pair arbitration.
//!
//! DOMAIN: Public routing for pure parametric surface evaluation and analytic
//! surface-pair relation classification.
//!
//! DEPENDENCIES: surface schema types and local ambiguity-policy support.

mod analytic_pair_classifiers;
mod geometry;
mod normal_evaluation;
mod point_evaluation;
mod surface_pair_relations;

#[cfg(test)]
mod tests;

pub use surface_pair_relations::classify_surface_pair;
