use worth_proof::TransitionOutcome;
use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_physical_format::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry, PhysicalFrameHeader,
    PhysicalPageHeader, PhysicalRawSecurityMetadataProjectionSource, PhysicalRootManifest,
    PhysicalSecurityMetadataDenial, SegmentManifestEntry, SegmentPageManifestEntry,
};

use crate::{
    StoreAuthenticityRequirement, StoreCurrentSecurityScopeWitnessSet, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreRawSecurityScopeDeclaration, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalSecurityMetadataCarrier {
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
}

impl StorePhysicalSecurityMetadataCarrier {
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

    pub fn for_page_header(
        witnesses: &StoreCurrentSecurityScopeWitnessSet,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::from_current_security_scope(witnesses, key_version_posture, legacy_posture)
    }

    pub fn for_frame_header(
        witnesses: &StoreCurrentSecurityScopeWitnessSet,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::from_current_security_scope(witnesses, key_version_posture, legacy_posture)
    }

    pub fn for_manifest(
        witnesses: &StoreCurrentSecurityScopeWitnessSet,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::from_current_security_scope(witnesses, key_version_posture, legacy_posture)
    }

    pub fn for_recovery_root_admission(
        witnesses: &StoreCurrentSecurityScopeWitnessSet,
        key_version_posture: StoreKeyVersionPosture,
        legacy_posture: StoreLegacySecurityPosture,
    ) -> Self {
        Self::from_current_security_scope(witnesses, key_version_posture, legacy_posture)
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
pub struct StoreRawPhysicalSecurityMetadataDeclaration {
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
}

impl StoreRawPhysicalSecurityMetadataDeclaration {
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
pub struct StoreRawPhysicalSecurityMetadataProjection {
    source: PhysicalRawSecurityMetadataProjectionSource,
    declaration: StoreRawPhysicalSecurityMetadataDeclaration,
}

impl StoreRawPhysicalSecurityMetadataProjection {
    pub const fn serde_loaded(declaration: StoreRawPhysicalSecurityMetadataDeclaration) -> Self {
        Self {
            source: PhysicalRawSecurityMetadataProjectionSource::SerdeLoaded,
            declaration,
        }
    }

    pub const fn terminal_projected(
        declaration: StoreRawPhysicalSecurityMetadataDeclaration,
    ) -> Self {
        Self {
            source: PhysicalRawSecurityMetadataProjectionSource::TerminalProjected,
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

    pub const fn source(self) -> PhysicalRawSecurityMetadataProjectionSource {
        self.source
    }

    pub const fn declaration(self) -> StoreRawPhysicalSecurityMetadataDeclaration {
        self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePhysicalSecurityMetadataEnvelope<T> {
    artifact: T,
    security_metadata: StorePhysicalSecurityMetadataCarrier,
}

impl<T> StorePhysicalSecurityMetadataEnvelope<T> {
    const fn new(artifact: T, security_metadata: StorePhysicalSecurityMetadataCarrier) -> Self {
        Self {
            artifact,
            security_metadata,
        }
    }

    pub const fn artifact(&self) -> &T {
        &self.artifact
    }

    pub const fn security_metadata(&self) -> StorePhysicalSecurityMetadataCarrier {
        self.security_metadata
    }
}

impl StorePhysicalSecurityMetadataEnvelope<PhysicalPageHeader> {
    pub const fn page_header(
        header: PhysicalPageHeader,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(header, security_metadata)
    }

    pub const fn header(&self) -> PhysicalPageHeader {
        self.artifact
    }
}

impl StorePhysicalSecurityMetadataEnvelope<PhysicalFrameHeader> {
    pub const fn frame_header(
        header: PhysicalFrameHeader,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(header, security_metadata)
    }

    pub const fn header(&self) -> PhysicalFrameHeader {
        self.artifact
    }
}

impl StorePhysicalSecurityMetadataEnvelope<PhysicalRootManifest> {
    pub const fn root_manifest(
        manifest: PhysicalRootManifest,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(manifest, security_metadata)
    }

    pub const fn manifest(&self) -> &PhysicalRootManifest {
        &self.artifact
    }
}

pub type StoreSegmentSecurityMetadataEnvelope =
    StorePhysicalSecurityMetadataEnvelope<SegmentManifestEntry>;
pub type StoreSegmentPageSecurityMetadataEnvelope =
    StorePhysicalSecurityMetadataEnvelope<SegmentPageManifestEntry>;
pub type StoreExtentSecurityMetadataEnvelope =
    StorePhysicalSecurityMetadataEnvelope<ExtentManifestEntry>;
pub type StoreAllocationClassSecurityMetadataEnvelope =
    StorePhysicalSecurityMetadataEnvelope<AllocationClassManifestEntry>;
pub type StoreFreeSpaceSecurityMetadataEnvelope =
    StorePhysicalSecurityMetadataEnvelope<FreeSpaceManifestEntry>;

impl StorePhysicalSecurityMetadataEnvelope<SegmentManifestEntry> {
    pub const fn segment_manifest_entry(
        entry: SegmentManifestEntry,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl StorePhysicalSecurityMetadataEnvelope<SegmentPageManifestEntry> {
    pub const fn segment_page_manifest_entry(
        entry: SegmentPageManifestEntry,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl StorePhysicalSecurityMetadataEnvelope<ExtentManifestEntry> {
    pub const fn extent_manifest_entry(
        entry: ExtentManifestEntry,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl StorePhysicalSecurityMetadataEnvelope<AllocationClassManifestEntry> {
    pub const fn allocation_class_manifest_entry(
        entry: AllocationClassManifestEntry,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl StorePhysicalSecurityMetadataEnvelope<FreeSpaceManifestEntry> {
    pub const fn free_space_manifest_entry(
        entry: FreeSpaceManifestEntry,
        security_metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorePhysicalSecurityMetadataAdmissionInput {
    Candidate(StorePhysicalSecurityMetadataCarrier),
    MissingPlatformMetadata,
    UnsupportedPlatformMetadata,
    UnavailablePlatformMetadata,
}

pub fn admit_store_physical_security_metadata(
    input: StorePhysicalSecurityMetadataAdmissionInput,
) -> TransitionOutcome<StorePhysicalSecurityMetadataCarrier, PhysicalSecurityMetadataDenial> {
    match input {
        StorePhysicalSecurityMetadataAdmissionInput::Candidate(metadata) => {
            admit_candidate_physical_security_metadata(metadata)
        }
        StorePhysicalSecurityMetadataAdmissionInput::MissingPlatformMetadata => {
            TransitionOutcome::denied(
                PhysicalSecurityMetadataDenial::MissingPlatformSecurityMetadata,
            )
        }
        StorePhysicalSecurityMetadataAdmissionInput::UnsupportedPlatformMetadata => {
            TransitionOutcome::denied(
                PhysicalSecurityMetadataDenial::UnsupportedPlatformSecurityMetadata,
            )
        }
        StorePhysicalSecurityMetadataAdmissionInput::UnavailablePlatformMetadata => {
            TransitionOutcome::denied(
                PhysicalSecurityMetadataDenial::UnavailablePlatformSecurityMetadata,
            )
        }
    }
}

fn admit_candidate_physical_security_metadata(
    metadata: StorePhysicalSecurityMetadataCarrier,
) -> TransitionOutcome<StorePhysicalSecurityMetadataCarrier, PhysicalSecurityMetadataDenial> {
    if metadata
        .legacy_posture()
        .requires_readmission_when_unscoped()
    {
        return TransitionOutcome::denied(
            PhysicalSecurityMetadataDenial::LegacyReadmissionRequired,
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
            PhysicalSecurityMetadataDenial::UnsupportedPlatformSecurityMetadata,
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
            PhysicalSecurityMetadataDenial::UnavailablePlatformSecurityMetadata,
        );
    }

    TransitionOutcome::success(metadata)
}
