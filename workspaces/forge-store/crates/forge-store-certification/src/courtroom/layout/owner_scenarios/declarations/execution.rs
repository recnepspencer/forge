use super::super::{LayoutOwnerObservationLedger, LayoutOwnerScenarioExecutionDenial};

pub(in crate::courtroom::layout::owner_scenarios) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) -> Result<(), LayoutOwnerScenarioExecutionDenial> {
    super::artifact_family::execute(ledger)?;
    super::key_domain::execute(ledger);
    super::bootstrap::execute(ledger);
    super::scan::execute(ledger);
    super::budget::execute(ledger);
    Ok(())
}
