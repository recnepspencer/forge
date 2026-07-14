use super::LsmExecutionOwnerCaseDeclaration;

pub fn lsm_execution_owner_case_inventory() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration>
{
    super::compaction::owner_cases().chain(super::replay::owner_cases())
}
