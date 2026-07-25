use super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationActivation,
    WorthUiPreparedApplicationCutoverOutcome,
};

mod admission;
mod outcome;

pub use outcome::{
    WorthUiMountedApplicationReplacementInFlight, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedReplacementAdmissionDenial, WorthUiMountedReplacementCompletionDenial,
    WorthUiMountedReplacementPreparationOutcome, WorthUiMountedReplacementRetentionDenial,
    WorthUiPreparedMountedApplicationReplacement,
};

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
        let (state, admission, capability_report) = admitted.into_parts();
        let outcome = state.session.mounted_presentation.present(
            admission.into_attempt(),
            state.session.host_session.effect_port(),
            crate::mounting::UiMountedHostPresentationAuthority::new(
                state.session.host_session.identity().as_u64(),
                state.session.host_session.protocol(),
                &capability_report,
                state.session.host_session.mounted_presentation_lease(),
            ),
            now,
        );
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
                WorthUiMountedApplicationReplacementOutcome::InFlight(Box::new(
                    WorthUiMountedApplicationReplacementInFlight {
                        session,
                        application,
                        mounted_successor,
                        publication,
                        handle,
                    },
                ))
            }
            crate::mounting::UiMountedPresentationOutcome::PresentationIndeterminate(frame) => {
                session.host_observations.record_indeterminate_frame(
                    frame.frame().canonical_core().frame(),
                    frame.report().affected_bindings(),
                );
                WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(Box::new(
                    frame,
                ))
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

    pub fn complete(
        mut self: Box<Self>,
        now: u64,
    ) -> WorthUiMountedApplicationReplacementOutcome<'session> {
        let observed = self.handle.clone();
        let outcome = self.session.mounted_presentation.complete(
            observed,
            self.session.host_session.effect_port(),
            now,
        );
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(denial) => {
                return WorthUiMountedApplicationReplacementOutcome::CompletionDenied(Box::new(
                    WorthUiMountedReplacementCompletionDenial {
                        denial,
                        in_flight: *self,
                    },
                ));
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
                WorthUiMountedApplicationReplacementOutcome::PresentationIndeterminate(Box::new(
                    frame,
                ))
            }
        }
    }
}
