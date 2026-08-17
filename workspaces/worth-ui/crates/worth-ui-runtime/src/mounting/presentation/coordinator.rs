use std::collections::{BTreeMap, BTreeSet};
use std::{cell::RefCell, rc::Rc};

use worth_ui_host_contract::{
    UiMountedPresentationAttemptIdentity, UiPresentationDeadline, UiSurfaceBindingGeneration,
};

use super::consumption_view::UiMountedHostPresentationAuthority;
use super::outcome::{
    UiMountedIndeterminateFrame, UiMountedPresentationOutcome, UiMountedPresentationReceipt,
    UiMountedSurfacePresentationReceipt, UiMountedSurfacePresentationRejection,
    UiPresentationIndeterminateReport,
};
use super::preflight::validate_before_effects;
use super::terminal::{
    aggregate_affected, frame_rejections, rejected_outcome, UiIndeterminatePresentationEvidence,
};
use super::{UiMountedPresentationAttempt, UiMountedPresentationInFlight};
use crate::facade::UiHostEffectPort;

mod admission;
mod presentation_attempt;
mod presented;
mod semantic_text_raster;
mod settlement;
mod text_pins;
mod work_preparation;

use presentation_attempt::{
    present_one_surface, UiMountedPresentationProgress, UiMountedPresentationStart,
};
pub(crate) use text_pins::{UiMountedTextPinCandidate, UiMountedTextPinState};

const DEFAULT_IN_FLIGHT_LIMIT: usize = 1;

pub struct UiMountedPresentationCoordinator {
    shutting_down: bool,
    in_flight_limit: usize,
    active: Rc<RefCell<BTreeSet<UiMountedPresentationAttemptIdentity>>>,
    in_flight: BTreeMap<
        UiMountedPresentationAttemptIdentity,
        super::state::UiMountedPresentationInFlightState,
    >,
    presentation_states: super::work_producer::UiMountedPresentationCandidates,
    text: crate::native_platform::text_presentation::UiNativeMountedTextCoordinator,
    host_truth: crate::mounting::UiMountedHostTruthCoordinator,
}

struct UiMountedPresentationSettlement<'host> {
    frame: super::super::UiPreparedMountedFrame,
    retention: super::super::retention::UiMountedRetentionReservation,
    attempt: UiMountedPresentationAttemptIdentity,
    deadline: UiPresentationDeadline,
    pending: Vec<super::state::UiPendingMountedSurface>,
    rejected: Vec<UiMountedSurfacePresentationRejection>,
    completed: Vec<UiMountedSurfacePresentationReceipt>,
    candidates: super::work_producer::UiMountedPresentationCandidates,
    host: UiHostEffectPort<'host>,
}

impl Default for UiMountedPresentationCoordinator {
    fn default() -> Self {
        Self {
            shutting_down: false,
            in_flight_limit: DEFAULT_IN_FLIGHT_LIMIT,
            active: Rc::new(RefCell::new(BTreeSet::new())),
            in_flight: BTreeMap::new(),
            presentation_states: BTreeMap::new(),
            text: Default::default(),
            host_truth: Default::default(),
        }
    }
}

impl UiMountedPresentationCoordinator {
    pub(crate) fn prepare_text_pin_deregistration(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> UiMountedTextPinCandidate {
        self.text.deregistration_candidate(binding)
    }

    pub(crate) fn commit_text_pin_deregistration(&mut self, candidate: UiMountedTextPinCandidate) {
        self.text.commit_surface_candidate(candidate);
    }

    pub(crate) fn present(
        &mut self,
        attempt: UiMountedPresentationAttempt,
        host: UiHostEffectPort<'_>,
        authority: UiMountedHostPresentationAuthority<'_>,
        now: u64,
    ) -> UiMountedPresentationOutcome {
        let (frame, retention, attempt, deadline) = attempt.into_parts();
        if deadline.expired_at(now) {
            self.active.borrow_mut().remove(&attempt);
            let rejections = frame_rejections(
                &frame,
                worth_ui_host_contract::UiHostSurfacePresentationDenial::DeadlineExpired,
            );
            return rejected_outcome(attempt, frame, retention, rejections);
        }
        self.present_all(UiMountedPresentationStart {
            frame,
            retention,
            attempt,
            deadline,
            host,
            authority,
        })
    }

    pub fn reconcile(
        &mut self,
        reconciliation: super::UiHostPresentationReconciliation,
        current_frame: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    ) -> bool {
        self.host_truth
            .reconcile_presentation(reconciliation, current_frame)
    }

    pub(crate) fn binding_requires_reconciliation(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> bool {
        self.host_truth.binding_requires_reconciliation(binding)
    }

    pub(crate) fn commit_current_frame_reconciliation(
        &mut self,
        replacements: &[super::UiMountedSurfaceReconciliationBinding],
    ) {
        self.host_truth
            .commit_current_frame_reconciliation(replacements);
    }

    pub(crate) fn reconcile_candidate_only_deregistration(
        &mut self,
        binding: UiSurfaceBindingGeneration,
    ) {
        self.host_truth
            .reconcile_candidate_only_deregistration(binding);
    }

    pub(crate) fn has_active_attempt(&self) -> bool {
        !self.active.borrow().is_empty()
    }

    pub(crate) fn host_truth_mut(&mut self) -> &mut crate::mounting::UiMountedHostTruthCoordinator {
        &mut self.host_truth
    }

    fn present_all(
        &mut self,
        start: UiMountedPresentationStart<'_, '_>,
    ) -> UiMountedPresentationOutcome {
        if let Err(rejections) =
            validate_before_effects(&start.frame, start.host.adapter(), start.authority)
        {
            self.active.borrow_mut().remove(&start.attempt);
            return rejected_outcome(start.attempt, start.frame, start.retention, rejections);
        }
        let prepared = match work_preparation::prepare(
            &start.frame,
            &self.presentation_states,
            &start.authority,
        ) {
            Ok(prepared) => prepared,
            Err(denial) => {
                self.active.borrow_mut().remove(&start.attempt);
                let rejections = frame_rejections(&start.frame, denial);
                return rejected_outcome(start.attempt, start.frame, start.retention, rejections);
            }
        };
        let mut progress = UiMountedPresentationProgress::default();
        for (surface, prepared_surface) in start.frame.surfaces().iter().zip(&prepared.surfaces) {
            if let Err(evidence) = present_one_surface(
                &start,
                surface,
                &prepared_surface.work,
                &prepared_surface.expected_effects,
                &mut progress,
                &mut self.text,
            ) {
                return self.indeterminate(start.frame, start.retention, start.attempt, evidence);
            }
        }
        self.finish_or_wait(UiMountedPresentationSettlement {
            frame: start.frame,
            retention: start.retention,
            attempt: start.attempt,
            deadline: start.deadline,
            pending: progress.pending,
            rejected: progress.rejected,
            completed: progress.completed,
            candidates: prepared.candidates,
            host: start.host,
        })
    }

    fn finish_or_wait(
        &mut self,
        settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        if !settlement.pending.is_empty() {
            return self.retain_in_flight(settlement);
        }
        if settlement.completed.is_empty() {
            return self.finish_rejected(settlement);
        }
        if !settlement.rejected.is_empty() {
            return self.finish_partially_presented(settlement);
        }
        self.finish_presented(settlement)
    }

    fn retain_in_flight(
        &mut self,
        settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        let cost = match UiMountedPresentationReceipt::compose_cost(
            settlement.frame.cost_report(),
            &settlement.completed,
        ) {
            Ok(cost) => cost,
            Err(_) => {
                let affected = aggregate_affected(
                    &settlement.completed,
                    &settlement.pending,
                    &settlement.rejected,
                );
                settlement::cancel_all(settlement.pending, settlement.host);
                return self.indeterminate(
                    settlement.frame,
                    settlement.retention,
                    settlement.attempt,
                    UiIndeterminatePresentationEvidence::new(affected, settlement.completed),
                );
            }
        };
        let state = super::state::UiMountedPresentationInFlightState {
            frame: settlement.frame,
            retention: settlement.retention,
            attempt: settlement.attempt,
            deadline: settlement.deadline,
            pending: settlement.pending,
            rejected: settlement.rejected,
            completed: settlement.completed,
            candidates: settlement.candidates,
        };
        let handle = UiMountedPresentationInFlight::from_state(&state, cost);
        self.in_flight.insert(state.attempt, state);
        UiMountedPresentationOutcome::InFlight(handle)
    }

    fn finish_rejected(
        &mut self,
        settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        self.active.borrow_mut().remove(&settlement.attempt);
        for rejection in &settlement.rejected {
            if rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired
            {
                self.presentation_states.remove(&rejection.binding());
            }
        }
        rejected_outcome(
            settlement.attempt,
            settlement.frame,
            settlement.retention,
            settlement.rejected,
        )
    }

    fn finish_partially_presented(
        &mut self,
        settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        let affected = aggregate_affected(&settlement.completed, &[], &settlement.rejected);
        self.indeterminate(
            settlement.frame,
            settlement.retention,
            settlement.attempt,
            UiIndeterminatePresentationEvidence::new(affected, settlement.completed),
        )
    }

    fn indeterminate(
        &mut self,
        frame: super::super::UiPreparedMountedFrame,
        retention: super::super::retention::UiMountedRetentionReservation,
        attempt: UiMountedPresentationAttemptIdentity,
        evidence: UiIndeterminatePresentationEvidence,
    ) -> UiMountedPresentationOutcome {
        let (affected, cost) = evidence.into_terminal_parts(frame.cost_report());
        self.active.borrow_mut().remove(&attempt);
        for binding in &affected {
            self.presentation_states.remove(binding);
            let requirement = frame
                .surfaces()
                .iter()
                .find(|surface| surface.requirement().binding() == *binding)
                .expect("affected binding belongs to the retained prepared frame")
                .requirement();
            self.host_truth.block_presentation(requirement);
        }
        drop(retention);
        let report = UiPresentationIndeterminateReport::new(attempt, affected);
        UiMountedPresentationOutcome::PresentationIndeterminate(UiMountedIndeterminateFrame::new(
            frame, report, cost,
        ))
    }
}
