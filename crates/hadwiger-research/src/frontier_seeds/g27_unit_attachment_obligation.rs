use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_mutation_eligibility::{
    screen_g27_quadratic_survivor_mutation_eligibility_checked, G27MutationEligibilityReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27UnitAttachmentObligationReport {
    core: HadwigerArtifactCore,
    eligibility_report: G27MutationEligibilityReport,
    blocked_candidate_count: usize,
    required_certificate_language: String,
    required_targets: Vec<String>,
}

impl G27UnitAttachmentObligationReport {
    pub fn eligibility_report(&self) -> &G27MutationEligibilityReport {
        &self.eligibility_report
    }

    pub fn blocked_candidate_count(&self) -> usize {
        self.blocked_candidate_count
    }

    pub fn required_certificate_language(&self) -> &str {
        &self.required_certificate_language
    }

    pub fn required_targets(&self) -> &[String] {
        &self.required_targets
    }

    pub fn blocks_slack_response_until_satisfied(&self) -> bool {
        self.blocked_candidate_count > 0
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27UnitAttachmentObligationReport, core);

pub fn materialize_g27_unit_attachment_obligation_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27UnitAttachmentObligationReport, G27GeometricFractionalError> {
    let eligibility_report = screen_g27_quadratic_survivor_mutation_eligibility_checked(handle)?;
    let required_targets = eligibility_report
        .search_report()
        .moser_scan()
        .source_lead()
        .isometry_detail()
        .mapping_pairs()
        .iter()
        .flat_map(|(source, target)| [source.clone(), target.clone()])
        .collect::<Vec<_>>();
    let required_certificate_language =
        "exact algebraic squared-distance replay for outside-Moser anchor attachments to row-685 vertices"
            .to_string();
    let blocked_candidate_count = eligibility_report.blockers().len();
    let core = artifact_core(
        HadwigerArtifactKind::G27UnitAttachmentObligationReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_unit_attachment_obligation".to_string(),
        },
        vec![eligibility_report.reference()],
        obligation_payload(
            blocked_candidate_count,
            &required_certificate_language,
            &required_targets,
        ),
    )?;
    Ok(G27UnitAttachmentObligationReport {
        core,
        eligibility_report,
        blocked_candidate_count,
        required_certificate_language,
        required_targets,
    })
}

fn obligation_payload(
    blocked_candidate_count: usize,
    language: &str,
    targets: &[String],
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_unit_obligation.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "blocked_candidate_count",
            blocked_candidate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("required_certificate_language", language),
        HadwigerArtifactPayloadEntry::text("required_targets", targets.join(",")),
    ]
}
