use crate::facade::BranchId;

use super::super::fixture::FintechWorld;

pub(crate) fn open_analysis_branch(world: &mut FintechWorld) -> BranchId {
    world.create_analysis_branch()
}

pub(crate) fn open_audit_branch(world: &mut FintechWorld) -> BranchId {
    let branch = BranchId("audit".to_string());
    world
        .runtime
        .create_branch(branch.clone(), &BranchId("main".to_string()))
        .unwrap();
    branch
}
