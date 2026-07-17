use std::collections::BTreeMap;

use worth_store_operations::{
    derive_operational_audit_records, AuditCompletenessReceipt, ExpectedAuditTransitionSet,
};

use super::{S10ScenarioCertificationDenial, S10ScenarioProductionEvidence};

pub(super) fn require_audits_from_control_history(
    production: S10ScenarioProductionEvidence<'_>,
    provided: &[AuditCompletenessReceipt],
) -> Result<(), S10ScenarioCertificationDenial> {
    let records = derive_operational_audit_records(production.control_records())
        .map_err(|_| S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory)?;
    let mut by_operation = BTreeMap::new();
    for receipt in provided {
        if by_operation
            .insert(receipt.operation_id().as_str(), receipt)
            .is_some()
        {
            return Err(S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory);
        }
    }
    let operations = production
        .control_records()
        .iter()
        .map(|record| record.operation_id())
        .collect::<std::collections::BTreeSet<_>>();
    if operations.len() != by_operation.len() {
        return Err(S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory);
    }
    for operation in operations {
        let provided = by_operation
            .get(operation.as_str())
            .ok_or(S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory)?;
        let expected = ExpectedAuditTransitionSet::from_durable_control_records(
            operation.clone(),
            production.control_records(),
        )
        .map_err(|_| S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory)?;
        let derived = expected.verify(&records).map_err(|_| {
            S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory
        })?;
        if &derived != *provided {
            return Err(S10ScenarioCertificationDenial::AuditNotDerivedFromScenarioControlHistory);
        }
    }
    Ok(())
}
