use std::collections::BTreeSet;

use forge_store_layout_indexes::{declarations::layout_declarations, ObserveOwnerCase};
use forge_store_test_support::{
    execute_security_scope_harness_scenario, security_scope_metadata_preserved_scenario,
};

use super::super::{LayoutOwnerObservationLedger, LayoutOwnerScenarioExecutionDenial};

pub(super) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) -> Result<(), LayoutOwnerScenarioExecutionDenial> {
    let security_execution =
        execute_security_scope_harness_scenario(security_scope_metadata_preserved_scenario());
    let security = security_execution
        .accepted_security_scope()
        .ok_or(LayoutOwnerScenarioExecutionDenial::CurrentSecurityScopeUnavailable)?;
    let mut observed = BTreeSet::new();

    for row in layout_declarations().artifact_families().rows() {
        let outcome = layout_declarations()
            .admit_physical_artifact_family(row.declaration(), security.witnesses());
        if observed.insert(outcome.case_id()) {
            ledger.record_artifact_family_admission(outcome.owner_case_observation());
        }
    }
    Ok(())
}
