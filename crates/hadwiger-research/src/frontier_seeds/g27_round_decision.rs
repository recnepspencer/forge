use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_unit_attachment_obligation::{
    materialize_g27_unit_attachment_obligation_checked, G27UnitAttachmentObligationReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27RoundDecisionPosture {
    FundUnitAttachmentCertificateLanguage,
}

impl G27RoundDecisionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundUnitAttachmentCertificateLanguage => {
                "fund_unit_attachment_certificate_language"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27RoundDecisionReport {
    core: HadwigerArtifactCore,
    obligation_report: G27UnitAttachmentObligationReport,
    decision: G27RoundDecisionPosture,
    funded_next_program: String,
    blocked_lane: String,
}

impl G27RoundDecisionReport {
    pub fn obligation_report(&self) -> &G27UnitAttachmentObligationReport {
        &self.obligation_report
    }

    pub fn decision(&self) -> G27RoundDecisionPosture {
        self.decision
    }

    pub fn funded_next_program(&self) -> &str {
        &self.funded_next_program
    }

    pub fn blocked_lane(&self) -> &str {
        &self.blocked_lane
    }

    pub fn keeps_row_685_funded(&self) -> bool {
        self.funded_next_program.contains("row-685")
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27RoundDecisionReport, core);

pub fn decide_g27_row_685_next_program_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27RoundDecisionReport, G27GeometricFractionalError> {
    let obligation_report = materialize_g27_unit_attachment_obligation_checked(handle)?;
    let decision = G27RoundDecisionPosture::FundUnitAttachmentCertificateLanguage;
    let funded_next_program =
        "row-685 exact outside-Moser unit-attachment certificate language".to_string();
    let blocked_lane =
        "mutated graph construction and geometric-fractional slack response remain blocked"
            .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27RoundDecisionReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_row_685_decision".to_string(),
        },
        vec![obligation_report.reference()],
        decision_payload(decision, &funded_next_program, &blocked_lane),
    )?;
    Ok(G27RoundDecisionReport {
        core,
        obligation_report,
        decision,
        funded_next_program,
        blocked_lane,
    })
}

fn decision_payload(
    decision: G27RoundDecisionPosture,
    funded_next_program: &str,
    blocked_lane: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_decision.v1"),
        HadwigerArtifactPayloadEntry::text("decision", decision.as_str()),
        HadwigerArtifactPayloadEntry::text("funded_next_program", funded_next_program),
        HadwigerArtifactPayloadEntry::text("blocked_lane", blocked_lane),
    ]
}
