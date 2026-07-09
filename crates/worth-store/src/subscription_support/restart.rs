use super::{
    classification_error, SubscriptionResumeClassification, SubscriptionSupportArtifactId,
    SubscriptionSupportClassificationReport, SubscriptionSupportDriftCause,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportRole, SupportActionBreadthBudget,
    SupportActionId,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartShard {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
}

impl SubscriptionSupportRestartShard {
    pub fn for_family(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
    ) -> Self {
        Self {
            family_id,
            family_kind,
        }
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(crate) fn shard_key(&self) -> String {
        format!("subscription-support-restart:{}", self.family_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartReconstructionRequest {
    shard: SubscriptionSupportRestartShard,
    max_support_rows: u64,
}

impl SubscriptionSupportRestartReconstructionRequest {
    pub fn new(
        shard: SubscriptionSupportRestartShard,
        max_support_rows: u64,
    ) -> Result<Self, StoreError> {
        if max_support_rows == 0 {
            return Err(classification_error(
                "subscription-support restart reconstruction requires a non-zero row bound",
            ));
        }
        Ok(Self {
            shard,
            max_support_rows,
        })
    }

    pub(crate) fn shard(&self) -> &SubscriptionSupportRestartShard {
        &self.shard
    }

    pub(crate) fn max_support_rows(&self) -> u64 {
        self.max_support_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartReconstructionReport {
    shard: SubscriptionSupportRestartShard,
    reports: Vec<SubscriptionSupportClassificationReport>,
    support_rows_read: u64,
    restart_shards_touched: u64,
    global_scan_count: u64,
}

impl SubscriptionSupportRestartReconstructionReport {
    pub(crate) fn new(
        shard: SubscriptionSupportRestartShard,
        reports: Vec<SubscriptionSupportClassificationReport>,
        support_rows_read: u64,
    ) -> Self {
        Self {
            shard,
            reports,
            support_rows_read,
            restart_shards_touched: 1,
            global_scan_count: 0,
        }
    }

    pub fn shard(&self) -> &SubscriptionSupportRestartShard {
        &self.shard
    }

    pub fn reports(&self) -> &[SubscriptionSupportClassificationReport] {
        &self.reports
    }

    pub fn support_rows_read(&self) -> u64 {
        self.support_rows_read
    }

    pub fn restart_shards_touched(&self) -> u64 {
        self.restart_shards_touched
    }

    pub fn global_scan_count(&self) -> u64 {
        self.global_scan_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMissingSupportRecoveryRequest {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    missing_artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    rebuild_maintenance_admission: Option<SubscriptionSupportMissingSupportMaintenanceAdmission>,
}

impl SubscriptionSupportMissingSupportRecoveryRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        missing_artifact_id: SubscriptionSupportArtifactId,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
        portability_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let request = Self {
            family_id,
            family_kind,
            support_role,
            missing_artifact_id,
            basis_digest: basis_digest.into(),
            cursor_digest: cursor_digest.into(),
            checkpoint_digest: checkpoint_digest.into(),
            compatibility_digest: compatibility_digest.into(),
            portability_digest: portability_digest.into(),
            rebuild_maintenance_admission: None,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn with_rebuild_maintenance_admission(
        mut self,
        retained_rebuild_basis_digest: impl Into<String>,
        maintenance_admission: SubscriptionSupportMissingSupportMaintenanceAdmission,
    ) -> Result<Self, StoreError> {
        self.rebuild_maintenance_admission = Some(
            maintenance_admission
                .bind_retained_rebuild_basis_digest(retained_rebuild_basis_digest)?,
        );
        self.validate()?;
        Ok(self)
    }

    pub(crate) fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub(crate) fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(crate) fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub(crate) fn missing_artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.missing_artifact_id
    }

    pub(crate) fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub(crate) fn cursor_digest(&self) -> &str {
        &self.cursor_digest
    }

    pub(crate) fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }

    pub(crate) fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub(crate) fn portability_digest(&self) -> &str {
        &self.portability_digest
    }

    pub(crate) fn rebuild_maintenance_admission(
        &self,
    ) -> Option<&SubscriptionSupportMissingSupportMaintenanceAdmission> {
        self.rebuild_maintenance_admission.as_ref()
    }

    pub(crate) fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.rebuild_maintenance_admission.as_ref().and_then(
            SubscriptionSupportMissingSupportMaintenanceAdmission::retained_rebuild_basis_digest,
        )
    }

    fn validate(&self) -> Result<(), StoreError> {
        if self.basis_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires basis evidence",
            ));
        }
        if self.cursor_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires cursor evidence",
            ));
        }
        if self.checkpoint_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires checkpoint evidence",
            ));
        }
        if self.compatibility_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires compatibility evidence",
            ));
        }
        if self.portability_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires portability evidence",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMissingSupportMaintenanceAdmission {
    retained_rebuild_basis_digest: Option<String>,
    action_id: SupportActionId,
    breadth_budget: SupportActionBreadthBudget,
    payload_header_bytes: u64,
}

impl SubscriptionSupportMissingSupportMaintenanceAdmission {
    pub fn new(
        action_id: SupportActionId,
        breadth_budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            retained_rebuild_basis_digest: None,
            action_id,
            breadth_budget,
            payload_header_bytes,
        })
    }

    fn bind_retained_rebuild_basis_digest(
        mut self,
        retained_rebuild_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let retained_rebuild_basis_digest = retained_rebuild_basis_digest.into();
        if retained_rebuild_basis_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support missing support recovery requires non-empty retained rebuild basis evidence",
            ));
        }
        self.retained_rebuild_basis_digest = Some(retained_rebuild_basis_digest);
        Ok(self)
    }

    pub(crate) fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub(crate) fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub(crate) fn breadth_budget(&self) -> &SupportActionBreadthBudget {
        &self.breadth_budget
    }

    pub(crate) fn payload_header_bytes(&self) -> u64 {
        self.payload_header_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMissingSupportRecoveryReport {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    missing_artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    retained_rebuild_basis_digest: Option<String>,
    classification: SubscriptionResumeClassification,
    primary_cause: SubscriptionSupportDriftCause,
    maintenance_report: Option<SubscriptionSupportMaintenanceReport>,
}

impl SubscriptionSupportMissingSupportRecoveryReport {
    pub(crate) fn new(
        request: &SubscriptionSupportMissingSupportRecoveryRequest,
        classification: SubscriptionResumeClassification,
        maintenance_report: Option<SubscriptionSupportMaintenanceReport>,
    ) -> Self {
        Self {
            family_id: request.family_id.clone(),
            family_kind: request.family_kind,
            missing_artifact_id: request.missing_artifact_id.clone(),
            basis_digest: request.basis_digest.clone(),
            cursor_digest: request.cursor_digest.clone(),
            checkpoint_digest: request.checkpoint_digest.clone(),
            retained_rebuild_basis_digest: request
                .retained_rebuild_basis_digest()
                .map(str::to_string),
            classification,
            primary_cause: SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch,
            maintenance_report,
        }
    }

    pub fn classification(&self) -> SubscriptionResumeClassification {
        self.classification
    }

    pub fn primary_cause(&self) -> SubscriptionSupportDriftCause {
        self.primary_cause
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub fn maintenance_report(&self) -> Option<&SubscriptionSupportMaintenanceReport> {
        self.maintenance_report.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRuntimeHandoffRequest {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    artifact_id: SubscriptionSupportArtifactId,
    source_runtime_owner: String,
    target_runtime_owner: String,
}

impl SubscriptionSupportRuntimeHandoffRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        artifact_id: SubscriptionSupportArtifactId,
        source_runtime_owner: impl Into<String>,
        target_runtime_owner: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let request = Self {
            family_id,
            family_kind,
            artifact_id,
            source_runtime_owner: source_runtime_owner.into(),
            target_runtime_owner: target_runtime_owner.into(),
        };
        request.validate()?;
        Ok(request)
    }

    pub(crate) fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub(crate) fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(crate) fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    fn validate(&self) -> Result<(), StoreError> {
        if self.source_runtime_owner.trim().is_empty() {
            return Err(classification_error(
                "subscription-support runtime handoff requires a source runtime owner",
            ));
        }
        if self.target_runtime_owner.trim().is_empty() {
            return Err(classification_error(
                "subscription-support runtime handoff requires a target runtime owner",
            ));
        }
        if self.source_runtime_owner == self.target_runtime_owner {
            return Err(classification_error(
                "subscription-support runtime handoff requires distinct runtime owners",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRuntimeHandoffReport {
    source_runtime_owner: String,
    target_runtime_owner: String,
    durable_report: SubscriptionSupportClassificationReport,
    delivery_session_memory_persisted: bool,
}

impl SubscriptionSupportRuntimeHandoffReport {
    pub(crate) fn new(
        request: &SubscriptionSupportRuntimeHandoffRequest,
        durable_report: SubscriptionSupportClassificationReport,
    ) -> Self {
        Self {
            source_runtime_owner: request.source_runtime_owner.clone(),
            target_runtime_owner: request.target_runtime_owner.clone(),
            durable_report,
            delivery_session_memory_persisted: false,
        }
    }

    pub fn durable_report(&self) -> &SubscriptionSupportClassificationReport {
        &self.durable_report
    }

    pub fn delivery_session_memory_persisted(&self) -> bool {
        self.delivery_session_memory_persisted
    }
}
