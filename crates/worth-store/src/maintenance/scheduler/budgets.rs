use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IoBudgetUnits(u64);

impl IoBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CpuBudgetUnits(u64);

impl CpuBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MemoryBudgetUnits(u64);

impl MemoryBudgetUnits {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PublicationSlotBudget(u64);

impl PublicationSlotBudget {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ForegroundLatencyGuard(u64);

impl ForegroundLatencyGuard {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceDescriptorDemand {
    predicted_io: IoBudgetUnits,
    predicted_cpu: CpuBudgetUnits,
    predicted_memory: MemoryBudgetUnits,
    predicted_publication: PublicationSlotBudget,
    foreground_latency_guard: ForegroundLatencyGuard,
}

impl MaintenanceDescriptorDemand {
    pub(crate) fn new(
        predicted_io: IoBudgetUnits,
        predicted_cpu: CpuBudgetUnits,
        predicted_memory: MemoryBudgetUnits,
        predicted_publication: PublicationSlotBudget,
        foreground_latency_guard: ForegroundLatencyGuard,
    ) -> Self {
        Self {
            predicted_io,
            predicted_cpu,
            predicted_memory,
            predicted_publication,
            foreground_latency_guard,
        }
    }

    pub fn predicted_io(&self) -> IoBudgetUnits {
        self.predicted_io
    }

    pub fn predicted_cpu(&self) -> CpuBudgetUnits {
        self.predicted_cpu
    }

    pub fn predicted_memory(&self) -> MemoryBudgetUnits {
        self.predicted_memory
    }

    pub fn predicted_publication(&self) -> PublicationSlotBudget {
        self.predicted_publication
    }

    pub fn foreground_latency_guard(&self) -> ForegroundLatencyGuard {
        self.foreground_latency_guard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaintenanceQuantum(u64);

impl MaintenanceQuantum {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PacingWindow(u64);

impl PacingWindow {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn units(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlanGeneration(u64);

impl PlanGeneration {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupersessionEpoch(u64);

impl SupersessionEpoch {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FreshnessWindow(u64);

impl FreshnessWindow {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}
