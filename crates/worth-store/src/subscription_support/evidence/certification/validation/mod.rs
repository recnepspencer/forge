mod boundedness;
mod compatibility;
mod drift;
mod expectations;
mod maintenance;
mod operational_guards;
mod portability;
mod rejections;
mod resume;
mod retention;

use super::lane::SubscriptionSupportCertificationLaneKind;
use super::outcome::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;

pub(super) fn validate_lane_semantics(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    match outcome.lane() {
        SubscriptionSupportCertificationLaneKind::ExactResumeControl => resume::validate_exact_resume(outcome),
        SubscriptionSupportCertificationLaneKind::RestartExactResume => resume::validate_exact_resume(outcome),
        SubscriptionSupportCertificationLaneKind::RebuildRequiredMissingSupport => resume::validate_missing_support_rebuild(outcome),
        SubscriptionSupportCertificationLaneKind::DegradedButRecoverable => resume::validate_degraded_recoverable(outcome),
        SubscriptionSupportCertificationLaneKind::NotResumableBasisDrift => drift::validate_basis_drift(outcome),
        SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift => drift::validate_cursor_drift(outcome),
        SubscriptionSupportCertificationLaneKind::SupportDigestDrift => drift::validate_support_digest_drift(outcome),
        SubscriptionSupportCertificationLaneKind::CompatibilityDrift => drift::validate_compatibility_drift(outcome),
        SubscriptionSupportCertificationLaneKind::CursorOnlyExactResumeRejected => rejections::validate_basic_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::CrossFamilyReuseRejected => drift::validate_cross_family_reuse(outcome),
        SubscriptionSupportCertificationLaneKind::SessionMemoryLossNonAuthoritative => resume::validate_session_memory_loss(outcome),
        SubscriptionSupportCertificationLaneKind::TierRecallCostOnly => resume::validate_tier_recall(outcome),
        SubscriptionSupportCertificationLaneKind::RuntimeHandoffEquivalence => resume::validate_exact_resume(outcome),
        SubscriptionSupportCertificationLaneKind::UnknownUpstreamAuthorityRejected => rejections::validate_basic_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::NonCanonicalScopeRejected => rejections::validate_basic_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::UnsupportedFamilyKindRejected => rejections::validate_basic_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::MultiDriftBasisPrecedence => drift::validate_basis_precedence(outcome),
        SubscriptionSupportCertificationLaneKind::MultiDriftCompatibilityPrecedence => drift::validate_compatibility_precedence(outcome),
        SubscriptionSupportCertificationLaneKind::RebuildBasisMissingNotResumable => drift::validate_missing_rebuild_basis(outcome),
        SubscriptionSupportCertificationLaneKind::BackendAccessStructureDebt => rejections::validate_access_structure_debt(outcome),
        SubscriptionSupportCertificationLaneKind::DecodedRowPublicationRejected => rejections::validate_basic_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::OversizedPayloadRejectedBeforeDecode => rejections::validate_oversized_payload(outcome),
        SubscriptionSupportCertificationLaneKind::RestartShardBoundedReconstruction => resume::validate_restart_shard(outcome),
        SubscriptionSupportCertificationLaneKind::ResultCostSurfaceExact => resume::validate_result_cost_surface(outcome),
        SubscriptionSupportCertificationLaneKind::BatchClassificationDebt => boundedness::validate_batch_classification_debt(outcome),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityExactMigration => compatibility::validate_exact_compatibility(outcome),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityDegraded => compatibility::validate_degraded_compatibility(outcome),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityOldReaderRejected => compatibility::validate_compatibility_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected => compatibility::validate_compatibility_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityVersionSkewRejected => compatibility::validate_compatibility_rejection(outcome),
        SubscriptionSupportCertificationLaneKind::SupportRetentionExactPreserved => retention::validate_retention_exact(outcome),
        SubscriptionSupportCertificationLaneKind::SupportRetentionCompactedExact => retention::validate_retention_compacted(outcome),
        SubscriptionSupportCertificationLaneKind::SupportRetentionReclaimedRebuildable => retention::validate_retention_reclaimed(outcome),
        SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy => retention::validate_retention_expired(outcome),
        SubscriptionSupportCertificationLaneKind::SupportPortabilityFullScopeReplicated => portability::validate_portability_full(outcome),
        SubscriptionSupportCertificationLaneKind::SupportPortabilityPartialOmission => portability::validate_portability_partial(outcome),
        SubscriptionSupportCertificationLaneKind::SupportPortabilityImportAdmitted => portability::validate_portability_import(outcome),
        SubscriptionSupportCertificationLaneKind::SupportPortabilityImportMissingBasisNotResumable => portability::validate_portability_missing_basis(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceRebuildAdmitted => maintenance::validate_maintenance_rebuild(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceRefreshAdmitted => maintenance::validate_maintenance_refresh(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceCompatibilityMigrationAdmitted => maintenance::validate_maintenance_compatibility(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceDegradationRecoveryAdmitted => maintenance::validate_maintenance_degradation(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceInterruptedRestartRecovered => maintenance::validate_maintenance_interrupted(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceDelayedDebtReported => maintenance::validate_maintenance_delay(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceCoalescedRebuildAdmitted => maintenance::validate_maintenance_coalesced(outcome),
        SubscriptionSupportCertificationLaneKind::SupportFamilyLocalBatchBounded => boundedness::validate_family_local_bounded(outcome),
        SubscriptionSupportCertificationLaneKind::SupportBasisLocalBatchBounded => boundedness::validate_basis_local_bounded(outcome),
        SubscriptionSupportCertificationLaneKind::SupportPortabilityScopeBatchBounded => boundedness::validate_portability_scope_bounded(outcome),
        SubscriptionSupportCertificationLaneKind::SupportMaintenanceKeyBatchBounded => boundedness::validate_maintenance_key_bounded(outcome),
        SubscriptionSupportCertificationLaneKind::SupportStoreGlobalDensityRejected => operational_guards::validate_store_global_density(outcome),
        SubscriptionSupportCertificationLaneKind::SupportForegroundOperationalWorkRejected => operational_guards::validate_foreground_work(outcome),
        SubscriptionSupportCertificationLaneKind::SupportBatchReceiptReuseVerified => operational_guards::validate_batch_receipt_reuse(outcome),
        SubscriptionSupportCertificationLaneKind::SupportActionPublicationCrashRecovered => operational_guards::validate_action_recovery(outcome),
        SubscriptionSupportCertificationLaneKind::SupportGlobalScanRecoveryForbidden => operational_guards::validate_global_scan(outcome),
        SubscriptionSupportCertificationLaneKind::SupportHiddenExactLossForbidden => operational_guards::validate_hidden_exact_loss(outcome),
    }
}
