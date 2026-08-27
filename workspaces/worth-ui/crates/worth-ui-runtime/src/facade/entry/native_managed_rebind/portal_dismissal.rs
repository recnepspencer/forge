use super::{WorthUiNativeManagedRebindProgress, WorthUiNativePendingManagedRebind};

#[derive(Clone, Copy)]
pub(in crate::facade::entry) struct UiRetainedPortalDismissalRequest {
    cause: crate::facade::interaction::UiDismissInteractionCause,
    sequence: worth_ui_host_contract::UiHostObservationSequence,
    time_basis: worth_ui_host_contract::UiHostObservationTimeBasis,
}

impl UiRetainedPortalDismissalRequest {
    fn retain(interaction: crate::facade::interaction::UiDismissInteraction) -> Self {
        Self {
            cause: interaction.cause(),
            sequence: interaction.sequence(),
            time_basis: interaction.time_basis(),
        }
    }

    fn rebase(
        self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> crate::facade::interaction::UiDismissInteraction {
        match self.cause {
            crate::facade::interaction::UiDismissInteractionCause::Escape => {
                crate::facade::interaction::UiDismissInteraction::escape(
                    presentation,
                    self.sequence,
                    self.time_basis,
                )
            }
            crate::facade::interaction::UiDismissInteractionCause::OutsidePress(position) => {
                crate::facade::interaction::UiDismissInteraction::outside_press(
                    presentation,
                    self.sequence,
                    self.time_basis,
                    position,
                )
            }
        }
    }
}

pub enum WorthUiNativeManagedPortalDismissalOutcome {
    Ignored,
    Retained,
    Published(super::super::portal_dismissal::UiPortalDismissalPublicationReceipt),
    Pending,
    Stopped(WorthUiNativePortalDismissalStop),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiNativePortalDismissalStop {
    Busy,
    IdentityExhausted,
    Transition,
    Proposal,
    Preparation,
    HostRejectedBeforeEffects,
    MountedRetention,
    MountedPresentation,
    Superseded,
    Indeterminate,
}

enum NormalizedPortalDismissal {
    Ignored,
    Published(super::super::portal_dismissal::UiPortalDismissalPublicationReceipt),
    Pending(super::super::portal_dismissal::DetachedUiPortalDismissalInFlight),
    Indeterminate(super::super::portal_dismissal::DetachedUiPortalDismissalIndeterminate),
    Stopped(WorthUiNativePortalDismissalStop),
}

impl super::super::WorthUiNativeApplicationShell {
    pub fn begin_managed_portal_dismissal(
        &mut self,
        interaction: crate::facade::interaction::UiDismissInteraction,
        now_tick: u64,
    ) -> WorthUiNativeManagedPortalDismissalOutcome {
        if matches!(
            &self.pending_managed_rebind,
            Some(
                WorthUiNativePendingManagedRebind::PortalDismissal(_)
                    | WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(_)
                    | WorthUiNativePendingManagedRebind::PortalDismissalReconstruction { .. }
                    | WorthUiNativePendingManagedRebind::PortalDismissalReconstructionDeferred { .. }
            )
        ) {
            self.retained_portal_dismissal =
                Some(UiRetainedPortalDismissalRequest::retain(interaction));
            return WorthUiNativeManagedPortalDismissalOutcome::Pending;
        }
        if self
            .pending_managed_rebind
            .as_ref()
            .is_some_and(WorthUiNativePendingManagedRebind::carries_portal_intent_consequence)
        {
            self.retained_portal_dismissal =
                Some(UiRetainedPortalDismissalRequest::retain(interaction));
            return WorthUiNativeManagedPortalDismissalOutcome::Retained;
        }
        if self.pending_managed_rebind.is_some() {
            return WorthUiNativeManagedPortalDismissalOutcome::Stopped(
                WorthUiNativePortalDismissalStop::Busy,
            );
        }
        self.retained_portal_dismissal = None;
        let outcome = self.session.publish_portal_dismissal(interaction, now_tick);
        match normalize(outcome) {
            NormalizedPortalDismissal::Ignored => {
                WorthUiNativeManagedPortalDismissalOutcome::Ignored
            }
            NormalizedPortalDismissal::Published(receipt) => {
                WorthUiNativeManagedPortalDismissalOutcome::Published(receipt)
            }
            NormalizedPortalDismissal::Pending(pending) => {
                self.pending_managed_rebind =
                    Some(WorthUiNativePendingManagedRebind::PortalDismissal(pending));
                WorthUiNativeManagedPortalDismissalOutcome::Pending
            }
            NormalizedPortalDismissal::Indeterminate(pending) => {
                self.pending_managed_rebind =
                    Some(WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(pending));
                WorthUiNativeManagedPortalDismissalOutcome::Pending
            }
            NormalizedPortalDismissal::Stopped(stop) => {
                WorthUiNativeManagedPortalDismissalOutcome::Stopped(stop)
            }
        }
    }

    pub fn continue_retained_portal_dismissal_after_managed_intent(
        &mut self,
        now_tick: u64,
    ) -> WorthUiNativeManagedPortalDismissalOutcome {
        if self.pending_managed_rebind.is_some() {
            return WorthUiNativeManagedPortalDismissalOutcome::Stopped(
                WorthUiNativePortalDismissalStop::Busy,
            );
        }
        let Some(retained) = self.retained_portal_dismissal.take() else {
            return WorthUiNativeManagedPortalDismissalOutcome::Ignored;
        };
        let Some(presentation) = self.session.portal.topmost_presentation() else {
            return WorthUiNativeManagedPortalDismissalOutcome::Ignored;
        };
        self.begin_managed_portal_dismissal(retained.rebase(presentation), now_tick)
    }
}

pub(super) fn finish(
    pending_slot: &mut Option<WorthUiNativePendingManagedRebind>,
    outcome: super::super::portal_dismissal::UiPortalDismissalPublicationOutcome<'_>,
) -> WorthUiNativeManagedRebindProgress {
    match normalize(outcome) {
        NormalizedPortalDismissal::Published(receipt) => {
            WorthUiNativeManagedRebindProgress::PortalDismissed(receipt)
        }
        NormalizedPortalDismissal::Pending(pending) => {
            *pending_slot = Some(WorthUiNativePendingManagedRebind::PortalDismissal(pending));
            WorthUiNativeManagedRebindProgress::AwaitingProgress
        }
        NormalizedPortalDismissal::Indeterminate(pending) => {
            *pending_slot =
                Some(WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(pending));
            WorthUiNativeManagedRebindProgress::AwaitingProgress
        }
        NormalizedPortalDismissal::Stopped(stop) => WorthUiNativeManagedRebindProgress::Stopped(
            super::WorthUiNativeManagedRebindStop::PortalDismissal(stop),
        ),
        NormalizedPortalDismissal::Ignored => {
            unreachable!("an admitted in-flight dismissal cannot become ignored")
        }
    }
}

fn normalize(
    outcome: super::super::portal_dismissal::UiPortalDismissalPublicationOutcome<'_>,
) -> NormalizedPortalDismissal {
    use super::super::portal_dismissal::UiPortalDismissalPublicationOutcome as Outcome;
    match outcome {
        Outcome::Ignored => NormalizedPortalDismissal::Ignored,
        Outcome::Published(receipt) => NormalizedPortalDismissal::Published(receipt),
        Outcome::InFlight(completion) => {
            NormalizedPortalDismissal::Pending(completion.detach_for_native())
        }
        Outcome::Indeterminate(recovery) => {
            NormalizedPortalDismissal::Indeterminate(recovery.detach_for_native())
        }
        Outcome::Stopped(stop) => NormalizedPortalDismissal::Stopped(map_stop(stop)),
    }
}

fn map_stop(
    stop: super::super::portal_dismissal::UiPortalDismissalPublicationStop,
) -> WorthUiNativePortalDismissalStop {
    use super::super::portal_dismissal::UiPortalDismissalPublicationStop as Stop;
    match stop {
        Stop::IdentityExhausted => WorthUiNativePortalDismissalStop::IdentityExhausted,
        Stop::Transition => WorthUiNativePortalDismissalStop::Transition,
        Stop::Proposal => WorthUiNativePortalDismissalStop::Proposal,
        Stop::Preparation => WorthUiNativePortalDismissalStop::Preparation,
        Stop::HostRejectedBeforeEffects => {
            WorthUiNativePortalDismissalStop::HostRejectedBeforeEffects
        }
        Stop::MountedRetention => WorthUiNativePortalDismissalStop::MountedRetention,
        Stop::MountedPresentation => WorthUiNativePortalDismissalStop::MountedPresentation,
        Stop::Superseded => WorthUiNativePortalDismissalStop::Superseded,
    }
}

impl super::super::WorthUiNativeApplicationShell {
    pub(super) fn progress_indeterminate_portal_dismissal(
        &mut self,
        pending: super::super::portal_dismissal::DetachedUiPortalDismissalIndeterminate,
        progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
    ) -> Result<WorthUiNativeManagedRebindProgress, super::WorthUiNativeManagedRebindDenial> {
        if pending.session_identity() != self.session.session_identity() {
            return Err(super::WorthUiNativeManagedRebindDenial::SessionMismatch);
        }
        let session = pending.session_identity();
        let (frame, proposal) = pending.into_parts();
        self.managed_rebind_completion_tick = self.managed_rebind_completion_tick.saturating_add(1);
        let recovery = self.progress_indeterminate_presentation_recovery(
            frame,
            progress,
            u64::MAX,
            self.managed_rebind_completion_tick,
        );
        match recovery {
            super::super::native_application_shell::WorthUiNativePhysicalPresentationRecovery::Awaiting(frame) => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(
                        super::super::portal_dismissal::DetachedUiPortalDismissalIndeterminate::from_parts(
                            session, frame, proposal,
                        ),
                    ),
                );
                Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress)
            }
            super::super::native_application_shell::WorthUiNativePhysicalPresentationRecovery::Blocked {
                frame,
                denial,
            } => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(
                        super::super::portal_dismissal::DetachedUiPortalDismissalIndeterminate::from_parts(
                            session, frame, proposal,
                        ),
                    ),
                );
                Ok(WorthUiNativeManagedRebindProgress::RecoveryBlocked(denial))
            }
            super::super::native_application_shell::WorthUiNativePhysicalPresentationRecovery::Recovered(outcome) => {
                self.finish_portal_dismissal_recovery(proposal, outcome)
            }
        }
    }

    pub(super) fn progress_portal_dismissal_reconstruction(
        &mut self,
        proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
        progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
    ) -> Result<WorthUiNativeManagedRebindProgress, super::WorthUiNativeManagedRebindDenial> {
        if !matches_reconstruction_progress(&in_flight, progress) {
            self.pending_managed_rebind = Some(
                WorthUiNativePendingManagedRebind::PortalDismissalReconstruction {
                    proposal,
                    in_flight,
                },
            );
            return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
        }
        self.managed_rebind_completion_tick = self.managed_rebind_completion_tick.saturating_add(1);
        let outcome = self
            .session
            .complete_mounted_presentation(in_flight, self.managed_rebind_completion_tick);
        self.finish_portal_dismissal_recovery(proposal, outcome)
    }

    pub(super) fn progress_deferred_portal_dismissal_reconstruction(
        &mut self,
        proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
    ) -> Result<WorthUiNativeManagedRebindProgress, super::WorthUiNativeManagedRebindDenial> {
        self.managed_rebind_completion_tick = self.managed_rebind_completion_tick.saturating_add(1);
        match self.reconstruct_current_presentation(u64::MAX, self.managed_rebind_completion_tick) {
            Ok(outcome) => self.finish_portal_dismissal_recovery(proposal, outcome),
            Err(()) => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PortalDismissalReconstructionDeferred {
                        proposal,
                    },
                );
                Ok(WorthUiNativeManagedRebindProgress::RecoveryBlocked(
                    super::super::native_application_shell::WorthUiNativePresentationRecoveryDenial::CurrentPresentationUnavailable,
                ))
            }
        }
    }
    fn finish_portal_dismissal_recovery(
        &mut self,
        proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
        outcome: crate::mounting::UiMountedFrameOutcome,
    ) -> Result<WorthUiNativeManagedRebindProgress, super::WorthUiNativeManagedRebindDenial> {
        match outcome {
            crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PortalDismissalReconstruction {
                        proposal,
                        in_flight,
                    },
                );
                Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress)
            }
            crate::mounting::UiMountedFrameOutcome::Published(_)
            | crate::mounting::UiMountedFrameOutcome::Unchanged(_)
            | crate::mounting::UiMountedFrameOutcome::Reconciled(_) => {
                self.session
                    .application
                    .settle_indeterminate_portal_service_proposal_to_predecessor(
                        proposal,
                        &mut self.session.focus,
                        &mut self.session.motion,
                    );
                Ok(self.replay_retained_portal_dismissal_after_recovery())
            }
            _ => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PortalDismissalReconstructionDeferred {
                        proposal,
                    },
                );
                Ok(WorthUiNativeManagedRebindProgress::RecoveryBlocked(
                    super::super::native_application_shell::WorthUiNativePresentationRecoveryDenial::CurrentPresentationUnavailable,
                ))
            }
        }
    }

    fn replay_retained_portal_dismissal_after_recovery(
        &mut self,
    ) -> WorthUiNativeManagedRebindProgress {
        let Some(retained) = self.retained_portal_dismissal.take() else {
            return WorthUiNativeManagedRebindProgress::RecoveredToPredecessor(
                super::WorthUiNativePredecessorRecovery::PortalDismissal,
            );
        };
        let Some(presentation) = self.session.portal.topmost_presentation() else {
            return WorthUiNativeManagedRebindProgress::RecoveredToPredecessor(
                super::WorthUiNativePredecessorRecovery::PortalDismissal,
            );
        };
        match self.begin_managed_portal_dismissal(
            retained.rebase(presentation),
            self.managed_rebind_completion_tick,
        ) {
            WorthUiNativeManagedPortalDismissalOutcome::Ignored => {
                WorthUiNativeManagedRebindProgress::RecoveredToPredecessor(
                    super::WorthUiNativePredecessorRecovery::PortalDismissal,
                )
            }
            WorthUiNativeManagedPortalDismissalOutcome::Retained => {
                unreachable!("recovery replay runs without another managed intent consequence")
            }
            WorthUiNativeManagedPortalDismissalOutcome::Published(receipt) => {
                WorthUiNativeManagedRebindProgress::PortalDismissed(receipt)
            }
            WorthUiNativeManagedPortalDismissalOutcome::Pending => {
                WorthUiNativeManagedRebindProgress::AwaitingProgress
            }
            WorthUiNativeManagedPortalDismissalOutcome::Stopped(stop) => {
                WorthUiNativeManagedRebindProgress::Stopped(
                    super::WorthUiNativeManagedRebindStop::PortalDismissal(stop),
                )
            }
        }
    }
}

fn matches_reconstruction_progress(
    in_flight: &crate::mounting::UiMountedPresentationInFlight,
    progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
) -> bool {
    let class = match progress.class() {
        worth_ui_host_native::UiNativePhysicalProgressClass::Presentation => {
            worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface
        }
        worth_ui_host_native::UiNativePhysicalProgressClass::TextAtlas => {
            worth_ui_host_contract::UiHostPresentationProgressClass::TextAtlas
        }
        worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery => return false,
    };
    in_flight.awaits_progress_class(class)
        && progress.presentation().map_or(true, |presentation| {
            presentation.attempt() == in_flight.attempt()
                && in_flight
                    .pending_bindings()
                    .any(|binding| binding == presentation.binding())
        })
}
