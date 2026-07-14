use super::super::LayoutOwnerObservationLedger;

pub(in crate::courtroom::layout::owner_scenarios) fn execute(
    ledger: &mut LayoutOwnerObservationLedger,
) {
    super::posture::execute(ledger);
    super::mutation::execute(ledger);
    super::lsm::execute(ledger);
    super::rebuild::execute(ledger);
}
