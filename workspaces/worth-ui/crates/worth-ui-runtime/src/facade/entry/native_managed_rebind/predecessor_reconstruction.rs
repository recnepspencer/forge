pub(super) enum RequiredPredecessorReconstruction<'session> {
    NotRequired(crate::runtime::rebind::UiRebindOutcome<'session>),
    Required(crate::runtime::rebind::UiDetachedRebindRetry),
}

pub(super) fn detach_required_predecessor_reconstruction<'session>(
    outcome: crate::runtime::rebind::UiRebindOutcome<'session>,
) -> RequiredPredecessorReconstruction<'session> {
    let crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial) = outcome else {
        return RequiredPredecessorReconstruction::NotRequired(outcome);
    };
    let rejections = denial.host_rejections();
    if !rejections.is_empty()
        && rejections.iter().all(|rejection| {
            rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired
        })
    {
        match denial.detach_retry_for_native() {
            Ok(retry) => RequiredPredecessorReconstruction::Required(retry),
            Err(denial) => RequiredPredecessorReconstruction::NotRequired(
                crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial),
            ),
        }
    } else {
        RequiredPredecessorReconstruction::NotRequired(
            crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial),
        )
    }
}

pub(super) fn reconstruction_matches_progress(
    in_flight: &crate::mounting::UiMountedPresentationInFlight,
    progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
) -> bool {
    progress.class() == worth_ui_host_native::UiNativePhysicalProgressClass::Presentation
        && in_flight.awaits_progress_class(
            worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface,
        )
        && progress.presentation().is_some_and(|presentation| {
            presentation.attempt() == in_flight.attempt()
                && in_flight
                    .pending_bindings()
                    .any(|binding| binding == presentation.binding())
        })
}

pub(super) fn reconstruction_settled(outcome: &crate::mounting::UiMountedFrameOutcome) -> bool {
    matches!(
        outcome,
        crate::mounting::UiMountedFrameOutcome::Published(_)
            | crate::mounting::UiMountedFrameOutcome::Unchanged(_)
            | crate::mounting::UiMountedFrameOutcome::Reconciled(_)
    )
}
