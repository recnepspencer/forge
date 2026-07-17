#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioScaleProfile {
    Smoke,
    Ci,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioScaleEvidence {
    profile: ScenarioScaleProfile,
    dimensions: ScenarioWorkloadDimensions,
    resident_budget_bytes: u64,
    schedules_executed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioWorkloadDimensions {
    store_bytes: u64,
    blob_bytes: u64,
    wal_tail_bytes: u64,
    damaged_region_bytes: u64,
    artifact_count: u64,
    candidate_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioScaleDenial {
    EmptyWorkload,
    StoreBelowProfileRatio,
    ReleaseBlobBreadthMissing,
}

impl ScenarioScaleEvidence {
    pub fn admit(
        profile: ScenarioScaleProfile,
        dimensions: ScenarioWorkloadDimensions,
        resident_budget_bytes: u64,
        schedules_executed: u64,
    ) -> Result<Self, ScenarioScaleDenial> {
        if dimensions.contains_zero() || resident_budget_bytes == 0 || schedules_executed == 0 {
            return Err(ScenarioScaleDenial::EmptyWorkload);
        }
        let ratio = match profile {
            ScenarioScaleProfile::Smoke => 1,
            ScenarioScaleProfile::Ci => 2,
            ScenarioScaleProfile::Release => 8,
        };
        if dimensions.store_bytes < resident_budget_bytes.saturating_mul(ratio) {
            return Err(ScenarioScaleDenial::StoreBelowProfileRatio);
        }
        if profile == ScenarioScaleProfile::Release
            && dimensions.blob_bytes < 2 * 1024 * 1024 * 1024
        {
            return Err(ScenarioScaleDenial::ReleaseBlobBreadthMissing);
        }
        Ok(Self {
            profile,
            dimensions,
            resident_budget_bytes,
            schedules_executed,
        })
    }

    pub const fn profile(self) -> ScenarioScaleProfile {
        self.profile
    }
    pub const fn store_bytes(self) -> u64 {
        self.dimensions.store_bytes
    }
    pub const fn resident_budget_bytes(self) -> u64 {
        self.resident_budget_bytes
    }
    pub const fn blob_bytes(self) -> u64 {
        self.dimensions.blob_bytes
    }
    pub const fn schedules_executed(self) -> u64 {
        self.schedules_executed
    }
    pub const fn dimensions(self) -> ScenarioWorkloadDimensions {
        self.dimensions
    }
}

impl ScenarioWorkloadDimensions {
    pub const fn new(
        store_bytes: u64,
        blob_bytes: u64,
        wal_tail_bytes: u64,
        damaged_region_bytes: u64,
        artifact_count: u64,
        candidate_count: u64,
    ) -> Self {
        Self {
            store_bytes,
            blob_bytes,
            wal_tail_bytes,
            damaged_region_bytes,
            artifact_count,
            candidate_count,
        }
    }

    pub const fn store_bytes(self) -> u64 {
        self.store_bytes
    }
    pub const fn blob_bytes(self) -> u64 {
        self.blob_bytes
    }
    pub const fn wal_tail_bytes(self) -> u64 {
        self.wal_tail_bytes
    }
    pub const fn damaged_region_bytes(self) -> u64 {
        self.damaged_region_bytes
    }
    pub const fn artifact_count(self) -> u64 {
        self.artifact_count
    }
    pub const fn candidate_count(self) -> u64 {
        self.candidate_count
    }

    pub(super) const fn contains_zero(self) -> bool {
        self.store_bytes == 0
            || self.blob_bytes == 0
            || self.wal_tail_bytes == 0
            || self.damaged_region_bytes == 0
            || self.artifact_count == 0
            || self.candidate_count == 0
    }

    pub(super) const fn total_breadth(self) -> u128 {
        self.store_bytes as u128
            + self.blob_bytes as u128
            + self.wal_tail_bytes as u128
            + self.damaged_region_bytes as u128
            + self.artifact_count as u128
            + self.candidate_count as u128
    }

    pub(super) const fn dominates(self, other: Self) -> bool {
        self.store_bytes >= other.store_bytes
            && self.blob_bytes >= other.blob_bytes
            && self.wal_tail_bytes >= other.wal_tail_bytes
            && self.damaged_region_bytes >= other.damaged_region_bytes
            && self.artifact_count >= other.artifact_count
            && self.candidate_count >= other.candidate_count
    }

    pub(super) const fn strictly_expands(self, other: Self) -> bool {
        self.store_bytes > other.store_bytes
            || self.blob_bytes > other.blob_bytes
            || self.wal_tail_bytes > other.wal_tail_bytes
            || self.damaged_region_bytes > other.damaged_region_bytes
            || self.artifact_count > other.artifact_count
            || self.candidate_count > other.candidate_count
    }
}
