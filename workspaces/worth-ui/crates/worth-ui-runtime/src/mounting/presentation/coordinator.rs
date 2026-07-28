use std::collections::{BTreeMap, BTreeSet};
use std::{cell::RefCell, rc::Rc};

use worth_ui_host_contract::{
    UiHostSurfacePresentationOutcome, UiMountedPresentationAttemptIdentity, UiPresentationDeadline,
    UiSurfaceBindingGeneration,
};

use super::consumption_view::{
    UiMountedHostPresentationAuthority, UiRuntimeMountedFrameConsumptionInput,
};
use super::outcome::{
    UiMountedIndeterminateFrame, UiMountedPresentationOutcome, UiMountedPresentationReceipt,
    UiMountedPresentationWitness, UiMountedPresentedFrame, UiMountedSurfacePresentationReceipt,
    UiMountedSurfacePresentationRejection, UiPresentationIndeterminateReport,
};
use super::preflight::validate_before_effects;
use super::terminal::{
    aggregate_affected, completion_satisfies, frame_rejections, rejected_outcome,
    UiIndeterminatePresentationEvidence,
};
use super::{UiMountedPresentationAttempt, UiMountedPresentationInFlight};
use crate::facade::UiHostEffectPort;

mod admission;
mod settlement;

const DEFAULT_IN_FLIGHT_LIMIT: usize = 1;

pub struct UiMountedPresentationCoordinator {
    shutting_down: bool,
    in_flight_limit: usize,
    active: Rc<RefCell<BTreeSet<UiMountedPresentationAttemptIdentity>>>,
    in_flight: BTreeMap<
        UiMountedPresentationAttemptIdentity,
        super::state::UiMountedPresentationInFlightState,
    >,
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
    host: UiHostEffectPort<'host>,
}

struct UiMountedPresentationStart<'host, 'authority> {
    frame: super::super::UiPreparedMountedFrame,
    retention: super::super::retention::UiMountedRetentionReservation,
    attempt: UiMountedPresentationAttemptIdentity,
    deadline: UiPresentationDeadline,
    host: UiHostEffectPort<'host>,
    authority: UiMountedHostPresentationAuthority<'authority>,
}

#[derive(Default)]
struct UiMountedPresentationProgress {
    pending: Vec<super::state::UiPendingMountedSurface>,
    rejected: Vec<UiMountedSurfacePresentationRejection>,
    completed: Vec<UiMountedSurfacePresentationReceipt>,
}

impl Default for UiMountedPresentationCoordinator {
    fn default() -> Self {
        Self {
            shutting_down: false,
            in_flight_limit: DEFAULT_IN_FLIGHT_LIMIT,
            active: Rc::new(RefCell::new(BTreeSet::new())),
            in_flight: BTreeMap::new(),
            host_truth: Default::default(),
        }
    }
}

fn present_one_surface(
    start: &UiMountedPresentationStart<'_, '_>,
    surface: &super::super::UiMountedSurfaceReceipt,
    progress: &mut UiMountedPresentationProgress,
) -> Result<(), UiIndeterminatePresentationEvidence> {
    let requirement = surface.requirement();
    let view = start.authority.bind(UiRuntimeMountedFrameConsumptionInput {
        attempt: start.attempt,
        deadline: start.deadline,
        requirement,
        projection: surface.projection(),
    });
    match start
        .host
        .adapter()
        .present_mounted_surface(start.host.authority(), &view)
    {
        UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial) => {
            progress
                .rejected
                .push(UiMountedSurfacePresentationRejection::new(
                    requirement.binding(),
                    denial,
                ));
            Ok(())
        }
        UiHostSurfacePresentationOutcome::InFlight(token) => {
            progress
                .pending
                .push(super::state::UiPendingMountedSurface {
                    binding: requirement.binding(),
                    token,
                });
            Ok(())
        }
        UiHostSurfacePresentationOutcome::PresentationIndeterminate => Err(
            terminalize_surface_uncertainty(progress, start.host, requirement.binding(), None),
        ),
        UiHostSurfacePresentationOutcome::Presented(completion) => {
            if !completion_satisfies(surface, &completion) {
                return Err(terminalize_surface_uncertainty(
                    progress,
                    start.host,
                    requirement.binding(),
                    Some(completion.cost()),
                ));
            }
            let (epoch, effects, adapter_cost) = completion.into_parts();
            progress
                .completed
                .push(UiMountedSurfacePresentationReceipt::new(
                    requirement,
                    epoch,
                    effects,
                    adapter_cost,
                ));
            Ok(())
        }
    }
}

fn terminalize_surface_uncertainty(
    progress: &mut UiMountedPresentationProgress,
    host: UiHostEffectPort<'_>,
    binding: UiSurfaceBindingGeneration,
    additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
) -> UiIndeterminatePresentationEvidence {
    let mut affected =
        aggregate_affected(&progress.completed, &progress.pending, &progress.rejected);
    affected.push(binding);
    settlement::cancel_all(std::mem::take(&mut progress.pending), host);
    let evidence =
        UiIndeterminatePresentationEvidence::new(affected, std::mem::take(&mut progress.completed));
    match additional_cost {
        Some(cost) => evidence.with_additional_adapter_cost(cost),
        None => evidence,
    }
}

impl UiMountedPresentationCoordinator {
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
        let mut progress = UiMountedPresentationProgress::default();
        for surface in start.frame.surfaces() {
            if let Err(evidence) = present_one_surface(&start, surface, &mut progress) {
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

    fn finish_presented(
        &mut self,
        mut settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        self.active.borrow_mut().remove(&settlement.attempt);
        let attempt = settlement.attempt;
        let frame_identity = settlement.frame.canonical_core().frame();
        let frame_cost = settlement.frame.cost_report();
        let mut completed = std::mem::take(&mut settlement.completed);
        completed.sort_by_key(UiMountedSurfacePresentationReceipt::binding);
        let cost = match UiMountedPresentationReceipt::compose_cost(frame_cost, &completed) {
            Ok(cost) => cost,
            Err(_) => {
                let affected = settlement
                    .frame
                    .surfaces()
                    .iter()
                    .map(|surface| surface.requirement().binding())
                    .collect();
                return self.indeterminate(
                    settlement.frame,
                    settlement.retention,
                    attempt,
                    UiIndeterminatePresentationEvidence::new(affected, completed),
                );
            }
        };
        let receipt = UiMountedPresentationReceipt::new(attempt, frame_identity, cost, completed);
        UiMountedPresentationOutcome::Presented(UiMountedPresentedFrame::new(
            settlement.frame,
            settlement.retention,
            receipt,
            UiMountedPresentationWitness::new(attempt),
        ))
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
