//! Constraint-set admission orchestrator — delegates to the named pipeline lane.

#[cfg(test)]
pub(super) use super::constraint_pipeline::admit_constraint_set;
pub(super) use super::constraint_pipeline::{
    admit_constraint_basis, admit_constraint_basis_with_portal,
};
