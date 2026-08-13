use super::super::LayoutOwnerObservationLedger;

pub(in crate::courtroom::layout::owner_scenarios) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) {
    super::classification::record(ledger);
    super::quarantine::record(ledger);
    super::import::record(ledger);
}
