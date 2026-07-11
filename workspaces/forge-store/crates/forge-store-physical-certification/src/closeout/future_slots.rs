use crate::{
    PhysicalIsolationHarnessFutureExtensionReservation, PhysicalIsolationHarnessFutureExtensionSlot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FuturePhysicalHarnessExtensionFamily {
    HardwareIoQos,
    BlobLifecycle,
    S10RepairPitr,
    S11TenantSecurity,
    S12FullCertificationCampaign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureHarnessExtensionSlotReport {
    family: FuturePhysicalHarnessExtensionFamily,
    reservation: PhysicalIsolationHarnessFutureExtensionReservation,
    implements_future_behavior: bool,
    can_satisfy_physical_isolation_readiness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureHarnessExtensionSlotInventory {
    slots: Vec<FutureHarnessExtensionSlotReport>,
}

impl FutureHarnessExtensionSlotReport {
    const fn reserved(
        family: FuturePhysicalHarnessExtensionFamily,
        slot: PhysicalIsolationHarnessFutureExtensionSlot,
    ) -> Self {
        Self {
            family,
            reservation: PhysicalIsolationHarnessFutureExtensionReservation::reserved(slot),
            implements_future_behavior: false,
            can_satisfy_physical_isolation_readiness: false,
        }
    }

    pub const fn family(&self) -> FuturePhysicalHarnessExtensionFamily {
        self.family
    }

    pub const fn reservation(&self) -> &PhysicalIsolationHarnessFutureExtensionReservation {
        &self.reservation
    }

    pub const fn implements_future_behavior(&self) -> bool {
        self.implements_future_behavior
    }

    pub const fn can_satisfy_physical_isolation_readiness(&self) -> bool {
        self.can_satisfy_physical_isolation_readiness
    }
}

impl FutureHarnessExtensionSlotInventory {
    pub fn simulation_harness_reserved_future_slots() -> Self {
        Self {
            slots: vec![
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::HardwareIoQos,
                    PhysicalIsolationHarnessFutureExtensionSlot::HardwareQualification,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::BlobLifecycle,
                    PhysicalIsolationHarnessFutureExtensionSlot::BlobLifecycle,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S10RepairPitr,
                    PhysicalIsolationHarnessFutureExtensionSlot::RepairPitr,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S11TenantSecurity,
                    PhysicalIsolationHarnessFutureExtensionSlot::TenantSecurity,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S12FullCertificationCampaign,
                    PhysicalIsolationHarnessFutureExtensionSlot::FullS12Campaign,
                ),
            ],
        }
    }

    pub fn slots(&self) -> &[FutureHarnessExtensionSlotReport] {
        &self.slots
    }

    pub fn all_reserved_without_future_behavior(&self) -> bool {
        self.slots.iter().all(|slot| {
            !slot.implements_future_behavior() && !slot.can_satisfy_physical_isolation_readiness()
        })
    }
}
