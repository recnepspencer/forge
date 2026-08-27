use super::{UiServiceProposalConflictDisposition, UiServiceProposalConflictPolicy};

#[test]
fn occupancy_contract_is_bounded_and_has_no_queue_outcome() {
    fn exhaustive(disposition: UiServiceProposalConflictDisposition) -> u8 {
        match disposition {
            UiServiceProposalConflictDisposition::Occupied => 1,
            UiServiceProposalConflictDisposition::Superseded => 2,
            UiServiceProposalConflictDisposition::Coalesced => 3,
            UiServiceProposalConflictDisposition::CancelledBeforeEffect => 4,
        }
    }
    assert_eq!(
        exhaustive(UiServiceProposalConflictDisposition::Occupied),
        1
    );
    assert_eq!(
        [
            UiServiceProposalConflictPolicy::RejectOccupied,
            UiServiceProposalConflictPolicy::SupersedeBeforeEffect,
            UiServiceProposalConflictPolicy::CoalesceExact,
            UiServiceProposalConflictPolicy::CancelBeforeEffect,
        ]
        .len(),
        4
    );
}
