use std::cmp::Ordering;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_same_field_marginal_pressure_support::NormalizedScore;
use super::g27_same_field_pressure_interface_support::{approx_unit_distance, g27_points};
use super::g27_w_circles_exact_geometry_audit::audit_g27_w_circles_607_exact_geometry_checked;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_vertices, squared_distance,
};

const G27_ANCHOR_INDEX: usize = 22;
const W_ANCHOR_INDEX: usize = 253;
const LIFT_GAP_WEIGHT_NUMERATOR: i128 = 51_749;
const TOP_CHANNEL_LIMIT: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27SameFieldMarginalPressurePosture {
    HeuristicOnlyNeedsReducedCostModel,
    FundExactReducedCostFollowup,
}

impl G27SameFieldMarginalPressurePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeuristicOnlyNeedsReducedCostModel => "heuristic_only_needs_reduced_cost_model",
            Self::FundExactReducedCostFollowup => "fund_exact_reduced_cost_followup",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MarginalPressureContactChannel {
    g27_vertex: usize,
    w_vertex: usize,
    w_weight: i128,
    g27_tight_participation: usize,
    normalized_contribution: NormalizedScore,
}

impl G27MarginalPressureContactChannel {
    pub fn g27_vertex(&self) -> usize {
        self.g27_vertex
    }

    pub fn w_vertex(&self) -> usize {
        self.w_vertex
    }

    pub fn w_weight(&self) -> i128 {
        self.w_weight
    }

    pub fn g27_tight_participation(&self) -> usize {
        self.g27_tight_participation
    }

    pub fn normalized_contribution_token(&self) -> String {
        self.normalized_contribution.stable_token()
    }

    fn stable_token(&self) -> String {
        format!(
            "g{}:w{}:weight{}:pressure{}:normalized{}",
            self.g27_vertex,
            self.w_vertex,
            self.w_weight,
            self.g27_tight_participation,
            self.normalized_contribution.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SameFieldMarginalPressureReport {
    core: HadwigerArtifactCore,
    g27_anchor: usize,
    w_anchor: usize,
    exact_contact_count: usize,
    contact_weight_sum: i128,
    normalized_score: NormalizedScore,
    top_one_share: NormalizedScore,
    top_five_share: NormalizedScore,
    top_ten_share: NormalizedScore,
    top_channels: Vec<G27MarginalPressureContactChannel>,
    posture: G27SameFieldMarginalPressurePosture,
    conclusion: String,
}

impl G27SameFieldMarginalPressureReport {
    pub fn g27_anchor(&self) -> usize {
        self.g27_anchor
    }

    pub fn w_anchor(&self) -> usize {
        self.w_anchor
    }

    pub fn exact_contact_count(&self) -> usize {
        self.exact_contact_count
    }

    pub fn contact_weight_sum(&self) -> i128 {
        self.contact_weight_sum
    }

    pub fn normalized_score_token(&self) -> String {
        self.normalized_score.stable_token()
    }

    pub fn normalized_score_clears_lift_numerator(&self) -> bool {
        self.normalized_score.cmp_integer(LIFT_GAP_WEIGHT_NUMERATOR) != Ordering::Less
    }

    pub fn top_one_share_token(&self) -> String {
        self.top_one_share.stable_token()
    }

    pub fn top_five_share_token(&self) -> String {
        self.top_five_share.stable_token()
    }

    pub fn top_ten_share_token(&self) -> String {
        self.top_ten_share.stable_token()
    }

    pub fn top_channels(&self) -> &[G27MarginalPressureContactChannel] {
        &self.top_channels
    }

    pub fn posture(&self) -> G27SameFieldMarginalPressurePosture {
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

impl_hadwiger_artifact!(G27SameFieldMarginalPressureReport, core);

pub fn analyze_g27_w_circles_marginal_pressure_channel_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SameFieldMarginalPressureReport, G27GeometricFractionalError> {
    let exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let g_pressures = g27_vertex_pressures(&retained_g27_tight_atom_masks()?);
    let w_points = parse_w_vertices()?;
    let w_weights = parse_w_integer_weights()?;
    let mut channels = contact_channels(&g_points, &g_pressures, &w_points, &w_weights);
    let contact_weight_sum = channels
        .iter()
        .map(|channel| channel.w_weight)
        .sum::<i128>();
    channels.sort_by(|left, right| {
        right
            .normalized_contribution
            .cmp(&left.normalized_contribution)
            .then_with(|| right.w_weight.cmp(&left.w_weight))
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let normalized_score = channels
        .iter()
        .fold(NormalizedScore::zero(), |sum, channel| {
            sum.add(channel.normalized_contribution)
        });
    let top_one_share = share(&channels, 1, normalized_score);
    let top_five_share = share(&channels, 5, normalized_score);
    let top_ten_share = share(&channels, 10, normalized_score);
    let top_channels = channels
        .iter()
        .take(TOP_CHANNEL_LIMIT)
        .cloned()
        .collect::<Vec<_>>();
    let posture = decide_posture(&top_channels, top_five_share);
    let conclusion = conclusion(posture, &top_channels, normalized_score);
    let core = artifact_core(
        HadwigerArtifactKind::G27SameFieldPressureInterfaceSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_marginal_pressure_channel".to_string(),
        },
        vec![exact_geometry.reference()],
        payload(
            channels.len(),
            contact_weight_sum,
            normalized_score,
            top_one_share,
            top_five_share,
            top_ten_share,
            &top_channels,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27SameFieldMarginalPressureReport {
        core,
        g27_anchor: G27_ANCHOR_INDEX + 1,
        w_anchor: W_ANCHOR_INDEX + 1,
        exact_contact_count: channels.len(),
        contact_weight_sum,
        normalized_score,
        top_one_share,
        top_five_share,
        top_ten_share,
        top_channels,
        posture,
        conclusion,
    })
}

fn contact_channels(
    g_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    g_pressures: &[usize; 27],
    w_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    w_weights: &[i128],
) -> Vec<G27MarginalPressureContactChannel> {
    let translation = g_points[G27_ANCHOR_INDEX].sub(w_points[W_ANCHOR_INDEX]);
    let mut channels = Vec::new();
    for (g_index, g_point) in g_points.iter().enumerate() {
        for (w_index, w_point) in w_points.iter().enumerate() {
            if g_index == G27_ANCHOR_INDEX && w_index == W_ANCHOR_INDEX {
                continue;
            }
            let translated_w = w_point.add(translation);
            if approx_unit_distance(*g_point, translated_w)
                && squared_distance(*g_point, translated_w).is_one()
            {
                let pressure = g_pressures[g_index];
                channels.push(G27MarginalPressureContactChannel {
                    g27_vertex: g_index + 1,
                    w_vertex: w_index + 1,
                    w_weight: w_weights[w_index],
                    g27_tight_participation: pressure,
                    normalized_contribution: NormalizedScore::new(
                        w_weights[w_index],
                        pressure as i128 + 1,
                    ),
                });
            }
        }
    }
    channels
}

fn g27_vertex_pressures(tight_atoms: &[u32]) -> [usize; 27] {
    let mut pressures = [0usize; 27];
    for atom in tight_atoms {
        for (vertex, pressure) in pressures.iter_mut().enumerate() {
            if atom & (1u32 << vertex) != 0 {
                *pressure += 1;
            }
        }
    }
    pressures
}

fn share(
    channels: &[G27MarginalPressureContactChannel],
    count: usize,
    total: NormalizedScore,
) -> NormalizedScore {
    let partial = channels
        .iter()
        .take(count)
        .fold(NormalizedScore::zero(), |sum, channel| {
            sum.add(channel.normalized_contribution)
        });
    partial.div(total)
}

fn decide_posture(
    top_channels: &[G27MarginalPressureContactChannel],
    top_five_share: NormalizedScore,
) -> G27SameFieldMarginalPressurePosture {
    let has_low_pressure_channel = top_channels
        .iter()
        .take(5)
        .any(|channel| channel.g27_tight_participation <= 16);
    if has_low_pressure_channel && top_five_share.cmp_fraction(1, 4) != Ordering::Less {
        G27SameFieldMarginalPressurePosture::FundExactReducedCostFollowup
    } else {
        G27SameFieldMarginalPressurePosture::HeuristicOnlyNeedsReducedCostModel
    }
}

fn conclusion(
    posture: G27SameFieldMarginalPressurePosture,
    top_channels: &[G27MarginalPressureContactChannel],
    normalized_score: NormalizedScore,
) -> String {
    let top = top_channels
        .first()
        .map(G27MarginalPressureContactChannel::stable_token)
        .unwrap_or_else(|| "none".to_string());
    match posture {
        G27SameFieldMarginalPressurePosture::FundExactReducedCostFollowup => format!(
            "fund reduced-cost followup: heuristic normalized score {} has a concentrated low-pressure top channel {top}",
            normalized_score.stable_token()
        ),
        G27SameFieldMarginalPressurePosture::HeuristicOnlyNeedsReducedCostModel => format!(
            "do not treat pressure-normalized score {} as lift evidence; top channel {top} remains a heuristic contact diagnostic until reduced-cost accounting exists",
            normalized_score.stable_token()
        ),
    }
}

fn payload(
    contact_count: usize,
    contact_weight_sum: i128,
    normalized_score: NormalizedScore,
    top_one_share: NormalizedScore,
    top_five_share: NormalizedScore,
    top_ten_share: NormalizedScore,
    top_channels: &[G27MarginalPressureContactChannel],
    posture: G27SameFieldMarginalPressurePosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_same_field_marginal_pressure.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("g27_anchor", (G27_ANCHOR_INDEX + 1) as u128),
        HadwigerArtifactPayloadEntry::unsigned("w_anchor", (W_ANCHOR_INDEX + 1) as u128),
        HadwigerArtifactPayloadEntry::unsigned("exact_contact_count", contact_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("contact_weight_sum", contact_weight_sum as u128),
        HadwigerArtifactPayloadEntry::text("normalized_score", normalized_score.stable_token()),
        HadwigerArtifactPayloadEntry::text("top_one_share", top_one_share.stable_token()),
        HadwigerArtifactPayloadEntry::text("top_five_share", top_five_share.stable_token()),
        HadwigerArtifactPayloadEntry::text("top_ten_share", top_ten_share.stable_token()),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for channel in top_channels {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "top_channel",
            channel.stable_token(),
        ));
    }
    payload
}
