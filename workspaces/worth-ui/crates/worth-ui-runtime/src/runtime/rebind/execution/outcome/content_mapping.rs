use super::*;

pub(crate) fn map_content_first_attempt<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    generation: crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    outcome: crate::facade::entry::WorthUiMountedContentRebindOutcome<'session>,
) -> UiRebindOutcome<'session> {
    map_content_outcome(plan, registration, generation, outcome, true)
}

pub(crate) fn map_content_completion<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    generation: crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    outcome: crate::facade::entry::WorthUiMountedContentRebindOutcome<'session>,
) -> UiRebindOutcome<'session> {
    map_content_outcome(plan, registration, generation, outcome, false)
}

pub(crate) fn map_content_cancellation<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    generation: crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    outcome: crate::facade::entry::WorthUiMountedContentRebindOutcome<'session>,
) -> UiRebindOutcome<'session> {
    match outcome {
        crate::facade::entry::WorthUiMountedContentRebindOutcome::RejectedBeforeEffects(
            prepared,
        ) => {
            drop(prepared);
            UiRebindOutcome::CancelledBeforeEffects(UiRebindCancellationReceipt::cancelled())
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::Published(mounted) => {
            UiRebindOutcome::InternalDefect(
                UiRebindInternalDefectOutcome::unexpected_content_cancellation_publication(
                    plan,
                    registration,
                    generation,
                    mounted,
                ),
            )
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::InFlight(inner) => {
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::content(
                plan,
                registration,
                generation,
                inner,
            ))
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::PresentationIndeterminate(
            inner,
        ) => recover_content(plan, registration, inner),
        crate::facade::entry::WorthUiMountedContentRebindOutcome::CompletionDenied(_) => {
            UiRebindOutcome::InternalDefect(
                UiRebindInternalDefectOutcome::completion_authority_rejected(plan, registration),
            )
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::RetentionDenied { .. }
        | crate::facade::entry::WorthUiMountedContentRebindOutcome::AdmissionDenied { .. } => {
            unreachable!("in-flight cancellation cannot return a preparation-stage denial")
        }
    }
}

fn map_content_outcome<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    generation: crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    outcome: crate::facade::entry::WorthUiMountedContentRebindOutcome<'session>,
    first_attempt: bool,
) -> UiRebindOutcome<'session> {
    match outcome {
        crate::facade::entry::WorthUiMountedContentRebindOutcome::Published(mounted) => {
            publish_content(plan, registration, generation, mounted)
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::RejectedBeforeEffects(
            prepared,
        ) if first_attempt => retry(
            plan,
            registration,
            prepared,
            UiRebindStoppedPhase::HostPresentation,
            UiRebindDenialCause::HostRejectedBeforeEffects,
        ),
        crate::facade::entry::WorthUiMountedContentRebindOutcome::RetentionDenied {
            denial,
            retry: prepared,
        } if first_attempt => retry(
            plan,
            registration,
            prepared,
            UiRebindStoppedPhase::MountedRetentionAdmission,
            UiRebindDenialCause::MountedRetention(denial),
        ),
        crate::facade::entry::WorthUiMountedContentRebindOutcome::AdmissionDenied {
            denial,
            retry: prepared,
        } if first_attempt => retry(
            plan,
            registration,
            prepared,
            UiRebindStoppedPhase::MountedPresentationAdmission,
            UiRebindDenialCause::MountedPresentation(denial),
        ),
        crate::facade::entry::WorthUiMountedContentRebindOutcome::InFlight(inner) => {
            if first_attempt {
                registration
                    .retain_completion()
                    .expect("pre-effect admission reserved completion capacity");
            }
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::content(
                plan,
                registration,
                generation,
                inner,
            ))
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::PresentationIndeterminate(
            inner,
        ) => recover_content(plan, registration, inner),
        crate::facade::entry::WorthUiMountedContentRebindOutcome::CompletionDenied(_) => {
            UiRebindOutcome::InternalDefect(
                UiRebindInternalDefectOutcome::completion_authority_rejected(plan, registration),
            )
        }
        crate::facade::entry::WorthUiMountedContentRebindOutcome::RejectedBeforeEffects(_)
        | crate::facade::entry::WorthUiMountedContentRebindOutcome::RetentionDenied { .. }
        | crate::facade::entry::WorthUiMountedContentRebindOutcome::AdmissionDenied { .. } => {
            unreachable!("content completion cannot return a preparation-stage denial")
        }
    }
}

fn retry<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    prepared: Box<crate::facade::entry::WorthUiPreparedMountedContentRebind<'session>>,
    stopped_phase: UiRebindStoppedPhase,
    cause: UiRebindDenialCause,
) -> UiRebindOutcome<'session> {
    registration
        .return_to_pending()
        .expect("the executing plan vacated its pending slot");
    UiRebindOutcome::RejectedBeforeEffects(UiRebindDenialReceipt::retry(
        plan,
        registration,
        super::super::preparation::UiPreparedRebindKind::Content(prepared),
        stopped_phase,
        cause,
    ))
}

fn publish_content<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    generation: crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
) -> UiRebindOutcome<'session> {
    match super::super::UiRebindReceipt::content(plan, registration, generation, mounted) {
        Ok(receipt) => UiRebindOutcome::Published(receipt),
        Err(defect) => UiRebindOutcome::InternalDefect(defect),
    }
}

fn recover_content<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    inner: Box<crate::facade::entry::WorthUiMountedContentRebindIndeterminate<'session>>,
) -> UiRebindOutcome<'session> {
    registration
        .retain_recovery()
        .expect("pre-effect admission reserved recovery capacity");
    UiRebindOutcome::Indeterminate(super::super::UiRebindRecoveryHandle::content(
        plan,
        registration,
        inner,
    ))
}
