use super::super::super::LayoutOwnerObservationLedger;

pub(in crate::courtroom::layout::owner_scenarios::maintenance) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) {
    super::admission::execute(ledger);
    super::parity::execute(ledger);
}
