mod operator_family_closure;
#[cfg(test)]
mod operator_family_closure_tests;
mod operator_family_closure_types;
mod primitive_family_closure;
mod primitive_family_closure_types;
mod primitive_family_wire_closure;

pub use operator_family_closure_types::MilestoneThreeOperatorFamilyClosureRow;
pub use primitive_family_closure_types::MilestoneThreePrimitiveFamilyClosureRow;

pub(in crate::certification::topology_operator_closeout) use operator_family_closure::{
    build_operator_family_closure_rows, ensure_operator_family_closure_rows,
    operator_family_closure_labels,
};
pub(in crate::certification::topology_operator_closeout) use primitive_family_closure::{
    certify_milestone_three_primitive_family_closure_impl, ensure_primitive_family_closure_rows,
    primitive_family_closure_labels,
};




