use worth_store_layout_indexes::{
    access_shapes, AccessLaneClassification, FullDeclaredScanBasis, ObserveOwnerCase,
};

use super::super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    for lane in [
        AccessLaneClassification::Verifier,
        AccessLaneClassification::Foreground,
    ] {
        let outcome =
            access_shapes().full_declared_scan(lane, FullDeclaredScanBasis::DeclaredFullTraversal);
        ledger.record_full_declared_scan(outcome.owner_case_observation());
    }
}
