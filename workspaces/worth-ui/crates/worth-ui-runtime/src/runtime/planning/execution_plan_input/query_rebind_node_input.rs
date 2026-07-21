use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiQueryBindingPosture, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome, WorthUiQueryRebindRequiredSurface,
};

pub(super) fn transition(entry: &WorthUiQueryLiveRebindEntry) -> WorthUiNodeLifecycleTransition {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(_) => WorthUiNodeLifecycleTransition::Preserve,
        WorthUiQueryLiveRebindOutcome::Rebind(rebind)
            if rebind.reason()
                == crate::runtime::WorthUiQueryBindingRebindReason::FreshCandidateBinding =>
        {
            WorthUiNodeLifecycleTransition::Create
        }
        WorthUiQueryLiveRebindOutcome::Rebind(_) => WorthUiNodeLifecycleTransition::Rebind,
        WorthUiQueryLiveRebindOutcome::Retire(_) | WorthUiQueryLiveRebindOutcome::Deny(_) => {
            WorthUiNodeLifecycleTransition::Drop
        }
    }
}

pub(super) fn posture(entry: &WorthUiQueryLiveRebindEntry) -> Option<WorthUiQueryBindingPosture> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(value) => Some(value.preserved_posture().clone()),
        WorthUiQueryLiveRebindOutcome::Rebind(value) => Some(value.candidate_posture().clone()),
        WorthUiQueryLiveRebindOutcome::Retire(value) => Some(value.active_posture().clone()),
        WorthUiQueryLiveRebindOutcome::Deny(value) => value
            .candidate_posture()
            .or_else(|| value.active_posture())
            .cloned(),
    }
}

pub(super) fn required_surfaces(
    entry: &WorthUiQueryLiveRebindEntry,
) -> Vec<WorthUiQueryRebindRequiredSurface> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Rebind(value) => value.required_query_surfaces().to_vec(),
        _ => Vec::new(),
    }
}

pub(super) fn preservation_receipt(
    entry: &WorthUiQueryLiveRebindEntry,
) -> Option<crate::runtime::WorthUiQueryBindingPreservationReceipt> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(value) => Some(value.preservation_receipt()),
        _ => None,
    }
}
