mod binding;
mod binding_state;
mod contract;
mod identity;
mod ingress;
mod lifecycle;
mod model;
mod mutation;
mod recipient_affinity;
mod stop;
mod transition;

#[cfg(test)]
mod focused_submit_tests;
#[cfg(test)]
mod tests;

pub use binding::{
    UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop,
    UiLocalInputRecipientBindingStopReason,
};
pub(crate) use contract::UiLocalInputRecipientContractKind;
pub use contract::{
    UiDraftByteBudget, UiDraftByteBudgetDenial, UiDraftRecipientContractDenial,
    UiLocalInputRecipientContract, UiLocalInputRecipientFamily, UI_DRAFT_SESSION_LIMIT,
    UI_DRAFT_UTF8_BYTE_LIMIT,
};
pub use identity::{UiDraftFieldIdentity, UiDraftSessionIdentity};
pub use stop::{UiLocalInputStop, UiLocalInputStopReason};
pub use transition::{
    UiDraftMutationKind, UiDraftMutationReceipt, UiLocalInputRecipientBindingReceipt,
};

pub(crate) use model::{UiDraftProcessingOutcome, UiDraftRuntimeState, UiDraftStateSnapshot};
pub(crate) use recipient_affinity::UiLocalInputRecipientBindingContext;
