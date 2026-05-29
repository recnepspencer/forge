mod accepted_branch_authority_projection;
mod accepted_branch_execution;
mod accepted_branch_local;
mod accepted_branch_scenarios;
mod branch_local_acceptance;
mod branch_local_parity;

pub(in crate::certification::topology_operator_closeout) use branch_local_acceptance::ensure_branch_local_edit_parity_rows;
pub(in crate::certification::topology_operator_closeout) use branch_local_parity::certify_milestone_three_branch_local_edit_parity_impl;




