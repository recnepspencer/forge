use super::ProductUnpublishedNextAction;

const RETAINED_NEXT_ACTION_CAPACITY: usize = 3;

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

pub(crate) fn next_actions_for_progress(
    progress: &crate::publication::CompositeAttemptProgress,
) -> Vec<ProductUnpublishedNextAction> {
    let mut actions = Vec::with_capacity(3);
    if progress.relational_requires_settlement()
        && progress.signal_posture() == crate::publication::SignalAttemptProgressPosture::Untouched
    {
        actions.push(ProductUnpublishedNextAction::SettleOwnerEffects);
    }
    actions.push(ProductUnpublishedNextAction::ReleaseObligations);
    actions.push(ProductUnpublishedNextAction::Inspect);
    actions
}
