use forge_proof::TransitionOutcome;
use forge_store_aspect_native::StorePhysicalBoundaryWitness;

use crate::{
    StoreAuthenticityRequirement, StoreCurrentSecurityScopeWitnessSet, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreRawSecurityScopeDeclaration, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityMetadata {
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
}

impl StoreSecurityMetadata {
    #[cfg(test)]
    pub(crate) const fn from_scope_parts(
        key_scope: StoreKeyScope,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
        legacy_posture: StoreLegacySecurityPosture,
        key_version_posture: StoreKeyVersionPosture,
    ) -> Self {
        Self {
            key_scope,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
            legacy_posture,
            key_version_posture,
        }
    }

    pub fn from_current_security_scope(
        witnesses: &StoreCurrentSecurityScopeWitnessSet,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self {
            key_scope: witnesses.key_scope().key_scope(),
            tenant_scope: witnesses.tenant_scope().tenant_scope(),
            authenticity_requirement: witnesses.authenticity_scope().requirement(),
            custody_posture: witnesses.custody_scope().custody_posture(),
            legacy_posture,
            key_version_posture,
        }
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(self) -> StoreCustodyPosture {
        self.custody_posture
    }

    pub const fn legacy_posture(self) -> StoreLegacySecurityPosture {
        self.legacy_posture
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRawSecurityMetadataDeclaration {
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
}

impl StoreRawSecurityMetadataDeclaration {
    pub const fn new(
        key_scope: StoreKeyScope,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
        legacy_posture: StoreLegacySecurityPosture,
        key_version_posture: StoreKeyVersionPosture,
    ) -> Self {
        Self {
            key_scope,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
            legacy_posture,
            key_version_posture,
        }
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(self) -> StoreCustodyPosture {
        self.custody_posture
    }

    pub const fn legacy_posture(self) -> StoreLegacySecurityPosture {
        self.legacy_posture
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityMetadataProjectionSource {
    SerdeLoaded,
    TerminalProjected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRawSecurityMetadataProjection {
    source: StoreSecurityMetadataProjectionSource,
    declaration: StoreRawSecurityMetadataDeclaration,
}

impl StoreRawSecurityMetadataProjection {
    pub const fn serde_loaded(declaration: StoreRawSecurityMetadataDeclaration) -> Self {
        Self {
            source: StoreSecurityMetadataProjectionSource::SerdeLoaded,
            declaration,
        }
    }

    pub const fn terminal_projected(declaration: StoreRawSecurityMetadataDeclaration) -> Self {
        Self {
            source: StoreSecurityMetadataProjectionSource::TerminalProjected,
            declaration,
        }
    }

    pub fn to_raw_security_scope_declaration(
        self,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> StoreRawSecurityScopeDeclaration {
        StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
            physical_witness,
            self.declaration.key_scope(),
            self.declaration.key_version_posture(),
            self.declaration.tenant_scope(),
            Some(self.declaration.authenticity_requirement()),
            Some(self.declaration.custody_posture()),
        )
    }

    pub const fn source(self) -> StoreSecurityMetadataProjectionSource {
        self.source
    }

    pub const fn declaration(self) -> StoreRawSecurityMetadataDeclaration {
        self.declaration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityMetadataAdmissionInput {
    Candidate(StoreSecurityMetadata),
    MissingMetadata,
    UnsupportedMetadata,
    UnavailableMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityMetadataAdmissionDenial {
    MissingMetadata,
    UnsupportedMetadata,
    UnavailableMetadata,
    LegacyReadmissionRequired,
}

pub fn admit_store_security_metadata(
    input: StoreSecurityMetadataAdmissionInput,
) -> TransitionOutcome<StoreSecurityMetadata, StoreSecurityMetadataAdmissionDenial> {
    match input {
        StoreSecurityMetadataAdmissionInput::Candidate(metadata) => admit_candidate(metadata),
        StoreSecurityMetadataAdmissionInput::MissingMetadata => {
            TransitionOutcome::denied(StoreSecurityMetadataAdmissionDenial::MissingMetadata)
        }
        StoreSecurityMetadataAdmissionInput::UnsupportedMetadata => {
            TransitionOutcome::denied(StoreSecurityMetadataAdmissionDenial::UnsupportedMetadata)
        }
        StoreSecurityMetadataAdmissionInput::UnavailableMetadata => {
            TransitionOutcome::denied(StoreSecurityMetadataAdmissionDenial::UnavailableMetadata)
        }
    }
}

fn admit_candidate(
    metadata: StoreSecurityMetadata,
) -> TransitionOutcome<StoreSecurityMetadata, StoreSecurityMetadataAdmissionDenial> {
    if metadata
        .legacy_posture()
        .requires_readmission_when_unscoped()
    {
        return TransitionOutcome::denied(
            StoreSecurityMetadataAdmissionDenial::LegacyReadmissionRequired,
        );
    }

    if matches!(
        metadata.key_version_posture(),
        StoreKeyVersionPosture::Unsupported | StoreKeyVersionPosture::Denied
    ) || matches!(
        metadata.custody_posture(),
        StoreCustodyPosture::CustodyUnsupported | StoreCustodyPosture::CustodyDenied
    ) {
        return TransitionOutcome::denied(
            StoreSecurityMetadataAdmissionDenial::UnsupportedMetadata,
        );
    }

    if matches!(
        metadata.key_version_posture(),
        StoreKeyVersionPosture::Unavailable | StoreKeyVersionPosture::Stale
    ) || matches!(
        metadata.custody_posture(),
        StoreCustodyPosture::CustodyUnavailable | StoreCustodyPosture::ImportedUnreadmitted
    ) {
        return TransitionOutcome::denied(
            StoreSecurityMetadataAdmissionDenial::UnavailableMetadata,
        );
    }

    TransitionOutcome::success(metadata)
}
