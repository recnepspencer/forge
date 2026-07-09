use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustReceiptStatus {
    Proven,
    Missing,
    Rejected,
}

impl SupportTrustReceiptStatus {
    pub fn is_proven(self) -> bool {
        self == Self::Proven
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportResumeClassificationReceipt {
    artifact_id: SubscriptionSupportArtifactId,
    classification: SubscriptionResumeClassification,
    proof_digest: String,
    status: SupportTrustReceiptStatus,
}

impl SupportResumeClassificationReceipt {
    pub fn new(
        artifact_id: SubscriptionSupportArtifactId,
        classification: SubscriptionResumeClassification,
        proof_digest: impl Into<String>,
        status: SupportTrustReceiptStatus,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            artifact_id,
            classification,
            proof_digest: require_non_empty("resume proof digest", proof_digest)?,
            status,
        })
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn classification(&self) -> SubscriptionResumeClassification {
        self.classification
    }

    pub fn status(&self) -> SupportTrustReceiptStatus {
        self.status
    }

    pub(crate) fn receipt_bytes(&self) -> u64 {
        self.proof_digest.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportOperationalVerdictReceipt {
    basis: SubscriptionSupportOperationalBasis,
    verdict: SubscriptionSupportOperationalVerdict,
    proof_digest: String,
    status: SupportTrustReceiptStatus,
}

impl SupportOperationalVerdictReceipt {
    pub fn new(
        basis: SubscriptionSupportOperationalBasis,
        verdict: SubscriptionSupportOperationalVerdict,
        proof_digest: impl Into<String>,
        status: SupportTrustReceiptStatus,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            basis,
            verdict,
            proof_digest: require_non_empty("operational proof digest", proof_digest)?,
            status,
        })
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn status(&self) -> SupportTrustReceiptStatus {
        self.status
    }

    pub(crate) fn receipt_bytes(&self) -> u64 {
        self.proof_digest.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportFamilyRoleReceipt {
    family_id: SubscriptionSupportFamilyId,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    proof_digest: String,
    status: SupportTrustReceiptStatus,
}

impl SupportFamilyRoleReceipt {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        proof_digest: impl Into<String>,
        status: SupportTrustReceiptStatus,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            family_id,
            support_role,
            artifact_id,
            proof_digest: require_non_empty("family-role proof digest", proof_digest)?,
            status,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn status(&self) -> SupportTrustReceiptStatus {
        self.status
    }

    pub(crate) fn receipt_bytes(&self) -> u64 {
        self.proof_digest.len() as u64
    }
}

macro_rules! digest_receipt {
    ($name:ident, $accessor:ident, $label:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            artifact_id: SubscriptionSupportArtifactId,
            digest: String,
            status: SupportTrustReceiptStatus,
        }

        impl $name {
            pub fn new(
                artifact_id: SubscriptionSupportArtifactId,
                digest: impl Into<String>,
                status: SupportTrustReceiptStatus,
            ) -> Result<Self, SupportTrustFailure> {
                Ok(Self {
                    artifact_id,
                    digest: require_non_empty($label, digest)?,
                    status,
                })
            }

            pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
                &self.artifact_id
            }

            pub fn $accessor(&self) -> &str {
                &self.digest
            }

            pub fn status(&self) -> SupportTrustReceiptStatus {
                self.status
            }

            pub(crate) fn receipt_bytes(&self) -> u64 {
                self.digest.len() as u64
            }
        }
    };
}

digest_receipt!(SupportBasisReceipt, basis_digest, "basis proof digest");
digest_receipt!(
    SupportCursorCheckpointReceipt,
    cursor_checkpoint_digest,
    "cursor/checkpoint proof digest"
);
digest_receipt!(
    SupportCompatibilityReceipt,
    compatibility_digest,
    "compatibility proof digest"
);
digest_receipt!(
    SupportPortabilityReceipt,
    portability_digest,
    "portability proof digest"
);
digest_receipt!(
    SupportRetentionReceipt,
    retention_digest,
    "retention proof digest"
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceReceipt {
    artifact_id: SubscriptionSupportArtifactId,
    maintenance_admission_key: String,
    proof_digest: String,
    status: SupportTrustReceiptStatus,
}

impl SupportMaintenanceReceipt {
    pub fn new(
        artifact_id: SubscriptionSupportArtifactId,
        maintenance_admission_key: impl Into<String>,
        proof_digest: impl Into<String>,
        status: SupportTrustReceiptStatus,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            artifact_id,
            maintenance_admission_key: require_non_empty(
                "maintenance admission key",
                maintenance_admission_key,
            )?,
            proof_digest: require_non_empty("maintenance proof digest", proof_digest)?,
            status,
        })
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn maintenance_admission_key(&self) -> &str {
        &self.maintenance_admission_key
    }

    pub fn status(&self) -> SupportTrustReceiptStatus {
        self.status
    }

    pub(crate) fn receipt_bytes(&self) -> u64 {
        (self.maintenance_admission_key.len() + self.proof_digest.len()) as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportImportAdmissionReceipt {
    artifact_id: SubscriptionSupportArtifactId,
    target_family_id: SubscriptionSupportFamilyId,
    admission_digest: String,
    status: SupportTrustReceiptStatus,
}

impl SupportImportAdmissionReceipt {
    pub fn new(
        artifact_id: SubscriptionSupportArtifactId,
        target_family_id: SubscriptionSupportFamilyId,
        admission_digest: impl Into<String>,
        status: SupportTrustReceiptStatus,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            artifact_id,
            target_family_id,
            admission_digest: require_non_empty("import admission digest", admission_digest)?,
            status,
        })
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn target_family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.target_family_id
    }

    pub fn status(&self) -> SupportTrustReceiptStatus {
        self.status
    }

    pub(crate) fn receipt_bytes(&self) -> u64 {
        self.admission_digest.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustReceiptBundle {
    resume: SupportResumeClassificationReceipt,
    operational: SupportOperationalVerdictReceipt,
    family_role: SupportFamilyRoleReceipt,
    basis: SupportBasisReceipt,
    cursor_checkpoint: SupportCursorCheckpointReceipt,
    compatibility: SupportCompatibilityReceipt,
    portability: SupportPortabilityReceipt,
    retention: Option<SupportRetentionReceipt>,
    maintenance: Option<SupportMaintenanceReceipt>,
    import_admission: Option<SupportImportAdmissionReceipt>,
}

impl SupportTrustReceiptBundle {
    pub fn new(
        resume: SupportResumeClassificationReceipt,
        operational: SupportOperationalVerdictReceipt,
        family_role: SupportFamilyRoleReceipt,
        basis: SupportBasisReceipt,
        cursor_checkpoint: SupportCursorCheckpointReceipt,
        compatibility: SupportCompatibilityReceipt,
        portability: SupportPortabilityReceipt,
    ) -> Self {
        Self {
            resume,
            operational,
            family_role,
            basis,
            cursor_checkpoint,
            compatibility,
            portability,
            retention: None,
            maintenance: None,
            import_admission: None,
        }
    }

    pub fn with_retention(mut self, receipt: SupportRetentionReceipt) -> Self {
        self.retention = Some(receipt);
        self
    }

    pub fn with_maintenance(mut self, receipt: SupportMaintenanceReceipt) -> Self {
        self.maintenance = Some(receipt);
        self
    }

    pub fn with_import_admission(mut self, receipt: SupportImportAdmissionReceipt) -> Self {
        self.import_admission = Some(receipt);
        self
    }

    pub fn receipt_count(&self) -> u64 {
        7 + u64::from(self.retention.is_some())
            + u64::from(self.maintenance.is_some())
            + u64::from(self.import_admission.is_some())
    }

    pub fn receipt_bytes(&self) -> u64 {
        self.resume.receipt_bytes()
            + self.operational.receipt_bytes()
            + self.family_role.receipt_bytes()
            + self.basis.receipt_bytes()
            + self.cursor_checkpoint.receipt_bytes()
            + self.compatibility.receipt_bytes()
            + self.portability.receipt_bytes()
            + self
                .retention
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
            + self
                .maintenance
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
            + self
                .import_admission
                .as_ref()
                .map_or(0, |receipt| receipt.receipt_bytes())
    }

    pub(crate) fn resume(&self) -> &SupportResumeClassificationReceipt {
        &self.resume
    }

    pub(crate) fn operational(&self) -> &SupportOperationalVerdictReceipt {
        &self.operational
    }

    pub(crate) fn family_role(&self) -> &SupportFamilyRoleReceipt {
        &self.family_role
    }

    pub(crate) fn basis(&self) -> &SupportBasisReceipt {
        &self.basis
    }

    pub(crate) fn cursor_checkpoint(&self) -> &SupportCursorCheckpointReceipt {
        &self.cursor_checkpoint
    }

    pub(crate) fn compatibility(&self) -> &SupportCompatibilityReceipt {
        &self.compatibility
    }

    pub(crate) fn portability(&self) -> &SupportPortabilityReceipt {
        &self.portability
    }

    pub(crate) fn retention(&self) -> Option<&SupportRetentionReceipt> {
        self.retention.as_ref()
    }

    pub(crate) fn maintenance(&self) -> Option<&SupportMaintenanceReceipt> {
        self.maintenance.as_ref()
    }

    pub(crate) fn import_admission(&self) -> Option<&SupportImportAdmissionReceipt> {
        self.import_admission.as_ref()
    }
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust {label} must be non-empty"),
        ));
    }
    Ok(value)
}
