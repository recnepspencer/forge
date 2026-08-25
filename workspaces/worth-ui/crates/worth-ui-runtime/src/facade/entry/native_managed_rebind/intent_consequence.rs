use super::{
    WorthUiNativeManagedRebindProgress, WorthUiNativeManagedRebindStop,
    WorthUiNativePendingManagedRebind,
};

pub(in crate::facade::entry) enum ManagedIntentConsequenceNormalization {
    NoConsequences(crate::runtime::intent_execution::UiIntentConsequenceCompletionReceipt),
    Published(crate::runtime::rebind::UiRebindReceipt),
    Pending(
        crate::facade::entry::intent_consequence_publication::DetachedUiIntentConsequenceInFlight,
    ),
    Stopped(WorthUiNativeManagedRebindStop),
}

pub(in crate::facade::entry) fn normalize_managed_intent_consequence(
    outcome: crate::facade::entry::UiIntentConsequencePublicationOutcome<'_>,
) -> ManagedIntentConsequenceNormalization {
    use crate::facade::entry::UiIntentConsequencePublicationOutcome as Outcome;
    match outcome {
        Outcome::NoConsequences(receipt) => {
            ManagedIntentConsequenceNormalization::NoConsequences(receipt)
        }
        Outcome::Published(receipt) => ManagedIntentConsequenceNormalization::Published(receipt),
        Outcome::InFlight(completion) => {
            ManagedIntentConsequenceNormalization::Pending(completion.detach_for_native())
        }
        Outcome::Stopped(stop) => {
            let (reason, recovery) = stop.into_parts();
            drop(recovery);
            ManagedIntentConsequenceNormalization::Stopped(
                WorthUiNativeManagedRebindStop::IntentConsequence(reason),
            )
        }
        Outcome::Indeterminate(recovery) => {
            let _ = recovery.into_session_for_shutdown();
            ManagedIntentConsequenceNormalization::Stopped(
                WorthUiNativeManagedRebindStop::Indeterminate,
            )
        }
        Outcome::InternalDefect(defect) => ManagedIntentConsequenceNormalization::Stopped(
            WorthUiNativeManagedRebindStop::InternalDefect(defect.kind()),
        ),
    }
}

pub(super) fn finish(
    pending: &mut Option<WorthUiNativePendingManagedRebind>,
    outcome: crate::facade::entry::UiIntentConsequencePublicationOutcome<'_>,
) -> WorthUiNativeManagedRebindProgress {
    match normalize_managed_intent_consequence(outcome) {
        ManagedIntentConsequenceNormalization::Published(receipt) => {
            WorthUiNativeManagedRebindProgress::Published(receipt)
        }
        ManagedIntentConsequenceNormalization::Pending(completion) => {
            *pending = Some(WorthUiNativePendingManagedRebind::IntentConsequence(
                completion,
            ));
            WorthUiNativeManagedRebindProgress::AwaitingProgress
        }
        ManagedIntentConsequenceNormalization::Stopped(stop) => {
            WorthUiNativeManagedRebindProgress::Stopped(stop)
        }
        ManagedIntentConsequenceNormalization::NoConsequences(_) => {
            unreachable!("admitted consequence publication cannot become consequence-free")
        }
    }
}
