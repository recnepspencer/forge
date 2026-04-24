use super::{
    stable_digest, SubscriptionResumeClassification, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationReport, SubscriptionSupportCompatibilityOutcome,
    SubscriptionSupportCompatibilityReport, SubscriptionSupportDensityClass,
    SubscriptionSupportDriftCause, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportMissingSupportRecoveryReport, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportPortabilityReport,
    SubscriptionSupportPostActionReport, SubscriptionSupportResultCostSurface,
};
use crate::failure::{StoreError, StoreErrorKind};
use crate::{
    SubscriptionSupportAccessStructureReport, SupportBatchProofKind, SupportBatchReceiptReuseReport,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubscriptionSupportCounterSnapshot {
    declarations_admitted: u64,
    declarations_rejected: u64,
    artifacts_published: u64,
    artifacts_fetched: u64,
    family_catalog_lookups: u64,
    lookup_keys_used: u64,
    rows_read: u64,
    duplicate_retries: u64,
    identity_collisions: u64,
    access_structure_debts: u64,
    malformed_support_records: u64,
    missing_manifest_rejections: u64,
    compatibility_drifts: u64,
    exact_classifications: u64,
    degraded_classifications: u64,
    rebuild_required_classifications: u64,
    denied_classifications: u64,
    budget_denials: u64,
    restart_reconstruction_count: u64,
    restart_shards_touched: u64,
    restart_global_scan_count: u64,
    rebuild_basis_plan_count: u64,
    runtime_handoff_count: u64,
    operational_verdict_translation_count: u64,
    operational_verdict_translation_rejections: u64,
    support_action_envelope_publications: u64,
    support_hot_path_rejections: u64,
    support_batch_receipt_reuse_count: u64,
    support_store_global_debt_rejections: u64,
    support_retention_plan_count: u64,
    support_retention_affected_entries: u64,
    support_retained_family_count: u64,
    support_reclaimed_family_count: u64,
    support_compacted_basis_count: u64,
    support_expired_family_count: u64,
    support_reclaim_consequence_count: u64,
    support_policy_expiration_count: u64,
    support_compatibility_plan_count: u64,
    support_compatibility_affected_entries: u64,
    support_manifest_admission_count: u64,
    support_compatibility_receipt_binding_count: u64,
    support_exact_compatible_migration_count: u64,
    support_degraded_compatibility_count: u64,
    support_version_skew_rejection_count: u64,
    support_portability_plan_count: u64,
    support_portability_manifest_entries: u64,
    support_portability_required_basis_count: u64,
    support_portability_omitted_support_count: u64,
    support_replication_inclusion_count: u64,
    support_replication_omission_count: u64,
    support_import_admission_count: u64,
    support_import_rejection_count: u64,
    support_capsule_manifest_budget_denial_count: u64,
    support_maintenance_descriptor_count: u64,
    support_maintenance_rebuild_debt_count: u64,
    support_maintenance_refresh_count: u64,
    support_maintenance_compatibility_migration_count: u64,
    support_maintenance_degradation_recovery_count: u64,
    support_maintenance_coalesced_duplicate_count: u64,
    support_maintenance_interrupted_restart_recovery_count: u64,
}

impl SubscriptionSupportCounterSnapshot {
    pub fn declarations_admitted(&self) -> u64 {
        self.declarations_admitted
    }

    pub fn declarations_rejected(&self) -> u64 {
        self.declarations_rejected
    }

    pub fn budget_denials(&self) -> u64 {
        self.budget_denials
    }

    pub fn artifacts_published(&self) -> u64 {
        self.artifacts_published
    }

    pub fn artifacts_fetched(&self) -> u64 {
        self.artifacts_fetched
    }

    pub fn family_catalog_lookups(&self) -> u64 {
        self.family_catalog_lookups
    }

    pub fn lookup_keys_used(&self) -> u64 {
        self.lookup_keys_used
    }

    pub fn rows_read(&self) -> u64 {
        self.rows_read
    }

    pub fn duplicate_retries(&self) -> u64 {
        self.duplicate_retries
    }

    pub fn identity_collisions(&self) -> u64 {
        self.identity_collisions
    }

    pub fn access_structure_debts(&self) -> u64 {
        self.access_structure_debts
    }

    pub fn malformed_support_records(&self) -> u64 {
        self.malformed_support_records
    }

    pub fn exact_classifications(&self) -> u64 {
        self.exact_classifications
    }

    pub fn degraded_classifications(&self) -> u64 {
        self.degraded_classifications
    }

    pub fn rebuild_required_classifications(&self) -> u64 {
        self.rebuild_required_classifications
    }

    pub fn denied_classifications(&self) -> u64 {
        self.denied_classifications
    }

    pub fn restart_reconstruction_count(&self) -> u64 {
        self.restart_reconstruction_count
    }

    pub fn restart_shards_touched(&self) -> u64 {
        self.restart_shards_touched
    }

    pub fn restart_global_scan_count(&self) -> u64 {
        self.restart_global_scan_count
    }

    pub fn rebuild_basis_plan_count(&self) -> u64 {
        self.rebuild_basis_plan_count
    }

    pub fn runtime_handoff_count(&self) -> u64 {
        self.runtime_handoff_count
    }

    pub fn operational_verdict_translation_count(&self) -> u64 {
        self.operational_verdict_translation_count
    }

    pub fn operational_verdict_translation_rejections(&self) -> u64 {
        self.operational_verdict_translation_rejections
    }

    pub fn support_action_envelope_publications(&self) -> u64 {
        self.support_action_envelope_publications
    }

    pub fn support_hot_path_rejections(&self) -> u64 {
        self.support_hot_path_rejections
    }

    pub fn support_batch_receipt_reuse_count(&self) -> u64 {
        self.support_batch_receipt_reuse_count
    }

    pub fn support_store_global_debt_rejections(&self) -> u64 {
        self.support_store_global_debt_rejections
    }

    pub fn support_retention_plan_count(&self) -> u64 {
        self.support_retention_plan_count
    }

    pub fn support_retention_affected_entries(&self) -> u64 {
        self.support_retention_affected_entries
    }

    pub fn support_retained_family_count(&self) -> u64 {
        self.support_retained_family_count
    }

    pub fn support_reclaimed_family_count(&self) -> u64 {
        self.support_reclaimed_family_count
    }

    pub fn support_compacted_basis_count(&self) -> u64 {
        self.support_compacted_basis_count
    }

    pub fn support_expired_family_count(&self) -> u64 {
        self.support_expired_family_count
    }

    pub fn support_reclaim_consequence_count(&self) -> u64 {
        self.support_reclaim_consequence_count
    }

    pub fn support_policy_expiration_count(&self) -> u64 {
        self.support_policy_expiration_count
    }

    pub fn support_compatibility_plan_count(&self) -> u64 {
        self.support_compatibility_plan_count
    }

    pub fn support_compatibility_affected_entries(&self) -> u64 {
        self.support_compatibility_affected_entries
    }

    pub fn support_manifest_admission_count(&self) -> u64 {
        self.support_manifest_admission_count
    }

    pub fn support_compatibility_receipt_binding_count(&self) -> u64 {
        self.support_compatibility_receipt_binding_count
    }

    pub fn support_exact_compatible_migration_count(&self) -> u64 {
        self.support_exact_compatible_migration_count
    }

    pub fn support_degraded_compatibility_count(&self) -> u64 {
        self.support_degraded_compatibility_count
    }

    pub fn support_version_skew_rejection_count(&self) -> u64 {
        self.support_version_skew_rejection_count
    }

    pub fn support_portability_plan_count(&self) -> u64 {
        self.support_portability_plan_count
    }

    pub fn support_portability_manifest_entries(&self) -> u64 {
        self.support_portability_manifest_entries
    }

    pub fn support_portability_required_basis_count(&self) -> u64 {
        self.support_portability_required_basis_count
    }

    pub fn support_portability_omitted_support_count(&self) -> u64 {
        self.support_portability_omitted_support_count
    }

    pub fn support_replication_inclusion_count(&self) -> u64 {
        self.support_replication_inclusion_count
    }

    pub fn support_replication_omission_count(&self) -> u64 {
        self.support_replication_omission_count
    }

    pub fn support_import_admission_count(&self) -> u64 {
        self.support_import_admission_count
    }

    pub fn support_import_rejection_count(&self) -> u64 {
        self.support_import_rejection_count
    }

    pub fn support_capsule_manifest_budget_denial_count(&self) -> u64 {
        self.support_capsule_manifest_budget_denial_count
    }

    pub fn support_maintenance_descriptor_count(&self) -> u64 {
        self.support_maintenance_descriptor_count
    }

    pub fn support_maintenance_rebuild_debt_count(&self) -> u64 {
        self.support_maintenance_rebuild_debt_count
    }

    pub fn support_maintenance_refresh_count(&self) -> u64 {
        self.support_maintenance_refresh_count
    }

    pub fn support_maintenance_compatibility_migration_count(&self) -> u64 {
        self.support_maintenance_compatibility_migration_count
    }

    pub fn support_maintenance_degradation_recovery_count(&self) -> u64 {
        self.support_maintenance_degradation_recovery_count
    }

    pub fn support_maintenance_coalesced_duplicate_count(&self) -> u64 {
        self.support_maintenance_coalesced_duplicate_count
    }

    pub fn support_maintenance_interrupted_restart_recovery_count(&self) -> u64 {
        self.support_maintenance_interrupted_restart_recovery_count
    }

    pub(crate) fn record_access_structure_debt(&mut self) {
        self.access_structure_debts += 1;
    }

    pub(crate) fn record_admitted(&mut self) {
        self.declarations_admitted += 1;
    }

    pub(crate) fn record_rejected(&mut self) {
        self.declarations_rejected += 1;
    }

    pub(crate) fn record_published(&mut self) {
        self.artifacts_published += 1;
    }

    pub(crate) fn record_fetch(&mut self, lookup_keys: u64, rows_read: u64) {
        self.artifacts_fetched += 1;
        self.lookup_keys_used += lookup_keys;
        self.rows_read += rows_read;
    }

    pub(crate) fn record_family_catalog_lookup(&mut self) {
        self.family_catalog_lookups += 1;
    }

    pub(crate) fn record_duplicate_retry(&mut self) {
        self.duplicate_retries += 1;
    }

    pub(crate) fn record_identity_collision(&mut self) {
        self.identity_collisions += 1;
    }

    pub(crate) fn record_malformed_support_record(&mut self) {
        self.malformed_support_records += 1;
    }

    pub(crate) fn record_classification(
        &mut self,
        classification: SubscriptionResumeClassification,
    ) {
        match classification {
            SubscriptionResumeClassification::Exact => self.exact_classifications += 1,
            SubscriptionResumeClassification::Degraded => self.degraded_classifications += 1,
            SubscriptionResumeClassification::RebuildRequired => {
                self.rebuild_required_classifications += 1;
            }
            SubscriptionResumeClassification::NotResumable => self.denied_classifications += 1,
        }
    }

    pub(crate) fn record_budget_denial(&mut self) {
        self.budget_denials += 1;
    }

    pub(crate) fn record_restart_reconstruction(&mut self, shards_touched: u64) {
        self.restart_reconstruction_count += 1;
        self.restart_shards_touched += shards_touched;
    }

    pub(crate) fn record_rebuild_basis_plan(&mut self) {
        self.rebuild_basis_plan_count += 1;
    }

    pub(crate) fn record_runtime_handoff(&mut self) {
        self.runtime_handoff_count += 1;
    }

    pub(crate) fn record_operational_verdict_translation(&mut self) {
        self.operational_verdict_translation_count += 1;
    }

    pub(crate) fn record_operational_verdict_translation_rejection(&mut self) {
        self.operational_verdict_translation_rejections += 1;
    }

    pub(crate) fn record_support_action_envelope_publication(&mut self) {
        self.support_action_envelope_publications += 1;
    }

    pub(crate) fn record_support_hot_path_rejection(&mut self) {
        self.support_hot_path_rejections += 1;
    }

    pub(crate) fn record_support_batch_receipt_reuse(&mut self) {
        self.support_batch_receipt_reuse_count += 1;
    }

    pub(crate) fn record_support_store_global_debt_rejection(&mut self) {
        self.support_store_global_debt_rejections += 1;
    }

    pub(crate) fn record_support_retention_plan(&mut self, affected_entries: u64) {
        self.support_retention_plan_count += 1;
        self.support_retention_affected_entries += affected_entries;
    }

    pub(crate) fn record_support_retained_family(&mut self) {
        self.support_retained_family_count += 1;
    }

    pub(crate) fn record_support_reclaimed_family(&mut self) {
        self.support_reclaimed_family_count += 1;
    }

    pub(crate) fn record_support_compacted_basis(&mut self) {
        self.support_compacted_basis_count += 1;
    }

    pub(crate) fn record_support_expired_family(&mut self) {
        self.support_expired_family_count += 1;
    }

    pub(crate) fn record_support_reclaim_consequence(&mut self) {
        self.support_reclaim_consequence_count += 1;
    }

    pub(crate) fn record_support_policy_expiration(&mut self) {
        self.support_policy_expiration_count += 1;
    }

    pub(crate) fn record_support_compatibility_plan(&mut self, affected_entries: u64) {
        self.support_compatibility_plan_count += 1;
        self.support_compatibility_affected_entries += affected_entries;
        self.support_manifest_admission_count += 1;
        self.support_compatibility_receipt_binding_count += 1;
    }

    pub(crate) fn record_support_exact_compatible_migration(&mut self) {
        self.support_exact_compatible_migration_count += 1;
    }

    pub(crate) fn record_support_degraded_compatibility(&mut self) {
        self.support_degraded_compatibility_count += 1;
    }

    pub(crate) fn record_support_version_skew_rejection(&mut self) {
        self.support_version_skew_rejection_count += 1;
    }

    pub(crate) fn record_support_portability_plan(
        &mut self,
        manifest_entries: u64,
        required_basis_count: u64,
        omitted_support_count: u64,
    ) {
        self.support_portability_plan_count += 1;
        self.support_portability_manifest_entries += manifest_entries;
        self.support_portability_required_basis_count += required_basis_count;
        self.support_portability_omitted_support_count += omitted_support_count;
    }

    pub(crate) fn record_support_replication_inclusion(&mut self, included_support_count: u64) {
        self.support_replication_inclusion_count += included_support_count;
    }

    pub(crate) fn record_support_replication_omission(&mut self, omitted_support_count: u64) {
        self.support_replication_omission_count += omitted_support_count;
    }

    pub(crate) fn record_support_import_admission(&mut self) {
        self.support_import_admission_count += 1;
    }

    pub(crate) fn record_support_import_rejection(&mut self) {
        self.support_import_rejection_count += 1;
    }

    pub(crate) fn record_support_capsule_manifest_budget_denial(&mut self) {
        self.support_capsule_manifest_budget_denial_count += 1;
    }

    pub(crate) fn record_support_maintenance_plan(
        &mut self,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
    ) {
        self.support_maintenance_descriptor_count += descriptor_count;
        self.support_maintenance_coalesced_duplicate_count += coalesced_duplicate_count;
    }

    pub(crate) fn record_support_maintenance_rebuild_descriptor(&mut self) {
        self.support_maintenance_rebuild_debt_count += 1;
    }

    pub(crate) fn record_support_maintenance_refresh_descriptor(&mut self) {
        self.support_maintenance_refresh_count += 1;
    }

    pub(crate) fn record_support_maintenance_compatibility_migration_descriptor(&mut self) {
        self.support_maintenance_compatibility_migration_count += 1;
    }

    pub(crate) fn record_support_maintenance_degradation_recovery_descriptor(&mut self) {
        self.support_maintenance_degradation_recovery_count += 1;
    }

    pub(crate) fn record_support_maintenance_interrupted_restart_recovery(&mut self) {
        self.support_maintenance_interrupted_restart_recovery_count += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationBundle {
    catalog_family_count: usize,
    counter_snapshot: SubscriptionSupportCounterSnapshot,
    classification_digest: String,
    matrix: Option<SubscriptionSupportCertificationMatrix>,
    truth_digest: String,
    artifact_digest: String,
    subscription_support_digest: String,
    replay_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
}

impl SubscriptionSupportCertificationBundle {
    pub fn new(
        catalog: &SubscriptionSupportCatalog,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
        reports: &[SubscriptionSupportClassificationReport],
    ) -> Result<Self, StoreError> {
        let classification_digest = stable_digest(&reports)?;
        let counter_digest = stable_digest(&counter_snapshot)?;
        Ok(Self {
            catalog_family_count: catalog.family_count(),
            counter_snapshot,
            classification_digest: classification_digest.clone(),
            matrix: None,
            truth_digest: classification_digest.clone(),
            artifact_digest: classification_digest.clone(),
            subscription_support_digest: classification_digest.clone(),
            replay_digest: classification_digest.clone(),
            diagnostics_digest: classification_digest,
            counter_digest,
        })
    }

    pub fn from_lane_outcomes(
        catalog: &SubscriptionSupportCatalog,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
        reports: &[SubscriptionSupportClassificationReport],
        lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    ) -> Result<Self, StoreError> {
        let matrix = SubscriptionSupportCertificationMatrix::from_lane_outcomes(lane_outcomes)?;
        Ok(Self {
            catalog_family_count: catalog.family_count(),
            counter_snapshot,
            classification_digest: stable_digest(&reports)?,
            truth_digest: stable_digest(&matrix.truth_digests())?,
            artifact_digest: stable_digest(&matrix.artifact_digests())?,
            subscription_support_digest: stable_digest(&matrix.subscription_support_digests())?,
            replay_digest: stable_digest(&matrix.replay_digests())?,
            diagnostics_digest: stable_digest(&matrix.diagnostics_digests())?,
            counter_digest: stable_digest(&matrix.counter_digests())?,
            matrix: Some(matrix),
        })
    }

    pub fn catalog_family_count(&self) -> usize {
        self.catalog_family_count
    }

    pub fn classification_digest(&self) -> &str {
        &self.classification_digest
    }

    pub fn matrix(&self) -> Option<&SubscriptionSupportCertificationMatrix> {
        self.matrix.as_ref()
    }

    pub fn counter_snapshot(&self) -> &SubscriptionSupportCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubscriptionSupportCertificationLaneKind {
    ExactResumeControl,
    RestartExactResume,
    RebuildRequiredMissingSupport,
    DegradedButRecoverable,
    NotResumableBasisDrift,
    NotResumableCursorDrift,
    SupportDigestDrift,
    CompatibilityDrift,
    CursorOnlyExactResumeRejected,
    CrossFamilyReuseRejected,
    SessionMemoryLossNonAuthoritative,
    TierRecallCostOnly,
    RuntimeHandoffEquivalence,
    UnknownUpstreamAuthorityRejected,
    NonCanonicalScopeRejected,
    UnsupportedFamilyKindRejected,
    MultiDriftBasisPrecedence,
    MultiDriftCompatibilityPrecedence,
    RebuildBasisMissingNotResumable,
    BackendAccessStructureDebt,
    DecodedRowPublicationRejected,
    OversizedPayloadRejectedBeforeDecode,
    RestartShardBoundedReconstruction,
    ResultCostSurfaceExact,
    BatchClassificationDebt,
    SupportCompatibilityExactMigration,
    SupportCompatibilityDegraded,
    SupportCompatibilityOldReaderRejected,
    SupportCompatibilityUnknownFamilyRejected,
    SupportCompatibilityVersionSkewRejected,
    SupportRetentionExactPreserved,
    SupportRetentionCompactedExact,
    SupportRetentionReclaimedRebuildable,
    SupportRetentionExpiredByPolicy,
    SupportPortabilityFullScopeReplicated,
    SupportPortabilityPartialOmission,
    SupportPortabilityImportAdmitted,
    SupportPortabilityImportMissingBasisNotResumable,
    SupportMaintenanceRebuildAdmitted,
    SupportMaintenanceRefreshAdmitted,
    SupportMaintenanceCompatibilityMigrationAdmitted,
    SupportMaintenanceDegradationRecoveryAdmitted,
    SupportMaintenanceInterruptedRestartRecovered,
    SupportMaintenanceCoalescedRebuildAdmitted,
    SupportFamilyLocalBatchBounded,
    SupportBasisLocalBatchBounded,
    SupportPortabilityScopeBatchBounded,
    SupportMaintenanceKeyBatchBounded,
    SupportStoreGlobalDensityRejected,
    SupportForegroundOperationalWorkRejected,
    SupportBatchReceiptReuseVerified,
}

impl SubscriptionSupportCertificationLaneKind {
    pub fn phase_5b_required() -> &'static [Self] {
        &PHASE_5B_REQUIRED_CERTIFICATION_LANES
    }

    pub fn phase_6a_required() -> &'static [Self] {
        &PHASE_6A_REQUIRED_CERTIFICATION_LANES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportCertificationMatrixStatus {
    Phase5BComplete,
    Phase6AOperationalParticipationComplete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationLaneOutcome {
    lane: SubscriptionSupportCertificationLaneKind,
    classification: Option<SubscriptionResumeClassification>,
    primary_cause: Option<SubscriptionSupportDriftCause>,
    suppressed_causes: Vec<SubscriptionSupportDriftCause>,
    truth_digest: String,
    artifact_digest: String,
    subscription_support_digest: String,
    replay_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    cost_surface: Option<SubscriptionSupportResultCostSurface>,
    batch_receipt_reuse_report: Option<SupportBatchReceiptReuseReport>,
    counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn from_classification_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportClassificationReport,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: Some(report.classification()),
            primary_cause: report.primary_cause(),
            suppressed_causes: report.suppressed_causes().to_vec(),
            truth_digest: stable_digest(&(
                lane,
                report.classification(),
                report.primary_cause(),
                report.suppressed_causes(),
            ))?,
            artifact_digest: stable_digest(&(report.artifact_id(), report.declaration_digest()))?,
            subscription_support_digest: stable_digest(&(
                report.artifact_id(),
                report.classification(),
                report.cost_surface(),
            ))?,
            replay_digest: stable_digest(&(
                lane,
                report.cost_surface(),
                report.counter_snapshot(),
            ))?,
            diagnostics_digest: stable_digest(&(
                report.primary_cause(),
                report.suppressed_causes(),
            ))?,
            counter_digest: stable_digest(report.counter_snapshot())?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot: report.counter_snapshot().clone(),
        })
    }

    pub fn from_missing_support_recovery(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportMissingSupportRecoveryReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: Some(report.classification()),
            primary_cause: Some(report.primary_cause()),
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, report.classification(), report.primary_cause()))?,
            artifact_digest: stable_digest(report)?,
            subscription_support_digest: stable_digest(&(report, report.maintenance_report()))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(
                report.primary_cause(),
                report.maintenance_report(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_access_structure_debt(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportAccessStructureReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, report.has_debt()))?,
            artifact_digest: stable_digest(&report.required().to_vec())?,
            subscription_support_digest: stable_digest(&report.debted().to_vec())?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report)?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_typed_rejection(
        lane: SubscriptionSupportCertificationLaneKind,
        error_kind: StoreErrorKind,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(lane, &error_kind))?,
            artifact_digest: stable_digest(&(lane, "typed-rejection"))?,
            subscription_support_digest: stable_digest(&(lane, &error_kind, &counter_snapshot))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(lane, &error_kind))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_batch_receipt_reuse_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SupportBatchReceiptReuseReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: None,
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.density_class(),
                report.affected_entries(),
            ))?,
            artifact_digest: stable_digest(&(lane, report.reused_proofs()))?,
            subscription_support_digest: stable_digest(&(lane, report, &counter_snapshot))?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report)?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: None,
            batch_receipt_reuse_report: Some(report.clone()),
            counter_snapshot,
        })
    }

    pub fn from_compatibility_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportCompatibilityReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        let (classification, primary_cause) = match report.outcome() {
            SubscriptionSupportCompatibilityOutcome::ExactMigrated(_) => {
                (Some(SubscriptionResumeClassification::Exact), None)
            }
            SubscriptionSupportCompatibilityOutcome::Degraded(_) => (
                Some(SubscriptionResumeClassification::Degraded),
                Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift),
            ),
            SubscriptionSupportCompatibilityOutcome::Rejected(_) => (None, None),
        };
        Ok(Self {
            lane,
            classification,
            primary_cause,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().milestone12_relation(),
                report.participation_record().milestone12_rejection_kind(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(
                lane,
                report.participation_record().milestone12_receipt_digest(),
                &counter_snapshot,
            ))?,
            diagnostics_digest: stable_digest(&(
                report.outcome().outcome_kind(),
                report.participation_record().milestone12_rejection_kind(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_retention_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportPostActionReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(report.retention_record().verdict()),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.retention_record().decision_kind(),
                report.retention_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.retention_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(report.materialization())?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_portability_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportPortabilityReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(
                report.participation_record().verdict(),
            ),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(
                lane,
                report.manifest().manifest_digest(),
                &counter_snapshot,
            ))?,
            diagnostics_digest: stable_digest(report.outcome())?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn from_maintenance_report(
        lane: SubscriptionSupportCertificationLaneKind,
        report: &SubscriptionSupportMaintenanceReport,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            lane,
            classification: operational_verdict_classification(
                report.participation_record().verdict(),
            ),
            primary_cause: None,
            suppressed_causes: Vec::new(),
            truth_digest: stable_digest(&(
                lane,
                report.participation_record().decision_kind(),
                report.participation_record().verdict(),
            ))?,
            artifact_digest: stable_digest(&(
                report.completed_action().envelope().action_id(),
                report.participation_record().affected_set_digest(),
            ))?,
            subscription_support_digest: stable_digest(report)?,
            replay_digest: stable_digest(&(lane, &counter_snapshot))?,
            diagnostics_digest: stable_digest(&(
                report.participation_record().descriptor_count(),
                report.participation_record().coalesced_duplicate_count(),
            ))?,
            counter_digest: stable_digest(&counter_snapshot)?,
            cost_surface: Some(report.cost_surface()),
            batch_receipt_reuse_report: None,
            counter_snapshot,
        })
    }

    pub fn lane(&self) -> SubscriptionSupportCertificationLaneKind {
        self.lane
    }

    pub fn classification(&self) -> Option<SubscriptionResumeClassification> {
        self.classification
    }

    pub fn primary_cause(&self) -> Option<SubscriptionSupportDriftCause> {
        self.primary_cause
    }

    pub fn suppressed_causes(&self) -> &[SubscriptionSupportDriftCause] {
        &self.suppressed_causes
    }

    pub fn counter_snapshot(&self) -> &SubscriptionSupportCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn cost_surface(&self) -> Option<SubscriptionSupportResultCostSurface> {
        self.cost_surface
    }

    pub fn batch_receipt_reuse_report(&self) -> Option<&SupportBatchReceiptReuseReport> {
        self.batch_receipt_reuse_report.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationMatrix {
    lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    status: SubscriptionSupportCertificationMatrixStatus,
}

impl SubscriptionSupportCertificationMatrix {
    pub fn from_lane_outcomes(
        mut lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    ) -> Result<Self, StoreError> {
        lane_outcomes.sort_by_key(SubscriptionSupportCertificationLaneOutcome::lane);
        let mut seen = BTreeSet::new();
        for outcome in &lane_outcomes {
            if !seen.insert(outcome.lane()) {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support certification matrix received a duplicate lane",
                ));
            }
            validate_lane_semantics(outcome)?;
        }
        for required in SubscriptionSupportCertificationLaneKind::phase_5b_required() {
            if !seen.contains(required) {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support certification matrix is missing a required Phase 5B lane",
                ));
            }
        }
        let phase_6a_complete = SubscriptionSupportCertificationLaneKind::phase_6a_required()
            .iter()
            .all(|required| seen.contains(required));
        Ok(Self {
            lane_outcomes,
            status: if phase_6a_complete {
                SubscriptionSupportCertificationMatrixStatus::Phase6AOperationalParticipationComplete
            } else {
                SubscriptionSupportCertificationMatrixStatus::Phase5BComplete
            },
        })
    }

    pub fn lane_outcomes(&self) -> &[SubscriptionSupportCertificationLaneOutcome] {
        &self.lane_outcomes
    }

    pub fn status(&self) -> SubscriptionSupportCertificationMatrixStatus {
        self.status
    }

    fn truth_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.truth_digest.as_str())
            .collect()
    }

    fn artifact_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.artifact_digest.as_str())
            .collect()
    }

    fn subscription_support_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.subscription_support_digest.as_str())
            .collect()
    }

    fn replay_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.replay_digest.as_str())
            .collect()
    }

    fn diagnostics_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.diagnostics_digest.as_str())
            .collect()
    }

    fn counter_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.counter_digest.as_str())
            .collect()
    }
}

fn validate_lane_semantics(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    use SubscriptionResumeClassification::{Degraded, Exact, NotResumable, RebuildRequired};
    use SubscriptionSupportCertificationLaneKind::{
        BackendAccessStructureDebt, BatchClassificationDebt, CompatibilityDrift,
        CrossFamilyReuseRejected, CursorOnlyExactResumeRejected, DecodedRowPublicationRejected,
        DegradedButRecoverable, ExactResumeControl, MultiDriftBasisPrecedence,
        MultiDriftCompatibilityPrecedence, NonCanonicalScopeRejected, NotResumableBasisDrift,
        NotResumableCursorDrift, OversizedPayloadRejectedBeforeDecode,
        RebuildBasisMissingNotResumable, RebuildRequiredMissingSupport, RestartExactResume,
        RestartShardBoundedReconstruction, ResultCostSurfaceExact, RuntimeHandoffEquivalence,
        SessionMemoryLossNonAuthoritative, SupportBasisLocalBatchBounded,
        SupportBatchReceiptReuseVerified, SupportCompatibilityDegraded,
        SupportCompatibilityExactMigration, SupportCompatibilityOldReaderRejected,
        SupportCompatibilityUnknownFamilyRejected, SupportCompatibilityVersionSkewRejected,
        SupportDigestDrift, SupportFamilyLocalBatchBounded,
        SupportForegroundOperationalWorkRejected, SupportMaintenanceCoalescedRebuildAdmitted,
        SupportMaintenanceCompatibilityMigrationAdmitted,
        SupportMaintenanceDegradationRecoveryAdmitted,
        SupportMaintenanceInterruptedRestartRecovered, SupportMaintenanceKeyBatchBounded,
        SupportMaintenanceRebuildAdmitted, SupportMaintenanceRefreshAdmitted,
        SupportPortabilityFullScopeReplicated, SupportPortabilityImportAdmitted,
        SupportPortabilityImportMissingBasisNotResumable, SupportPortabilityPartialOmission,
        SupportPortabilityScopeBatchBounded, SupportRetentionCompactedExact,
        SupportRetentionExactPreserved, SupportRetentionExpiredByPolicy,
        SupportRetentionReclaimedRebuildable, SupportStoreGlobalDensityRejected,
        TierRecallCostOnly, UnknownUpstreamAuthorityRejected, UnsupportedFamilyKindRejected,
    };
    use SubscriptionSupportDriftCause::{
        SubscriptionSupportBasisDrift, SubscriptionSupportCompatibilityDrift,
        SubscriptionSupportCursorDrift, SubscriptionSupportDigestMismatch,
        SubscriptionSupportFamilyMismatch, SubscriptionSupportPlacementUnavailable,
        SubscriptionSupportSessionMemoryMissing,
    };

    match outcome.lane {
        ExactResumeControl | RestartExactResume | RuntimeHandoffEquivalence => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
        }
        ResultCostSurfaceExact => {
            require_classification(outcome, Exact)?;
            let cost_surface = require_cost_surface(outcome)?;
            if cost_surface.plan_family()
                != SubscriptionSupportPlanFamily::ExactResumeClassificationPlan
                || cost_surface.density_class()
                    != SubscriptionSupportDensityClass::SparseIdentityClassification
                || cost_surface.scanned_support_rows() == 0
            {
                return invalid_lane(
                    outcome,
                    "exact cost surface must bind exact sparse direct-lookup work",
                );
            }
        }
        RestartShardBoundedReconstruction => {
            require_classification(outcome, Exact)?;
            if require_cost_surface(outcome)?.restart_shards_touched() != 1 {
                return invalid_lane(
                    outcome,
                    "restart reconstruction must touch exactly one shard",
                );
            }
        }
        DegradedButRecoverable => {
            require_classification(outcome, Degraded)?;
            require_no_primary_cause(outcome)?;
        }
        RebuildRequiredMissingSupport => {
            require_classification(outcome, RebuildRequired)?;
            require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
            if outcome
                .counter_snapshot
                .support_maintenance_rebuild_debt_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "rebuild-required missing support lane must publish a maintenance rebuild descriptor",
                );
            }
        }
        RebuildBasisMissingNotResumable => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
            if outcome
                .counter_snapshot
                .support_maintenance_rebuild_debt_count()
                != 0
            {
                return invalid_lane(
                    outcome,
                    "missing rebuild basis lane must not claim maintenance rebuild admission",
                );
            }
        }
        NotResumableBasisDrift => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportBasisDrift)?;
        }
        NotResumableCursorDrift => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportCursorDrift)?;
        }
        SupportDigestDrift => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
        }
        CompatibilityDrift => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
        }
        CrossFamilyReuseRejected => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportFamilyMismatch)?;
        }
        SessionMemoryLossNonAuthoritative => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportSessionMemoryMissing)?;
        }
        TierRecallCostOnly => {
            require_classification(outcome, Exact)?;
            require_primary_cause(outcome, SubscriptionSupportPlacementUnavailable)?;
        }
        MultiDriftBasisPrecedence => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportBasisDrift)?;
            require_suppressed_causes(
                outcome,
                &[
                    SubscriptionSupportCursorDrift,
                    SubscriptionSupportDigestMismatch,
                ],
            )?;
        }
        MultiDriftCompatibilityPrecedence => {
            require_classification(outcome, NotResumable)?;
            require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
            require_suppressed_causes(outcome, &[SubscriptionSupportDigestMismatch])?;
        }
        BatchClassificationDebt => {
            require_classification(outcome, NotResumable)?;
            if require_cost_surface(outcome)?.density_class()
                != SubscriptionSupportDensityClass::FamilyBatchClassificationDebt
            {
                return invalid_lane(
                    outcome,
                    "batch debt lane must carry family-batch debt density",
                );
            }
        }
        OversizedPayloadRejectedBeforeDecode => {
            require_rejection(outcome)?;
            if outcome.counter_snapshot.budget_denials() == 0 {
                return invalid_lane(
                    outcome,
                    "oversized payload lane must bind a budget denial counter",
                );
            }
        }
        BackendAccessStructureDebt => {
            require_rejection(outcome)?;
        }
        CursorOnlyExactResumeRejected
        | UnknownUpstreamAuthorityRejected
        | NonCanonicalScopeRejected
        | UnsupportedFamilyKindRejected
        | DecodedRowPublicationRejected => {
            require_rejection(outcome)?;
        }
        SupportCompatibilityExactMigration => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_exact_compatible_migration_count()
                == 0
                || outcome
                    .counter_snapshot
                    .support_compatibility_receipt_binding_count()
                    == 0
            {
                return invalid_lane(
                    outcome,
                    "exact support compatibility lane must bind exact migration and receipt counters",
                );
            }
        }
        SupportCompatibilityDegraded => {
            require_classification(outcome, Degraded)?;
            require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
            if outcome
                .counter_snapshot
                .support_degraded_compatibility_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "degraded support compatibility lane must bind degraded compatibility counter",
                );
            }
        }
        SupportCompatibilityOldReaderRejected
        | SupportCompatibilityUnknownFamilyRejected
        | SupportCompatibilityVersionSkewRejected => {
            require_rejection(outcome)?;
            if outcome
                .counter_snapshot
                .support_version_skew_rejection_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "support compatibility rejection lane must bind version-skew rejection counter",
                );
            }
        }
        SupportRetentionExactPreserved => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_retained_family_count() == 0 {
                return invalid_lane(
                    outcome,
                    "retained support lane must bind retained-family counter",
                );
            }
        }
        SupportRetentionCompactedExact => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_compacted_basis_count() == 0 {
                return invalid_lane(
                    outcome,
                    "compacted support lane must bind compacted-basis counter",
                );
            }
        }
        SupportRetentionReclaimedRebuildable => {
            require_classification(outcome, RebuildRequired)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_reclaimed_family_count() == 0
                || outcome.counter_snapshot.support_reclaim_consequence_count() == 0
            {
                return invalid_lane(
                    outcome,
                    "reclaimed rebuildable lane must bind reclaim and consequence counters",
                );
            }
        }
        SupportRetentionExpiredByPolicy => {
            require_rejection(outcome)?;
            if outcome.counter_snapshot.support_expired_family_count() == 0
                || outcome.counter_snapshot.support_policy_expiration_count() == 0
            {
                return invalid_lane(
                    outcome,
                    "expired support lane must bind expiration and policy counters",
                );
            }
        }
        SupportPortabilityFullScopeReplicated => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_portability_plan_count() == 0
                || outcome
                    .counter_snapshot
                    .support_replication_inclusion_count()
                    == 0
            {
                return invalid_lane(
                    outcome,
                    "full-scope portability lane must bind plan and inclusion counters",
                );
            }
        }
        SupportPortabilityPartialOmission => {
            require_classification(outcome, Degraded)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_replication_omission_count()
                == 0
                || outcome
                    .counter_snapshot
                    .support_portability_omitted_support_count()
                    == 0
            {
                return invalid_lane(
                    outcome,
                    "partial omission portability lane must bind omission counters",
                );
            }
        }
        SupportPortabilityImportAdmitted => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_import_admission_count() == 0 {
                return invalid_lane(
                    outcome,
                    "import-admitted portability lane must bind import admission counter",
                );
            }
        }
        SupportPortabilityImportMissingBasisNotResumable => {
            require_classification(outcome, NotResumable)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_import_admission_count() == 0
                || outcome
                    .counter_snapshot
                    .support_portability_required_basis_count()
                    == 0
            {
                return invalid_lane(
                    outcome,
                    "missing-basis import lane must bind import admission and required-basis counters",
                );
            }
        }
        SupportMaintenanceRebuildAdmitted => {
            require_classification(outcome, RebuildRequired)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_maintenance_rebuild_debt_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance rebuild lane must bind rebuild descriptor counter",
                );
            }
        }
        SupportMaintenanceRefreshAdmitted => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome.counter_snapshot.support_maintenance_refresh_count() == 0 {
                return invalid_lane(
                    outcome,
                    "maintenance refresh lane must bind refresh counter",
                );
            }
        }
        SupportMaintenanceCompatibilityMigrationAdmitted => {
            require_classification(outcome, Exact)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_maintenance_compatibility_migration_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance compatibility-migration lane must bind migration counter",
                );
            }
        }
        SupportMaintenanceDegradationRecoveryAdmitted => {
            require_classification(outcome, Degraded)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_maintenance_degradation_recovery_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance degradation-recovery lane must bind degradation counter",
                );
            }
        }
        SupportMaintenanceInterruptedRestartRecovered => {
            match outcome.classification {
                Some(Exact | Degraded | RebuildRequired) => {}
                _ => {
                    return invalid_lane(
                        outcome,
                        "maintenance interrupted-restart lane must preserve the recovered work posture",
                    );
                }
            }
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_maintenance_interrupted_restart_recovery_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance interrupted-restart lane must bind restart-recovery counter",
                );
            }
        }
        SupportMaintenanceCoalescedRebuildAdmitted => {
            require_classification(outcome, RebuildRequired)?;
            require_no_primary_cause(outcome)?;
            if outcome
                .counter_snapshot
                .support_maintenance_rebuild_debt_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance coalesced rebuild lane must still bind rebuild admission",
                );
            }
            if outcome
                .counter_snapshot
                .support_maintenance_coalesced_duplicate_count()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "maintenance coalesced lane must bind duplicate-coalescing counter",
                );
            }
        }
        SupportFamilyLocalBatchBounded => {
            require_classification(outcome, Exact)?;
            let cost_surface = require_cost_surface(outcome)?;
            if cost_surface.plan_family()
                != SubscriptionSupportPlanFamily::RetentionParticipationPlan
                || cost_surface.density_class() != SubscriptionSupportDensityClass::FamilyLocalBatch
                || cost_surface.allocation_scope()
                    != crate::SubscriptionSupportAllocationScope::FamilyLocalBatch
                || cost_surface.scanned_support_rows() == 0
                || outcome.counter_snapshot.support_retention_plan_count() == 0
                || outcome
                    .counter_snapshot
                    .support_retention_affected_entries()
                    != cost_surface.scanned_support_rows()
            {
                return invalid_lane(
                    outcome,
                    "family-local bounded lane must bind retention family-local breadth exactly",
                );
            }
        }
        SupportBasisLocalBatchBounded => {
            require_classification(outcome, Exact)?;
            let cost_surface = require_cost_surface(outcome)?;
            if cost_surface.plan_family()
                != SubscriptionSupportPlanFamily::CompatibilityParticipationPlan
                || cost_surface.density_class() != SubscriptionSupportDensityClass::BasisLocalBatch
                || cost_surface.allocation_scope()
                    != crate::SubscriptionSupportAllocationScope::ActionLocal
                || cost_surface.scanned_support_rows() == 0
                || outcome.counter_snapshot.support_compatibility_plan_count() == 0
                || outcome.counter_snapshot.support_manifest_admission_count() == 0
                || outcome
                    .counter_snapshot
                    .support_compatibility_affected_entries()
                    != cost_surface.scanned_support_rows()
            {
                return invalid_lane(
                    outcome,
                    "basis-local bounded lane must bind compatibility basis-local breadth exactly",
                );
            }
        }
        SupportPortabilityScopeBatchBounded => {
            require_classification(outcome, Exact)?;
            let cost_surface = require_cost_surface(outcome)?;
            if cost_surface.plan_family()
                != SubscriptionSupportPlanFamily::PortabilityParticipationPlan
                || cost_surface.density_class()
                    != SubscriptionSupportDensityClass::PortabilityScopeBatch
                || cost_surface.allocation_scope()
                    != crate::SubscriptionSupportAllocationScope::PortabilityManifest
                || cost_surface.scanned_support_rows() == 0
                || outcome.counter_snapshot.support_portability_plan_count() == 0
                || outcome
                    .counter_snapshot
                    .support_portability_manifest_entries()
                    != cost_surface.scanned_support_rows()
            {
                return invalid_lane(
                    outcome,
                    "portability bounded lane must bind portability manifest breadth exactly",
                );
            }
        }
        SupportMaintenanceKeyBatchBounded => {
            require_classification(outcome, RebuildRequired)?;
            let cost_surface = require_cost_surface(outcome)?;
            if cost_surface.plan_family()
                != SubscriptionSupportPlanFamily::MaintenanceParticipationPlan
                || cost_surface.density_class()
                    != SubscriptionSupportDensityClass::MaintenanceKeyBatch
                || cost_surface.allocation_scope()
                    != crate::SubscriptionSupportAllocationScope::FamilyLocalBatch
                || cost_surface.scanned_support_rows() == 0
                || outcome
                    .counter_snapshot
                    .support_maintenance_descriptor_count()
                    != cost_surface.scanned_support_rows()
            {
                return invalid_lane(
                    outcome,
                    "maintenance bounded lane must bind maintenance-key breadth exactly",
                );
            }
        }
        SupportStoreGlobalDensityRejected => {
            require_rejection(outcome)?;
            if outcome
                .counter_snapshot
                .support_store_global_debt_rejections()
                == 0
            {
                return invalid_lane(
                    outcome,
                    "store-global rejection lane must bind the store-global debt counter",
                );
            }
        }
        SupportForegroundOperationalWorkRejected => {
            require_rejection(outcome)?;
            if outcome.counter_snapshot.support_hot_path_rejections() == 0 {
                return invalid_lane(
                    outcome,
                    "foreground rejection lane must bind the hot-path rejection counter",
                );
            }
        }
        SupportBatchReceiptReuseVerified => {
            require_no_primary_cause(outcome)?;
            if outcome.classification.is_some() {
                return invalid_lane(
                    outcome,
                    "batch receipt reuse lane must not masquerade as a resume classification",
                );
            }
            let Some(report) = outcome.batch_receipt_reuse_report() else {
                return invalid_lane(
                    outcome,
                    "batch receipt reuse lane must carry explicit reuse evidence",
                );
            };
            let required_proofs = [
                SupportBatchProofKind::CompatibilityReceipt,
                SupportBatchProofKind::BasisEvidence,
                SupportBatchProofKind::CursorCheckpointEvidence,
                SupportBatchProofKind::PortabilityScopeEvidence,
            ];
            if report.density_class() == crate::SupportProgramDensityClass::StoreGlobalDebt
                || report.affected_entries() == 0
                || report.reused_proofs() != required_proofs
                || outcome.counter_snapshot.support_batch_receipt_reuse_count()
                    != required_proofs.len() as u64
            {
                return invalid_lane(
                    outcome,
                    "batch receipt reuse lane must prove the full named reuse set exactly once each",
                );
            }
        }
    }
    Ok(())
}

fn operational_verdict_classification(
    verdict: SubscriptionSupportOperationalVerdict,
) -> Option<SubscriptionResumeClassification> {
    match verdict {
        SubscriptionSupportOperationalVerdict::ExactResumePreserved => {
            Some(SubscriptionResumeClassification::Exact)
        }
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved => {
            Some(SubscriptionResumeClassification::Degraded)
        }
        SubscriptionSupportOperationalVerdict::RebuildRequired => {
            Some(SubscriptionResumeClassification::RebuildRequired)
        }
        SubscriptionSupportOperationalVerdict::NotResumable => {
            Some(SubscriptionResumeClassification::NotResumable)
        }
        SubscriptionSupportOperationalVerdict::RejectedByPolicy => None,
    }
}

fn require_classification(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: SubscriptionResumeClassification,
) -> Result<(), StoreError> {
    if outcome.classification != Some(expected) {
        return invalid_lane(outcome, "certification lane has the wrong classification");
    }
    Ok(())
}

fn require_rejection(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    if outcome.classification.is_some() || outcome.primary_cause.is_some() {
        return invalid_lane(
            outcome,
            "typed rejection lane must not carry resume classification evidence",
        );
    }
    Ok(())
}

fn require_no_primary_cause(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    if outcome.primary_cause.is_some() || !outcome.suppressed_causes.is_empty() {
        return invalid_lane(
            outcome,
            "clean certification lane must not carry drift causes",
        );
    }
    Ok(())
}

fn require_primary_cause(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: SubscriptionSupportDriftCause,
) -> Result<(), StoreError> {
    if outcome.primary_cause != Some(expected) {
        return invalid_lane(
            outcome,
            "certification lane has the wrong primary drift cause",
        );
    }
    Ok(())
}

fn require_suppressed_causes(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: &[SubscriptionSupportDriftCause],
) -> Result<(), StoreError> {
    if outcome.suppressed_causes != expected {
        return invalid_lane(
            outcome,
            "certification lane has the wrong suppressed drift causes",
        );
    }
    Ok(())
}

fn require_cost_surface(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<SubscriptionSupportResultCostSurface, StoreError> {
    outcome.cost_surface.ok_or_else(|| {
        lane_error(
            outcome,
            "certification lane must carry a result cost surface",
        )
    })
}

fn invalid_lane<T>(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    message: &'static str,
) -> Result<T, StoreError> {
    Err(lane_error(outcome, message))
}

fn lane_error(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    message: &'static str,
) -> StoreError {
    StoreError::new(
        StoreErrorKind::SubscriptionSupportClassificationViolation,
        format!("{message}: {:?}", outcome.lane()),
    )
}

const PHASE_5B_REQUIRED_CERTIFICATION_LANES: [SubscriptionSupportCertificationLaneKind; 25] = [
    SubscriptionSupportCertificationLaneKind::ExactResumeControl,
    SubscriptionSupportCertificationLaneKind::RestartExactResume,
    SubscriptionSupportCertificationLaneKind::RebuildRequiredMissingSupport,
    SubscriptionSupportCertificationLaneKind::DegradedButRecoverable,
    SubscriptionSupportCertificationLaneKind::NotResumableBasisDrift,
    SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
    SubscriptionSupportCertificationLaneKind::SupportDigestDrift,
    SubscriptionSupportCertificationLaneKind::CompatibilityDrift,
    SubscriptionSupportCertificationLaneKind::CursorOnlyExactResumeRejected,
    SubscriptionSupportCertificationLaneKind::CrossFamilyReuseRejected,
    SubscriptionSupportCertificationLaneKind::SessionMemoryLossNonAuthoritative,
    SubscriptionSupportCertificationLaneKind::TierRecallCostOnly,
    SubscriptionSupportCertificationLaneKind::RuntimeHandoffEquivalence,
    SubscriptionSupportCertificationLaneKind::UnknownUpstreamAuthorityRejected,
    SubscriptionSupportCertificationLaneKind::NonCanonicalScopeRejected,
    SubscriptionSupportCertificationLaneKind::UnsupportedFamilyKindRejected,
    SubscriptionSupportCertificationLaneKind::MultiDriftBasisPrecedence,
    SubscriptionSupportCertificationLaneKind::MultiDriftCompatibilityPrecedence,
    SubscriptionSupportCertificationLaneKind::RebuildBasisMissingNotResumable,
    SubscriptionSupportCertificationLaneKind::BackendAccessStructureDebt,
    SubscriptionSupportCertificationLaneKind::DecodedRowPublicationRejected,
    SubscriptionSupportCertificationLaneKind::OversizedPayloadRejectedBeforeDecode,
    SubscriptionSupportCertificationLaneKind::RestartShardBoundedReconstruction,
    SubscriptionSupportCertificationLaneKind::ResultCostSurfaceExact,
    SubscriptionSupportCertificationLaneKind::BatchClassificationDebt,
];

const PHASE_6A_REQUIRED_CERTIFICATION_LANES: [SubscriptionSupportCertificationLaneKind; 44] = [
    SubscriptionSupportCertificationLaneKind::ExactResumeControl,
    SubscriptionSupportCertificationLaneKind::RestartExactResume,
    SubscriptionSupportCertificationLaneKind::RebuildRequiredMissingSupport,
    SubscriptionSupportCertificationLaneKind::DegradedButRecoverable,
    SubscriptionSupportCertificationLaneKind::NotResumableBasisDrift,
    SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
    SubscriptionSupportCertificationLaneKind::SupportDigestDrift,
    SubscriptionSupportCertificationLaneKind::CompatibilityDrift,
    SubscriptionSupportCertificationLaneKind::CursorOnlyExactResumeRejected,
    SubscriptionSupportCertificationLaneKind::CrossFamilyReuseRejected,
    SubscriptionSupportCertificationLaneKind::SessionMemoryLossNonAuthoritative,
    SubscriptionSupportCertificationLaneKind::TierRecallCostOnly,
    SubscriptionSupportCertificationLaneKind::RuntimeHandoffEquivalence,
    SubscriptionSupportCertificationLaneKind::UnknownUpstreamAuthorityRejected,
    SubscriptionSupportCertificationLaneKind::NonCanonicalScopeRejected,
    SubscriptionSupportCertificationLaneKind::UnsupportedFamilyKindRejected,
    SubscriptionSupportCertificationLaneKind::MultiDriftBasisPrecedence,
    SubscriptionSupportCertificationLaneKind::MultiDriftCompatibilityPrecedence,
    SubscriptionSupportCertificationLaneKind::RebuildBasisMissingNotResumable,
    SubscriptionSupportCertificationLaneKind::BackendAccessStructureDebt,
    SubscriptionSupportCertificationLaneKind::DecodedRowPublicationRejected,
    SubscriptionSupportCertificationLaneKind::OversizedPayloadRejectedBeforeDecode,
    SubscriptionSupportCertificationLaneKind::RestartShardBoundedReconstruction,
    SubscriptionSupportCertificationLaneKind::ResultCostSurfaceExact,
    SubscriptionSupportCertificationLaneKind::BatchClassificationDebt,
    SubscriptionSupportCertificationLaneKind::SupportCompatibilityExactMigration,
    SubscriptionSupportCertificationLaneKind::SupportCompatibilityDegraded,
    SubscriptionSupportCertificationLaneKind::SupportCompatibilityOldReaderRejected,
    SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected,
    SubscriptionSupportCertificationLaneKind::SupportCompatibilityVersionSkewRejected,
    SubscriptionSupportCertificationLaneKind::SupportRetentionExactPreserved,
    SubscriptionSupportCertificationLaneKind::SupportRetentionCompactedExact,
    SubscriptionSupportCertificationLaneKind::SupportRetentionReclaimedRebuildable,
    SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy,
    SubscriptionSupportCertificationLaneKind::SupportPortabilityFullScopeReplicated,
    SubscriptionSupportCertificationLaneKind::SupportPortabilityPartialOmission,
    SubscriptionSupportCertificationLaneKind::SupportPortabilityImportAdmitted,
    SubscriptionSupportCertificationLaneKind::SupportPortabilityImportMissingBasisNotResumable,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceRebuildAdmitted,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceRefreshAdmitted,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceCompatibilityMigrationAdmitted,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceDegradationRecoveryAdmitted,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceInterruptedRestartRecovered,
    SubscriptionSupportCertificationLaneKind::SupportMaintenanceCoalescedRebuildAdmitted,
];
