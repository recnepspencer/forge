mod carriage_and_governance;
mod compatibility;
mod convergence;
mod declaration_identity;
mod reproducibility;
mod search;
mod transformation;

use super::WorthQueryPortableArtifactContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactContractValidationDenialKind {
    MissingRequiredContract,
    InvalidFamilyIdentity,
    UnversionedSchema,
    UnversionedProtocol,
    CallerDigestIdentity,
    AmbiguousOwnership,
    InvalidSemanticEvidence,
    MissingReusableComparator,
    ReproducibilityAuthorityMismatch,
    InvalidSearchContract,
    InvalidConvergenceContract,
    InvalidTransformationContract,
    InvalidAccessPathContract,
    DerivedReconstructionClaimsAuthority,
    InvalidCarriageContract,
    InvalidStructuralCounterContract,
    InvalidStageRole,
    InvalidGovernanceContract,
    UnsupportedSchemaVersion,
    UnsupportedProtocolVersion,
    RetiredSchemaVersion,
    AmbiguousMigration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactContractValidationDenial {
    kind: WorthQueryArtifactContractValidationDenialKind,
    subject: String,
}

impl WorthQueryArtifactContractValidationDenial {
    pub(crate) fn new(
        kind: WorthQueryArtifactContractValidationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryArtifactContractValidationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

pub(crate) fn validate_artifact_contract(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    declaration_identity::validate(contract)?;
    reproducibility::validate(contract)?;
    search::validate(contract)?;
    convergence::validate(contract)?;
    transformation::validate(contract)?;
    if !crate::domain_computation::validate_artifact_access_path(&contract.access_path) {
        return Err(WorthQueryArtifactContractValidationDenial::new(
            WorthQueryArtifactContractValidationDenialKind::InvalidAccessPathContract,
            contract.family.as_str(),
        ));
    }
    carriage_and_governance::validate(contract)?;
    compatibility::validate(contract)
}

pub(super) fn portable_text(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
