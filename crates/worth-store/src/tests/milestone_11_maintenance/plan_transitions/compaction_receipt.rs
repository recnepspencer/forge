use super::super::*;

pub(super) fn admitted_compaction(
    receipt: &crate::MaintenanceAdmissionReceipt,
) -> crate::AdmittedMaintenanceDeclaration {
    receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("compaction declaration")
        .clone()
}

pub(super) fn equivalent_compaction_pair(
    receipt: &crate::MaintenanceAdmissionReceipt,
    duplicate_id: &str,
) -> (
    crate::AdmittedMaintenanceDeclaration,
    crate::AdmittedMaintenanceDeclaration,
) {
    let duplicate = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| declaration.declaration().id().as_str() == duplicate_id)
        .expect("duplicate compaction declaration should exist")
        .clone();
    let leader = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            declaration.declaration().id() != duplicate.declaration().id()
                && declaration.descriptor().equivalence_key()
                    == duplicate.descriptor().equivalence_key()
        })
        .expect("leader compaction declaration should exist")
        .clone();
    (leader, duplicate)
}
