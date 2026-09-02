mod admission;
mod composite;
mod equivalence;

#[cfg(test)]
pub(crate) use admission::admit_current;
pub use admission::AdmittedCompositeRuntimeWorldBasis;
pub(crate) use equivalence::compare_exact;
