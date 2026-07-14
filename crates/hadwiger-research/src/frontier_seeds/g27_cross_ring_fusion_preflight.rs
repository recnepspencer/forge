use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_rotation_pin_pressure_score::score_g27_rotation_pin_exact_survivors_checked;
use super::g27_spindle_and_fusion_searches::{
    search_g27_cross_ring_fusion_candidates_checked, G27CrossRingFusionCandidate,
};

const MIN_SHARED_VERTEX_PRESSURE: usize = 20;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27CrossRingFusionPreflightPosture {
    FundColumnGeneration,
    SuppressedWeakG27Attachment,
}

impl G27CrossRingFusionPreflightPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundColumnGeneration => "fund_column_generation",
            Self::SuppressedWeakG27Attachment => "suppressed_weak_g27_attachment",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingFusionPreflightScore {
    core_label: String,
    foreign_radicand: u32,
    shared_vertex: String,
    shared_vertex_pressure: usize,
    retained_core_scale_score: usize,
    column_generation_priority_score: usize,
    column_generation_obligation: String,
}

impl G27CrossRingFusionPreflightScore {
    pub fn core_label(&self) -> &str {
        &self.core_label
    }

    pub fn foreign_radicand(&self) -> u32 {
        self.foreign_radicand
    }

    pub fn shared_vertex(&self) -> &str {
        &self.shared_vertex
    }

    pub fn shared_vertex_pressure(&self) -> usize {
        self.shared_vertex_pressure
    }

    pub fn retained_core_scale_score(&self) -> usize {
        self.retained_core_scale_score
    }

    pub fn column_generation_priority_score(&self) -> usize {
        self.column_generation_priority_score
    }

    pub fn column_generation_obligation(&self) -> &str {
        &self.column_generation_obligation
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:sqrt{}:shared{}:pressure{}:core{}:priority{}:{}",
            self.core_label,
            self.foreign_radicand,
            self.shared_vertex,
            self.shared_vertex_pressure,
            self.retained_core_scale_score,
            self.column_generation_priority_score,
            self.column_generation_obligation
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingFusionPreflightReport {
    core: HadwigerArtifactCore,
    scored_candidate_count: usize,
    selected_candidate: G27CrossRingFusionPreflightScore,
    retained_scores: Vec<G27CrossRingFusionPreflightScore>,
    posture: G27CrossRingFusionPreflightPosture,
    conclusion: String,
}

impl G27CrossRingFusionPreflightReport {
    pub fn scored_candidate_count(&self) -> usize {
        self.scored_candidate_count
    }

    pub fn selected_candidate(&self) -> &G27CrossRingFusionPreflightScore {
        &self.selected_candidate
    }

    pub fn retained_scores(&self) -> &[G27CrossRingFusionPreflightScore] {
        &self.retained_scores
    }

    pub fn posture(&self) -> G27CrossRingFusionPreflightPosture {
        self.posture
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27CrossRingFusionPreflightReport, core);

pub fn preflight_g27_cross_ring_fusion_column_generation_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27CrossRingFusionPreflightReport, G27GeometricFractionalError> {
    let spindle_retirement = score_g27_rotation_pin_exact_survivors_checked(handle)?;
    let search = search_g27_cross_ring_fusion_candidates_checked(handle)?;
    let tight_atoms = retained_g27_tight_atom_masks()?;
    let mut scores = search
        .retained_candidates()
        .iter()
        .map(|candidate| score_candidate(candidate, &tight_atoms))
        .collect::<Result<Vec<_>, _>>()?;
    scores.sort_by(|left, right| {
        right
            .column_generation_priority_score
            .cmp(&left.column_generation_priority_score)
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let selected_candidate =
        scores
            .first()
            .cloned()
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "cross_ring_fusion_scores",
            })?;
    let posture = if selected_candidate.shared_vertex_pressure >= MIN_SHARED_VERTEX_PRESSURE {
        G27CrossRingFusionPreflightPosture::FundColumnGeneration
    } else {
        G27CrossRingFusionPreflightPosture::SuppressedWeakG27Attachment
    };
    let conclusion = match posture {
        G27CrossRingFusionPreflightPosture::FundColumnGeneration => {
            "fund cross-ring column generation for the selected foreign-field core"
        }
        G27CrossRingFusionPreflightPosture::SuppressedWeakG27Attachment => {
            "suppress cross-ring fusion: retained shared vertex is below G27 pressure threshold"
        }
    }
    .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27CrossRingFusionPreflightReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_cross_ring_fusion_preflight".to_string(),
        },
        vec![spindle_retirement.reference(), search.reference()],
        payload(
            scores.len(),
            &selected_candidate,
            &scores,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27CrossRingFusionPreflightReport {
        core,
        scored_candidate_count: scores.len(),
        selected_candidate,
        retained_scores: scores,
        posture,
        conclusion,
    })
}

fn score_candidate(
    candidate: &G27CrossRingFusionCandidate,
    tight_atoms: &[u32],
) -> Result<G27CrossRingFusionPreflightScore, G27GeometricFractionalError> {
    let shared_vertex = parse_vertex(candidate.shared_vertex())?;
    let shared_vertex_pressure = vertex_pressure(shared_vertex, tight_atoms);
    let retained_core_scale_score = retained_core_scale(candidate.core_label());
    let pin_score = pin_family_score(candidate.pin_family());
    let column_generation_priority_score =
        shared_vertex_pressure * retained_core_scale_score + pin_score;
    Ok(G27CrossRingFusionPreflightScore {
        core_label: candidate.core_label().to_string(),
        foreign_radicand: candidate.foreign_radicand(),
        shared_vertex: candidate.shared_vertex().to_string(),
        shared_vertex_pressure,
        retained_core_scale_score,
        column_generation_priority_score,
        column_generation_obligation: format!(
            "build master/pricing replay for {} over sqrt{}",
            candidate.core_label(),
            candidate.foreign_radicand()
        ),
    })
}

fn vertex_pressure(vertex: usize, tight_atoms: &[u32]) -> usize {
    let bit = 1u32 << (vertex - 1);
    tight_atoms.iter().filter(|atom| **atom & bit != 0).count()
}

fn retained_core_scale(core_label: &str) -> usize {
    core_label
        .split('_')
        .next()
        .and_then(|prefix| prefix.parse::<usize>().ok())
        .unwrap_or(27)
}

fn pin_family_score(pin_family: &str) -> usize {
    match pin_family {
        "column_generation_required" => 30,
        "relative_rotation_cross_pin" => 20,
        "single_cross_unit_edge" => 10,
        _ => 0,
    }
}

fn parse_vertex(value: &str) -> Result<usize, G27GeometricFractionalError> {
    value
        .parse::<usize>()
        .map_err(|_| G27GeometricFractionalError::MalformedData {
            source: "fusion_shared_vertex",
        })
}

fn payload(
    scored_candidate_count: usize,
    selected_candidate: &G27CrossRingFusionPreflightScore,
    scores: &[G27CrossRingFusionPreflightScore],
    posture: G27CrossRingFusionPreflightPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_cross_fusion.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "scored_candidate_count",
            scored_candidate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("selected", selected_candidate.stable_token()),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for score in scores {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate_score",
            score.stable_token(),
        ));
    }
    payload
}
