use crate::application::WorthQueryMilestoneClosureStatus;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::certification::WorthQueryConsumerKitCertificationCaseRow;
use super::docs_report::WorthQueryConsumerKitDocsFamilyRow;
use super::family::{WorthQueryConsumerKitFamilyClosureRow, WorthQueryConsumerKitFamilyName};
use super::residue::WorthQueryConsumerKitResidueBreakdown;

pub(super) fn consumer_kit_family_closure_identity(
    family_name: WorthQueryConsumerKitFamilyName,
    status: WorthQueryMilestoneClosureStatus,
    evidence_label: &str,
    evidence_digest: &str,
    evidence_source_paths: &[&'static str],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationConsumerKitFamilyClosure)
        .field_shape(WorthQueryEvidenceTag::new("family"), family_name.as_str())
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(WorthQueryEvidenceTag::new("evidence_label"), evidence_label)
        .field_value(
            WorthQueryEvidenceTag::new("evidence_digest"),
            evidence_digest,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("evidence_source_path"),
            evidence_source_paths.iter().copied(),
        )
        .seal()
}

pub(super) fn consumer_kit_certification_identity(
    suite_name: &str,
    rows: &[WorthQueryConsumerKitFamilyClosureRow],
    case_rows: &[WorthQueryConsumerKitCertificationCaseRow],
    docs_agreement_digest: &str,
    residue_digest: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(
        WorthQueryEvidenceScope::ApplicationConsumerKitHostileCertification,
    )
    .field_shape(WorthQueryEvidenceTag::new("suite"), suite_name)
    .field_value_sequence(
        WorthQueryEvidenceTag::new("family_closure_digest"),
        rows.iter()
            .map(WorthQueryConsumerKitFamilyClosureRow::closure_digest),
    )
    .field_value_sequence(
        WorthQueryEvidenceTag::new("certification_case_digest"),
        case_rows
            .iter()
            .map(WorthQueryConsumerKitCertificationCaseRow::case_digest),
    )
    .field_value(
        WorthQueryEvidenceTag::new("docs_agreement_digest"),
        docs_agreement_digest,
    )
    .field_value(WorthQueryEvidenceTag::new("residue_digest"), residue_digest)
    .seal()
}

pub(super) fn consumer_kit_docs_agreement_identity(
    support_families: &[WorthQueryConsumerKitFamilyName],
    documented_families: &[WorthQueryConsumerKitFamilyName],
    family_rows: &[WorthQueryConsumerKitDocsFamilyRow],
    ordinary_path_language_present: bool,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationSupportReport)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "consumer-kit-docs-agreement",
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("support_family"),
            support_families
                .iter()
                .map(|family| WorthQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("documented_family"),
            documented_families
                .iter()
                .map(|family| WorthQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("docs_family_row_digest"),
            family_rows
                .iter()
                .map(WorthQueryConsumerKitDocsFamilyRow::row_digest),
        )
        .field_bool(
            WorthQueryEvidenceTag::new("ordinary_path_language_present"),
            ordinary_path_language_present,
        )
        .seal()
}

pub(super) fn consumer_kit_reference_residue_identity(
    query_owned_residue_count: usize,
    defended_residue_count: usize,
    breakdown: &WorthQueryConsumerKitResidueBreakdown,
    backend_applicability: &str,
    backend_applicability_certified: bool,
    residue_source_digest: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationConsumerKitReferenceResidue)
        .field_usize(
            WorthQueryEvidenceTag::new("query_owned_residue_count"),
            query_owned_residue_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("defended_residue_count"),
            defended_residue_count,
        )
        .field_value(
            WorthQueryEvidenceTag::new("residue_breakdown_digest"),
            breakdown.breakdown_digest(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("backend_applicability"),
            backend_applicability,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("backend_applicability_certified"),
            backend_applicability_certified,
        )
        .field_value(
            WorthQueryEvidenceTag::new("residue_source_digest"),
            residue_source_digest,
        )
        .seal()
}

pub(super) fn consumer_kit_embedded_source_identity(
    role: &str,
    source_paths: impl IntoIterator<Item = &'static str>,
    source_bodies: impl IntoIterator<Item = &'static str>,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationConsumerKitClosure)
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_value_sequence(WorthQueryEvidenceTag::new("source_path"), source_paths)
        .field_value_sequence(WorthQueryEvidenceTag::new("source_body"), source_bodies)
        .seal()
}

pub(super) fn consumer_kit_certification_case_identity(
    family: WorthQueryConsumerKitFamilyName,
    case_id: &str,
    tier: &str,
    requirement: &str,
    required_signal: &str,
    satisfied: bool,
    evidence_source_paths: &[&'static str],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(
        WorthQueryEvidenceScope::ApplicationConsumerKitHostileCertification,
    )
    .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
    .field_shape(WorthQueryEvidenceTag::new("case_id"), case_id)
    .field_shape(WorthQueryEvidenceTag::new("tier"), tier)
    .field_shape(WorthQueryEvidenceTag::new("requirement"), requirement)
    .field_shape(
        WorthQueryEvidenceTag::new("required_signal"),
        required_signal,
    )
    .field_bool(WorthQueryEvidenceTag::new("satisfied"), satisfied)
    .field_value_sequence(
        WorthQueryEvidenceTag::new("evidence_source_path"),
        evidence_source_paths.iter().copied(),
    )
    .seal()
}

pub(super) fn consumer_kit_docs_family_row_identity(
    family: WorthQueryConsumerKitFamilyName,
    ai_readme_present: bool,
    test_requirements_present: bool,
    closeout_present: bool,
    ordinary_path_present: bool,
    family_obligation_present: bool,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationSupportReport)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "consumer-kit-docs-family-row",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_bool(
            WorthQueryEvidenceTag::new("ai_readme_present"),
            ai_readme_present,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("test_requirements_present"),
            test_requirements_present,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("closeout_present"),
            closeout_present,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("ordinary_path_present"),
            ordinary_path_present,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("family_obligation_present"),
            family_obligation_present,
        )
        .seal()
}

pub(super) fn consumer_kit_residue_breakdown_identity(
    report_digest_residue_count: usize,
    prohibition_audit_residue_count: usize,
    support_pinning_residue_count: usize,
    test_backend_residue_count: usize,
    defended_worth_domain_residue_count: usize,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationConsumerKitReferenceResidue)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "consumer-kit-residue-breakdown",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("report_digest_residue_count"),
            report_digest_residue_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("prohibition_audit_residue_count"),
            prohibition_audit_residue_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("support_pinning_residue_count"),
            support_pinning_residue_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("test_backend_residue_count"),
            test_backend_residue_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("defended_worth_domain_residue_count"),
            defended_worth_domain_residue_count,
        )
        .seal()
}

pub(super) fn consumer_kit_closure_identity(
    status: WorthQueryMilestoneClosureStatus,
    rows: &[WorthQueryConsumerKitFamilyClosureRow],
    certification_digest: &str,
    docs_agreement_digest: &str,
    residue_digest: &str,
    defended_exclusions: &[String],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationConsumerKitClosure)
        .field_shape(WorthQueryEvidenceTag::new("milestone"), "worth-query-9.8")
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_value_sequence(
            WorthQueryEvidenceTag::new("required_family"),
            required_consumer_kit_families()
                .iter()
                .map(|family| WorthQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("family_closure_digest"),
            rows.iter()
                .map(WorthQueryConsumerKitFamilyClosureRow::closure_digest),
        )
        .field_value(
            WorthQueryEvidenceTag::new("certification_digest"),
            certification_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("docs_agreement_digest"),
            docs_agreement_digest,
        )
        .field_value(WorthQueryEvidenceTag::new("residue_digest"), residue_digest)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("defended_exclusion"),
            defended_exclusions.iter().map(String::as_str),
        )
        .seal()
}

pub(super) const fn required_consumer_kit_families() -> &'static [WorthQueryConsumerKitFamilyName] {
    &[
        WorthQueryConsumerKitFamilyName::EvidenceReportKit,
        WorthQueryConsumerKitFamilyName::HardProhibitionRegistry,
        WorthQueryConsumerKitFamilyName::BoundaryAudit,
        WorthQueryConsumerKitFamilyName::SupportSnapshot,
        WorthQueryConsumerKitFamilyName::SupportPinning,
        WorthQueryConsumerKitFamilyName::InMemoryTestBackend,
        WorthQueryConsumerKitFamilyName::ConsumerResidueAudit,
    ]
}
