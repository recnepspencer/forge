use crate::mounting::{
    UiMountedFrameOutcome, UiMountedFramePublicationCandidate, UiMountedFramePublicationReceipt,
    UiMountedFrameReconciliationCandidate, UiMountedFrameReuseWitness,
    UiMountedPresentationInFlight, UiMountedPresentationOutcome,
};

use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn present_prepared_mounted_frame(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> UiMountedFrameOutcome {
        let capability_report = self.host_session.capability_report().clone();
        let admission =
            match self
                .mounted_presentation
                .admit(frame, &capability_report, deadline, now)
            {
                Ok(admission) => admission,
                Err(rejection) => {
                    self.host_observations
                        .record_never_presented_frame(rejection.frame().canonical_core().frame());
                    return UiMountedFrameOutcome::AdmissionDenied(rejection);
                }
            };
        let reservation = UiMountedFramePublicationCandidate::reserve(
            &admission,
            self.mounted_identity.view().current_frame(),
        );
        let attempt = admission.attempt();
        let replaced = self
            .mounted_publication_reservations
            .insert(attempt, reservation);
        assert!(
            replaced.is_none(),
            "runtime-minted presentation attempts must be unique"
        );
        let outcome = self.mounted_presentation.present(
            admission.into_attempt(),
            self.host_session.effect_port(),
            crate::mounting::UiMountedHostPresentationAuthority::new(
                self.host_session.identity().as_u64(),
                self.host_session.protocol(),
                &capability_report,
                self.host_session.mounted_presentation_lease(),
            ),
            now,
        );
        self.finish_mounted_presentation(outcome)
    }

    pub fn present_current_mounted_frame_for_reconciliation(
        &mut self,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, crate::mounting::UiMountedIdentityDenial> {
        let current = self
            .mounted_identity
            .publication_receipt()
            .cloned()
            .ok_or(crate::mounting::UiMountedIdentityDenial::NoPublishedMountedFrame)?;
        let capability_report = self.host_session.capability_report().clone();
        let frame = self.mounted_identity.prepare_current_reconciliation_frame(
            replacements,
            self.host_session.protocol(),
            &capability_report,
        )?;
        let admission = match self.mounted_presentation.admit_reconciliation(
            frame,
            replacements,
            &capability_report,
            deadline,
            now,
        ) {
            Ok(admission) => admission,
            Err(rejection) => return Ok(UiMountedFrameOutcome::AdmissionDenied(rejection)),
        };
        let reservation =
            UiMountedFrameReconciliationCandidate::reserve(&admission, &current, replacements);
        let attempt = admission.attempt();
        let replaced = self
            .mounted_reconciliation_reservations
            .insert(attempt, reservation);
        assert!(
            replaced.is_none(),
            "runtime-minted reconciliation attempts must be unique"
        );
        let outcome = self.mounted_presentation.present(
            admission.into_attempt(),
            self.host_session.effect_port(),
            crate::mounting::UiMountedHostPresentationAuthority::new(
                self.host_session.identity().as_u64(),
                self.host_session.protocol(),
                &capability_report,
                self.host_session.mounted_presentation_lease(),
            ),
            now,
        );
        Ok(self.finish_mounted_presentation(outcome))
    }

    pub fn complete_mounted_presentation(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
        now: u64,
    ) -> UiMountedFrameOutcome {
        let outcome =
            self.mounted_presentation
                .complete(in_flight, self.host_session.effect_port(), now);
        match outcome {
            Ok(outcome) => self.finish_mounted_presentation(outcome),
            Err(denial) => UiMountedFrameOutcome::CompletionDenied(denial),
        }
    }

    pub fn cancel_mounted_presentation(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
    ) -> UiMountedFrameOutcome {
        let outcome = self
            .mounted_presentation
            .cancel(in_flight, self.host_session.effect_port());
        match outcome {
            Ok(outcome) => self.finish_mounted_presentation(outcome),
            Err(denial) => UiMountedFrameOutcome::CompletionDenied(denial),
        }
    }

    pub fn reuse_current_mounted_frame(
        &self,
        witness: &UiMountedFrameReuseWitness,
    ) -> Option<UiMountedFrameOutcome> {
        self.mounted_identity
            .reuse_receipt(witness)
            .map(UiMountedFrameOutcome::Unchanged)
    }

    pub fn current_mounted_publication(&self) -> Option<&UiMountedFramePublicationReceipt> {
        self.mounted_identity.publication_receipt()
    }

    pub fn reconcile_mounted_presentation(
        &mut self,
        reconciliation: crate::mounting::UiHostPresentationReconciliation,
    ) -> bool {
        self.mounted_presentation
            .reconcile(reconciliation, self.mounted_identity.view().current_frame())
    }

    pub(super) fn finish_mounted_presentation(
        &mut self,
        outcome: UiMountedPresentationOutcome,
    ) -> UiMountedFrameOutcome {
        let attempt = presentation_attempt(&outcome);
        if self
            .mounted_reconciliation_reservations
            .contains_key(&attempt)
        {
            return self.finish_mounted_reconciliation(outcome, attempt);
        }
        match outcome {
            UiMountedPresentationOutcome::Presented(presented) => {
                let attempt = presented.receipt().attempt();
                let reservation = self
                    .mounted_publication_reservations
                    .remove(&attempt)
                    .expect("every presented attempt has a pre-effect publication reservation");
                let receipt = reservation.commit_presented(presented, &mut self.mounted_identity);
                UiMountedFrameOutcome::Published(receipt)
            }
            UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                self.remove_mounted_publication_reservation(rejected.attempt());
                self.host_observations
                    .record_rejected_frame(rejected.frame().canonical_core().frame());
                UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
            }
            UiMountedPresentationOutcome::InFlight(in_flight) => {
                UiMountedFrameOutcome::InFlight(in_flight)
            }
            UiMountedPresentationOutcome::PresentationIndeterminate(indeterminate) => {
                self.remove_mounted_publication_reservation(indeterminate.report().attempt());
                self.host_observations.record_indeterminate_frame(
                    indeterminate.frame().canonical_core().frame(),
                    indeterminate.report().affected_bindings(),
                );
                UiMountedFrameOutcome::PresentationIndeterminate(indeterminate)
            }
        }
    }

    fn finish_mounted_reconciliation(
        &mut self,
        outcome: UiMountedPresentationOutcome,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) -> UiMountedFrameOutcome {
        match outcome {
            UiMountedPresentationOutcome::Presented(presented) => {
                let reservation = self
                    .mounted_reconciliation_reservations
                    .remove(&attempt)
                    .expect("every reconciliation presentation retains a reservation");
                self.mounted_presentation
                    .commit_current_frame_reconciliation(reservation.replacements());
                UiMountedFrameOutcome::Reconciled(
                    reservation.commit_presented(presented, &mut self.mounted_identity),
                )
            }
            UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
                self.remove_mounted_reconciliation_reservation(rejected.attempt());
                UiMountedFrameOutcome::RejectedBeforeEffects(rejected)
            }
            UiMountedPresentationOutcome::InFlight(in_flight) => {
                UiMountedFrameOutcome::InFlight(in_flight)
            }
            UiMountedPresentationOutcome::PresentationIndeterminate(indeterminate) => {
                self.remove_mounted_reconciliation_reservation(indeterminate.report().attempt());
                self.host_observations.record_indeterminate_frame(
                    indeterminate.frame().canonical_core().frame(),
                    indeterminate.report().affected_bindings(),
                );
                UiMountedFrameOutcome::PresentationIndeterminate(indeterminate)
            }
        }
    }

    fn remove_mounted_publication_reservation(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) {
        self.mounted_publication_reservations
            .remove(&attempt)
            .expect("every admitted attempt has a publication reservation");
    }

    fn remove_mounted_reconciliation_reservation(
        &mut self,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    ) {
        self.mounted_reconciliation_reservations
            .remove(&attempt)
            .expect("every admitted reconciliation has a reservation");
    }
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
