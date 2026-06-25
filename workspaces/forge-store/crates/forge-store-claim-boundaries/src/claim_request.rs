use crate::{
    BackendFamily, ForbiddenPlatformClaim, ForbiddenPlatformClaimReason, LegacyBackendFamily,
    StoreBackendCapabilityTier,
};
use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClaimBoundary {
    tier: StoreBackendCapabilityTier,
    scope: RoadmapScope,
}

impl ClaimBoundary {
    pub const fn new(tier: StoreBackendCapabilityTier, scope: RoadmapScope) -> Self {
        Self { tier, scope }
    }

    pub const fn tier(&self) -> StoreBackendCapabilityTier {
        self.tier
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendClaimRequest {
    backend_family: BackendFamily,
    requested_boundary: ClaimBoundary,
}

impl BackendClaimRequest {
    pub const fn legacy(
        legacy_family: LegacyBackendFamily,
        requested_tier: StoreBackendCapabilityTier,
        scope: RoadmapScope,
    ) -> Self {
        Self {
            backend_family: BackendFamily::legacy(legacy_family),
            requested_boundary: ClaimBoundary::new(requested_tier, scope),
        }
    }

    pub const fn physical_candidate(
        requested_tier: StoreBackendCapabilityTier,
        scope: RoadmapScope,
    ) -> Self {
        Self {
            backend_family: BackendFamily::PhysicalFoundationCandidate,
            requested_boundary: ClaimBoundary::new(requested_tier, scope),
        }
    }

    pub fn audit_forbidden_platform_claim(
        self,
    ) -> Result<ClassifiedBackendClaim, ForbiddenPlatformClaim> {
        if self.backend_family.is_legacy()
            && self.requested_boundary.tier() == StoreBackendCapabilityTier::PlatformGrade
        {
            return Err(ForbiddenPlatformClaim::new(
                self.backend_family,
                self.requested_boundary.tier(),
                self.requested_boundary.scope(),
                ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation,
            ));
        }

        Ok(ClassifiedBackendClaim {
            backend_family: self.backend_family,
            boundary: self.requested_boundary,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassifiedBackendClaim {
    backend_family: BackendFamily,
    boundary: ClaimBoundary,
}

impl ClassifiedBackendClaim {
    pub const fn backend_family(&self) -> BackendFamily {
        self.backend_family
    }

    pub const fn boundary(&self) -> ClaimBoundary {
        self.boundary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ForbiddenPlatformClaimReason, LegacyBackendFamily};
    use forge_store_contracts::ROADMAP_2_S1_SCOPE;

    #[test]
    fn legacy_heap_platform_grade_claim_is_forbidden() {
        let denial = BackendClaimRequest::legacy(
            LegacyBackendFamily::Heap,
            StoreBackendCapabilityTier::PlatformGrade,
            ROADMAP_2_S1_SCOPE,
        )
        .audit_forbidden_platform_claim()
        .expect_err("heap cannot claim platform grade");

        assert_eq!(
            denial.reason(),
            ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation
        );
    }

    #[test]
    fn legacy_file_platform_grade_claim_is_forbidden() {
        let denial = BackendClaimRequest::legacy(
            LegacyBackendFamily::File,
            StoreBackendCapabilityTier::PlatformGrade,
            ROADMAP_2_S1_SCOPE,
        )
        .audit_forbidden_platform_claim()
        .expect_err("file cannot claim platform grade");

        assert_eq!(
            denial.reason(),
            ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation
        );
    }

    #[test]
    fn legacy_sqlite_platform_grade_claim_is_forbidden() {
        let denial = BackendClaimRequest::legacy(
            LegacyBackendFamily::Sqlite,
            StoreBackendCapabilityTier::PlatformGrade,
            ROADMAP_2_S1_SCOPE,
        )
        .audit_forbidden_platform_claim()
        .expect_err("SQLite cannot claim platform grade");

        assert_eq!(
            denial.reason(),
            ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimPromotionRejection {
    ForbiddenPlatformClaim(ForbiddenPlatformClaim),
    MissingPlatformGradeEvidence,
    PhysicalDebtCannotPromote,
}
