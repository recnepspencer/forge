use crate::application::ForgeQueryMilestoneClosureStatus;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::certification::ForgeQueryConsumerKitCertificationCaseRow;
use super::docs_report::ForgeQueryConsumerKitDocsFamilyRow;
use super::family::{ForgeQueryConsumerKitFamilyClosureRow, ForgeQueryConsumerKitFamilyName};
use super::residue::ForgeQueryConsumerKitResidueBreakdown;

pub(super) fn consumer_kit_family_closure_identity(
    family_name: ForgeQueryConsumerKitFamilyName,
    status: ForgeQueryMilestoneClosureStatus,
    evidence_label: &str,
    evidence_digest: &str,
    evidence_source_paths: &[&'static str],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationConsumerKitFamilyClosure)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family_name.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("evidence_label"), evidence_label)
        .field_value(
            ForgeQueryEvidenceTag::new("evidence_digest"),
            evidence_digest,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("evidence_source_path"),
            evidence_source_paths.iter().copied(),
        )
        .seal()
}

pub(super) fn consumer_kit_certification_identity(
    suite_name: &str,
    rows: &[ForgeQueryConsumerKitFamilyClosureRow],
    case_rows: &[ForgeQueryConsumerKitCertificationCaseRow],
    docs_agreement_digest: &str,
    residue_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::ApplicationConsumerKitHostileCertification,
    )
    .field_shape(ForgeQueryEvidenceTag::new("suite"), suite_name)
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("family_closure_digest"),
        rows.iter()
            .map(ForgeQueryConsumerKitFamilyClosureRow::closure_digest),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("certification_case_digest"),
        case_rows
            .iter()
            .map(ForgeQueryConsumerKitCertificationCaseRow::case_digest),
    )
    .field_value(
        ForgeQueryEvidenceTag::new("docs_agreement_digest"),
        docs_agreement_digest,
    )
    .field_value(ForgeQueryEvidenceTag::new("residue_digest"), residue_digest)
    .seal()
}

pub(super) fn consumer_kit_docs_agreement_identity(
    support_families: &[ForgeQueryConsumerKitFamilyName],
    documented_families: &[ForgeQueryConsumerKitFamilyName],
    family_rows: &[ForgeQueryConsumerKitDocsFamilyRow],
    ordinary_path_language_present: bool,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "consumer-kit-docs-agreement",
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("support_family"),
            support_families
                .iter()
                .map(|family| ForgeQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("documented_family"),
            documented_families
                .iter()
                .map(|family| ForgeQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("docs_family_row_digest"),
            family_rows
                .iter()
                .map(ForgeQueryConsumerKitDocsFamilyRow::row_digest),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("ordinary_path_language_present"),
            ordinary_path_language_present,
        )
        .seal()
}

pub(super) fn consumer_kit_reference_residue_identity(
    query_owned_residue_count: usize,
    defended_residue_count: usize,
    breakdown: &ForgeQueryConsumerKitResidueBreakdown,
    backend_applicability: &str,
    backend_applicability_certified: bool,
    residue_source_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationConsumerKitReferenceResidue)
        .field_usize(
            ForgeQueryEvidenceTag::new("query_owned_residue_count"),
            query_owned_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("defended_residue_count"),
            defended_residue_count,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("residue_breakdown_digest"),
            breakdown.breakdown_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("backend_applicability"),
            backend_applicability,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("backend_applicability_certified"),
            backend_applicability_certified,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("residue_source_digest"),
            residue_source_digest,
        )
        .seal()
}

pub(super) fn consumer_kit_embedded_source_identity(
    role: &str,
    source_paths: impl IntoIterator<Item = &'static str>,
    source_bodies: impl IntoIterator<Item = &'static str>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationConsumerKitClosure)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value_sequence(ForgeQueryEvidenceTag::new("source_path"), source_paths)
        .field_value_sequence(ForgeQueryEvidenceTag::new("source_body"), source_bodies)
        .seal()
}

pub(super) fn consumer_kit_certification_case_identity(
    family: ForgeQueryConsumerKitFamilyName,
    case_id: &str,
    tier: &str,
    requirement: &str,
    required_signal: &str,
    satisfied: bool,
    evidence_source_paths: &[&'static str],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::ApplicationConsumerKitHostileCertification,
    )
    .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("case_id"), case_id)
    .field_shape(ForgeQueryEvidenceTag::new("tier"), tier)
    .field_shape(ForgeQueryEvidenceTag::new("requirement"), requirement)
    .field_shape(
        ForgeQueryEvidenceTag::new("required_signal"),
        required_signal,
    )
    .field_bool(ForgeQueryEvidenceTag::new("satisfied"), satisfied)
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("evidence_source_path"),
        evidence_source_paths.iter().copied(),
    )
    .seal()
}

pub(super) fn consumer_kit_docs_family_row_identity(
    family: ForgeQueryConsumerKitFamilyName,
    ai_readme_present: bool,
    test_requirements_present: bool,
    closeout_present: bool,
    ordinary_path_present: bool,
    family_obligation_present: bool,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "consumer-kit-docs-family-row",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_bool(
            ForgeQueryEvidenceTag::new("ai_readme_present"),
            ai_readme_present,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("test_requirements_present"),
            test_requirements_present,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("closeout_present"),
            closeout_present,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("ordinary_path_present"),
            ordinary_path_present,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("family_obligation_present"),
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
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationConsumerKitReferenceResidue)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "consumer-kit-residue-breakdown",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("report_digest_residue_count"),
            report_digest_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("prohibition_audit_residue_count"),
            prohibition_audit_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("support_pinning_residue_count"),
            support_pinning_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("test_backend_residue_count"),
            test_backend_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("defended_worth_domain_residue_count"),
            defended_worth_domain_residue_count,
        )
        .seal()
}

pub(super) fn consumer_kit_closure_identity(
    status: ForgeQueryMilestoneClosureStatus,
    rows: &[ForgeQueryConsumerKitFamilyClosureRow],
    certification_digest: &str,
    docs_agreement_digest: &str,
    residue_digest: &str,
    defended_exclusions: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationConsumerKitClosure)
        .field_shape(ForgeQueryEvidenceTag::new("milestone"), "forge-query-9.8")
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("required_family"),
            required_consumer_kit_families()
                .iter()
                .map(|family| ForgeQueryConsumerKitFamilyName::as_str(*family)),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("family_closure_digest"),
            rows.iter()
                .map(ForgeQueryConsumerKitFamilyClosureRow::closure_digest),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("certification_digest"),
            certification_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("docs_agreement_digest"),
            docs_agreement_digest,
        )
        .field_value(ForgeQueryEvidenceTag::new("residue_digest"), residue_digest)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("defended_exclusion"),
            defended_exclusions.iter().map(String::as_str),
        )
        .seal()
}

pub(super) const fn required_consumer_kit_families() -> &'static [ForgeQueryConsumerKitFamilyName] {
    &[
        ForgeQueryConsumerKitFamilyName::EvidenceReportKit,
        ForgeQueryConsumerKitFamilyName::HardProhibitionRegistry,
        ForgeQueryConsumerKitFamilyName::BoundaryAudit,
        ForgeQueryConsumerKitFamilyName::SupportSnapshot,
        ForgeQueryConsumerKitFamilyName::SupportPinning,
        ForgeQueryConsumerKitFamilyName::InMemoryTestBackend,
        ForgeQueryConsumerKitFamilyName::ConsumerResidueAudit,
        ForgeQueryConsumerKitFamilyName::ReferenceConsumerAdoption,
    ]
}
