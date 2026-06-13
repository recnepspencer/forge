use crate::identity::hash_parts;

use super::super::bridge_parity::QuerySubscriptionBridgeParityExplanation;
use super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::diagnostic::QuerySubscriptionAdmittedDiagnosticBundle;
use super::super::family::QuerySubscriptionFamily;
use super::super::support::QuerySubscriptionSupportReport;
use super::coverage::CertifiedFamilyCoverageHandle;
use super::error::{
    QuerySubscriptionRuntimeCertificationCounters, QuerySubscriptionRuntimeCertificationError,
    QuerySubscriptionRuntimeCertificationErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionRuntimeCertificationScope {
    family: QuerySubscriptionFamily,
    support_report: QuerySubscriptionSupportReport,
    bridge_parity: QuerySubscriptionBridgeParityExplanation,
    admitted_diagnostic_bundle: QuerySubscriptionAdmittedDiagnosticBundle,
    lifecycle_certification: SubscriptionLifecycleCertificationBundle,
    coverage_handle: CertifiedFamilyCoverageHandle,
    scope_digest: String,
    counters: QuerySubscriptionRuntimeCertificationCounters,
}

impl QuerySubscriptionRuntimeCertificationScope {
    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn support_report(&self) -> &QuerySubscriptionSupportReport {
        &self.support_report
    }

    pub fn bridge_parity(&self) -> &QuerySubscriptionBridgeParityExplanation {
        &self.bridge_parity
    }

    pub fn admitted_diagnostic_bundle(&self) -> &QuerySubscriptionAdmittedDiagnosticBundle {
        &self.admitted_diagnostic_bundle
    }

    pub fn lifecycle_certification(&self) -> &SubscriptionLifecycleCertificationBundle {
        &self.lifecycle_certification
    }

    pub fn coverage_handle(&self) -> &CertifiedFamilyCoverageHandle {
        &self.coverage_handle
    }

    pub fn scope_digest(&self) -> &str {
        &self.scope_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionRuntimeCertificationCounters {
        &self.counters
    }
}

pub fn build_query_subscription_runtime_certification_scope(
    support_report: QuerySubscriptionSupportReport,
    bridge_parity: QuerySubscriptionBridgeParityExplanation,
    admitted_diagnostic_bundle: QuerySubscriptionAdmittedDiagnosticBundle,
    lifecycle_certification: SubscriptionLifecycleCertificationBundle,
    coverage_handle: CertifiedFamilyCoverageHandle,
) -> Result<QuerySubscriptionRuntimeCertificationScope, QuerySubscriptionRuntimeCertificationError>
{
    let family = support_report.support_subject().family().clone();
    if bridge_parity.query_family_label() != family.as_str()
        || admitted_diagnostic_bundle
            .semantic_labels()
            .query_family_label()
            != family.as_str()
        || coverage_handle.family() != &family
    {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::ScopeFamilyMismatch,
            "runtime certification scope requires support, parity, diagnostic, and coverage artifacts for the same query subscription family",
            &[
                format!("support_family:{}", support_report.support_subject().family().as_str()),
                format!("parity_family:{}", bridge_parity.query_family_label()),
                format!(
                    "diagnostic_family:{}",
                    admitted_diagnostic_bundle.semantic_labels().query_family_label()
                ),
                format!("coverage_family:{}", coverage_handle.family().as_str()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if support_report.support_posture()
        != &super::super::support::QuerySubscriptionSupportPosture::RuntimeBackedCertified
    {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CertificationSupportPostureDenied,
            "runtime certification scope requires runtime-backed certified support reports",
            &[
                format!(
                    "support_posture:{}",
                    support_report.support_posture().as_str()
                ),
                format!("support_report:{}", support_report.report_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if support_report.support_subject().declaration_digest()
        != lifecycle_certification.query_declaration_for_reporting()
        || bridge_parity.comparison().query_declaration_digest()
            != lifecycle_certification.query_declaration_for_reporting()
        || bridge_parity.comparison().bridge_declaration_digest()
            != lifecycle_certification.bridge_declaration_for_reporting()
        || admitted_diagnostic_bundle.support_report_digest() != support_report.report_digest()
        || admitted_diagnostic_bundle.lifecycle_certification_digest()
            != lifecycle_certification.certification_bundle_for_reporting()
    {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::ScopeSourceMismatch,
            "runtime certification scope requires aligned support, parity, diagnostic, and lifecycle digests",
            &[
                format!(
                    "support_declaration:{}",
                    support_report.support_subject().declaration_digest()
                ),
                format!(
                    "lifecycle_declaration:{}",
                    lifecycle_certification.query_declaration_for_reporting()
                ),
                format!(
                    "parity_declaration:{}",
                    bridge_parity.comparison().query_declaration_digest()
                ),
                format!(
                    "parity_bridge:{}",
                    bridge_parity.comparison().bridge_declaration_digest()
                ),
                format!(
                    "lifecycle_bridge:{}",
                    lifecycle_certification.bridge_declaration_for_reporting()
                ),
                format!(
                    "diagnostic_support:{}",
                    admitted_diagnostic_bundle.support_report_digest()
                ),
                format!("support:{}", support_report.report_digest()),
                format!(
                    "diagnostic_lifecycle:{}",
                    admitted_diagnostic_bundle.lifecycle_certification_digest()
                ),
                format!(
                    "lifecycle:{}",
                    lifecycle_certification.certification_bundle_for_reporting()
                ),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    if !coverage_handle.admitted_rows().iter().any(|row| {
        row.support_report_digest() == support_report.report_digest()
            && row.bridge_parity_digest() == bridge_parity.explanation_digest()
            && row.lifecycle_certification_digest()
                == lifecycle_certification.certification_bundle_for_reporting()
            && row.diagnostic_bundle_digest() == admitted_diagnostic_bundle.bundle_digest()
    }) {
        return Err(QuerySubscriptionRuntimeCertificationError::new(
            QuerySubscriptionRuntimeCertificationErrorKind::CoverageScopeMissingAdmittedRow,
            "runtime certification scope requires the indexed family coverage handle to contain the scope's admitted support, parity, lifecycle, and diagnostic row",
            &[
                format!("support:{}", support_report.report_digest()),
                format!("parity:{}", bridge_parity.explanation_digest()),
                format!(
                    "lifecycle:{}",
                    lifecycle_certification.certification_bundle_for_reporting()
                ),
                format!("diagnostic:{}", admitted_diagnostic_bundle.bundle_digest()),
                format!("coverage:{}", coverage_handle.family_coverage_digest()),
            ],
            QuerySubscriptionRuntimeCertificationCounters::default(),
        ));
    }

    let counters = QuerySubscriptionRuntimeCertificationCounters::scope_emitted();
    let scope_digest = hash_parts(&[
        "query_subscription_runtime_certification_scope_v1".to_string(),
        family.as_str().to_string(),
        support_report.report_digest().to_string(),
        bridge_parity.explanation_digest().to_string(),
        admitted_diagnostic_bundle.bundle_digest().to_string(),
        lifecycle_certification
            .certification_bundle_for_reporting()
            .to_string(),
        coverage_handle.family_coverage_digest().to_string(),
        counters.digest(),
    ]);

    Ok(QuerySubscriptionRuntimeCertificationScope {
        family,
        support_report,
        bridge_parity,
        admitted_diagnostic_bundle,
        lifecycle_certification,
        coverage_handle,
        scope_digest,
        counters,
    })
}
