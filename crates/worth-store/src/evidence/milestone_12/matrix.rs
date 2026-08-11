use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CompatibilityMatrixRow {
    AuthoritativeRead,
    AuthoritativeWrite,
    DerivedReuse,
    DerivedRebuild,
    DerivedSnapshotReuseAccepted,
    DerivedDeltaReuseAccepted,
    LayoutBasisSkewRejected,
    BulkResumeSkewRejected,
    MaintenanceSummaryRebuildAdmitted,
    TierManifestNonAuthorityPreserved,
    TierManifestSkewRejected,
    Restore,
    RestoreScopedBackupAdmitted,
    RestoreOutOfScopeRejected,
    RestorePublicationConflictRejected,
    RestoreMissingEdgeRejected,
    RollingUpgrade,
    RollingUpgradeTwoCapabilityAdmitted,
    RollingUpgradeMultiWriterRejected,
    RollingUpgradeAdapterRejected,
    AdapterParity,
}

impl Milestone12CompatibilityMatrixRow {
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthoritativeRead => "authoritative_read",
            Self::AuthoritativeWrite => "authoritative_write",
            Self::DerivedReuse => "derived_reuse",
            Self::DerivedRebuild => "derived_rebuild",
            Self::DerivedSnapshotReuseAccepted => "derived_snapshot_reuse_accepted",
            Self::DerivedDeltaReuseAccepted => "derived_delta_reuse_accepted",
            Self::LayoutBasisSkewRejected => "layout_basis_skew_rejected",
            Self::BulkResumeSkewRejected => "bulk_resume_skew_rejected",
            Self::MaintenanceSummaryRebuildAdmitted => "maintenance_summary_rebuild_admitted",
            Self::TierManifestNonAuthorityPreserved => "tier_manifest_non_authority_preserved",
            Self::TierManifestSkewRejected => "tier_manifest_skew_rejected",
            Self::Restore => "restore",
            Self::RestoreScopedBackupAdmitted => "restore_scoped_backup_admitted",
            Self::RestoreOutOfScopeRejected => "restore_out_of_scope_rejected",
            Self::RestorePublicationConflictRejected => "restore_publication_conflict_rejected",
            Self::RestoreMissingEdgeRejected => "restore_missing_edge_rejected",
            Self::RollingUpgrade => "rolling_upgrade",
            Self::RollingUpgradeTwoCapabilityAdmitted => "rolling_upgrade_two_capability_admitted",
            Self::RollingUpgradeMultiWriterRejected => "rolling_upgrade_multi_writer_rejected",
            Self::RollingUpgradeAdapterRejected => "rolling_upgrade_adapter_rejected",
            Self::AdapterParity => "adapter_parity",
        }
    }
}
