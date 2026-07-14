use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_dual_slack_attachment_replay::count_common_moser_basis_attachments;
use super::g27_geometric_fractional::{
    reproduce_g27_geometric_fractional_witness_checked, G27GeometricFractionalError,
};
use super::g27_geometric_fractional_data::{
    is_retained_g27_moser_unit_difference, retained_g27_coefficients,
};

const VERTEX_COUNT: usize = 27;
const TOP_CANDIDATE_LIMIT: usize = 5;
const FUNDING_SCORE: usize = 2_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27DualSlackInversionPosture {
    FundAlgebraicAttachmentReplay,
    RetiredMoserBasisCapped,
    RetiredWeakSlackInterface,
}

impl G27DualSlackInversionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundAlgebraicAttachmentReplay => "fund_algebraic_attachment_replay",
            Self::RetiredMoserBasisCapped => "retired_moser_basis_capped",
            Self::RetiredWeakSlackInterface => "retired_weak_slack_interface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27DualSlackInversionCandidate {
    vertex_label: String,
    own_tight_pressure: usize,
    top_neighbor_label: String,
    top_neighbor_pressure: usize,
    second_neighbor_label: String,
    second_neighbor_pressure: usize,
    tight_neighbor_count: usize,
    slack_inversion_score: usize,
    moser_basis_attachment_count: usize,
    exact_attachment_obligation: String,
}

impl G27DualSlackInversionCandidate {
    pub fn vertex_label(&self) -> &str {
        &self.vertex_label
    }

    pub fn own_tight_pressure(&self) -> usize {
        self.own_tight_pressure
    }

    pub fn top_neighbor_label(&self) -> &str {
        &self.top_neighbor_label
    }

    pub fn top_neighbor_pressure(&self) -> usize {
        self.top_neighbor_pressure
    }

    pub fn second_neighbor_label(&self) -> &str {
        &self.second_neighbor_label
    }

    pub fn second_neighbor_pressure(&self) -> usize {
        self.second_neighbor_pressure
    }

    pub fn tight_neighbor_count(&self) -> usize {
        self.tight_neighbor_count
    }

    pub fn slack_inversion_score(&self) -> usize {
        self.slack_inversion_score
    }

    pub fn moser_basis_attachment_count(&self) -> usize {
        self.moser_basis_attachment_count
    }

    pub fn exact_attachment_obligation(&self) -> &str {
        &self.exact_attachment_obligation
    }

    fn stable_token(&self) -> String {
        format!(
            "v{}:p{}:n{}:{}:n{}:{}:tight_neighbors{}:score{}:moser_anchors{}:{}",
            self.vertex_label,
            self.own_tight_pressure,
            self.top_neighbor_label,
            self.top_neighbor_pressure,
            self.second_neighbor_label,
            self.second_neighbor_pressure,
            self.tight_neighbor_count,
            self.slack_inversion_score,
            self.moser_basis_attachment_count,
            self.exact_attachment_obligation
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27DualSlackInversionReport {
    core: HadwigerArtifactCore,
    candidates: Vec<G27DualSlackInversionCandidate>,
    funded_candidate: Option<G27DualSlackInversionCandidate>,
    posture: G27DualSlackInversionPosture,
    conclusion: String,
}

impl G27DualSlackInversionReport {
    pub fn candidates(&self) -> &[G27DualSlackInversionCandidate] {
        &self.candidates
    }

    pub fn funded_candidate(&self) -> Option<&G27DualSlackInversionCandidate> {
        self.funded_candidate.as_ref()
    }

    pub fn posture(&self) -> G27DualSlackInversionPosture {
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

impl_hadwiger_artifact!(G27DualSlackInversionReport, core);

pub fn analyze_g27_dual_slack_inversion_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27DualSlackInversionReport, G27GeometricFractionalError> {
    let reproduction = reproduce_g27_geometric_fractional_witness_checked(handle)?;
    let coefficients = retained_g27_coefficients()?;
    let pressure = vertex_pressures(reproduction.dual_replay().pressure_report().top_vertices())?;
    let adjacency = retained_adjacency(&coefficients)?;
    let candidates = rank_dual_slack_candidates(&pressure, &adjacency, &coefficients)?;
    let funded_candidate = candidates
        .iter()
        .find(|candidate| {
            candidate.slack_inversion_score >= FUNDING_SCORE
                && candidate.moser_basis_attachment_count == 0
        })
        .cloned();
    let posture = if funded_candidate.is_some() {
        G27DualSlackInversionPosture::FundAlgebraicAttachmentReplay
    } else if candidates
        .iter()
        .any(|candidate| candidate.moser_basis_attachment_count > 0)
    {
        G27DualSlackInversionPosture::RetiredMoserBasisCapped
    } else {
        G27DualSlackInversionPosture::RetiredWeakSlackInterface
    };
    let conclusion = conclusion(posture, funded_candidate.as_ref());
    let core = artifact_core(
        HadwigerArtifactKind::G27DualSlackInversionReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_dual_slack_inversion_admission".to_string(),
        },
        vec![reproduction.evaluation().reference()],
        payload(&candidates, funded_candidate.as_ref(), posture, &conclusion),
    )?;
    Ok(G27DualSlackInversionReport {
        core,
        candidates,
        funded_candidate,
        posture,
        conclusion,
    })
}

fn vertex_pressures(
    rows: &[super::g27_geometric_fractional_slack_analysis::G27PressureVertex],
) -> Result<[usize; VERTEX_COUNT], G27GeometricFractionalError> {
    let mut pressures = [0usize; VERTEX_COUNT];
    for row in rows {
        let index = parse_vertex(row.vertex_label())?;
        pressures[index] = row.tight_atom_participation();
    }
    Ok(pressures)
}

fn retained_adjacency(
    coefficients: &[[i32; 4]],
) -> Result<Vec<Vec<usize>>, G27GeometricFractionalError> {
    let mut adjacency = vec![Vec::new(); VERTEX_COUNT];
    for left in 0..VERTEX_COUNT {
        for right in (left + 1)..VERTEX_COUNT {
            let diff = [
                coefficients[left][0] - coefficients[right][0],
                coefficients[left][1] - coefficients[right][1],
                coefficients[left][2] - coefficients[right][2],
                coefficients[left][3] - coefficients[right][3],
            ];
            if is_retained_g27_moser_unit_difference(diff) {
                adjacency[left].push(right);
                adjacency[right].push(left);
            }
        }
    }
    Ok(adjacency)
}

fn rank_dual_slack_candidates(
    pressures: &[usize; VERTEX_COUNT],
    adjacency: &[Vec<usize>],
    coefficients: &[[i32; 4]],
) -> Result<Vec<G27DualSlackInversionCandidate>, G27GeometricFractionalError> {
    let max_pressure = pressures.iter().copied().max().unwrap_or(0);
    let median_pressure = sorted_pressures(pressures)[VERTEX_COUNT / 2];
    let mut candidates = Vec::new();
    for vertex in 0..VERTEX_COUNT {
        if pressures[vertex] > median_pressure || adjacency[vertex].len() < 2 {
            continue;
        }
        let mut neighbors = adjacency[vertex].clone();
        neighbors.sort_by(|left, right| {
            pressures[*right]
                .cmp(&pressures[*left])
                .then_with(|| left.cmp(right))
        });
        let top = neighbors[0];
        let second = neighbors[1];
        let tight_neighbor_count = neighbors
            .iter()
            .filter(|neighbor| pressures[**neighbor] > median_pressure)
            .count();
        let slack_gap = max_pressure.saturating_sub(pressures[vertex]);
        let neighbor_pressure = pressures[top] + pressures[second];
        let slack_inversion_score = slack_gap * tight_neighbor_count + neighbor_pressure;
        let moser_basis_attachment_count =
            count_common_moser_basis_attachments(coefficients, vertex, top, second)?;
        candidates.push(G27DualSlackInversionCandidate {
            vertex_label: (vertex + 1).to_string(),
            own_tight_pressure: pressures[vertex],
            top_neighbor_label: (top + 1).to_string(),
            top_neighbor_pressure: pressures[top],
            second_neighbor_label: (second + 1).to_string(),
            second_neighbor_pressure: pressures[second],
            tight_neighbor_count,
            slack_inversion_score,
            moser_basis_attachment_count,
            exact_attachment_obligation: attachment_obligation(moser_basis_attachment_count),
        });
    }
    candidates.sort_by(|left, right| {
        right
            .slack_inversion_score
            .cmp(&left.slack_inversion_score)
            .then_with(|| {
                left.moser_basis_attachment_count
                    .cmp(&right.moser_basis_attachment_count)
            })
            .then_with(|| left.vertex_label.cmp(&right.vertex_label))
    });
    candidates.truncate(TOP_CANDIDATE_LIMIT);
    Ok(candidates)
}

fn sorted_pressures(pressures: &[usize; VERTEX_COUNT]) -> Vec<usize> {
    let mut sorted = pressures.to_vec();
    sorted.sort_unstable();
    sorted
}

fn attachment_obligation(moser_basis_attachment_count: usize) -> String {
    if moser_basis_attachment_count == 0 {
        "no retained Moser-basis triple-unit attachment; fund exact algebraic attachment replay"
            .to_string()
    } else {
        format!(
            "{moser_basis_attachment_count} retained Moser-basis triple-unit attachments exist; capped unless basis pivot proof appears"
        )
    }
}

fn conclusion(
    posture: G27DualSlackInversionPosture,
    funded: Option<&G27DualSlackInversionCandidate>,
) -> String {
    match (posture, funded) {
        (G27DualSlackInversionPosture::FundAlgebraicAttachmentReplay, Some(candidate)) => format!(
            "fund dual-slack inversion for vertex {}: low current pressure {}, tight-neighbor score {}, and no Moser-basis triple-unit attachment",
            candidate.vertex_label(),
            candidate.own_tight_pressure(),
            candidate.slack_inversion_score()
        ),
        (G27DualSlackInversionPosture::RetiredMoserBasisCapped, _) => {
            "retire broad dual-slack inversion: candidate interfaces already have retained Moser-basis attachments".to_string()
        }
        (G27DualSlackInversionPosture::RetiredWeakSlackInterface, _) => {
            "retire dual-slack inversion: no candidate has enough tight-neighborhood pressure".to_string()
        }
        (G27DualSlackInversionPosture::FundAlgebraicAttachmentReplay, None) => {
            "funding posture missing candidate".to_string()
        }
    }
}

fn payload(
    candidates: &[G27DualSlackInversionCandidate],
    funded: Option<&G27DualSlackInversionCandidate>,
    posture: G27DualSlackInversionPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_dual_slack_inversion.v1"),
        HadwigerArtifactPayloadEntry::unsigned("candidate_count", candidates.len() as u128),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    if let Some(candidate) = funded {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "funded_candidate",
            candidate.stable_token(),
        ));
    }
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}

fn parse_vertex(label: &str) -> Result<usize, G27GeometricFractionalError> {
    label
        .parse::<usize>()
        .ok()
        .and_then(|value| value.checked_sub(1))
        .filter(|index| *index < VERTEX_COUNT)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "g27_dual_slack_vertex",
        })
}
