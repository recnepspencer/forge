use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_rotation_pin_batch_exact_replay::{
    replay_g27_rotation_pin_batch_exact_checked, G27RotationPinCandidateExactReplay,
};

const FUND_LP_PREFLIGHT_SCORE: usize = 26;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27RotationPinPressureScorePosture {
    FundTwoPinLpPreflight,
    PivotToCrossRingFusion,
}

impl G27RotationPinPressureScorePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundTwoPinLpPreflight => "fund_two_pin_lp_preflight",
            Self::PivotToCrossRingFusion => "pivot_to_cross_ring_fusion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27RotationPinPressureCandidateScore {
    witness_vertex: String,
    pin_vertex: String,
    exact_pair_count: usize,
    tight_atom_pressure_score: usize,
    max_pair_pressure: usize,
}

impl G27RotationPinPressureCandidateScore {
    pub fn witness_vertex(&self) -> &str {
        &self.witness_vertex
    }

    pub fn pin_vertex(&self) -> &str {
        &self.pin_vertex
    }

    pub fn exact_pair_count(&self) -> usize {
        self.exact_pair_count
    }

    pub fn tight_atom_pressure_score(&self) -> usize {
        self.tight_atom_pressure_score
    }

    pub fn max_pair_pressure(&self) -> usize {
        self.max_pair_pressure
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.witness_vertex,
            self.pin_vertex,
            self.exact_pair_count,
            self.tight_atom_pressure_score,
            self.max_pair_pressure
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27RotationPinPressureScoreReport {
    core: HadwigerArtifactCore,
    scored_candidate_count: usize,
    fundable_candidate_count: usize,
    retained_scores: Vec<G27RotationPinPressureCandidateScore>,
    posture: G27RotationPinPressureScorePosture,
    conclusion: String,
}

impl G27RotationPinPressureScoreReport {
    pub fn scored_candidate_count(&self) -> usize {
        self.scored_candidate_count
    }

    pub fn fundable_candidate_count(&self) -> usize {
        self.fundable_candidate_count
    }

    pub fn retained_scores(&self) -> &[G27RotationPinPressureCandidateScore] {
        &self.retained_scores
    }

    pub fn posture(&self) -> G27RotationPinPressureScorePosture {
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

impl_hadwiger_artifact!(G27RotationPinPressureScoreReport, core);

pub fn score_g27_rotation_pin_exact_survivors_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27RotationPinPressureScoreReport, G27GeometricFractionalError> {
    let batch = replay_g27_rotation_pin_batch_exact_checked(handle)?;
    let tight_atoms = retained_g27_tight_atom_masks()?;
    let mut scores = batch
        .best_candidates()
        .iter()
        .filter(|candidate| candidate.max_exact_unit_pairs_per_branch() > 1)
        .map(|candidate| score_candidate(candidate, &tight_atoms))
        .collect::<Result<Vec<_>, _>>()?;
    scores.sort_by(|left, right| {
        right
            .tight_atom_pressure_score
            .cmp(&left.tight_atom_pressure_score)
            .then_with(|| right.max_pair_pressure.cmp(&left.max_pair_pressure))
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let fundable_candidate_count = scores
        .iter()
        .filter(|score| score.max_pair_pressure >= FUND_LP_PREFLIGHT_SCORE)
        .count();
    let posture = if fundable_candidate_count > 0 {
        G27RotationPinPressureScorePosture::FundTwoPinLpPreflight
    } else {
        G27RotationPinPressureScorePosture::PivotToCrossRingFusion
    };
    let conclusion = match posture {
        G27RotationPinPressureScorePosture::FundTwoPinLpPreflight => {
            "two-pin exact survivor touches retained top-pressure structure; fund bounded LP preflight"
        }
        G27RotationPinPressureScorePosture::PivotToCrossRingFusion => {
            "two-pin exact survivors are below top-pressure threshold; pivot to cross-ring fusion"
        }
    }
    .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27RotationPinPressureScoreReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_rotation_pin_pressure_score".to_string(),
        },
        vec![batch.reference()],
        payload(
            scores.len(),
            fundable_candidate_count,
            posture,
            &scores,
            &conclusion,
        ),
    )?;
    Ok(G27RotationPinPressureScoreReport {
        core,
        scored_candidate_count: scores.len(),
        fundable_candidate_count,
        retained_scores: scores,
        posture,
        conclusion,
    })
}

fn score_candidate(
    candidate: &G27RotationPinCandidateExactReplay,
    tight_atoms: &[u32],
) -> Result<G27RotationPinPressureCandidateScore, G27GeometricFractionalError> {
    let mut total_score = 0usize;
    let mut max_pair_pressure = 0usize;
    for (left, right) in candidate.exact_unit_pairs() {
        let score = pair_pressure(parse_vertex(left)?, parse_vertex(right)?, tight_atoms);
        total_score += score;
        max_pair_pressure = max_pair_pressure.max(score);
    }
    Ok(G27RotationPinPressureCandidateScore {
        witness_vertex: candidate.witness_vertex().to_string(),
        pin_vertex: candidate.pin_vertex().to_string(),
        exact_pair_count: candidate.exact_unit_pairs().len(),
        tight_atom_pressure_score: total_score,
        max_pair_pressure,
    })
}

fn pair_pressure(left: usize, right: usize, tight_atoms: &[u32]) -> usize {
    let left_bit = 1u32 << (left - 1);
    let right_bit = 1u32 << (right - 1);
    tight_atoms
        .iter()
        .filter(|atom| **atom & left_bit != 0 && **atom & right_bit != 0)
        .count()
}

fn parse_vertex(value: &str) -> Result<usize, G27GeometricFractionalError> {
    value
        .parse::<usize>()
        .map_err(|_| G27GeometricFractionalError::MalformedData {
            source: "pressure_score_vertex",
        })
}

fn payload(
    scored_candidate_count: usize,
    fundable_candidate_count: usize,
    posture: G27RotationPinPressureScorePosture,
    scores: &[G27RotationPinPressureCandidateScore],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_rotation_pressure.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "scored_candidate_count",
            scored_candidate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "fundable_candidate_count",
            fundable_candidate_count as u128,
        ),
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
