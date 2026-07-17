#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioScaleProfile {
    Smoke,
    Ci,
    Release,
}

impl ScenarioScaleProfile {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Smoke => "smoke",
            Self::Ci => "ci",
            Self::Release => "release",
        }
    }
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
    StoreFixtureMismatch,
    ArtifactCatalogMismatch,
    CandidateCatalogMismatch,
    PhysicalProfileMismatch,
    StoreBelowProfileRatio,
    ReleaseBlobBreadthMissing,
}

impl ScenarioScaleEvidence {
    pub fn from_execution(
        profile: ScenarioScaleProfile,
        execution: &super::S10ScenarioExecutionMatrix,
    ) -> Result<Self, ScenarioScaleDenial> {
        let manifest = execution.primary().replay().fixture_manifest();
        let scale = manifest.scale();
        let dimensions = ScenarioWorkloadDimensions::from_execution(execution);
        let resident_budget_bytes = scale.resident_memory_budget_bytes();
        let schedules_executed = execution.schedules_executed();
        if dimensions.contains_zero() || resident_budget_bytes == 0 || schedules_executed == 0 {
            return Err(ScenarioScaleDenial::EmptyWorkload);
        }
        let expected_physical_profile = match profile {
            ScenarioScaleProfile::Smoke => {
                worth_store_physical_certification::PhysicalSimulationProfile::DeveloperSmoke
            }
            ScenarioScaleProfile::Ci => {
                worth_store_physical_certification::PhysicalSimulationProfile::CiCertification
            }
            ScenarioScaleProfile::Release => {
                worth_store_physical_certification::PhysicalSimulationProfile::ReleaseCertification
            }
        };
        if execution.primary().replay().plan().profile() != expected_physical_profile {
            return Err(ScenarioScaleDenial::PhysicalProfileMismatch);
        }
        if dimensions.store_bytes != scale.declared_store_bytes() {
            return Err(ScenarioScaleDenial::StoreFixtureMismatch);
        }
        let catalog = manifest.artifact_catalog();
        let observed_artifacts = u64::from(catalog.persisted_pages())
            + u64::from(catalog.persisted_extents())
            + u64::from(catalog.discovered_references());
        if dimensions.artifact_count != observed_artifacts {
            return Err(ScenarioScaleDenial::ArtifactCatalogMismatch);
        }
        if dimensions.candidate_count != u64::from(catalog.root_manifest_candidates()) {
            return Err(ScenarioScaleDenial::CandidateCatalogMismatch);
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
    fn from_execution(execution: &super::S10ScenarioExecutionMatrix) -> Self {
        let manifest = execution.primary().replay().fixture_manifest();
        let scale = manifest.scale();
        let catalog = manifest.artifact_catalog();
        Self {
            store_bytes: scale.declared_store_bytes(),
            blob_bytes: scale.blob_bytes(),
            wal_tail_bytes: scale.wal_tail_bytes(),
            damaged_region_bytes: scale.damaged_region_bytes(),
            artifact_count: u64::from(catalog.persisted_pages())
                + u64::from(catalog.persisted_extents())
                + u64::from(catalog.discovered_references()),
            candidate_count: u64::from(catalog.root_manifest_candidates()),
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
