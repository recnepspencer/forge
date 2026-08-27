use super::super::WorthUiApplicationSessionState;

#[must_use = "portal proposal preparation retains compiler occupancy until staged or cancelled"]
pub(crate) struct UiPortalProposalPreparation {
    staging: crate::runtime::session::service_proposal::UiServiceProposalStaging,
    portal: crate::runtime::portal::UiStagedPortalServiceProposal,
    focus: crate::runtime::focus::UiStagedFocusServiceProposal,
    motion: Option<crate::runtime::motion::UiStagedMotionServiceProposal>,
}

#[must_use = "a staged portal proposal must settle with existing publication"]
pub(crate) struct UiStagedPortalProposalTransaction {
    pub(super) batch: crate::runtime::session::service_proposal::UiServiceProposalStagedBatch,
    pub(super) portal: crate::runtime::portal::UiStagedPortalServiceProposal,
    pub(super) focus: crate::runtime::focus::UiStagedFocusServiceProposal,
    pub(super) motion: Option<crate::runtime::motion::UiDerivedMotionServiceProposal>,
    pub(super) prepared_frame: worth_ui_host_contract::UiMountedFrameIdentity,
}

pub(super) struct UiPortalProposalSettlement {
    pub(super) settlement: crate::runtime::session::service_proposal::UiServiceProposalSettlement,
    pub(super) transition: crate::runtime::portal::UiPreparedPortalServiceTransition,
    pub(super) focus: crate::runtime::focus::UiStagedFocusServiceProposal,
    pub(super) motion: Option<crate::runtime::motion::UiDerivedMotionServiceProposal>,
    pub(super) prepared_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(super) publication:
        crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
    pub(super) scope:
        crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
}

#[must_use = "indeterminate portal and Focus successors must settle from presentation truth or shutdown"]
pub(crate) struct UiIndeterminatePortalProposalTransaction {
    pub(super) transaction: UiStagedPortalProposalTransaction,
}

#[derive(Debug)]
pub(crate) enum UiPortalProposalPreparationDenial {
    RequestBasis(crate::runtime::session::service_proposal::UiServiceRequestBasisDenial),
    Demand(crate::runtime::session::service_proposal::UiServiceProposalDemandConstructionDenial),
    Preflight(crate::runtime::session::service_proposal::UiServiceProposalPreflightDenial),
    Reservation(crate::runtime::session::service_proposal::UiServiceProposalReservationDenial),
    Staging(crate::runtime::session::service_proposal::UiServiceProposalStagingDenial),
    Publication(crate::runtime::session::service_proposal::UiServiceProposalPublicationDenial),
    Focus(crate::runtime::focus::UiPortalFocusTransitionDenial),
    MotionRequest(crate::runtime::motion::UiMotionTransitionRequestDenial),
    Motion(crate::runtime::motion::UiMotionStagingDenial),
    MountedFrameMismatch,
    Coalesced,
}

impl WorthUiApplicationSessionState {
    pub(crate) fn begin_portal_service_proposal(
        &mut self,
        handoff: &crate::runtime::intent_execution::UiIntentConsequenceHandoff,
        transition: crate::runtime::portal::UiPreparedPortalServiceTransition,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
        motion_request: Option<crate::runtime::motion::UiMotionTransitionRequest>,
    ) -> Result<UiPortalProposalPreparation, UiPortalProposalPreparationDenial> {
        let request = crate::runtime::session::service_proposal::UiServiceRequestBasis::<
            crate::runtime::session::service_proposal::UiAdmittedIntentServiceRequestAuthority,
        >::from_intent_consequence(handoff, application)
        .map_err(UiPortalProposalPreparationDenial::RequestBasis)?;
        self.begin_portal_service_proposal_from_request(
            request,
            transition,
            motion_state,
            motion_request,
        )
    }

    pub(crate) fn begin_portal_dismissal_service_proposal(
        &mut self,
        transition: crate::runtime::portal::UiPreparedPortalServiceTransition,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
        motion_request: Option<crate::runtime::motion::UiMotionTransitionRequest>,
    ) -> Result<UiPortalProposalPreparation, UiPortalProposalPreparationDenial> {
        let request = crate::runtime::session::service_proposal::UiServiceRequestBasis::<
            crate::runtime::session::service_proposal::UiPortalDismissalServiceRequestAuthority,
        >::from_portal_dismissal(&transition, presentation, application)
        .map_err(UiPortalProposalPreparationDenial::RequestBasis)?;
        self.begin_portal_service_proposal_from_request(
            request,
            transition,
            motion_state,
            motion_request,
        )
    }

    pub(crate) fn begin_portal_exit_terminal_service_proposal(
        &mut self,
        transition: crate::runtime::portal::UiPreparedPortalServiceTransition,
        retention: crate::runtime::portal::UiPortalExitRetentionReceipt,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) -> Result<UiPortalProposalPreparation, UiPortalProposalPreparationDenial> {
        let request = crate::runtime::session::service_proposal::UiServiceRequestBasis::<
            crate::runtime::session::service_proposal::UiPortalExitTerminalServiceRequestAuthority,
        >::from_portal_exit_terminal(
            &transition, retention, presentation, application
        )
        .map_err(UiPortalProposalPreparationDenial::RequestBasis)?;
        self.begin_portal_service_proposal_from_request(request, transition, motion_state, None)
    }

    fn begin_portal_service_proposal_from_request<Authority>(
        &mut self,
        request: crate::runtime::session::service_proposal::UiServiceRequestBasis<Authority>,
        transition: crate::runtime::portal::UiPreparedPortalServiceTransition,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
        motion_request: Option<crate::runtime::motion::UiMotionTransitionRequest>,
    ) -> Result<UiPortalProposalPreparation, UiPortalProposalPreparationDenial>
    where
        Authority: crate::runtime::session::service_proposal::UiServiceRequestOriginAuthority,
    {
        let coherence = request.coherence();
        let portal_family =
            crate::runtime::portal::UiStagedPortalServiceProposal::family_proposal(&transition);
        let focus_requirement = crate::runtime::focus::UiPortalFocusRequirement::new(
            portal_family.scope(),
            transition.portal().owner().mounted_instance_identity(),
            transition.opens_portal(),
            transition
                .closed_descendants()
                .iter()
                .map(|portal| {
                    crate::runtime::focus::UiPortalFocusBoundaryIdentity::from_scope(
                        crate::runtime::session::service_proposal::
                            UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(
                                portal.owner().mounted_instance_identity(),
                            ),
                    )
                })
                .collect(),
        );
        let focus_family = crate::runtime::focus::UiStagedFocusServiceProposal::family_proposal(
            &focus_requirement,
        );
        let mut family_proposals = vec![portal_family, focus_family];
        if let Some(request) = motion_request.as_ref() {
            family_proposals.push(
                crate::runtime::motion::UiStagedMotionServiceProposal::family_proposal(request),
            );
        }
        let candidate =
            crate::runtime::session::service_proposal::UiServiceProposalCandidate::from_request(
                &request,
                family_proposals,
            )
            .map_err(UiPortalProposalPreparationDenial::Demand)?;
        let proposal = candidate.identity();
        let support = self
            .app
            .prepared_authority()
            .intent_execution_bindings()
            .runtime_service_support();
        let preflighted = self
            .runtime
            .service_proposals
            .preflight(candidate, &coherence, support)
            .map_err(UiPortalProposalPreparationDenial::Preflight)?;
        let reservation = match self
            .runtime
            .service_proposals
            .reserve(preflighted)
            .map_err(UiPortalProposalPreparationDenial::Reservation)?
        {
            crate::runtime::session::service_proposal::UiServiceProposalReservationOutcome::Reserved(
                reservation,
            ) => reservation,
            crate::runtime::session::service_proposal::UiServiceProposalReservationOutcome::Coalesced { .. } => {
                return Err(UiPortalProposalPreparationDenial::Coalesced)
            }
        };
        let portal = crate::runtime::portal::UiStagedPortalServiceProposal::prepare(
            transition,
            proposal,
            portal_family.scope(),
        );
        let focus = crate::runtime::focus::UiStagedFocusServiceProposal::prepare(
            proposal,
            focus_requirement,
        );
        let motion = match motion_request {
            Some(request) => match motion_state.stage(proposal, request) {
                Ok(motion) => Some(motion),
                Err(denial) => {
                    self.runtime
                        .service_proposals
                        .cancel_before_effect(reservation)
                        .expect("reserved proposal remains cancellable before owner staging");
                    return Err(UiPortalProposalPreparationDenial::Motion(denial));
                }
            },
            None => None,
        };
        let mut staging = match self.runtime.service_proposals.begin_staging(reservation) {
            Ok(staging) => staging,
            Err(denial) => {
                if let Some(motion) = motion {
                    motion_state.discard_staged(motion);
                }
                return Err(UiPortalProposalPreparationDenial::Staging(denial));
            }
        };
        if let Err(denial) = self
            .runtime
            .service_proposals
            .advance_staging(&mut staging, portal.stage_receipt())
        {
            self.cancel_portal_staging(staging, &portal, &focus, motion_state, motion);
            return Err(UiPortalProposalPreparationDenial::Staging(denial));
        }
        if let Err(denial) = self
            .runtime
            .service_proposals
            .advance_staging(&mut staging, focus.family_stage_receipt())
        {
            self.cancel_portal_staging(staging, &portal, &focus, motion_state, motion);
            return Err(UiPortalProposalPreparationDenial::Staging(denial));
        }
        if let Some(receipt) = motion
            .as_ref()
            .map(crate::runtime::motion::UiStagedMotionServiceProposal::family_stage_receipt)
        {
            if let Err(denial) = self
                .runtime
                .service_proposals
                .advance_staging(&mut staging, receipt)
            {
                self.cancel_portal_staging(staging, &portal, &focus, motion_state, motion);
                return Err(UiPortalProposalPreparationDenial::Staging(denial));
            }
        }
        Ok(UiPortalProposalPreparation {
            staging,
            portal,
            focus,
            motion,
        })
    }

    pub(crate) fn bind_portal_service_proposal_frame(
        &mut self,
        mut preparation: UiPortalProposalPreparation,
        frame: &crate::mounting::UiPreparedMountedFrame,
        focus: &mut crate::runtime::focus::UiFocusRuntimeState,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) -> Result<UiStagedPortalProposalTransaction, UiPortalProposalPreparationDenial> {
        if let Err(denial) =
            focus.stage_portal_proposal(&preparation.focus, frame.focus_participation_snapshot())
        {
            self.cancel_portal_staging(
                preparation.staging,
                &preparation.portal,
                &preparation.focus,
                motion_state,
                preparation.motion,
            );
            return Err(UiPortalProposalPreparationDenial::Focus(denial));
        }
        let receipt = crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::existing_preparation(
            preparation.staging.identity(),
        );
        if let Err(denial) = self
            .runtime
            .service_proposals
            .advance_staging(&mut preparation.staging, receipt)
        {
            focus
                .discard_portal_proposal(preparation.focus.proposal())
                .expect("Focus owner discards the exact proposal staged above");
            self.cancel_portal_staging(
                preparation.staging,
                &preparation.portal,
                &preparation.focus,
                motion_state,
                preparation.motion,
            );
            return Err(UiPortalProposalPreparationDenial::Staging(denial));
        }
        if let Err(denial) = self.runtime.service_proposals.advance_staging(
            &mut preparation.staging,
            preparation.focus.resolution_receipt(),
        ) {
            focus
                .discard_portal_proposal(preparation.focus.proposal())
                .expect("Focus owner discards the exact resolved proposal");
            self.cancel_portal_staging(
                preparation.staging,
                &preparation.portal,
                &preparation.focus,
                motion_state,
                preparation.motion,
            );
            return Err(UiPortalProposalPreparationDenial::Staging(denial));
        }
        let motion = preparation
            .motion
            .map(|motion| motion_state.derive(motion, frame.canonical_core().frame()));
        if let Some(receipt) = motion
            .as_ref()
            .map(crate::runtime::motion::UiDerivedMotionServiceProposal::derivation_receipt)
        {
            if let Err(denial) = self
                .runtime
                .service_proposals
                .advance_staging(&mut preparation.staging, receipt)
            {
                focus
                    .discard_portal_proposal(preparation.focus.proposal())
                    .expect("Focus owner discards the exact Motion derivation candidate");
                if let Some(motion) = motion {
                    motion_state.discard_derived(motion);
                }
                self.cancel_portal_staging(
                    preparation.staging,
                    &preparation.portal,
                    &preparation.focus,
                    motion_state,
                    None,
                );
                return Err(UiPortalProposalPreparationDenial::Staging(denial));
            }
        }
        match self
            .runtime
            .service_proposals
            .finish_staging(preparation.staging)
        {
            Ok(batch) => Ok(UiStagedPortalProposalTransaction {
                batch,
                portal: preparation.portal,
                focus: preparation.focus,
                motion,
                prepared_frame: frame.canonical_core().frame(),
            }),
            Err((staging, denial)) => {
                focus
                    .discard_portal_proposal(preparation.focus.proposal())
                    .expect("Focus owner discards the exact complete staging candidate");
                if let Some(motion) = motion {
                    motion_state.discard_derived(motion);
                }
                self.cancel_portal_staging(
                    staging,
                    &preparation.portal,
                    &preparation.focus,
                    motion_state,
                    None,
                );
                Err(UiPortalProposalPreparationDenial::Staging(denial))
            }
        }
    }

    pub(crate) fn cancel_portal_service_proposal(
        &mut self,
        transaction: UiStagedPortalProposalTransaction,
        focus: &mut crate::runtime::focus::UiFocusRuntimeState,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) {
        focus
            .discard_portal_proposal(transaction.focus.proposal())
            .expect("cancelled transaction retains its exact Focus proposal");
        let motion_scope = transaction.motion.as_ref().map(|motion| motion.scope());
        if let Some(motion) = transaction.motion {
            motion_state.discard_derived(motion);
        }
        let teardown = self
            .runtime
            .service_proposals
            .cancel_staged(transaction.batch);
        self.finish_portal_teardown(
            teardown,
            &transaction.portal,
            &transaction.focus,
            motion_scope,
            crate::runtime::session::service_proposal::UiServiceProposalTerminalReason::CancelledBeforePublication,
        );
    }

    pub(crate) fn cancel_portal_service_proposal_preparation(
        &mut self,
        preparation: UiPortalProposalPreparation,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) {
        self.cancel_portal_staging(
            preparation.staging,
            &preparation.portal,
            &preparation.focus,
            motion_state,
            preparation.motion,
        );
    }
}
