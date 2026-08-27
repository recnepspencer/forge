use super::WorthUiActiveApplicationSession;

#[path = "portal_dismissal/handles.rs"]
mod handles;
#[path = "portal_dismissal/receipt.rs"]
mod receipt;
pub use receipt::UiPortalDismissalPublicationReceipt;

pub(crate) enum UiPortalDismissalPublicationOutcome<'session> {
    Ignored,
    Published(UiPortalDismissalPublicationReceipt),
    InFlight(UiPortalDismissalPublicationCompletion<'session>),
    Indeterminate(UiPortalDismissalPublicationRecovery<'session>),
    Stopped(UiPortalDismissalPublicationStop),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalDismissalPublicationStop {
    IdentityExhausted,
    Transition,
    Proposal,
    Preparation,
    HostRejectedBeforeEffects,
    MountedRetention,
    MountedPresentation,
    Superseded,
}

#[must_use = "portal dismissal presentation must be completed or cancelled"]
pub(crate) struct UiPortalDismissalPublicationCompletion<'session> {
    state: Option<Box<UiPortalDismissalInFlight<'session>>>,
}

#[must_use = "indeterminate portal dismissal requires shutdown reconciliation"]
pub(crate) struct UiPortalDismissalPublicationRecovery<'session> {
    state: Option<Box<UiPortalDismissalIndeterminate<'session>>>,
}

struct UiPortalDismissalAdmitted<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    proposal: Option<crate::runtime::session::UiStagedPortalProposalTransaction>,
}

struct UiPortalDismissalInFlight<'session> {
    admitted: UiPortalDismissalAdmitted<'session>,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

struct UiPortalDismissalIndeterminate<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    frame: crate::mounting::UiMountedIndeterminateFrame,
    proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
}

pub(in crate::facade::entry) struct DetachedUiPortalDismissalInFlight {
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    proposal: crate::runtime::session::UiStagedPortalProposalTransaction,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

pub(in crate::facade::entry) struct DetachedUiPortalDismissalIndeterminate {
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    frame: crate::mounting::UiMountedIndeterminateFrame,
    proposal: crate::runtime::session::UiIndeterminatePortalProposalTransaction,
}

impl WorthUiActiveApplicationSession {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn topmost_portal_presentation_for_certification(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationPresentationBasis> {
        self.portal.topmost_presentation()
    }

    pub(crate) fn publish_portal_dismissal(
        &mut self,
        interaction: crate::facade::interaction::UiDismissInteraction,
        now_tick: u64,
    ) -> UiPortalDismissalPublicationOutcome<'_> {
        let lineage = self.next_portal_service_event_identity;
        self.next_portal_service_event_identity = match lineage.checked_add(1) {
            Some(next) => next,
            None => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::IdentityExhausted,
                )
            }
        };
        let idempotency =
            crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity::issued(
                self.session_identity().as_u64(),
                lineage,
            );
        let trigger = match dismissal_trigger(interaction) {
            Some(trigger) => trigger,
            None => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Transition,
                )
            }
        };
        if crate::runtime::interaction::targeting::require_current_presentation(
            &self.mounted,
            interaction.presentation(),
        )
        .is_err()
        {
            return UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::Transition,
            );
        }
        let sampled_bounds = if matches!(
            trigger,
            crate::runtime::portal::UiPortalDismissalTrigger::OutsidePress { .. }
        ) {
            match self.portal.dismissal_target_identity(trigger) {
                Some(portal) => match self.mounted.committed_motion_geometry_for_instance(
                    portal.owner().mounted_instance_identity(),
                    interaction.presentation(),
                ) {
                    Ok(bounds) => bounds,
                    Err(_) => {
                        return UiPortalDismissalPublicationOutcome::Stopped(
                            UiPortalDismissalPublicationStop::Transition,
                        )
                    }
                },
                None => None,
            }
        } else {
            None
        };
        let dismissal = match self
            .portal
            .prepare_dismissal(trigger, sampled_bounds, idempotency)
        {
            Ok(crate::runtime::portal::UiPortalDismissalPreparation::Ignored(_)) => {
                return UiPortalDismissalPublicationOutcome::Ignored
            }
            Ok(crate::runtime::portal::UiPortalDismissalPreparation::Prepared(dismissal)) => {
                dismissal
            }
            Err(_) => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Transition,
                )
            }
        };
        if matches!(
            trigger,
            crate::runtime::portal::UiPortalDismissalTrigger::OutsidePress { .. }
        ) && dismissal.presentation() != interaction.presentation()
        {
            return UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::Transition,
            );
        }
        let presentation = dismissal.presentation();
        let transition = dismissal.into_transition();
        let revision = transition.successor_revision();
        let overlays = self
            .portal
            .mounted_projection_inputs(&transition, transition.closes_portal());
        let motion_request = match self.prepare_portal_motion_request(&transition) {
            Ok(request) => request,
            Err(_) => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Proposal,
                )
            }
        };
        let preparation = match self.application.begin_portal_dismissal_service_proposal(
            transition,
            presentation,
            self.active_generation_identity(),
            &mut self.motion,
            motion_request,
        ) {
            Ok(preparation) => preparation,
            Err(_) => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Proposal,
                )
            }
        };
        let frame = match self.prepare_intent_consequence_frame(
            crate::mounting::UiMountedSemanticContentInput::empty(),
            revision,
            overlays,
        ) {
            Ok(frame) => frame,
            Err(_) => {
                self.application
                    .cancel_portal_service_proposal_preparation(preparation, &mut self.motion);
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Preparation,
                );
            }
        };
        let proposal = match self.application.bind_portal_service_proposal_frame(
            preparation,
            &frame,
            &mut self.focus,
            &mut self.motion,
        ) {
            Ok(proposal) => proposal,
            Err(_) => {
                return UiPortalDismissalPublicationOutcome::Stopped(
                    UiPortalDismissalPublicationStop::Proposal,
                )
            }
        };
        let outcome = self.present_prepared_mounted_frame_internal(
            frame,
            worth_ui_host_contract::UiPresentationDeadline::at_tick(u64::MAX),
            now_tick,
        );
        finish(
            UiPortalDismissalAdmitted {
                session: self,
                proposal: Some(proposal),
            },
            outcome,
        )
    }
}

fn dismissal_trigger(
    interaction: crate::facade::interaction::UiDismissInteraction,
) -> Option<crate::runtime::portal::UiPortalDismissalTrigger> {
    match interaction.cause() {
        crate::facade::interaction::UiDismissInteractionCause::Escape => {
            Some(crate::runtime::portal::UiPortalDismissalTrigger::Escape)
        }
        crate::facade::interaction::UiDismissInteractionCause::OutsidePress(position) => {
            let basis = position.basis();
            if basis.coordinate_space()
                != worth_ui_host_contract::UiHostSurfaceCoordinateSpace::Viewport
                || basis.coordinate_unit()
                    != worth_ui_host_contract::UiHostSurfaceCoordinateUnit::LogicalPoint
            {
                return None;
            }
            let scale = worth_ui_host_contract::UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT as f64;
            let point = [
                (position.x_subpixels() as f64 / scale) as f32,
                (position.y_subpixels() as f64 / scale) as f32,
            ];
            Some(
                crate::runtime::portal::UiPortalDismissalTrigger::OutsidePress {
                    viewport_point_bits: point.map(f32::to_bits),
                },
            )
        }
    }
}

fn finish<'session>(
    mut admitted: UiPortalDismissalAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiPortalDismissalPublicationOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::Published(mounted) => {
            let (focus, motion, exit_retention) = admitted
                .session
                .application
                .settle_published_portal_service_proposal(
                    admitted
                        .proposal
                        .take()
                        .expect("published dismissal retains proposal"),
                    &mounted,
                    &mut admitted.session.portal,
                    &mut admitted.session.focus,
                    &mut admitted.session.motion,
                )
                .expect("published dismissal retains exact service proposal");
            admitted
                .session
                .rebind_portal_after_current_published_frame();
            let focus = admitted
                .session
                .place_committed_semantic_focus(focus, &mounted)
                .expect("dismissal focus restoration retains exact mounted basis");
            admitted
                .session
                .install_portal_exit_retention(exit_retention);
            admitted.session.install_committed_motion(motion);
            UiPortalDismissalPublicationOutcome::Published(
                UiPortalDismissalPublicationReceipt::new(mounted, focus),
            )
        }
        crate::mounting::UiMountedFrameOutcome::InFlight(mounted) => {
            UiPortalDismissalPublicationOutcome::InFlight(UiPortalDismissalPublicationCompletion {
                state: Some(Box::new(UiPortalDismissalInFlight { admitted, mounted })),
            })
        }
        crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(frame) => {
            let proposal = admitted
                .session
                .application
                .settle_indeterminate_portal_service_proposal(
                    admitted
                        .proposal
                        .take()
                        .expect("indeterminate dismissal retains proposal"),
                    &mut admitted.session.portal,
                    &mut admitted.session.focus,
                    &mut admitted.session.motion,
                )
                .expect("indeterminate dismissal retains exact service proposal");
            UiPortalDismissalPublicationOutcome::Indeterminate(
                UiPortalDismissalPublicationRecovery {
                    state: Some(Box::new(UiPortalDismissalIndeterminate {
                        session: admitted.session,
                        frame,
                        proposal,
                    })),
                },
            )
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(_) => {
            settle_rejected(&mut admitted);
            UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::HostRejectedBeforeEffects,
            )
        }
        crate::mounting::UiMountedFrameOutcome::RetentionDenied(_) => {
            settle_rejected(&mut admitted);
            UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::MountedRetention,
            )
        }
        crate::mounting::UiMountedFrameOutcome::AdmissionDenied(_) => {
            settle_rejected(&mut admitted);
            UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::MountedPresentation,
            )
        }
        crate::mounting::UiMountedFrameOutcome::Superseded(_) => {
            cancel(admitted);
            UiPortalDismissalPublicationOutcome::Stopped(
                UiPortalDismissalPublicationStop::Superseded,
            )
        }
        crate::mounting::UiMountedFrameOutcome::CompletionDenied(_) => {
            panic!("exact dismissal completion authority became unknown")
        }
        crate::mounting::UiMountedFrameOutcome::Unchanged(_)
        | crate::mounting::UiMountedFrameOutcome::Reconciled(_) => {
            unreachable!("portal dismissal always changes mounted overlay work")
        }
    }
}

pub(in crate::facade::entry) fn finish_detached_portal_proposal<'session>(
    session: &'session mut WorthUiActiveApplicationSession,
    proposal: crate::runtime::session::UiStagedPortalProposalTransaction,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiPortalDismissalPublicationOutcome<'session> {
    finish(
        UiPortalDismissalAdmitted {
            session,
            proposal: Some(proposal),
        },
        outcome,
    )
}

fn settle_rejected(admitted: &mut UiPortalDismissalAdmitted<'_>) {
    let proposal = admitted
        .proposal
        .take()
        .expect("rejected dismissal retains proposal");
    admitted
        .session
        .application
        .settle_rejected_portal_service_proposal(
            proposal,
            &mut admitted.session.focus,
            &mut admitted.session.motion,
        )
        .expect("before-effect dismissal rejection retains exact proposal");
}

fn cancel(mut admitted: UiPortalDismissalAdmitted<'_>) {
    admitted.session.application.cancel_portal_service_proposal(
        admitted
            .proposal
            .take()
            .expect("cancelled dismissal retains proposal"),
        &mut admitted.session.focus,
        &mut admitted.session.motion,
    );
}
