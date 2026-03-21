use super::*;

pub(crate) fn create_branch_from_main(
    runtime: &mut RelationalRuntime,
    branch_name: &str,
) -> BranchId {
    let branch_id = BranchId(branch_name.to_string());
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .unwrap();
    branch_id
}

pub(crate) fn checkpoint_and_recover_with<F>(
    runtime: &mut RelationalRuntime,
    recovered_factory: F,
) -> (
    crate::logic::runtime::RecoveryOutcome,
    RelationalRuntime,
)
where
    F: FnOnce() -> RelationalRuntime,
{
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = recovered_factory();
    let outcome = recovered.durability_authority().recover(plan).unwrap();
    (outcome, recovered)
}
