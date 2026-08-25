use super::{
    WorthUiNativeManagedRebindProgress, WorthUiNativeManagedRebindStop,
    WorthUiNativePendingManagedRebind,
};

pub(in crate::facade::entry) enum ManagedIntentPostureNormalization {
    Published(crate::runtime::rebind::UiRebindReceipt),
    Pending(crate::facade::entry::native_intent_posture::DetachedNativeIntentPostureInFlight),
    Stopped(WorthUiNativeManagedRebindStop),
}

pub(in crate::facade::entry) fn normalize_managed_intent_posture(
    outcome: crate::facade::entry::native_intent_posture::
        WorthUiNativeIntentPosturePublicationOutcome<'_>,
) -> ManagedIntentPostureNormalization {
    use crate::facade::entry::native_intent_posture::WorthUiNativeIntentPosturePublicationOutcome as Outcome;
    match outcome {
        Outcome::Published(receipt) => ManagedIntentPostureNormalization::Published(receipt),
        Outcome::InFlight(completion) => {
            ManagedIntentPostureNormalization::Pending(completion.detach_for_native())
        }
        Outcome::RejectedBeforeEffects(retry) => ManagedIntentPostureNormalization::Stopped(
            WorthUiNativeManagedRebindStop::IntentPosture(retry.into_stop()),
        ),
        Outcome::Stopped(stop) => ManagedIntentPostureNormalization::Stopped(
            WorthUiNativeManagedRebindStop::IntentPosture(stop),
        ),
        Outcome::Indeterminate(recovery) => {
            let _ = recovery.into_session_for_shutdown();
            ManagedIntentPostureNormalization::Stopped(
                WorthUiNativeManagedRebindStop::Indeterminate,
            )
        }
        Outcome::InternalDefect(defect) => ManagedIntentPostureNormalization::Stopped(
            WorthUiNativeManagedRebindStop::InternalDefect(defect.kind()),
        ),
    }
}

pub(super) fn retry_progressed_text_atlas_deferral<'session>(
    outcome: crate::facade::entry::native_intent_posture::
        WorthUiNativeIntentPosturePublicationOutcome<'session>,
    now_tick: u64,
) -> crate::facade::entry::native_intent_posture::WorthUiNativeIntentPosturePublicationOutcome<
    'session,
> {
    use crate::facade::entry::native_intent_posture::WorthUiNativeIntentPosturePublicationOutcome as Outcome;
    let Outcome::RejectedBeforeEffects(retry) = outcome else {
        return outcome;
    };
    if !retry.rejections().is_empty()
        && retry.rejections().iter().all(|rejection| {
            rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::
                    TextAtlasPresentationDeferred
        })
    {
        retry.retry(now_tick)
    } else {
        Outcome::RejectedBeforeEffects(retry)
    }
}

pub(super) fn finish(
    pending: &mut Option<WorthUiNativePendingManagedRebind>,
    outcome: crate::facade::entry::native_intent_posture::
        WorthUiNativeIntentPosturePublicationOutcome<'_>,
) -> WorthUiNativeManagedRebindProgress {
    match normalize_managed_intent_posture(outcome) {
        ManagedIntentPostureNormalization::Published(receipt) => {
            WorthUiNativeManagedRebindProgress::Published(receipt)
        }
        ManagedIntentPostureNormalization::Pending(completion) => {
            *pending = Some(WorthUiNativePendingManagedRebind::IntentPosture(completion));
            WorthUiNativeManagedRebindProgress::AwaitingProgress
        }
        ManagedIntentPostureNormalization::Stopped(stop) => {
            WorthUiNativeManagedRebindProgress::Stopped(stop)
        }
    }
}
