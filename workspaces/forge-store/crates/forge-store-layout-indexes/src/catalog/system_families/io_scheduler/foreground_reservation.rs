use crate::access_planning::AccessShape;
use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};

use forge_store_io_scheduler::foreground_reservation::{
    ForegroundFairnessClass, ForegroundReservationCounterSnapshot, ForegroundReservationReceipt,
};
use forge_store_io_scheduler::{
    BackgroundResourceBudget, BandwidthToken, FlushPermit, QueueSlot, SyncDebt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerReservationInterferencePosture {
    StableReadEnvelopeBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerReservationLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    fairness_class: ForegroundFairnessClass,
    requested_budget: BackgroundResourceBudget,
    admitted_budget: BackgroundResourceBudget,
    interference_posture: SchedulerReservationInterferencePosture,
    counters: ForegroundReservationCounterSnapshot,
}

pub fn project_scheduler_reservation(
    receipt: ForegroundReservationReceipt,
) -> SchedulerReservationLayoutReport {
    SchedulerReservationLayoutReport {
        family_id: DurableArtifactFamilyId::SchedulerReservationIndex,
        access_shape: AccessShape::PointLookup,
        rebuild_posture: DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        fairness_class: receipt.fairness_class(),
        requested_budget: receipt.counters().requested().into_background_budget(),
        admitted_budget: receipt
            .counters()
            .admitted_budget()
            .into_background_budget(),
        interference_posture: SchedulerReservationInterferencePosture::StableReadEnvelopeBound,
        counters: receipt.counters(),
    }
}

impl SchedulerReservationLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn fairness_class(&self) -> ForegroundFairnessClass {
        self.fairness_class
    }

    pub const fn requested_budget(&self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted_budget
    }

    pub const fn interference_posture(&self) -> SchedulerReservationInterferencePosture {
        self.interference_posture
    }

    pub const fn exact_counters(&self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }
}

trait IntoBackgroundBudget {
    fn into_background_budget(self) -> BackgroundResourceBudget;
}

impl IntoBackgroundBudget
    for forge_store_io_scheduler::foreground_reservation::ForegroundResourceBudget
{
    fn into_background_budget(self) -> BackgroundResourceBudget {
        let mut budget = BackgroundResourceBudget::new();
        if self.queue_slots() > 0 {
            budget = budget.with_queue_slots(
                QueueSlot::new(self.queue_slots())
                    .expect("foreground queue slots should stay valid"),
            );
        }
        if self.bandwidth_tokens() > 0 {
            budget = budget.with_bandwidth(
                BandwidthToken::bytes(self.bandwidth_tokens())
                    .expect("foreground bandwidth tokens should stay valid"),
            );
        }
        if self.flush_permits() > 0 {
            budget = budget.with_flush_permits(
                FlushPermit::new(self.flush_permits())
                    .expect("foreground flush permits should stay valid"),
            );
        }
        if self.sync_debt() > 0 {
            budget = budget.with_sync_debt(
                SyncDebt::units(self.sync_debt()).expect("foreground sync debt should stay valid"),
            );
        }
        budget
    }
}
