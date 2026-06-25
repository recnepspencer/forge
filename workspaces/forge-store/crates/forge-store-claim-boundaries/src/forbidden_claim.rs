use crate::{BackendFamily, LegacyBackendFamily, StoreBackendCapabilityTier};
use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForbiddenPlatformClaimReason {
    LegacyBackendCannotClaimPhysicalFoundation,
    HeapMaterializationCannotSatisfyPhysicalSubstrate,
    BackendResidueCannotSatisfyManifestAuthority,
    MissingPhysicalFoundationEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenPlatformClaim {
    backend_family: BackendFamily,
    attempted_tier: StoreBackendCapabilityTier,
    scope: RoadmapScope,
    reason: ForbiddenPlatformClaimReason,
}

impl ForbiddenPlatformClaim {
    pub(crate) const fn new(
        backend_family: BackendFamily,
        attempted_tier: StoreBackendCapabilityTier,
        scope: RoadmapScope,
        reason: ForbiddenPlatformClaimReason,
    ) -> Self {
        Self {
            backend_family,
            attempted_tier,
            scope,
            reason,
        }
    }

    pub const fn backend_family(&self) -> BackendFamily {
        self.backend_family
    }

    pub const fn attempted_tier(&self) -> StoreBackendCapabilityTier {
        self.attempted_tier
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn reason(&self) -> ForbiddenPlatformClaimReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyBackendClassificationReport {
    backend_family: BackendFamily,
    admitted_tier: StoreBackendCapabilityTier,
    forbidden_claims: Vec<ForbiddenPlatformClaim>,
}

impl LegacyBackendClassificationReport {
    pub fn classify(
        legacy_family: LegacyBackendFamily,
        admitted_tier: StoreBackendCapabilityTier,
        scope: RoadmapScope,
    ) -> Result<Self, ForbiddenPlatformClaim> {
        let backend_family = BackendFamily::legacy(legacy_family);
        if admitted_tier == StoreBackendCapabilityTier::PlatformGrade {
            return Err(ForbiddenPlatformClaim::new(
                backend_family,
                admitted_tier,
                scope,
                ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation,
            ));
        }

        Ok(Self {
            backend_family,
            admitted_tier,
            forbidden_claims: Vec::new(),
        })
    }

    pub const fn backend_family(&self) -> BackendFamily {
        self.backend_family
    }

    pub const fn admitted_tier(&self) -> StoreBackendCapabilityTier {
        self.admitted_tier
    }

    pub fn forbidden_claims(&self) -> &[ForbiddenPlatformClaim] {
        &self.forbidden_claims
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ForbiddenPlatformClaimReason, LegacyBackendFamily};
    use forge_store_contracts::ROADMAP_2_S1_SCOPE;

    #[test]
    fn legacy_classification_rejects_platform_grade_tier() {
        let denial = LegacyBackendClassificationReport::classify(
            LegacyBackendFamily::Heap,
            StoreBackendCapabilityTier::PlatformGrade,
            ROADMAP_2_S1_SCOPE,
        )
        .expect_err("legacy classification cannot admit platform grade");

        assert_eq!(
            denial.reason(),
            ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation
        );
    }

    #[test]
    fn legacy_classification_admits_non_platform_tier() {
        let report = LegacyBackendClassificationReport::classify(
            LegacyBackendFamily::Sqlite,
            StoreBackendCapabilityTier::Compatibility,
            ROADMAP_2_S1_SCOPE,
        )
        .expect("compatibility classification is allowed");

        assert_eq!(
            report.admitted_tier(),
            StoreBackendCapabilityTier::Compatibility
        );
        assert!(report.forbidden_claims().is_empty());
    }
}
