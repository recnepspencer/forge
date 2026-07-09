use super::{
    classification_error, SubscriptionResumeClassification,
    SubscriptionSupportActionPublicationRecoveryReport, SubscriptionSupportArtifactId,
    SubscriptionSupportCompatibilityOutcome, SubscriptionSupportCompatibilityReport,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportMaintenanceDebtReport, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportPortabilityOutcome, SubscriptionSupportPortabilityReport,
    SubscriptionSupportPostActionReport, SubscriptionSupportRetentionMaterialization,
    SubscriptionSupportRole,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportActionOrigin {
    Retention,
    Compatibility,
    ReplicationExport,
    ReplicationImport,
    Maintenance,
    RestartRecovery,
    TierRecall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportOperationalVerdict {
    ExactResumePreserved,
    DegradedResumePreserved,
    RebuildRequired,
    NotResumable,
    RejectedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionSupportOperationalBasis {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    action_origin: SubscriptionSupportActionOrigin,
}

impl SubscriptionSupportOperationalBasis {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
        portability_digest: impl Into<String>,
        action_origin: SubscriptionSupportActionOrigin,
    ) -> Result<Self, StoreError> {
        let basis_digest = require_non_empty("basis", basis_digest)?;
        let cursor_digest = require_non_empty("cursor", cursor_digest)?;
        let checkpoint_digest = require_non_empty("checkpoint", checkpoint_digest)?;
        let compatibility_digest = require_non_empty("compatibility", compatibility_digest)?;
        let portability_digest = require_non_empty("portability", portability_digest)?;
        Ok(Self {
            family_id,
            family_kind,
            support_role,
            artifact_id,
            basis_digest,
            cursor_digest,
            checkpoint_digest,
            compatibility_digest,
            portability_digest,
            action_origin,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn cursor_digest(&self) -> &str {
        &self.cursor_digest
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn portability_digest(&self) -> &str {
        &self.portability_digest
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactResumePreservationWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl ExactResumePreservationWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Result<Self, StoreError> {
        if basis.action_origin == SubscriptionSupportActionOrigin::TierRecall {
            return Err(classification_error(
                "tier recall may change support access cost but cannot prove exact operational preservation",
            ));
        }
        Ok(Self { basis })
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedResumePreservationWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl DegradedResumePreservationWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Self {
        Self { basis }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRebuildAdmissionWitness {
    basis: SubscriptionSupportOperationalBasis,
    maintenance_admission_key: String,
}

impl SupportRebuildAdmissionWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let maintenance_admission_key =
            require_non_empty("maintenance-admission", maintenance_admission_key)?;
        Ok(Self {
            basis,
            maintenance_admission_key,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportNonResumableWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl SupportNonResumableWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Self {
        Self { basis }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPolicyRejectionWitness {
    basis: SubscriptionSupportOperationalBasis,
    policy_reason: String,
}

impl SupportPolicyRejectionWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        policy_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let policy_reason = require_non_empty("policy rejection", policy_reason)?;
        Ok(Self {
            basis,
            policy_reason,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ResumeClassificationTranslationPlan {
    Exact(ExactResumePreservationWitness),
    Degraded(DegradedResumePreservationWitness),
    Rebuild(SupportRebuildAdmissionWitness),
    NotResumable(SupportNonResumableWitness),
    PolicyRejected(SupportPolicyRejectionWitness),
}

impl ResumeClassificationTranslationPlan {
    pub(crate) fn from_operational_verdict(
        verdict: SubscriptionSupportOperationalVerdict,
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: Option<String>,
        policy_reason: Option<String>,
    ) -> Result<Self, StoreError> {
        match verdict {
            SubscriptionSupportOperationalVerdict::ExactResumePreserved => {
                ExactResumePreservationWitness::new(basis).map(Self::Exact)
            }
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved => Ok(Self::Degraded(
                DegradedResumePreservationWitness::new(basis),
            )),
            SubscriptionSupportOperationalVerdict::RebuildRequired => {
                let key = maintenance_admission_key.ok_or_else(|| {
                    classification_error(
                        "rebuild-required support verdicts require maintenance admission proof",
                    )
                })?;
                SupportRebuildAdmissionWitness::new(basis, key).map(Self::Rebuild)
            }
            SubscriptionSupportOperationalVerdict::NotResumable => {
                Ok(Self::NotResumable(SupportNonResumableWitness::new(basis)))
            }
            SubscriptionSupportOperationalVerdict::RejectedByPolicy => {
                let reason = policy_reason.ok_or_else(|| {
                    classification_error("policy-rejected support verdicts require a policy reason")
                })?;
                SupportPolicyRejectionWitness::new(basis, reason).map(Self::PolicyRejected)
            }
        }
    }

    pub(crate) fn lower(self) -> PostActionResumeClassificationInput {
        match self {
            Self::Exact(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::Exact,
                operational_verdict: SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                artifact_id: witness.basis.artifact_id,
            },
            Self::Degraded(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::Degraded,
                operational_verdict: SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                artifact_id: witness.basis.artifact_id,
            },
            Self::Rebuild(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::RebuildRequired,
                operational_verdict: SubscriptionSupportOperationalVerdict::RebuildRequired,
                artifact_id: witness.basis.artifact_id,
            },
            Self::NotResumable(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::NotResumable,
                artifact_id: witness.basis.artifact_id,
            },
            Self::PolicyRejected(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                artifact_id: witness.basis.artifact_id,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PostActionResumeClassificationInput {
    classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    artifact_id: SubscriptionSupportArtifactId,
}

impl PostActionResumeClassificationInput {
    pub fn classification(&self) -> SubscriptionResumeClassification {
        self.classification
    }

    pub fn operational_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.operational_verdict
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportOperationalVerdictTranslationRequest {
    verdict: SubscriptionSupportOperationalVerdict,
    basis: SubscriptionSupportOperationalBasis,
    maintenance_admission_key: Option<String>,
    policy_reason: Option<String>,
    exact_only: bool,
}

impl SubscriptionSupportOperationalVerdictTranslationRequest {
    pub fn from_retention_report(
        report: &SubscriptionSupportPostActionReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        validate_report_translation_basis(&basis, report.translation_basis(), "retention")?;
        let policy_reason = match report.materialization() {
            SubscriptionSupportRetentionMaterialization::Expired(expired) => {
                Some(expired.policy_reason().to_string())
            }
            _ => None,
        };
        Ok(Self {
            verdict: report.retention_record().verdict(),
            basis,
            maintenance_admission_key: report
                .materialization()
                .maintenance_admission_key()
                .map(ToOwned::to_owned),
            policy_reason,
            exact_only: false,
        })
    }

    pub fn exact_from_retention_report(
        report: &SubscriptionSupportPostActionReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let mut request = Self::from_retention_report(report, basis)?;
        request.exact_only = true;
        Ok(request)
    }

    pub fn from_compatibility_report(
        report: &SubscriptionSupportCompatibilityReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        validate_report_translation_basis(&basis, report.translation_basis(), "compatibility")?;
        let policy_reason = match report.outcome() {
            SubscriptionSupportCompatibilityOutcome::Rejected(rejection) => {
                Some(rejection.rejection_reason().to_string())
            }
            _ => None,
        };
        Ok(Self {
            verdict: report.completed_action().envelope().verdict(),
            basis,
            maintenance_admission_key: None,
            policy_reason,
            exact_only: false,
        })
    }

    pub fn exact_from_compatibility_report(
        report: &SubscriptionSupportCompatibilityReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let mut request = Self::from_compatibility_report(report, basis)?;
        request.exact_only = true;
        Ok(request)
    }

    pub fn from_portability_report(
        report: &SubscriptionSupportPortabilityReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        validate_report_translation_basis(&basis, report.translation_basis(), "portability")?;
        let policy_reason = match report.outcome() {
            SubscriptionSupportPortabilityOutcome::Rejected(rejection) => {
                Some(rejection.rejection_reason().to_string())
            }
            SubscriptionSupportPortabilityOutcome::ImportedNotResumable(not_resumable) => {
                Some(not_resumable.denial_reason().to_string())
            }
            _ => None,
        };
        Ok(Self {
            verdict: report.completed_action().envelope().verdict(),
            basis,
            maintenance_admission_key: None,
            policy_reason,
            exact_only: false,
        })
    }

    pub fn exact_from_portability_report(
        report: &SubscriptionSupportPortabilityReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let mut request = Self::from_portability_report(report, basis)?;
        request.exact_only = true;
        Ok(request)
    }

    pub fn from_maintenance_report(
        report: &SubscriptionSupportMaintenanceReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let descriptor = report
            .descriptor_records()
            .iter()
            .find(|record| record.artifact_id() == basis.artifact_id())
            .ok_or_else(|| {
                classification_error(
                    "subscription-support maintenance translation requires a descriptor record for the requested artifact",
                )
            })?;
        validate_descriptor_translation_basis(&basis, descriptor)?;
        Ok(Self {
            verdict: report.participation_record().verdict(),
            basis,
            maintenance_admission_key: Some(descriptor.maintenance_key().to_string()),
            policy_reason: None,
            exact_only: false,
        })
    }

    pub fn exact_from_maintenance_report(
        report: &SubscriptionSupportMaintenanceReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let mut request = Self::from_maintenance_report(report, basis)?;
        request.exact_only = true;
        Ok(request)
    }

    pub fn exact_from_maintenance_debt_report(
        report: &SubscriptionSupportMaintenanceDebtReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        let reported_basis = report
            .translation_bases()
            .iter()
            .find(|candidate| candidate.artifact_id() == basis.artifact_id())
            .ok_or_else(|| {
                classification_error(
                    "subscription-support maintenance debt translation requires a reported basis for the requested artifact",
                )
            })?;
        validate_report_translation_basis(&basis, reported_basis, "maintenance debt")?;
        Ok(Self {
            verdict: report.debt_summary().verdict(),
            basis,
            maintenance_admission_key: None,
            policy_reason: Some(report.debt_summary().delay_reason().to_string()),
            exact_only: true,
        })
    }

    pub fn from_action_publication_recovery(
        report: &SubscriptionSupportActionPublicationRecoveryReport,
        basis: SubscriptionSupportOperationalBasis,
    ) -> Result<Self, StoreError> {
        validate_report_translation_basis(
            &basis,
            report.translation_basis(),
            "action-publication recovery",
        )?;
        if report.completed_action().is_none() {
            return Err(classification_error(
                "subscription-support interrupted action recovery cannot translate an unpublished consequence",
            ));
        }
        Ok(Self {
            verdict: report.verdict(),
            basis,
            maintenance_admission_key: None,
            policy_reason: None,
            exact_only: false,
        })
    }

    pub(crate) fn into_plan(self) -> Result<ResumeClassificationTranslationPlan, StoreError> {
        if self.exact_only {
            ensure_exact_translation_allowed(
                self.verdict,
                "subscription-support operational report",
            )?;
        }
        ResumeClassificationTranslationPlan::from_operational_verdict(
            self.verdict,
            self.basis,
            self.maintenance_admission_key,
            self.policy_reason,
        )
    }
}

fn ensure_exact_translation_allowed(
    verdict: SubscriptionSupportOperationalVerdict,
    label: &'static str,
) -> Result<(), StoreError> {
    if verdict != SubscriptionSupportOperationalVerdict::ExactResumePreserved {
        return Err(classification_error(format!(
            "{label} cannot claim exact resume translation"
        )));
    }
    Ok(())
}

fn validate_basis_for_translation(
    basis: &SubscriptionSupportOperationalBasis,
    artifact_id: &SubscriptionSupportArtifactId,
    action_origin: SubscriptionSupportActionOrigin,
) -> Result<(), StoreError> {
    if basis.artifact_id() != artifact_id {
        return Err(classification_error(
            "subscription-support translation basis must match the report artifact id",
        ));
    }
    if basis.action_origin() != action_origin {
        return Err(classification_error(
            "subscription-support translation basis must match the report action origin",
        ));
    }
    Ok(())
}

fn validate_basis_digests_for_descriptor(
    basis: &SubscriptionSupportOperationalBasis,
    descriptor: &crate::SupportMaintenanceDescriptorRecord,
) -> Result<(), StoreError> {
    if basis.basis_digest() != descriptor.basis_digest()
        || basis.cursor_digest() != descriptor.cursor_digest()
        || basis.checkpoint_digest() != descriptor.checkpoint_digest()
        || basis.compatibility_digest() != descriptor.compatibility_digest()
        || basis.portability_digest() != descriptor.portability_digest()
    {
        return Err(classification_error(
            "subscription-support maintenance translation basis drifted from the admitted descriptor record",
        ));
    }
    Ok(())
}

fn validate_report_translation_basis(
    basis: &SubscriptionSupportOperationalBasis,
    reported_basis: &SubscriptionSupportOperationalBasis,
    label: &'static str,
) -> Result<(), StoreError> {
    if basis != reported_basis {
        return Err(classification_error(format!(
            "subscription-support {label} translation basis must match the report-proven operational basis"
        )));
    }
    Ok(())
}

fn validate_descriptor_translation_basis(
    basis: &SubscriptionSupportOperationalBasis,
    descriptor: &crate::SupportMaintenanceDescriptorRecord,
) -> Result<(), StoreError> {
    validate_basis_for_translation(
        basis,
        descriptor.artifact_id(),
        SubscriptionSupportActionOrigin::Maintenance,
    )?;
    if basis.family_id() != descriptor.family_id()
        || basis.family_kind() != descriptor.family_kind()
        || basis.support_role() != descriptor.support_role()
    {
        return Err(classification_error(
            "subscription-support maintenance translation basis must match the descriptor family and role",
        ));
    }
    validate_basis_digests_for_descriptor(basis, descriptor)
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support operational {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
