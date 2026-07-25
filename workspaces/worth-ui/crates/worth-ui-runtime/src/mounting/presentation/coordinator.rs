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
        self.present_all(frame, retention, attempt, deadline, host, authority)
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
        frame: super::super::UiPreparedMountedFrame,
        retention: super::super::retention::UiMountedRetentionReservation,
        attempt: UiMountedPresentationAttemptIdentity,
        deadline: UiPresentationDeadline,
        host: UiHostEffectPort<'_>,
        authority: UiMountedHostPresentationAuthority<'_>,
    ) -> UiMountedPresentationOutcome {
        if let Err(rejections) = validate_before_effects(&frame, host.adapter(), authority) {
            self.active.borrow_mut().remove(&attempt);
            return rejected_outcome(attempt, frame, retention, rejections);
        }
        let mut pending = Vec::new();
        let mut rejected = Vec::new();
        let mut completed = Vec::new();
        for surface in frame.surfaces() {
            let requirement = surface.requirement();
            let view = authority.bind(UiRuntimeMountedFrameConsumptionInput {
                attempt,
                deadline,
                requirement,
                projection: surface.projection(),
            });
            match host
                .adapter()
                .present_mounted_surface(host.authority(), &view)
            {
                UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial) => {
                    rejected.push(UiMountedSurfacePresentationRejection::new(
                        requirement.binding(),
                        denial,
                    ));
                }
                UiHostSurfacePresentationOutcome::PresentationIndeterminate => {
                    let mut affected = aggregate_affected(&completed, &pending, &rejected);
                    affected.push(requirement.binding());
                    settlement::cancel_all(pending, host);
                    return self.indeterminate(
                        frame,
                        retention,
                        attempt,
                        UiIndeterminatePresentationEvidence::new(affected, completed),
                    );
                }
                UiHostSurfacePresentationOutcome::InFlight(token) => {
                    pending.push(super::state::UiPendingMountedSurface {
                        binding: requirement.binding(),
                        token,
                    })
                }
                UiHostSurfacePresentationOutcome::Presented(completion) => {
                    if !completion_satisfies(surface, &completion) {
                        let mut affected = aggregate_affected(&completed, &pending, &rejected);
                        affected.push(requirement.binding());
                        settlement::cancel_all(pending, host);
                        let evidence =
                            UiIndeterminatePresentationEvidence::new(affected, completed)
                                .with_additional_adapter_cost(completion.cost());
                        return self.indeterminate(frame, retention, attempt, evidence);
                    }
                    let (effects, adapter_cost) = completion.into_parts();
                    completed.push(UiMountedSurfacePresentationReceipt::new(
                        requirement.binding(),
                        effects,
                        adapter_cost,
                    ));
                }
            }
        }
        self.finish_or_wait(UiMountedPresentationSettlement {
            frame,
            retention,
            attempt,
            deadline,
            pending,
            rejected,
            completed,
            host,
        })
    }

    fn finish_or_wait(
        &mut self,
        settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        let UiMountedPresentationSettlement {
            frame,
            retention,
            attempt,
            deadline,
            pending,
            rejected,
            mut completed,
            host,
        } = settlement;
        if !pending.is_empty() {
            let cost =
                match UiMountedPresentationReceipt::compose_cost(frame.cost_report(), &completed) {
                    Ok(cost) => cost,
                    Err(_) => {
                        let affected = aggregate_affected(&completed, &pending, &rejected);
                        settlement::cancel_all(pending, host);
                        return self.indeterminate(
                            frame,
                            retention,
                            attempt,
                            UiIndeterminatePresentationEvidence::new(affected, completed),
                        );
                    }
                };
            let state = super::state::UiMountedPresentationInFlightState {
                frame,
                retention,
                attempt,
                deadline,
                pending,
                rejected,
                completed,
            };
            let handle = UiMountedPresentationInFlight::from_state(&state, cost);
            self.in_flight.insert(attempt, state);
            return UiMountedPresentationOutcome::InFlight(handle);
        }
        if completed.is_empty() {
            self.active.borrow_mut().remove(&attempt);
            return rejected_outcome(attempt, frame, retention, rejected);
        }
        if !rejected.is_empty() {
            let affected = aggregate_affected(&completed, &[], &rejected);
            return self.indeterminate(
                frame,
                retention,
                attempt,
                UiIndeterminatePresentationEvidence::new(affected, completed),
            );
        }
        self.active.borrow_mut().remove(&attempt);
        completed.sort_by_key(UiMountedSurfacePresentationReceipt::binding);
        let cost = match UiMountedPresentationReceipt::compose_cost(frame.cost_report(), &completed)
        {
            Ok(cost) => cost,
            Err(_) => {
                let affected = frame
                    .surfaces()
                    .iter()
                    .map(|surface| surface.requirement().binding())
                    .collect();
                return self.indeterminate(
                    frame,
                    retention,
                    attempt,
                    UiIndeterminatePresentationEvidence::new(affected, completed),
                );
            }
        };
        let receipt = UiMountedPresentationReceipt::new(
            attempt,
            frame.canonical_core().frame(),
            cost,
            completed,
        );
        UiMountedPresentationOutcome::Presented(UiMountedPresentedFrame::new(
            frame,
            retention,
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
