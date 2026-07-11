use crate::ScrubPlan;
use forge_foundational::{FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole};
use forge_store_aspect_native::StoreDigestEvidence;
use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryRoleMapping {
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
}

impl FoundationalBoundaryRoleMapping {
    pub const fn store_physical_authority() -> Self {
        Self::new(
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        )
    }

    pub const fn store_derived_projection() -> Self {
        Self::new(
            FoundationalBoundaryArtifactCategory::Report,
            FoundationalBoundaryArtifactRole::DerivedProjection,
        )
    }

    pub const fn store_support_only() -> Self {
        Self::new(
            FoundationalBoundaryArtifactCategory::Report,
            FoundationalBoundaryArtifactRole::SupportOnly,
        )
    }

    pub const fn store_planned_work() -> Self {
        Self::new(
            FoundationalBoundaryArtifactCategory::Summary,
            FoundationalBoundaryArtifactRole::PlannedWork,
        )
    }

    pub const fn store_receipt_evidence() -> Self {
        Self::new(
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::ReceiptEvidence,
        )
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    const fn new(
        category: FoundationalBoundaryArtifactCategory,
        role: FoundationalBoundaryArtifactRole,
    ) -> Self {
        Self { category, role }
    }
}

macro_rules! claim_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            basis: StableDigest,
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn new(basis: StableDigest) -> Self {
                Self { basis }
            }

            pub fn basis(&self) -> &StableDigest {
                &self.basis
            }
        }
    };
}

claim_type!(StoreDerivedProjectionBoundaryClaim);
claim_type!(StoreSupportOnlyBoundaryClaim);
claim_type!(StorePlannedWorkBoundaryClaim);
claim_type!(StoreReceiptEvidenceBoundaryClaim);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePhysicalAuthorityBoundaryClaim {
    basis: StoreDigestEvidence,
}

impl StorePhysicalAuthorityBoundaryClaim {
    pub(crate) const fn new(basis: StoreDigestEvidence) -> Self {
        Self { basis }
    }

    pub const fn basis(&self) -> &StoreDigestEvidence {
        &self.basis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorePlannedWorkBoundaryKind {
    ScenarioPlan,
    ScrubPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePlannedWorkBoundaryReport {
    kind: StorePlannedWorkBoundaryKind,
    mapping: FoundationalBoundaryRoleMapping,
    claim: StorePlannedWorkBoundaryClaim,
    planned_window_count: u64,
    planned_byte_count: u64,
}

impl StorePlannedWorkBoundaryReport {
    pub fn from_scrub_plan(plan: &ScrubPlan<'_>) -> Self {
        let planned_byte_count = plan
            .windows()
            .iter()
            .copied()
            .map(|planned| planned.window().len_bytes())
            .sum();
        Self {
            kind: StorePlannedWorkBoundaryKind::ScrubPlan,
            mapping: FoundationalBoundaryRoleMapping::store_planned_work(),
            claim: StorePlannedWorkBoundaryClaim::new(planned_scrub_digest(plan)),
            planned_window_count: plan.windows().len() as u64,
            planned_byte_count,
        }
    }

    pub const fn kind(&self) -> StorePlannedWorkBoundaryKind {
        self.kind
    }

    pub const fn mapping(&self) -> &FoundationalBoundaryRoleMapping {
        &self.mapping
    }

    pub const fn claim(&self) -> &StorePlannedWorkBoundaryClaim {
        &self.claim
    }

    pub const fn planned_window_count(&self) -> u64 {
        self.planned_window_count
    }

    pub const fn planned_byte_count(&self) -> u64 {
        self.planned_byte_count
    }
}

fn planned_scrub_digest(plan: &ScrubPlan<'_>) -> StableDigest {
    StableDigest::new(format!(
        "s3-planned-scrub:{:?}:{}:{}",
        plan.mode(),
        plan.plan_identity(),
        plan.windows().len()
    ))
    .expect("S.3 planned-work digest basis is non-empty")
}
