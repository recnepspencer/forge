use super::*;

impl AdmittedLayoutMaterialization {
    pub fn family(&self) -> AdmittedPhysicalArtifactFamily {
        self.inner.family
    }

    pub fn coverage(&self) -> &LayoutCoverageWitness {
        &self.inner.coverage
    }

    pub fn source(&self) -> &LayoutMaterializationSourceIdentity {
        self.inner.coverage.source()
    }

    pub fn source_root_owner(&self) -> forge_store_physical_format::PhysicalGenerationOwner {
        self.inner.coverage.source().root_owner()
    }

    pub fn source_format_version(&self) -> forge_store_physical_format::PhysicalFormatVersion {
        self.inner.coverage.source().format_version()
    }

    pub(super) fn from_admitted_coverage(
        family: AdmittedPhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
    ) -> Self {
        Self {
            inner: std::sync::Arc::new(AdmittedLayoutMaterializationData { family, coverage }),
        }
    }

    pub fn require_current_at(
        self,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> Result<crate::CurrentLayoutMaterialization, MaterializationDenial> {
        match crate::CurrentLayoutMaterialization::classify_at(self, frontier)? {
            crate::MaterializationFreshness::Current(current) => Ok(current),
            crate::MaterializationFreshness::Stale(_) => {
                Err(MaterializationDenial::MaterializationFrontierMismatch)
            }
        }
    }

    pub fn classify_freshness_at(
        self,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> Result<crate::MaterializationFreshness, MaterializationDenial> {
        crate::CurrentLayoutMaterialization::classify_at(self, frontier)
    }
}
