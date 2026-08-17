use super::WorthUiMountedSessionState;
use crate::mounting::{
    UiMountedFrameOutcome, UiMountedFramePublicationCandidate, UiMountedFramePublicationReceipt,
    UiMountedFrameReconciliationCandidate, UiMountedPresentationInFlight,
    UiMountedPresentationOutcome,
};

#[derive(Clone, Copy)]
pub(crate) struct UiMountedObservationValidationBasis<'session> {
    retention: &'session crate::mounting::UiMountedFrameRetentionCoordinator,
    presentation: &'session crate::mounting::UiMountedPresentationCoordinator,
}

pub(crate) struct UiMountedPublicationTransition {
    outcome: UiMountedFrameOutcome,
    observation: Option<UiMountedHostObservationTransition>,
}

pub(crate) enum UiMountedHostObservationTransition {
    NeverPresented(worth_ui_host_contract::UiMountedFrameIdentity),
    Rejected(worth_ui_host_contract::UiMountedFrameIdentity),
    Indeterminate {
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        bindings: Box<[worth_ui_host_contract::UiSurfaceBindingGeneration]>,
    },
}

impl WorthUiMountedSessionState {
    pub(crate) fn observation_validation_basis(&self) -> UiMountedObservationValidationBasis<'_> {
        UiMountedObservationValidationBasis {
            retention: &self.retention,
            presentation: &self.presentation,
        }
    }

    pub(crate) fn present_prepared_frame(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> UiMountedPublicationTransition {
        let capability_report = host.capability_report().clone();
        let admitted = match self.identity.admit_prepared_frame_authority(frame) {
            Ok(admitted) => admitted,
            Err(rejection) => {
                let frame = rejection.frame().canonical_core().frame();
                return UiMountedPublicationTransition::with_observation(
                    UiMountedFrameOutcome::AdmissionDenied(rejection),
                    UiMountedHostObservationTransition::NeverPresented(frame),
                );
            }
        };
        let retained = match self.retention.prepare_publication(admitted) {
            Ok(retained) => retained,
            Err(rejection) => {
                let frame = rejection.frame().canonical_core().frame();
                return UiMountedPublicationTransition::with_observation(
                    UiMountedFrameOutcome::RetentionDenied(rejection),
                    UiMountedHostObservationTransition::NeverPresented(frame),
                );
            }
        };
        let admission =
            match self
                .presentation
                .admit_current(retained, &capability_report, deadline, now)
            {
                Ok(admission) => admission,
                Err(rejection) => {
                    let frame = rejection.frame().canonical_core().frame();
                    return UiMountedPublicationTransition::with_observation(
                        UiMountedFrameOutcome::AdmissionDenied(rejection),
                        UiMountedHostObservationTransition::NeverPresented(frame),
                    );
                }
            };
        let reservation = UiMountedFramePublicationCandidate::reserve(
            &admission,
            self.identity.view().current_frame(),
        );
        let attempt = admission.attempt();
        let replaced = self.publication_reservations.insert(attempt, reservation);
        assert!(
            replaced.is_none(),
            "runtime-minted presentation attempts must be unique"
        );
        let outcome = self.presentation.present(
            admission.into_attempt(),
            host.effect_port(),
            mounted_host_authority(host, &capability_report),
            now,
        );
        self.finish_presentation(outcome)
    }

    pub(crate) fn present_current_for_reconciliation(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedPublicationTransition, crate::mounting::UiMountedIdentityDenial> {
        let current = self
            .identity
            .publication_receipt()
            .cloned()
            .ok_or(crate::mounting::UiMountedIdentityDenial::NoPublishedMountedFrame)?;
        let capability_report = host.capability_report().clone();
        let frame = self.identity.prepare_current_reconciliation_frame(
            replacements,
            host.protocol(),
            &capability_report,
        )?;
        let retained = match self.retention.prepare_reconciliation(frame) {
            Ok(retained) => retained,
            Err(rejection) => {
                return Ok(UiMountedPublicationTransition::new(
                    UiMountedFrameOutcome::RetentionDenied(rejection),
                ));
            }
        };
        let admission = match self.presentation.admit_reconciliation(
            retained,
            replacements,
            &capability_report,
            deadline,
            now,
        ) {
            Ok(admission) => admission,
            Err(rejection) => {
                return Ok(UiMountedPublicationTransition::new(
                    UiMountedFrameOutcome::AdmissionDenied(rejection),
                ));
            }
        };
        let reservation =
            UiMountedFrameReconciliationCandidate::reserve(&admission, &current, replacements);
        let attempt = admission.attempt();
        let replaced = self
            .reconciliation_reservations
            .insert(attempt, reservation);
        assert!(
            replaced.is_none(),
            "runtime-minted reconciliation attempts must be unique"
        );
        let outcome = self.presentation.present(
            admission.into_attempt(),
            host.effect_port(),
            mounted_host_authority(host, &capability_report),
            now,
        );
        Ok(self.finish_presentation(outcome))
    }

    pub(crate) fn complete_presentation(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        in_flight: UiMountedPresentationInFlight,
        now: u64,
    ) -> UiMountedPublicationTransition {
        match self
            .presentation
            .complete(in_flight, host.effect_port(), now)
        {
            Ok(outcome) => self.finish_presentation(outcome),
            Err(denial) => {
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::CompletionDenied(denial))
            }
        }
    }

    pub(crate) fn cancel_presentation(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        in_flight: UiMountedPresentationInFlight,
    ) -> UiMountedPublicationTransition {
        match self.presentation.cancel(in_flight, host.effect_port()) {
            Ok(outcome) => self.finish_presentation(outcome),
            Err(denial) => {
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::CompletionDenied(denial))
            }
        }
    }

    pub(crate) fn supersede_presentation(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        in_flight: UiMountedPresentationInFlight,
    ) -> UiMountedPublicationTransition {
        match self.presentation.supersede(in_flight, host.effect_port()) {
            Ok(outcome) => self.finish_presentation(outcome),
            Err(denial) => {
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::CompletionDenied(denial))
            }
        }
    }

    pub(crate) fn finish_presentation(
        &mut self,
        outcome: UiMountedPresentationOutcome,
    ) -> UiMountedPublicationTransition {
        let attempt = presentation_attempt(&outcome);
        if self.reconciliation_reservations.contains_key(&attempt) {
            return self.finish_reconciliation(outcome, attempt);
        }
        match outcome {
            UiMountedPresentationOutcome::Presented(presented) => {
                let attempt = presented.receipt().attempt();
                let reservation = self
                    .publication_reservations
                    .remove(&attempt)
                    .expect("every presented attempt has a pre-effect publication reservation");
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::Published(
                    reservation.commit_presented(presented, &mut self.identity),
                ))
            }
            UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                self.remove_publication_reservation(rejected.attempt());
                let frame = rejected.frame().canonical_core().frame();
                UiMountedPublicationTransition::with_observation(
                    UiMountedFrameOutcome::RejectedBeforeEffects(rejected),
                    UiMountedHostObservationTransition::Rejected(frame),
                )
            }
            UiMountedPresentationOutcome::InFlight(in_flight) => {
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::InFlight(in_flight))
            }
            UiMountedPresentationOutcome::PresentationIndeterminate(indeterminate) => {
                self.remove_publication_reservation(indeterminate.report().attempt());
                let observation = indeterminate_observation(&indeterminate);
                UiMountedPublicationTransition::with_observation(
                    UiMountedFrameOutcome::PresentationIndeterminate(indeterminate),
                    observation,
                )
            }
        }
    }

    pub(crate) fn current_publication(&self) -> Option<&UiMountedFramePublicationReceipt> {
        self.identity.publication_receipt()
    }

    pub(crate) fn reconcile_presentation(
        &mut self,
        reconciliation: crate::mounting::UiHostPresentationReconciliation,
    ) -> bool {
        self.presentation
            .reconcile(reconciliation, self.identity.view().current_frame())
    }

    fn finish_reconciliation(
        &mut self,
        outcome: UiMountedPresentationOutcome,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) -> UiMountedPublicationTransition {
        match outcome {
            UiMountedPresentationOutcome::Presented(presented) => {
                let reservation = self
                    .reconciliation_reservations
                    .remove(&attempt)
                    .expect("every reconciliation presentation retains a reservation");
                self.presentation
                    .commit_current_frame_reconciliation(reservation.replacements());
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::Reconciled(
                    reservation.commit_presented(presented, &mut self.identity),
                ))
            }
            UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                self.remove_reconciliation_reservation(rejected.attempt());
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::RejectedBeforeEffects(
                    rejected,
                ))
            }
            UiMountedPresentationOutcome::InFlight(in_flight) => {
                UiMountedPublicationTransition::new(UiMountedFrameOutcome::InFlight(in_flight))
            }
            UiMountedPresentationOutcome::PresentationIndeterminate(indeterminate) => {
                self.remove_reconciliation_reservation(indeterminate.report().attempt());
                let observation = indeterminate_observation(&indeterminate);
                UiMountedPublicationTransition::with_observation(
                    UiMountedFrameOutcome::PresentationIndeterminate(indeterminate),
                    observation,
                )
            }
        }
    }

    fn remove_publication_reservation(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) {
        self.publication_reservations
            .remove(&attempt)
            .expect("every admitted attempt has a publication reservation");
    }

    fn remove_reconciliation_reservation(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) {
        self.reconciliation_reservations
            .remove(&attempt)
            .expect("every admitted reconciliation has a reservation");
    }
}

impl UiMountedObservationValidationBasis<'_> {
    pub(crate) fn retention(&self) -> &crate::mounting::UiMountedFrameRetentionCoordinator {
        self.retention
    }

    pub(crate) fn binding_requires_reconciliation(
        self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> bool {
        self.presentation.binding_requires_reconciliation(binding)
    }
}

impl UiMountedPublicationTransition {
    fn new(outcome: UiMountedFrameOutcome) -> Self {
        Self {
            outcome,
            observation: None,
        }
    }

    fn with_observation(
        outcome: UiMountedFrameOutcome,
        observation: UiMountedHostObservationTransition,
    ) -> Self {
        Self {
            outcome,
            observation: Some(observation),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiMountedFrameOutcome,
        Option<UiMountedHostObservationTransition>,
    ) {
        (self.outcome, self.observation)
    }
}

pub(super) fn mounted_host_authority<'host>(
    host: &'host crate::facade::WorthUiHostSessionAuthority,
    capability_report: &'host worth_ui_host_contract::WorthUiHostCapabilityReport,
) -> crate::mounting::UiMountedHostPresentationAuthority<'host> {
    crate::mounting::UiMountedHostPresentationAuthority::new(
        host.identity().as_u64(),
        host.protocol(),
        capability_report,
        host.mounted_presentation_lease(),
    )
}

fn presentation_attempt(
    outcome: &UiMountedPresentationOutcome,
) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
    match outcome {
        UiMountedPresentationOutcome::Presented(frame) => frame.receipt().attempt(),
        UiMountedPresentationOutcome::RejectedBeforeEffects(frame) => frame.attempt(),
        UiMountedPresentationOutcome::InFlight(frame) => frame.attempt(),
        UiMountedPresentationOutcome::PresentationIndeterminate(frame) => frame.report().attempt(),
    }
}

pub(super) fn indeterminate_observation(
    frame: &crate::mounting::UiMountedIndeterminateFrame,
) -> UiMountedHostObservationTransition {
    UiMountedHostObservationTransition::Indeterminate {
        frame: frame.frame().canonical_core().frame(),
        bindings: frame
            .report()
            .affected_bindings()
            .to_vec()
            .into_boxed_slice(),
    }
}
