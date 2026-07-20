use std::collections::BTreeSet;

use worth_store_operations::{
    derive_operational_audit_records, AuditCompletenessReceipt, ExpectedAuditTransitionSet,
    SelectedOperationalControlState,
};
use worth_store_physical_certification::{
    DrivenOperationalTransition, OperationalRecoveryProductionDriver,
};

pub(super) fn derive_audits(
    driver: &OperationalRecoveryProductionDriver,
    selected: &SelectedOperationalControlState,
) -> Vec<AuditCompletenessReceipt> {
    let records = selected.durable_records();
    let audit_records = completed(driver.derive_audit(records).unwrap());
    let operations = records
        .iter()
        .map(|record| record.operation_id().clone())
        .collect::<BTreeSet<_>>();
    let receipts = operations
        .into_iter()
        .map(|operation| {
            ExpectedAuditTransitionSet::from_durable_control_records(operation, records)
                .unwrap()
                .verify(&audit_records)
                .unwrap()
        })
        .collect::<Vec<_>>();
    let _ = completed(driver.export_audit(&receipts[0], &audit_records).unwrap());
    assert_eq!(
        audit_records,
        derive_operational_audit_records(records).unwrap()
    );
    receipts
}

fn completed<T: std::fmt::Debug>(transition: DrivenOperationalTransition<T>) -> T {
    match transition {
        DrivenOperationalTransition::Completed(value) => value,
        other => panic!("uninterrupted driver returned {other:?}"),
    }
}
