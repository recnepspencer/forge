use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindReservationDenial {
    AdmissionClosed,
    PendingPlanCapacityExceeded { configured: usize },
    EffectingRebindCapacityExceeded { configured: usize },
    CompletionHandleCapacityExceeded { configured: usize },
    RecoveryHandleCapacityExceeded { configured: usize },
    RetainedComparisonSnapshotCapacityExceeded { configured: usize, required: usize },
    RetainedReceiptCapacityExceeded { configured: usize },
    IdentityExhausted,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiRebindShutdownReport {
    pending_plans: usize,
    effecting_rebinds: usize,
    completion_handles: usize,
    recovery_handles: usize,
    retained_comparison_snapshots: usize,
    retained_rebind_receipts: usize,
}

pub(crate) struct UiRebindRuntimeState {
    registry: Rc<RefCell<UiRebindRegistry>>,
}

pub(crate) struct UiRebindReservation {
    registry: Rc<RefCell<UiRebindRegistry>>,
    identity: u64,
    lane: Option<UiRebindRegistrationLane>,
}

pub(crate) struct UiRebindComparisonReservation {
    registry: Rc<RefCell<UiRebindRegistry>>,
    retained_snapshots: usize,
}

#[derive(Clone, Copy)]
enum UiRebindRegistrationLane {
    Pending,
    Effecting,
    Completion,
    Recovery,
    RetainedReceipt,
}

struct UiRebindRegistry {
    profile: super::super::UiRebindProfile,
    next_identity: u64,
    closed: bool,
    pending_plans: usize,
    effecting_rebinds: usize,
    completion_handles: usize,
    recovery_handles: usize,
    retained_comparison_snapshots: usize,
    retained_rebind_receipts: usize,
}

impl UiRebindRuntimeState {
    pub(crate) fn new(profile: super::super::UiRebindProfile) -> Self {
        Self {
            registry: Rc::new(RefCell::new(UiRebindRegistry {
                profile,
                next_identity: 1,
                closed: false,
                pending_plans: 0,
                effecting_rebinds: 0,
                completion_handles: 0,
                recovery_handles: 0,
                retained_comparison_snapshots: 0,
                retained_rebind_receipts: 0,
            })),
        }
    }

    pub(crate) fn reserve_plan(&self) -> Result<UiRebindReservation, UiRebindReservationDenial> {
        let mut registry = self.registry.borrow_mut();
        if registry.closed {
            return Err(UiRebindReservationDenial::AdmissionClosed);
        }
        let configured = registry.profile.concurrency().pending_plans;
        if registry.pending_plans >= configured {
            return Err(UiRebindReservationDenial::PendingPlanCapacityExceeded { configured });
        }
        let identity = registry.next_identity;
        registry.next_identity = identity
            .checked_add(1)
            .ok_or(UiRebindReservationDenial::IdentityExhausted)?;
        registry.pending_plans += 1;
        Ok(UiRebindReservation {
            registry: Rc::clone(&self.registry),
            identity,
            lane: Some(UiRebindRegistrationLane::Pending),
        })
    }

    pub(crate) fn reserve_comparison_snapshots(
        &self,
        required: usize,
    ) -> Result<UiRebindComparisonReservation, UiRebindReservationDenial> {
        let mut registry = self.registry.borrow_mut();
        if registry.closed {
            return Err(UiRebindReservationDenial::AdmissionClosed);
        }
        let configured = registry.profile.concurrency().retained_comparison_snapshots;
        let available = configured.saturating_sub(registry.retained_comparison_snapshots);
        if required > available {
            return Err(
                UiRebindReservationDenial::RetainedComparisonSnapshotCapacityExceeded {
                    configured,
                    required,
                },
            );
        }
        registry.retained_comparison_snapshots = registry
            .retained_comparison_snapshots
            .checked_add(required)
            .expect("bounded comparison snapshot count does not exhaust");
        drop(registry);
        Ok(UiRebindComparisonReservation {
            registry: Rc::clone(&self.registry),
            retained_snapshots: required,
        })
    }

    pub(crate) fn shutdown(&self) -> UiRebindShutdownReport {
        let mut registry = self.registry.borrow_mut();
        registry.closed = true;
        UiRebindShutdownReport {
            pending_plans: registry.pending_plans,
            effecting_rebinds: registry.effecting_rebinds,
            completion_handles: registry.completion_handles,
            recovery_handles: registry.recovery_handles,
            retained_comparison_snapshots: registry.retained_comparison_snapshots,
            retained_rebind_receipts: registry.retained_rebind_receipts,
        }
    }

    #[cfg(test)]
    pub(crate) fn pending_plan_count(&self) -> usize {
        self.registry.borrow().pending_plans
    }
}

impl UiRebindReservation {
    pub(crate) const fn identity(&self) -> u64 {
        self.identity
    }

    pub(crate) fn begin_effecting(&mut self) -> Result<(), UiRebindReservationDenial> {
        {
            let registry = self.registry.borrow();
            require_lane_capacity(&registry, UiRebindRegistrationLane::Completion)?;
            require_lane_capacity(&registry, UiRebindRegistrationLane::Recovery)?;
            require_lane_capacity(&registry, UiRebindRegistrationLane::RetainedReceipt)?;
        }
        self.transition_to(UiRebindRegistrationLane::Effecting)
    }

    pub(crate) fn return_to_pending(&mut self) -> Result<(), UiRebindReservationDenial> {
        self.transition_to(UiRebindRegistrationLane::Pending)
    }

    pub(crate) fn retain_completion(&mut self) -> Result<(), UiRebindReservationDenial> {
        self.transition_to(UiRebindRegistrationLane::Completion)
    }

    pub(crate) fn retain_recovery(&mut self) -> Result<(), UiRebindReservationDenial> {
        self.transition_to(UiRebindRegistrationLane::Recovery)
    }

    pub(crate) fn retain_receipt(&mut self) -> Result<(), UiRebindReservationDenial> {
        self.transition_to(UiRebindRegistrationLane::RetainedReceipt)
    }

    pub(crate) fn release(&mut self) {
        let Some(lane) = self.lane.take() else {
            return;
        };
        let mut registry = self.registry.borrow_mut();
        release_lane(&mut registry, lane);
    }

    fn transition_to(
        &mut self,
        target: UiRebindRegistrationLane,
    ) -> Result<(), UiRebindReservationDenial> {
        let source = self
            .lane
            .expect("active rebind reservation has one registry lane");
        let mut registry = self.registry.borrow_mut();
        require_lane_capacity(&registry, target)?;
        release_lane(&mut registry, source);
        retain_lane(&mut registry, target);
        self.lane = Some(target);
        Ok(())
    }
}

impl Drop for UiRebindReservation {
    fn drop(&mut self) {
        self.release();
    }
}

impl Drop for UiRebindComparisonReservation {
    fn drop(&mut self) {
        let mut registry = self.registry.borrow_mut();
        registry.retained_comparison_snapshots = registry
            .retained_comparison_snapshots
            .checked_sub(self.retained_snapshots)
            .expect("comparison reservation releases its exact snapshot count");
        self.retained_snapshots = 0;
    }
}

fn require_lane_capacity(
    registry: &UiRebindRegistry,
    lane: UiRebindRegistrationLane,
) -> Result<(), UiRebindReservationDenial> {
    let capacity = registry.profile.concurrency();
    let (observed, configured, denial) = match lane {
        UiRebindRegistrationLane::Pending => (
            registry.pending_plans,
            capacity.pending_plans,
            UiRebindReservationDenial::PendingPlanCapacityExceeded {
                configured: capacity.pending_plans,
            },
        ),
        UiRebindRegistrationLane::Effecting => (
            registry.effecting_rebinds,
            capacity.effecting_rebinds,
            UiRebindReservationDenial::EffectingRebindCapacityExceeded {
                configured: capacity.effecting_rebinds,
            },
        ),
        UiRebindRegistrationLane::Completion => (
            registry.completion_handles,
            capacity.completion_handles,
            UiRebindReservationDenial::CompletionHandleCapacityExceeded {
                configured: capacity.completion_handles,
            },
        ),
        UiRebindRegistrationLane::Recovery => (
            registry.recovery_handles,
            capacity.recovery_handles,
            UiRebindReservationDenial::RecoveryHandleCapacityExceeded {
                configured: capacity.recovery_handles,
            },
        ),
        UiRebindRegistrationLane::RetainedReceipt => (
            registry.retained_rebind_receipts,
            capacity.retained_rebind_receipts,
            UiRebindReservationDenial::RetainedReceiptCapacityExceeded {
                configured: capacity.retained_rebind_receipts,
            },
        ),
    };
    (observed < configured).then_some(()).ok_or(denial)
}

fn release_lane(registry: &mut UiRebindRegistry, lane: UiRebindRegistrationLane) {
    let counter = lane_counter_mut(registry, lane);
    *counter = counter
        .checked_sub(1)
        .expect("active rebind registration has one counted row");
}

fn retain_lane(registry: &mut UiRebindRegistry, lane: UiRebindRegistrationLane) {
    let counter = lane_counter_mut(registry, lane);
    *counter = counter
        .checked_add(1)
        .expect("bounded rebind registration counter does not exhaust");
}

fn lane_counter_mut(registry: &mut UiRebindRegistry, lane: UiRebindRegistrationLane) -> &mut usize {
    match lane {
        UiRebindRegistrationLane::Pending => &mut registry.pending_plans,
        UiRebindRegistrationLane::Effecting => &mut registry.effecting_rebinds,
        UiRebindRegistrationLane::Completion => &mut registry.completion_handles,
        UiRebindRegistrationLane::Recovery => &mut registry.recovery_handles,
        UiRebindRegistrationLane::RetainedReceipt => &mut registry.retained_rebind_receipts,
    }
}

impl UiRebindShutdownReport {
    pub const fn pending_plans(self) -> usize {
        self.pending_plans
    }

    pub const fn effecting_rebinds(self) -> usize {
        self.effecting_rebinds
    }

    pub const fn completion_handles(self) -> usize {
        self.completion_handles
    }

    pub const fn recovery_handles(self) -> usize {
        self.recovery_handles
    }

    pub const fn retained_comparison_snapshots(self) -> usize {
        self.retained_comparison_snapshots
    }

    pub const fn retained_rebind_receipts(self) -> usize {
        self.retained_rebind_receipts
    }

    pub const fn is_empty(self) -> bool {
        self.pending_plans == 0
            && self.effecting_rebinds == 0
            && self.completion_handles == 0
            && self.recovery_handles == 0
            && self.retained_comparison_snapshots == 0
            && self.retained_rebind_receipts == 0
    }
}

#[cfg(test)]
#[path = "state/tests.rs"]
mod tests;
