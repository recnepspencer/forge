use std::marker::PhantomData;

use super::categories::{
    ArtifactCategory, FoundationalBoundaryArtifactCategory, FoundationalBoundaryCategorySurface,
    ReceiptCategory, ReportCategory, SummaryCategory,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryArtifactRole {
    AuthoritativeCurrent,
    DerivedProjection,
    SupportOnly,
    PlannedWork,
    ReceiptEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBoundaryRoleDefinition {
    role: FoundationalBoundaryArtifactRole,
    name: &'static str,
    intended_claim: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalBoundaryRoleDefinition {
    pub const fn new(
        role: FoundationalBoundaryArtifactRole,
        name: &'static str,
        intended_claim: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            role,
            name,
            intended_claim,
            must_not_mean,
        }
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_claim(&self) -> &'static str {
        self.intended_claim
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

const AUTHORITATIVE_CURRENT_DEFINITION: FoundationalBoundaryRoleDefinition =
    FoundationalBoundaryRoleDefinition::new(
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        "authoritative_current",
        "current authoritative truth claim over a structured artifact boundary",
        "support-only description, planned work, derived projection, or receipt attestation",
    );
const DERIVED_PROJECTION_DEFINITION: FoundationalBoundaryRoleDefinition =
    FoundationalBoundaryRoleDefinition::new(
        FoundationalBoundaryArtifactRole::DerivedProjection,
        "derived_projection",
        "derived or projected boundary meaning that depends on authoritative truth without replacing it",
        "authoritative current truth or completed receipt evidence",
    );
const SUPPORT_ONLY_DEFINITION: FoundationalBoundaryRoleDefinition =
    FoundationalBoundaryRoleDefinition::new(
        FoundationalBoundaryArtifactRole::SupportOnly,
        "support_only",
        "support-facing or explanatory boundary meaning that stays descriptive",
        "authoritative truth or receipt attestation",
    );
const PLANNED_WORK_DEFINITION: FoundationalBoundaryRoleDefinition =
    FoundationalBoundaryRoleDefinition::new(
        FoundationalBoundaryArtifactRole::PlannedWork,
        "planned_work",
        "descriptive boundary meaning about intended work that has not yet become a receipt",
        "completed receipt evidence or authoritative current truth",
    );
const RECEIPT_EVIDENCE_DEFINITION: FoundationalBoundaryRoleDefinition =
    FoundationalBoundaryRoleDefinition::new(
        FoundationalBoundaryArtifactRole::ReceiptEvidence,
        "receipt_evidence",
        "completed-boundary attestation claim",
        "planned work, support-only description, or authoritative truth container",
    );

pub const fn boundary_role_definitions() -> [FoundationalBoundaryRoleDefinition; 5] {
    [
        AUTHORITATIVE_CURRENT_DEFINITION,
        DERIVED_PROJECTION_DEFINITION,
        SUPPORT_ONLY_DEFINITION,
        PLANNED_WORK_DEFINITION,
        RECEIPT_EVIDENCE_DEFINITION,
    ]
}

pub trait FoundationalBoundaryRoleMarker: sealed::Sealed {
    const ROLE: FoundationalBoundaryArtifactRole;
    fn definition() -> &'static FoundationalBoundaryRoleDefinition;
}

pub trait FoundationalDerivedProjectionCategory: sealed::Sealed {}
pub trait FoundationalSupportOnlyCategory: sealed::Sealed {}
pub trait FoundationalPlannedWorkCategory: sealed::Sealed {}
pub trait FoundationalReceiptEvidenceCategory: sealed::Sealed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthoritativeCurrentRole(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedProjectionRole(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportOnlyRole(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlannedWorkRole(());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptEvidenceRole(());

impl FoundationalBoundaryRoleMarker for AuthoritativeCurrentRole {
    const ROLE: FoundationalBoundaryArtifactRole =
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent;

    fn definition() -> &'static FoundationalBoundaryRoleDefinition {
        &AUTHORITATIVE_CURRENT_DEFINITION
    }
}

impl FoundationalBoundaryRoleMarker for DerivedProjectionRole {
    const ROLE: FoundationalBoundaryArtifactRole =
        FoundationalBoundaryArtifactRole::DerivedProjection;

    fn definition() -> &'static FoundationalBoundaryRoleDefinition {
        &DERIVED_PROJECTION_DEFINITION
    }
}

impl FoundationalBoundaryRoleMarker for SupportOnlyRole {
    const ROLE: FoundationalBoundaryArtifactRole = FoundationalBoundaryArtifactRole::SupportOnly;

    fn definition() -> &'static FoundationalBoundaryRoleDefinition {
        &SUPPORT_ONLY_DEFINITION
    }
}

impl FoundationalBoundaryRoleMarker for PlannedWorkRole {
    const ROLE: FoundationalBoundaryArtifactRole = FoundationalBoundaryArtifactRole::PlannedWork;

    fn definition() -> &'static FoundationalBoundaryRoleDefinition {
        &PLANNED_WORK_DEFINITION
    }
}

impl FoundationalBoundaryRoleMarker for ReceiptEvidenceRole {
    const ROLE: FoundationalBoundaryArtifactRole =
        FoundationalBoundaryArtifactRole::ReceiptEvidence;

    fn definition() -> &'static FoundationalBoundaryRoleDefinition {
        &RECEIPT_EVIDENCE_DEFINITION
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryRoleClaimDenial {
    AuthoritativeCurrentRequiresArtifactCategory,
    DerivedProjectionCannotUseReceiptCategory,
    SupportOnlyCannotUseReceiptCategory,
    PlannedWorkCannotUseReceiptCategory,
    ReceiptEvidenceRequiresReceiptCategory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryRoleClaim<Surface, Role> {
    surface: Surface,
    role: FoundationalBoundaryArtifactRole,
    marker: PhantomData<Role>,
}

impl<Surface, Role> FoundationalBoundaryRoleClaim<Surface, Role>
where
    Surface: FoundationalBoundaryCategorySurface,
    Role: FoundationalBoundaryRoleMarker,
{
    pub(crate) fn new(surface: Surface) -> Self {
        Self {
            surface,
            role: Role::ROLE,
            marker: PhantomData,
        }
    }

    pub const fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn into_surface(self) -> Surface {
        self.surface
    }

    pub fn into_parts(
        self,
    ) -> (
        Surface,
        FoundationalBoundaryArtifactCategory,
        FoundationalBoundaryArtifactRole,
    ) {
        let category = self.category();
        let role = self.role();
        (self.surface, category, role)
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub fn role_definition(&self) -> &'static FoundationalBoundaryRoleDefinition {
        Role::definition()
    }

    pub fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.surface.category()
    }
}

pub type FoundationalDerivedProjectionBoundaryClaim<Surface> =
    FoundationalBoundaryRoleClaim<Surface, DerivedProjectionRole>;
pub type FoundationalSupportOnlyBoundaryClaim<Surface> =
    FoundationalBoundaryRoleClaim<Surface, SupportOnlyRole>;
pub type FoundationalPlannedWorkBoundaryClaim<Surface> =
    FoundationalBoundaryRoleClaim<Surface, PlannedWorkRole>;
pub type FoundationalReceiptEvidenceBoundaryClaim<Surface> =
    FoundationalBoundaryRoleClaim<Surface, ReceiptEvidenceRole>;

pub fn evaluate_boundary_role_claim_legality(
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
) -> Result<(), FoundationalBoundaryRoleClaimDenial> {
    match (category, role) {
        (
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        ) => Ok(()),
        (
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::DerivedProjection,
        ) => Err(FoundationalBoundaryRoleClaimDenial::DerivedProjectionCannotUseReceiptCategory),
        (
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::SupportOnly,
        ) => Err(FoundationalBoundaryRoleClaimDenial::SupportOnlyCannotUseReceiptCategory),
        (
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::PlannedWork,
        ) => Err(FoundationalBoundaryRoleClaimDenial::PlannedWorkCannotUseReceiptCategory),
        (
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::ReceiptEvidence,
        ) => Ok(()),
        (
            FoundationalBoundaryArtifactCategory::Summary
            | FoundationalBoundaryArtifactCategory::Report
            | FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::DerivedProjection,
        ) => Ok(()),
        (
            FoundationalBoundaryArtifactCategory::Summary
            | FoundationalBoundaryArtifactCategory::Report
            | FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::SupportOnly,
        ) => Ok(()),
        (
            FoundationalBoundaryArtifactCategory::Summary
            | FoundationalBoundaryArtifactCategory::Report
            | FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::PlannedWork,
        ) => Ok(()),
        (
            FoundationalBoundaryArtifactCategory::Summary
            | FoundationalBoundaryArtifactCategory::Report
            | FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::ReceiptEvidence,
        ) => Err(FoundationalBoundaryRoleClaimDenial::ReceiptEvidenceRequiresReceiptCategory),
        (
            FoundationalBoundaryArtifactCategory::Summary
            | FoundationalBoundaryArtifactCategory::Report
            | FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        ) => Err(FoundationalBoundaryRoleClaimDenial::AuthoritativeCurrentRequiresArtifactCategory),
    }
}

pub fn claim_derived_projection_boundary_surface<Surface>(
    surface: Surface,
) -> FoundationalDerivedProjectionBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface,
    Surface::Category: FoundationalDerivedProjectionCategory,
{
    FoundationalBoundaryRoleClaim::new(surface)
}

pub fn claim_support_only_boundary_surface<Surface>(
    surface: Surface,
) -> FoundationalSupportOnlyBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface,
    Surface::Category: FoundationalSupportOnlyCategory,
{
    FoundationalBoundaryRoleClaim::new(surface)
}

pub fn claim_planned_work_boundary_surface<Surface>(
    surface: Surface,
) -> FoundationalPlannedWorkBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface,
    Surface::Category: FoundationalPlannedWorkCategory,
{
    FoundationalBoundaryRoleClaim::new(surface)
}

pub fn claim_receipt_evidence_boundary_surface<Surface>(
    surface: Surface,
) -> FoundationalReceiptEvidenceBoundaryClaim<Surface>
where
    Surface: FoundationalBoundaryCategorySurface,
    Surface::Category: FoundationalReceiptEvidenceCategory,
{
    FoundationalBoundaryRoleClaim::new(surface)
}

mod sealed {
    use super::{
        AuthoritativeCurrentRole, DerivedProjectionRole, PlannedWorkRole, ReceiptEvidenceRole,
        SupportOnlyRole,
    };
    use crate::boundary_artifacts::categories::{
        ArtifactCategory, ReceiptCategory, ReportCategory, SummaryCategory,
    };

    pub trait Sealed {}

    impl Sealed for AuthoritativeCurrentRole {}
    impl Sealed for DerivedProjectionRole {}
    impl Sealed for SupportOnlyRole {}
    impl Sealed for PlannedWorkRole {}
    impl Sealed for ReceiptEvidenceRole {}
    impl Sealed for SummaryCategory {}
    impl Sealed for ReportCategory {}
    impl Sealed for ArtifactCategory {}
    impl Sealed for ReceiptCategory {}
}

impl FoundationalDerivedProjectionCategory for SummaryCategory {}
impl FoundationalDerivedProjectionCategory for ReportCategory {}
impl FoundationalDerivedProjectionCategory for ArtifactCategory {}

impl FoundationalSupportOnlyCategory for SummaryCategory {}
impl FoundationalSupportOnlyCategory for ReportCategory {}
impl FoundationalSupportOnlyCategory for ArtifactCategory {}

impl FoundationalPlannedWorkCategory for SummaryCategory {}
impl FoundationalPlannedWorkCategory for ReportCategory {}
impl FoundationalPlannedWorkCategory for ArtifactCategory {}

impl FoundationalReceiptEvidenceCategory for ReceiptCategory {}
