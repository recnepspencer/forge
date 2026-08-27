use super::*;

pub(crate) fn map_changed_first_attempt<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    outcome: crate::facade::WorthUiMountedApplicationReplacementOutcome<'session>,
) -> UiRebindOutcome<'session> {
    match outcome {
        crate::facade::WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => publish_changed(plan, registration, application, mounted),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(
            rejection,
        ) => {
            registration
                .return_to_pending()
                .expect("the executing plan vacated its pending slot");
            let (rejections, replacement) = rejection.into_parts();
            UiRebindOutcome::RejectedBeforeEffects(UiRebindDenialReceipt::retry_host(
                plan,
                registration,
                super::super::preparation::UiPreparedRebindKind::Changed(replacement),
                rejections,
            ))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::InFlight(inner) => {
            registration
                .retain_completion()
                .expect("pre-effect admission reserved completion capacity");
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::new(plan, registration, inner))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(
            inner,
        ) => recover(plan, registration, inner),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RetentionDenied(denial) => {
            let cause = UiRebindDenialCause::MountedRetention(denial.denial());
            pre_effect_retry(
                plan,
                registration,
                denial.into_replacement(),
                UiRebindStoppedPhase::MountedRetentionAdmission,
                cause,
            )
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(denial) => {
            let cause = UiRebindDenialCause::MountedPresentation(denial.denial());
            pre_effect_retry(
                plan,
                registration,
                denial.into_replacement(),
                UiRebindStoppedPhase::MountedPresentationAdmission,
                cause,
            )
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::CompletionDenied(_) => {
            unreachable!("first presentation cannot produce a completion denial")
        }
    }
}

pub(super) fn map_changed_completion<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    outcome: crate::facade::WorthUiMountedApplicationReplacementOutcome<'session>,
) -> UiRebindOutcome<'session> {
    match outcome {
        crate::facade::WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => publish_changed(plan, registration, application, mounted),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::InFlight(inner) => {
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::new(plan, registration, inner))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(
            inner,
        ) => recover(plan, registration, inner),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::CompletionDenied(denial) => {
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::new(
                plan,
                registration,
                Box::new(denial.into_in_flight()),
            ))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(
            rejection,
        ) => {
            let (rejections, replacement) = rejection.into_parts();
            registration
                .return_to_pending()
                .expect("completion retry returns its reservation to pending");
            UiRebindOutcome::RejectedBeforeEffects(UiRebindDenialReceipt::retry_host(
                plan,
                registration,
                super::super::preparation::UiPreparedRebindKind::Changed(replacement),
                rejections,
            ))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RetentionDenied(_)
        | crate::facade::WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(_) => {
            unreachable!("completion cannot return a preparation-stage outcome")
        }
    }
}

pub(super) fn map_changed_cancellation<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    outcome: crate::facade::WorthUiMountedApplicationReplacementOutcome<'session>,
) -> UiRebindOutcome<'session> {
    match outcome {
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(
            rejection,
        ) => {
            drop(rejection);
            UiRebindOutcome::CancelledBeforeEffects(UiRebindCancellationReceipt::cancelled())
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(
            inner,
        ) => recover(plan, registration, inner),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::InFlight(inner) => {
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::new(plan, registration, inner))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::CompletionDenied(denial) => {
            UiRebindOutcome::InFlight(UiRebindCompletionHandle::new(
                plan,
                registration,
                Box::new(denial.into_in_flight()),
            ))
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => UiRebindOutcome::InternalDefect(
            UiRebindInternalDefectOutcome::unexpected_cancellation_publication(
                plan,
                registration,
                application,
                mounted,
            ),
        ),
        crate::facade::WorthUiMountedApplicationReplacementOutcome::RetentionDenied(denial) => {
            let cause = UiRebindDenialCause::MountedRetention(denial.denial());
            pre_effect_retry(
                plan,
                registration,
                denial.into_replacement(),
                UiRebindStoppedPhase::MountedRetentionAdmission,
                cause,
            )
        }
        crate::facade::WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(denial) => {
            let cause = UiRebindDenialCause::MountedPresentation(denial.denial());
            pre_effect_retry(
                plan,
                registration,
                denial.into_replacement(),
                UiRebindStoppedPhase::MountedPresentationAdmission,
                cause,
            )
        }
    }
}

fn publish_changed<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: UiRebindReservation,
    application: crate::facade::WorthUiApplicationCutoverReceipt,
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
) -> UiRebindOutcome<'session> {
    match super::super::UiRebindReceipt::changed(plan, registration, application, mounted) {
        Ok(receipt) => UiRebindOutcome::Published(receipt),
        Err(defect) => UiRebindOutcome::InternalDefect(defect),
    }
}

fn pre_effect_retry<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    replacement: Box<crate::facade::WorthUiPreparedMountedApplicationReplacement<'session>>,
    stopped_phase: UiRebindStoppedPhase,
    cause: UiRebindDenialCause,
) -> UiRebindOutcome<'session> {
    registration
        .return_to_pending()
        .expect("the executing plan vacated its pending slot");
    UiRebindOutcome::RejectedBeforeEffects(UiRebindDenialReceipt::retry(
        plan,
        registration,
        super::super::preparation::UiPreparedRebindKind::Changed(replacement),
        stopped_phase,
        cause,
    ))
}

fn recover<'session>(
    plan: crate::runtime::rebind::UiRebindPlan,
    mut registration: UiRebindReservation,
    inner: Box<crate::facade::WorthUiMountedApplicationReplacementIndeterminate<'session>>,
) -> UiRebindOutcome<'session> {
    registration
        .retain_recovery()
        .expect("pre-effect admission reserved recovery capacity");
    UiRebindOutcome::Indeterminate(super::super::UiRebindRecoveryHandle::new(
        plan,
        registration,
        inner,
    ))
}
