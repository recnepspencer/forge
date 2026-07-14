#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreKeyScope {
    StoreManagedRoot,
    TenantEnvelope,
    ArtifactEnvelope,
    PageEnvelope,
    WalCheckpointEnvelope,
    BlobChunkEnvelope,
    BackupExportEnvelope,
    RepairScopeEnvelope,
    SecurityLifecycleFoundation,
}

impl StoreKeyScope {
    pub const fn is_store_owned_scope(self) -> bool {
        true
    }

    pub const fn is_kms_key_identifier(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKeyVersionPosture {
    Current,
    Stale,
    RebindRequired,
    Unsupported,
    Unavailable,
    Denied,
}

impl StoreKeyVersionPosture {
    pub const fn is_admissible_for_platform_lane(self) -> bool {
        matches!(self, Self::Current)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreTenantScope {
    StoreInternal,
    TenantPhysicalBoundary,
    MultiTenantPhysicalBoundary,
    BackupRestoreBoundary,
    RepairBlastRadius,
    ImportReadmissionBoundary,
    SecurityLifecycleFoundation,
}

impl StoreTenantScope {
    pub const fn is_store_physical_blast_radius(self) -> bool {
        true
    }

    pub const fn is_identity_provider_claim(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreCustodyPosture {
    InternalStoreCustody,
    ExportPrepared,
    ExportedOutOfCustody,
    ImportedUnreadmitted,
    Readmitted,
    CustodyUnavailable,
    CustodyDenied,
    CustodyUnsupported,
}

impl StoreCustodyPosture {
    pub const fn is_store_custody_vocabulary(self) -> bool {
        true
    }

    pub const fn is_iam_role(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreLegacySecurityPosture {
    NativeScoped,
    LegacyUnscoped,
    ReadmissionRequired,
    SecurityMetadataUnavailable,
    UnsupportedLegacyArtifact,
}

impl StoreLegacySecurityPosture {
    pub const fn requires_readmission_when_unscoped(self) -> bool {
        matches!(
            self,
            Self::LegacyUnscoped
                | Self::ReadmissionRequired
                | Self::SecurityMetadataUnavailable
                | Self::UnsupportedLegacyArtifact
        )
    }
}
