//! Constraint-set admission orchestrator — delegates to the named pipeline lane.

pub(super) use super::constraint_pipeline::{
    admit_constraint_basis, admit_constraint_basis_with_portal,
};
#[cfg(test)]
pub(super) use super::constraint_pipeline::{
    admit_constraint_set, admit_constraint_set_with_portal,
};
