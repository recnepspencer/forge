use super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationActivation,
    WorthUiPreparedApplicationCutoverOutcome,
};

mod admission;
mod outcome;

pub use outcome::{
    WorthUiMountedApplicationReplacementInFlight,
    WorthUiMountedApplicationReplacementIndeterminate, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedReplacementAdmissionDenial, WorthUiMountedReplacementCompletionDenial,
    WorthUiMountedReplacementPreparationOutcome, WorthUiMountedReplacementRetentionDenial,
    WorthUiPreparedMountedApplicationReplacement,
};

struct WorthUiPresentedApplicationReplacement<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    application: Box<WorthUiPreparedApplicationActivation>,
    mounted_successor: crate::mounting::UiMountedGraphReplacementSuccessor,
    mounted_receipt: crate::mounting::UiMountedFramePublicationReceipt,
}

impl WorthUiActiveApplicationSession {
    pub fn prepare_mounted_replacement(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
        request: crate::mounting::UiMountedFrameRequest,
    ) -> Result<WorthUiMountedReplacementPreparationOutcome<'_>, WorthUiApplicationCutoverDenial>
    {
        self.prepare_mounted_replacement_with_content(
            pending,
            admitted_delta,
            boundary,
            lane_parity_report,
            crate::mounting::UiMountedSemanticContentInput::empty(),
            request,
        )
    }

    pub(crate) fn prepare_mounted_replacement_with_content(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
        semantic_content: crate::mounting::UiMountedSemanticContentInput,
        request: crate::mounting::UiMountedFrameRequest,
    ) -> Result<WorthUiMountedReplacementPreparationOutcome<'_>, WorthUiApplicationCutoverDenial>
    {
        let candidate_graph = pending.next_app.graph_snapshot().clone();
        let candidate_generation = pending.next_app.generation_identity().clone();
        let prepared = self.prepare_application_cutover(
            pending,
            admitted_delta,
            boundary,
            lane_parity_report,
        )?;
        let application = match prepared {
            WorthUiPreparedApplicationCutoverOutcome::SemanticNoOp(receipt) => {
                return Ok(WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(
                    receipt,
                ));
            }
            WorthUiPreparedApplicationCutoverOutcome::Activation(application) => application,
        };
        let mounted_successor = self
            .mounted
            .prepare_graph_replacement_successor(crate::graph::UiGraphAuthority::new(
                &candidate_graph,
            ))
            .map_err(WorthUiApplicationCutoverDenial::MountedIdentity)?;
        let capability_report = self.host_session.capability_report();
        let frame = super::mounted_frame::prepare_candidate_mounted_frame(
            &application,
            &mounted_successor,
            crate::graph::UiGraphAuthority::new(&candidate_graph),
            super::mounted_frame::UiMountedReplacementReuseBasis {
                generation: candidate_generation,
                host_session: self.host_session.identity().as_u64(),
                protocol: self.host_session.protocol(),
                capability_generation: capability_report.observation_generation(),
                capability_profile_digest: capability_report.profile_identity_digest(),
            },
            semantic_content,
            request,
        )
        .map_err(WorthUiApplicationCutoverDenial::MountedFrame)?;
        Ok(WorthUiMountedReplacementPreparationOutcome::Prepared(
            Box::new(WorthUiPreparedMountedApplicationReplacement {
                session: self,
                application,
                mounted_successor,
                frame,
            }),
        ))
    }
}

impl<'session> WorthUiPreparedMountedApplicationReplacement<'session> {
    pub fn frame(&self) -> &crate::mounting::UiPreparedMountedFrame {
        &self.frame
    }

    pub fn present(
        self: Box<Self>,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        self.present_with_publication_tail(deadline, now, |presented| presented.commit_once())
    }

    #[cfg(feature = "certification-support")]
    #[doc(hidden)]
    pub fn present_observing_publication_tail_for_certification(
        self: Box<Self>,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
        observe: impl FnOnce(&mut dyn FnMut()),
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        self.present_with_publication_tail(deadline, now, |presented| {
            let mut presented = Some(presented);
            let mut outcome = None;
            let mut commit = || {
                outcome = Some(
                    presented
                        .take()
                        .expect("certification observer invokes the tail once")
                        .commit_once(),
                );
            };
            observe(&mut commit);
            assert!(
                presented.is_none(),
                "certification observer must invoke the publication tail"
            );
            outcome.expect("publication-tail observer produces the real outcome")
        })
    }

    fn present_with_publication_tail(
        self: Box<Self>,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
        publish: impl FnOnce(
            WorthUiPresentedApplicationReplacement<'session>,
        ) -> WorthUiMountedApplicationReplacementOutcome<'session>,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let Self {
            session,
            application,
            mounted_successor,
            frame,
        } = *self;
        let admitted = match admission::prepare_replacement_presentation(
            admission::WorthUiMountedReplacementAdmissionInput {
                session,
                application,
                mounted_successor,
                frame,
            },
            deadline,
            now,
        ) {
            Ok(admitted) => admitted,
            Err(outcome) => return *outcome,
        };
        let admission::WorthUiAdmittedMountedReplacement {
            session,
            application,
            mounted,
        } = admitted;
        let outcome =
            session
                .mounted
                .present_graph_replacement(&session.host_session, mounted, now);
        Self::finish(session, application, outcome, publish)
    }

    fn finish(
        session: &'session mut WorthUiActiveApplicationSession,
        application: Box<WorthUiPreparedApplicationActivation>,
        outcome: crate::mounting::UiMountedGraphReplacementPresentation,
        publish: impl FnOnce(
            WorthUiPresentedApplicationReplacement<'session>,
        ) -> WorthUiMountedApplicationReplacementOutcome<'session>,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        match outcome {
            crate::mounting::UiMountedGraphReplacementPresentation::Published {
                successor,
                receipt,
            } => publish(WorthUiPresentedApplicationReplacement {
                session,
                application,
                mounted_successor: successor,
                mounted_receipt: receipt,
            }),
            crate::mounting::UiMountedGraphReplacementPresentation::RejectedBeforeEffects {
                successor,
                frame,
                observation,
            } => {
                crate::facade::entry::mounted_publication::record_mounted_observation(
                    &mut session.host_exchange,
                    observation,
                );
                WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(Box::new(Self {
                    session,
                    application,
                    mounted_successor: successor,
                    frame,
                }))
            }
            crate::mounting::UiMountedGraphReplacementPresentation::InFlight(mounted) => {
                WorthUiMountedApplicationReplacementOutcome::InFlight(Box::new(
                    WorthUiMountedApplicationReplacementInFlight {
                        session,
                        application,
                        mounted,
                    },
                ))
            }
            crate::mounting::UiMountedGraphReplacementPresentation::PresentationIndeterminate {
                frame,
                observation,
            } => {
                crate::facade::entry::mounted_publication::record_mounted_observation(
                    &mut session.host_exchange,
                    observation,
                );
                WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(Box::new(
                    WorthUiMountedApplicationReplacementIndeterminate {
                        session,
                        application,
                        frame,
                    },
                ))
            }
        }
    }
}

impl<'session> WorthUiPresentedApplicationReplacement<'session> {
    fn commit_once(self) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let application = self
            .session
            .commit_application_activation(self.application, self.mounted_successor);
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted: self.mounted_receipt,
        }
    }
}

impl<'session> WorthUiMountedApplicationReplacementInFlight<'session> {
    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.mounted.handle().attempt()
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.mounted.handle().deadline()
    }

    pub fn pending_bindings(
        &self,
    ) -> impl ExactSizeIterator<Item = worth_ui_host_contract::UiSurfaceBindingGeneration> + '_
    {
        self.mounted.handle().pending_bindings()
    }

    pub fn cost_report(&self) -> crate::mounting::UiMountCostReport {
        self.mounted.handle().cost_report()
    }

    pub fn complete(
        self: Box<Self>,
        now: u64,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let Self {
            session,
            application,
            mounted,
        } = *self;
        let outcome =
            session
                .mounted
                .complete_graph_replacement(&session.host_session, mounted, now);
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(rejection) => {
                return WorthUiMountedApplicationReplacementOutcome::CompletionDenied(Box::new(
                    WorthUiMountedReplacementCompletionDenial {
                        denial: rejection.denial,
                        in_flight: WorthUiMountedApplicationReplacementInFlight {
                            session,
                            application,
                            mounted: *rejection.in_flight,
                        },
                    },
                ));
            }
        };
        WorthUiPreparedMountedApplicationReplacement::finish(
            session,
            application,
            outcome,
            |presented| presented.commit_once(),
        )
    }

    pub fn cancel(self: Box<Self>) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let Self {
            session,
            application,
            mounted,
        } = *self;
        let outcome = session
            .mounted
            .cancel_graph_replacement(&session.host_session, mounted);
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(rejection) => {
                return WorthUiMountedApplicationReplacementOutcome::CompletionDenied(Box::new(
                    WorthUiMountedReplacementCompletionDenial {
                        denial: rejection.denial,
                        in_flight: WorthUiMountedApplicationReplacementInFlight {
                            session,
                            application,
                            mounted: *rejection.in_flight,
                        },
                    },
                ));
            }
        };
        WorthUiPreparedMountedApplicationReplacement::finish(
            session,
            application,
            outcome,
            |presented| presented.commit_once(),
        )
    }
}
