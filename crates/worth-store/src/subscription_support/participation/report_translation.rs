use super::super::{
    classification_error, SubscriptionSupportActionPublicationRecoveryReport,
    SubscriptionSupportArtifactId, SubscriptionSupportCompatibilityOutcome,
    SubscriptionSupportCompatibilityReport, SubscriptionSupportMaintenanceDebtReport,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportPortabilityReport, SubscriptionSupportPostActionReport,
    SubscriptionSupportRetentionMaterialization,
};
use super::operational_basis::{
    SubscriptionSupportActionOrigin, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use super::resume_translation::ResumeClassificationTranslationPlan;
use crate::failure::StoreError;
use serde::Serialize;

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
