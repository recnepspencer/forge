use crate::runtime::{
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationOutcome,
    WorthUiNodeLifecycleTransition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSelectionRangeReconciliation {
    requires_backing_collection_identity: bool,
    query_posture_required_on_rebind: bool,
}

impl WorthUiSelectionRangeReconciliation {
    pub fn backing_collection_identity() -> Self {
        Self {
            requires_backing_collection_identity: true,
            query_posture_required_on_rebind: true,
        }
    }

    pub fn requires_backing_collection_identity(&self) -> bool {
        self.requires_backing_collection_identity
    }

    pub fn query_posture_required_on_rebind(&self) -> bool {
        self.query_posture_required_on_rebind
    }

    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve | WorthUiNodeLifecycleTransition::Move
        )
    }

    pub(crate) fn replacement_outcome(
        transition: WorthUiNodeLifecycleTransition,
        counters: &mut WorthUiDurableStateReconciliationCounters,
    ) -> Option<(WorthUiDurableStateReconciliationOutcome, &'static str)> {
        if matches!(
            transition,
            WorthUiNodeLifecycleTransition::Rebind | WorthUiNodeLifecycleTransition::Replace
        ) {
            counters.record_query_posture_required();
            Some((
                WorthUiDurableStateReconciliationOutcome::Drop,
                "selection range requires admitted backing collection identity",
            ))
        } else {
            None
        }
    }
}
