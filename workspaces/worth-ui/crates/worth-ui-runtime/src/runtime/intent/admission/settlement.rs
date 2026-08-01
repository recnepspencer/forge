use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentAdmissionCancellationReason {
    MountedInstanceRemoved,
    SurfaceRebound,
    ApplicationRebound,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentAdmissionSettlementPosture {
    Released,
    LifecycleCancelled(UiIntentAdmissionCancellationReason),
    AlreadySettled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentAdmissionSettlementReceipt {
    posture: UiIntentAdmissionSettlementPosture,
    active_after: usize,
}

pub(crate) struct UiIntentAdmissionLease {
    posture: AtomicU8,
}

const LEASE_ACTIVE: u8 = 0;
const LEASE_RELEASED: u8 = 1;
const LEASE_MOUNTED_INSTANCE_REMOVED: u8 = 2;
const LEASE_SURFACE_REBOUND: u8 = 3;
const LEASE_APPLICATION_REBOUND: u8 = 4;
const LEASE_SHUTDOWN: u8 = 5;
const LEASE_TRANSFERRED_TO_EXECUTION: u8 = 6;

impl UiIntentAdmissionLease {
    pub(crate) fn new() -> Self {
        Self {
            posture: AtomicU8::new(LEASE_ACTIVE),
        }
    }

    pub(crate) fn mark_released(&self) {
        self.posture.store(LEASE_RELEASED, Ordering::Release);
    }

    pub(crate) fn mark_cancelled(&self, reason: UiIntentAdmissionCancellationReason) {
        self.posture
            .store(cancelled_code(reason), Ordering::Release);
    }

    pub(crate) fn mark_transferred_to_execution(&self) {
        self.posture
            .store(LEASE_TRANSFERRED_TO_EXECUTION, Ordering::Release);
    }

    pub(crate) fn settlement_posture(&self) -> UiIntentAdmissionSettlementPosture {
        match self.posture.load(Ordering::Acquire) {
            LEASE_ACTIVE | LEASE_RELEASED | LEASE_TRANSFERRED_TO_EXECUTION => {
                UiIntentAdmissionSettlementPosture::AlreadySettled
            }
            LEASE_MOUNTED_INSTANCE_REMOVED => {
                UiIntentAdmissionSettlementPosture::LifecycleCancelled(
                    UiIntentAdmissionCancellationReason::MountedInstanceRemoved,
                )
            }
            LEASE_SURFACE_REBOUND => UiIntentAdmissionSettlementPosture::LifecycleCancelled(
                UiIntentAdmissionCancellationReason::SurfaceRebound,
            ),
            LEASE_APPLICATION_REBOUND => UiIntentAdmissionSettlementPosture::LifecycleCancelled(
                UiIntentAdmissionCancellationReason::ApplicationRebound,
            ),
            LEASE_SHUTDOWN => UiIntentAdmissionSettlementPosture::LifecycleCancelled(
                UiIntentAdmissionCancellationReason::Shutdown,
            ),
            _ => unreachable!("sealed admission lease posture"),
        }
    }
}

const fn cancelled_code(reason: UiIntentAdmissionCancellationReason) -> u8 {
    match reason {
        UiIntentAdmissionCancellationReason::MountedInstanceRemoved => {
            LEASE_MOUNTED_INSTANCE_REMOVED
        }
        UiIntentAdmissionCancellationReason::SurfaceRebound => LEASE_SURFACE_REBOUND,
        UiIntentAdmissionCancellationReason::ApplicationRebound => LEASE_APPLICATION_REBOUND,
        UiIntentAdmissionCancellationReason::Shutdown => LEASE_SHUTDOWN,
    }
}

impl UiIntentAdmissionSettlementReceipt {
    pub(crate) const fn new(
        posture: UiIntentAdmissionSettlementPosture,
        active_after: usize,
    ) -> Self {
        Self {
            posture,
            active_after,
        }
    }

    pub const fn posture(self) -> UiIntentAdmissionSettlementPosture {
        self.posture
    }

    pub const fn active_after(self) -> usize {
        self.active_after
    }
}
