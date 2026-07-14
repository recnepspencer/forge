use super::LsmMaintenanceOwnerCaseDeclaration;

pub fn lsm_maintenance_owner_case_inventory(
) -> impl Iterator<Item = LsmMaintenanceOwnerCaseDeclaration> {
    super::run_publication::owner_cases()
        .chain(super::replay::owner_cases())
        .chain(super::compaction::owner_cases())
}
