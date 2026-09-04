use super::ProductUnpublishedNextAction;
use crate::publication::{
    CompositeAttemptProgress, RelationalAttemptProgressPosture, SignalAttemptProgressPosture,
};

/// The bound is the exact number of distinct actions a retained record can
/// legally offer, so a retained list is never silently truncated.
const RETAINED_NEXT_ACTION_CAPACITY: usize = 5;

#[derive(Debug)]
pub(crate) struct RetainedNextActions {
    actions: [ProductUnpublishedNextAction; RETAINED_NEXT_ACTION_CAPACITY],
    length: u8,
}

impl RetainedNextActions {
    pub(crate) fn from_vec(actions: Vec<ProductUnpublishedNextAction>) -> Self {
        assert!(
            actions.len() <= RETAINED_NEXT_ACTION_CAPACITY,
            "retained recovery actions exceed the fixed contract bound"
        );
        let mut retained = Self {
            actions: [ProductUnpublishedNextAction::Inspect; RETAINED_NEXT_ACTION_CAPACITY],
            length: actions.len() as u8,
        };
        retained.actions[..actions.len()].copy_from_slice(&actions);
        retained
    }

    pub(crate) fn as_slice(&self) -> &[ProductUnpublishedNextAction] {
        &self.actions[..usize::from(self.length)]
    }
}

/// The single authority for what a retained owner-effect record permits next.
/// Every retained record, recovery custody transition, and close report derives
/// its list here, so two records that retain the same owner progress can never
/// advertise different continuations. The derivation is total: every
/// `CompositeAttemptProgress` shape yields a list, and the list never exceeds
/// the fixed contract bound.
pub(crate) fn next_actions_for_progress(
    progress: &CompositeAttemptProgress,
) -> Vec<ProductUnpublishedNextAction> {
    let mut actions = Vec::with_capacity(RETAINED_NEXT_ACTION_CAPACITY);
    if settlement_is_owed(progress) {
        actions.push(ProductUnpublishedNextAction::SettleOwnerEffects);
    }
    if publication_evidence_is_settled(progress) {
        actions.push(ProductUnpublishedNextAction::StartFreshCompositePublication);
    }
    actions.push(ProductUnpublishedNextAction::ReleaseObligations);
    actions.push(ProductUnpublishedNextAction::Inspect);
    assert!(
        actions.len() <= RETAINED_NEXT_ACTION_CAPACITY,
        "retained recovery actions exceed the fixed contract bound"
    );
    actions
}

/// A record owes settlement while its Relational commit is performed but not
/// settled and no Signal movement has been recorded against it.
fn settlement_is_owed(progress: &CompositeAttemptProgress) -> bool {
    progress.relational_requires_settlement()
        && progress.signal_posture() == SignalAttemptProgressPosture::Untouched
}

/// A fresh composite publication may start only once the retained record's
/// publication evidence is settled: the Relational commit named by the record
/// is durable and owes this world nothing further, so a new Query-admitted
/// operation can name it as its basis. `Performed` fork evidence is
/// branch-creation evidence rather than publication evidence and never permits
/// a fresh composite attempt on its own.
fn publication_evidence_is_settled(progress: &CompositeAttemptProgress) -> bool {
    progress.relational_posture() == RelationalAttemptProgressPosture::Settled
}
