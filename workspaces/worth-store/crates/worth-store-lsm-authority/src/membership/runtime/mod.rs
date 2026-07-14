mod compaction_selection;
mod persistence;
mod published_lookup;
mod reopen;
mod replacement;
mod state;

pub use compaction_selection::{
    select_lsm_compaction_membership, LsmMembershipSelectionOutcome, LsmMembershipSelectionView,
};
pub use persistence::{
    persist_lsm_membership_record, LsmMembershipPersistOutcome, LsmMembershipPersistView,
};
pub use published_lookup::{
    lookup_published_lsm_membership, LsmPublishedMembershipLookupOutcome,
    LsmPublishedMembershipLookupView,
};
pub use reopen::{
    open_lsm_membership, reopen_lsm_membership_from_store, LsmMembershipOpenOutcome,
    LsmMembershipOpenView,
};
pub use replacement::{
    admit_lsm_membership_replacement, admit_lsm_replacement_output,
    prepare_lsm_membership_activation, replace_lsm_membership, AdmittedLsmMembershipReplacement,
    AdmittedLsmReplacementOutput, LsmMembershipActivationDeclaration,
    LsmMembershipReplacementOutcome, LsmMembershipReplacementView, PublishedLsmMembershipIdentity,
    PublishedLsmMembershipReplacement,
};
pub use state::{
    LsmMembershipDenial, LsmMembershipReopenCounters, LsmMembershipReplayPosture,
    LsmMembershipSession,
};

pub(super) fn owner_cases() -> impl Iterator<Item = super::LsmMembershipOwnerCaseDeclaration> {
    reopen::owner_cases()
        .chain(persistence::owner_cases())
        .chain(compaction_selection::owner_cases())
        .chain(replacement::owner_cases())
        .chain(published_lookup::owner_cases())
}
