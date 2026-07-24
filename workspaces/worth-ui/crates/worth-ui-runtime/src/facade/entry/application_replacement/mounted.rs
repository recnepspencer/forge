use super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiApplicationCutoverReceipt, WorthUiApplicationSemanticNoOpReceipt,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationActivation,
    WorthUiPreparedApplicationCutoverOutcome,
};

pub struct WorthUiPreparedMountedApplicationReplacement<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    application: Box<WorthUiPreparedApplicationActivation>,
    mounted_successor: crate::mounting::UiMountedIdentityState,
    frame: crate::mounting::UiPreparedMountedFrame,
}

pub struct WorthUiMountedApplicationReplacementInFlight<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    application: Box<WorthUiPreparedApplicationActivation>,
    mounted_successor: crate::mounting::UiMountedIdentityState,
    publication: crate::mounting::UiMountedFramePublicationCandidate,
    handle: crate::mounting::UiMountedPresentationInFlight,
}

pub struct WorthUiMountedReplacementAdmissionDenial<'session> {
    denial: crate::mounting::UiMountedPresentationAdmissionDenial,
    replacement: Box<WorthUiPreparedMountedApplicationReplacement<'session>>,
}

pub struct WorthUiMountedReplacementCompletionDenial<'session> {
    denial: crate::mounting::UiMountedPresentationCompletionDenial,
    in_flight: WorthUiMountedApplicationReplacementInFlight<'session>,
}

struct WorthUiPresentedApplicationReplacement<'session> {
    state: WorthUiMountedReplacementPublicationState<'session>,
    presented: crate::mounting::UiMountedPresentedFrame,
}

struct WorthUiMountedReplacementPublicationState<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    application: Box<WorthUiPreparedApplicationActivation>,
    mounted_successor: crate::mounting::UiMountedIdentityState,
    publication: crate::mounting::UiMountedFramePublicationCandidate,
}

pub enum WorthUiMountedReplacementPreparationOutcome<'session> {
    SemanticNoOp(Box<WorthUiApplicationSemanticNoOpReceipt>),
    Prepared(Box<WorthUiPreparedMountedApplicationReplacement<'session>>),
}

pub enum WorthUiMountedApplicationReplacementOutcome<'session> {
    Published {
        application: WorthUiApplicationCutoverReceipt,
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
    },
    RejectedBeforeEffects(Box<WorthUiPreparedMountedApplicationReplacement<'session>>),
    InFlight(WorthUiMountedApplicationReplacementInFlight<'session>),
    PresentationIndeterminate(crate::mounting::UiMountedIndeterminateFrame),
    AdmissionDenied(WorthUiMountedReplacementAdmissionDenial<'session>),
    CompletionDenied(WorthUiMountedReplacementCompletionDenial<'session>),
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
        let mut mounted_successor = self
            .mounted_identity
            .prepare_graph_replacement_successor(crate::graph::UiGraphAuthority::new(
                &candidate_graph,
            ))
            .map_err(WorthUiApplicationCutoverDenial::MountedIdentity)?;
        let capability_report = self.host_session.capability_report();
        let frame = super::mounted_frame::prepare_candidate_mounted_frame(
            &application,
            &mut mounted_successor,
            crate::graph::UiGraphAuthority::new(&candidate_graph),
            super::mounted_frame::UiMountedReplacementReuseBasis {
                generation: candidate_generation,
                host_session: self.host_session.identity().as_u64(),
                protocol: self.host_session.protocol(),
                capability_generation: capability_report.observation_generation(),
                capability_profile_digest: capability_report.profile_identity_digest(),
            },
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
        let capability_report = session.host_session.capability_report().clone();
        let admission = match session.mounted_presentation.admit_current(
            &mounted_successor,
            frame,
            &capability_report,
            deadline,
            now,
        ) {
            Ok(admission) => admission,
            Err(rejection) => {
                let denial = rejection.denial();
                session
                    .host_observations
                    .record_never_presented_frame(rejection.frame().canonical_core().frame());
                return WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(
                    WorthUiMountedReplacementAdmissionDenial {
                        denial,
                        replacement: Box::new(Self {
                            session,
                            application,
                            mounted_successor,
                            frame: rejection.into_frame(),
                        }),
                    },
                );
            }
        };
        let publication = crate::mounting::UiMountedFramePublicationCandidate::reserve(
            &admission,
            session.mounted_identity.view().current_frame(),
        );
        let outcome = session.mounted_presentation.present(
            admission.into_attempt(),
            session.host_session.effect_port(),
            crate::mounting::UiMountedHostPresentationAuthority::new(
                session.host_session.identity().as_u64(),
                session.host_session.protocol(),
                &capability_report,
                session.host_session.mounted_presentation_lease(),
            ),
            now,
        );
        let state = WorthUiMountedReplacementPublicationState {
            session,
            application,
            mounted_successor,
            publication,
        };
        Self::finish(state, outcome, publish)
    }

    fn finish(
        state: WorthUiMountedReplacementPublicationState<'session>,
        outcome: crate::mounting::UiMountedPresentationOutcome,
        publish: impl FnOnce(
            WorthUiPresentedApplicationReplacement<'session>,
        ) -> WorthUiMountedApplicationReplacementOutcome<'session>,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let WorthUiMountedReplacementPublicationState {
            session,
            application,
            mounted_successor,
            publication,
        } = state;
        match outcome {
            crate::mounting::UiMountedPresentationOutcome::Presented(presented) => {
                publish(WorthUiPresentedApplicationReplacement {
                    state: WorthUiMountedReplacementPublicationState {
                        session,
                        application,
                        mounted_successor,
                        publication,
                    },
                    presented,
                })
            }
            crate::mounting::UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                session
                    .host_observations
                    .record_rejected_frame(rejected.frame().canonical_core().frame());
                WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(Box::new(Self {
                    session,
                    application,
                    mounted_successor,
                    frame: rejected.into_frame(),
                }))
            }
            crate::mounting::UiMountedPresentationOutcome::InFlight(handle) => {
                WorthUiMountedApplicationReplacementOutcome::InFlight(
                    WorthUiMountedApplicationReplacementInFlight {
                        session,
                        application,
                        mounted_successor,
                        publication,
                        handle,
                    },
                )
            }
            crate::mounting::UiMountedPresentationOutcome::PresentationIndeterminate(frame) => {
                session.host_observations.record_indeterminate_frame(
                    frame.frame().canonical_core().frame(),
                    frame.report().affected_bindings(),
                );
                WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(frame)
            }
        }
    }
}

impl<'session> WorthUiPresentedApplicationReplacement<'session> {
    fn commit_once(self) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let mut state = self.state;
        let mounted = state
            .publication
            .commit_presented(self.presented, &mut state.mounted_successor);
        let application = state
            .session
            .commit_application_activation(state.application, state.mounted_successor);
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        }
    }
}

impl<'session> WorthUiMountedApplicationReplacementInFlight<'session> {
    pub fn handle(&self) -> &crate::mounting::UiMountedPresentationInFlight {
        &self.handle
    }

    pub fn complete(mut self, now: u64) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let observed = self.handle.clone();
        let outcome = self.session.mounted_presentation.complete(
            observed,
            self.session.host_session.effect_port(),
            now,
        );
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(denial) => {
                return WorthUiMountedApplicationReplacementOutcome::CompletionDenied(
                    WorthUiMountedReplacementCompletionDenial {
                        denial,
                        in_flight: self,
                    },
                );
            }
        };
        match outcome {
            crate::mounting::UiMountedPresentationOutcome::Presented(presented) => {
                let receipt = self
                    .publication
                    .commit_presented(presented, &mut self.mounted_successor);
                let application = self
                    .session
                    .commit_application_activation(self.application, self.mounted_successor);
                WorthUiMountedApplicationReplacementOutcome::Published {
                    application,
                    mounted: receipt,
                }
            }
            crate::mounting::UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                self.session
                    .host_observations
                    .record_rejected_frame(rejected.frame().canonical_core().frame());
                WorthUiMountedApplicationReplacementOutcome::RejectedBeforeEffects(Box::new(
                    WorthUiPreparedMountedApplicationReplacement {
                        session: self.session,
                        application: self.application,
                        mounted_successor: self.mounted_successor,
                        frame: rejected.into_frame(),
                    },
                ))
            }
            crate::mounting::UiMountedPresentationOutcome::InFlight(handle) => {
                self.handle = handle;
                WorthUiMountedApplicationReplacementOutcome::InFlight(self)
            }
            crate::mounting::UiMountedPresentationOutcome::PresentationIndeterminate(frame) => {
                self.session.host_observations.record_indeterminate_frame(
                    frame.frame().canonical_core().frame(),
                    frame.report().affected_bindings(),
                );
                WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(frame)
            }
        }
    }
}

impl<'session> WorthUiMountedReplacementAdmissionDenial<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationAdmissionDenial {
        self.denial
    }

    pub fn into_replacement(self) -> Box<WorthUiPreparedMountedApplicationReplacement<'session>> {
        self.replacement
    }
}

impl<'session> WorthUiMountedReplacementCompletionDenial<'session> {
    pub fn denial(&self) -> crate::mounting::UiMountedPresentationCompletionDenial {
        self.denial
    }

    pub fn into_in_flight(self) -> WorthUiMountedApplicationReplacementInFlight<'session> {
        self.in_flight
    }
}
