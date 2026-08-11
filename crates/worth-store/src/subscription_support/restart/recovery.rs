use super::super::{
    classification_error, SubscriptionResumeClassification, SubscriptionSupportArtifactId,
    SubscriptionSupportClassificationReport, SubscriptionSupportDriftCause,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportRole,
};
use super::maintenance_admission::SubscriptionSupportMissingSupportMaintenanceAdmission;
use crate::failure::StoreError;
use serde::Serialize;

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
