use forge_store_security::{StoreCurrentSecurityScopeWitnessSet, StoreSecurityScopeIdentity};

use crate::catalog::{
    classify_family, require_production_authority, require_strategy_lifecycle,
    ArtifactFamilyClassification, ArtifactFamilyDenial, ArtifactFamilyDenialKind,
    ArtifactFamilyLifecycleAdmission, ArtifactFamilyStrategyLane,
    PhysicalArtifactFamilyDeclaration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArtifactFamilyAdmissionCaseId {
    Admitted,
    Denied(ArtifactFamilyDenialKind),
}

impl ArtifactFamilyAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "layout.artifact_family.admission.admitted",
            Self::Denied(ArtifactFamilyDenialKind::DerivedFamilyCannotMintProductionAuthority) => {
                "layout.artifact_family.admission.denied.derived"
            }
            Self::Denied(
                ArtifactFamilyDenialKind::DiagnosticFamilyCannotMintProductionAuthority,
            ) => "layout.artifact_family.admission.denied.diagnostic",
            Self::Denied(ArtifactFamilyDenialKind::TerminalProjectionCannotMintAuthority) => {
                "layout.artifact_family.admission.denied.terminal"
            }
            Self::Denied(ArtifactFamilyDenialKind::CourtroomCannotMintAuthority) => {
                "layout.artifact_family.admission.denied.courtroom"
            }
            Self::Denied(ArtifactFamilyDenialKind::VerifierLaneCannotEnterStrategyAdmission) => {
                "layout.artifact_family.admission.denied.verifier_lane"
            }
            Self::Denied(
                ArtifactFamilyDenialKind::ReadmissionFamilyCannotEnterStrategyAdmission,
            ) => "layout.artifact_family.admission.denied.readmission",
            Self::Denied(
                ArtifactFamilyDenialKind::TransferBoundaryFamilyCannotEnterStrategyAdmission,
            ) => "layout.artifact_family.admission.denied.transfer_boundary",
            Self::Denied(
                ArtifactFamilyDenialKind::OfflineImportOnlyFamilyCannotEnterStrategyAdmission,
            ) => "layout.artifact_family.admission.denied.offline_import",
            Self::Denied(_) => "layout.artifact_family.admission.denied.unadvertised",
        }
    }
}

pub fn artifact_family_admission_cases() -> impl Iterator<Item = ArtifactFamilyAdmissionCaseId> {
    use ArtifactFamilyDenialKind as Denial;
    [
        ArtifactFamilyAdmissionCaseId::Admitted,
        ArtifactFamilyAdmissionCaseId::Denied(Denial::DerivedFamilyCannotMintProductionAuthority),
        ArtifactFamilyAdmissionCaseId::Denied(
            Denial::DiagnosticFamilyCannotMintProductionAuthority,
        ),
        ArtifactFamilyAdmissionCaseId::Denied(Denial::TerminalProjectionCannotMintAuthority),
        ArtifactFamilyAdmissionCaseId::Denied(
            Denial::ReadmissionFamilyCannotEnterStrategyAdmission,
        ),
        ArtifactFamilyAdmissionCaseId::Denied(
            Denial::TransferBoundaryFamilyCannotEnterStrategyAdmission,
        ),
        ArtifactFamilyAdmissionCaseId::Denied(
            Denial::OfflineImportOnlyFamilyCannotEnterStrategyAdmission,
        ),
    ]
    .into_iter()
}

#[derive(Debug, PartialEq, Eq)]
enum ArtifactFamilyAdmissionCase {
    Admitted(AdmittedPhysicalArtifactFamily),
    Denied(ArtifactFamilyDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactFamilyAdmissionOutcome {
    case: ArtifactFamilyAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyAdmissionView<'a> {
    Admitted(&'a AdmittedPhysicalArtifactFamily),
    Denied(&'a ArtifactFamilyDenial),
}

impl ArtifactFamilyAdmissionOutcome {
    fn admit(
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Self {
        let case = match AdmittedPhysicalArtifactFamily::admit(declaration, security) {
            Ok(family) => ArtifactFamilyAdmissionCase::Admitted(family),
            Err(denial) => ArtifactFamilyAdmissionCase::Denied(denial),
        };
        Self { case }
    }

    pub const fn view(&self) -> ArtifactFamilyAdmissionView<'_> {
        match &self.case {
            ArtifactFamilyAdmissionCase::Admitted(family) => {
                ArtifactFamilyAdmissionView::Admitted(family)
            }
            ArtifactFamilyAdmissionCase::Denied(denial) => {
                ArtifactFamilyAdmissionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> ArtifactFamilyAdmissionCaseId {
        match &self.case {
            ArtifactFamilyAdmissionCase::Admitted(_) => ArtifactFamilyAdmissionCaseId::Admitted,
            ArtifactFamilyAdmissionCase::Denied(denial) => {
                ArtifactFamilyAdmissionCaseId::Denied(denial.kind())
            }
        }
    }

    pub fn into_result(self) -> Result<AdmittedPhysicalArtifactFamily, ArtifactFamilyDenial> {
        match self.case {
            ArtifactFamilyAdmissionCase::Admitted(family) => Ok(family),
            ArtifactFamilyAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> AdmittedPhysicalArtifactFamily {
        self.into_result().unwrap()
    }

    pub fn unwrap_err(self) -> ArtifactFamilyDenial {
        self.into_result().unwrap_err()
    }
}

impl PartialEq<Result<AdmittedPhysicalArtifactFamily, ArtifactFamilyDenial>>
    for ArtifactFamilyAdmissionOutcome
{
    fn eq(&self, other: &Result<AdmittedPhysicalArtifactFamily, ArtifactFamilyDenial>) -> bool {
        match (self.view(), other) {
            (ArtifactFamilyAdmissionView::Admitted(left), Ok(right)) => left == right,
            (ArtifactFamilyAdmissionView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalArtifactFamily {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    security_identity: StoreSecurityScopeIdentity,
    authority_identity: forge_store_authority::StoreCurrentAuthorityIdentity,
}

impl AdmittedPhysicalArtifactFamily {
    fn admit(
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<Self, ArtifactFamilyDenial> {
        let classification = classify_family(declaration);
        let authority = require_production_authority(classification)?;
        let lifecycle = require_strategy_lifecycle(authority)?;
        Ok(Self {
            lifecycle,
            security_identity: security.key_scope().identity(),
            authority_identity: security.authority_identity(),
        })
    }

    pub const fn declaration(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.lifecycle.declaration()
    }

    pub const fn family_id(self) -> forge_store_contracts::DurableArtifactFamilyId {
        self.lifecycle.family_id()
    }

    pub const fn admitted_lane(self) -> ArtifactFamilyStrategyLane {
        self.lifecycle.admitted_lane()
    }

    pub const fn classification(self) -> ArtifactFamilyClassification {
        self.lifecycle.authority().classification()
    }

    pub const fn security_identity(self) -> StoreSecurityScopeIdentity {
        self.security_identity
    }

    pub const fn authority_identity(self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub(crate) const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }
}

impl crate::catalog::LayoutDeclarationsFacade {
    pub fn admit_physical_artifact_family(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> ArtifactFamilyAdmissionOutcome {
        ArtifactFamilyAdmissionOutcome::admit(declaration, security)
    }
}
