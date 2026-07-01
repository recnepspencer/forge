use crate::{S5HarnessFutureExtensionReservation, S5HarnessFutureExtensionSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FuturePhysicalHarnessExtensionFamily {
    S6HardwareIoQos,
    S7BlobLifecycle,
    S10RepairPitr,
    S11TenantSecurity,
    S12FullCertificationCampaign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureHarnessExtensionSlotReport {
    family: FuturePhysicalHarnessExtensionFamily,
    reservation: S5HarnessFutureExtensionReservation,
    implements_future_behavior: bool,
    can_satisfy_s5_readiness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureHarnessExtensionSlotInventory {
    slots: Vec<FutureHarnessExtensionSlotReport>,
}

impl FutureHarnessExtensionSlotReport {
    const fn reserved(
        family: FuturePhysicalHarnessExtensionFamily,
        slot: S5HarnessFutureExtensionSlot,
    ) -> Self {
        Self {
            family,
            reservation: S5HarnessFutureExtensionReservation::reserved(slot),
            implements_future_behavior: false,
            can_satisfy_s5_readiness: false,
        }
    }

    pub const fn family(&self) -> FuturePhysicalHarnessExtensionFamily {
        self.family
    }

    pub const fn reservation(&self) -> &S5HarnessFutureExtensionReservation {
        &self.reservation
    }

    pub const fn implements_future_behavior(&self) -> bool {
        self.implements_future_behavior
    }

    pub const fn can_satisfy_s5_readiness(&self) -> bool {
        self.can_satisfy_s5_readiness
    }
}

impl FutureHarnessExtensionSlotInventory {
    pub fn s45_reserved_future_slots() -> Self {
        Self {
            slots: vec![
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S6HardwareIoQos,
                    S5HarnessFutureExtensionSlot::HardwareQualification,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S7BlobLifecycle,
                    S5HarnessFutureExtensionSlot::BlobLifecycle,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S10RepairPitr,
                    S5HarnessFutureExtensionSlot::RepairPitr,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S11TenantSecurity,
                    S5HarnessFutureExtensionSlot::TenantSecurity,
                ),
                FutureHarnessExtensionSlotReport::reserved(
                    FuturePhysicalHarnessExtensionFamily::S12FullCertificationCampaign,
                    S5HarnessFutureExtensionSlot::FullS12Campaign,
                ),
            ],
        }
    }

    pub fn slots(&self) -> &[FutureHarnessExtensionSlotReport] {
        &self.slots
    }

    pub fn all_reserved_without_future_behavior(&self) -> bool {
        self.slots
            .iter()
            .all(|slot| !slot.implements_future_behavior() && !slot.can_satisfy_s5_readiness())
    }
}
