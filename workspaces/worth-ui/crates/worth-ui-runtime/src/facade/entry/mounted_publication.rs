use crate::mounting::{
    UiMountedFrameOutcome, UiMountedFramePublicationReceipt, UiMountedPresentationInFlight,
    UiMountedPresentationOutcome,
};

use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(crate) fn present_prepared_mounted_frame_internal(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> UiMountedFrameOutcome {
        let transition =
            self.mounted
                .present_prepared_frame(&self.host_session, frame, deadline, now);
        finish_mounted_transition(&mut self.host_exchange, transition)
    }

    pub(crate) fn present_prepared_superseding_mounted_frame_internal(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        predecessor: crate::mounting::UiMountedSupersedingPresentationBasis,
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> UiMountedFrameOutcome {
        let transition = self.mounted.present_prepared_superseding_frame(
            &self.host_session,
            frame,
            predecessor,
            deadline,
            now,
        );
        finish_mounted_transition(&mut self.host_exchange, transition)
    }

    pub fn present_current_mounted_frame_for_reconciliation(
        &mut self,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, crate::mounting::UiMountedIdentityDenial> {
        let transition = self.mounted.present_current_for_reconciliation(
            &self.host_session,
            replacements,
            deadline,
            now,
        )?;
        Ok(finish_mounted_transition(
            &mut self.host_exchange,
            transition,
        ))
    }

    pub(crate) fn present_prepared_mounted_frame_for_reconciliation(
        &mut self,
        frame: crate::mounting::UiPreparedMountedFrame,
        replacements: &[crate::mounting::UiMountedSurfaceReconciliationBinding],
        deadline: worth_ui_host_contract::UiPresentationDeadline,
        now: u64,
    ) -> Result<UiMountedFrameOutcome, crate::mounting::UiMountedIdentityDenial> {
        let transition = self.mounted.present_prepared_for_reconciliation(
            &self.host_session,
            frame,
            replacements,
            deadline,
            now,
        )?;
        Ok(finish_mounted_transition(
            &mut self.host_exchange,
            transition,
        ))
    }

    pub fn complete_mounted_presentation(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
        now: u64,
    ) -> UiMountedFrameOutcome {
        let transition = self
            .mounted
            .complete_presentation(&self.host_session, in_flight, now);
        finish_mounted_transition(&mut self.host_exchange, transition)
    }

    pub fn cancel_mounted_presentation(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
    ) -> UiMountedFrameOutcome {
        let transition = self
            .mounted
            .cancel_presentation(&self.host_session, in_flight);
        finish_mounted_transition(&mut self.host_exchange, transition)
    }

    pub(crate) fn supersede_mounted_presentation(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
    ) -> UiMountedFrameOutcome {
        let transition = self
            .mounted
            .supersede_presentation(&self.host_session, in_flight);
        finish_mounted_transition(&mut self.host_exchange, transition)
    }

    pub(crate) fn admit_duplicate_native_presentation_observation(
        &mut self,
        presentation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), ()> {
        self.mounted
            .admit_duplicate_native_presentation_observation(presentation)
    }

    pub fn current_mounted_publication(&self) -> Option<&UiMountedFramePublicationReceipt> {
        self.mounted.current_publication()
    }

    pub fn reconcile_mounted_presentation(
        &mut self,
        reconciliation: crate::mounting::UiHostPresentationReconciliation,
    ) -> bool {
        self.mounted.reconcile_presentation(reconciliation)
    }

    pub(super) fn finish_mounted_presentation(
        &mut self,
        outcome: UiMountedPresentationOutcome,
    ) -> UiMountedFrameOutcome {
        let transition = self.mounted.finish_presentation(outcome);
        finish_mounted_transition(&mut self.host_exchange, transition)
    }
}

pub(super) fn finish_mounted_transition(
    host_exchange: &mut crate::host_exchange::WorthUiHostExchangeSessionState,
    transition: crate::mounting::UiMountedPublicationTransition,
) -> UiMountedFrameOutcome {
    let (outcome, observation) = transition.into_parts();
    match &outcome {
        UiMountedFrameOutcome::Published(receipt)
        | UiMountedFrameOutcome::Unchanged(receipt)
        | UiMountedFrameOutcome::Reconciled(receipt) => {
            host_exchange.record_presented_frame(receipt.frame());
        }
        _ => {}
    }
    if let Some(observation) = observation {
        record_mounted_observation(host_exchange, observation);
    }
    outcome
}

pub(super) fn record_mounted_observation(
    host_exchange: &mut crate::host_exchange::WorthUiHostExchangeSessionState,
    observation: crate::mounting::UiMountedHostObservationTransition,
) {
    match observation {
        crate::mounting::UiMountedHostObservationTransition::NeverPresented(frame) => {
            host_exchange.record_never_presented_frame(frame);
        }
        crate::mounting::UiMountedHostObservationTransition::Rejected(frame) => {
            host_exchange.record_rejected_frame(frame);
        }
        crate::mounting::UiMountedHostObservationTransition::Indeterminate { frame, bindings } => {
            host_exchange.record_indeterminate_frame(frame, &bindings);
        }
    }
}
