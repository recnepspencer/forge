use crate::runtime::interaction::{
    UiDraftByteBudget, UiDraftFieldIdentity, UiLocalInputRecipientContract,
};

/// Privileged world-compiler input for exercising the production draft owner
/// before Phase 3's declaration compiler becomes the ordinary minter.
pub fn draft_recipient_contract_for_certification(
    declared_field_slot: u16,
    budget: UiDraftByteBudget,
) -> UiLocalInputRecipientContract {
    UiLocalInputRecipientContract::draft(
        UiDraftFieldIdentity::from_declared_slot(declared_field_slot),
        budget,
    )
}
