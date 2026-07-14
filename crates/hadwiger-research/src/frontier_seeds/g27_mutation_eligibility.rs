use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_outside_moser_anchor::G27OutsideMoserAnchorCandidate;
use super::g27_quadratic_anchor_search::{
    search_g27_bounded_quadratic_anchors_checked, G27QuadraticAnchorSearchReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MutationEligibilityPosture {
    BlockedMissingUnitAttachmentEvidence,
}

impl G27MutationEligibilityPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockedMissingUnitAttachmentEvidence => {
                "blocked_missing_unit_attachment_evidence"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MutationEligibilityBlocker {
    candidate: G27OutsideMoserAnchorCandidate,
    posture: G27MutationEligibilityPosture,
    required_evidence: String,
}

impl G27MutationEligibilityBlocker {
    pub fn candidate(&self) -> &G27OutsideMoserAnchorCandidate {
        &self.candidate
    }

    pub fn posture(&self) -> G27MutationEligibilityPosture {
        self.posture
    }

    pub fn required_evidence(&self) -> &str {
        &self.required_evidence
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.candidate.stable_token(),
            self.posture.as_str(),
            self.required_evidence
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MutationEligibilityReport {
    core: HadwigerArtifactCore,
    search_report: G27QuadraticAnchorSearchReport,
    candidates_screened: usize,
    eligible_count: usize,
    blockers: Vec<G27MutationEligibilityBlocker>,
}

impl G27MutationEligibilityReport {
    pub fn search_report(&self) -> &G27QuadraticAnchorSearchReport {
        &self.search_report
    }

    pub fn candidates_screened(&self) -> usize {
        self.candidates_screened
    }

    pub fn eligible_count(&self) -> usize {
        self.eligible_count
    }

    pub fn blockers(&self) -> &[G27MutationEligibilityBlocker] {
        &self.blockers
    }

    pub fn admits_mutated_graph_artifacts(&self) -> bool {
        self.eligible_count > 0
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27MutationEligibilityReport, core);

pub fn screen_g27_quadratic_survivor_mutation_eligibility_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MutationEligibilityReport, G27GeometricFractionalError> {
    let search_report = search_g27_bounded_quadratic_anchors_checked(handle)?;
    let blockers =
        search_report
            .retained_survivors()
            .iter()
            .cloned()
            .map(|candidate| G27MutationEligibilityBlocker {
                candidate,
                posture: G27MutationEligibilityPosture::BlockedMissingUnitAttachmentEvidence,
                required_evidence:
                    "exact row-685 unit-attachment certificate for the outside-Moser anchor"
                        .to_string(),
            })
            .collect::<Vec<_>>();
    let core = artifact_core(
        HadwigerArtifactKind::G27MutationEligibilityReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_mutation_eligibility_screen".to_string(),
        },
        vec![search_report.reference()],
        eligibility_payload(&blockers),
    )?;
    Ok(G27MutationEligibilityReport {
        core,
        search_report,
        candidates_screened: blockers.len(),
        eligible_count: 0,
        blockers,
    })
}

fn eligibility_payload(
    blockers: &[G27MutationEligibilityBlocker],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_mutation_gate.v1"),
        HadwigerArtifactPayloadEntry::unsigned("candidates_screened", blockers.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("eligible_count", 0),
    ];
    for blocker in blockers {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "blocker",
            blocker.stable_token(),
        ));
    }
    payload
}
