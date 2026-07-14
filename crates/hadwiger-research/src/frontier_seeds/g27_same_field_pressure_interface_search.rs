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
use super::g27_same_field_pressure_interface_support::{approx_unit_distance, g27_points};
use super::g27_w_circles_exact_geometry_audit::audit_g27_w_circles_607_exact_geometry_checked;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_vertices, squared_distance, WExactPoint,
};

const G_TOP_DECILE_LIMIT: usize = 3;
const W_TOP_DECILE_LIMIT: usize = 61;
const RETAINED_CANDIDATE_LIMIT: usize = 8;
const LIFT_GAP_WEIGHT_NUMERATOR: i128 = 51_749;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27SameFieldPressureInterfacePosture {
    FundFusedPricingAfterWeightedCertificateReplay,
    RetiredNoDenseExactInterface,
}

impl G27SameFieldPressureInterfacePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundFusedPricingAfterWeightedCertificateReplay => {
                "fund_fused_pricing_after_weighted_certificate_replay"
            }
            Self::RetiredNoDenseExactInterface => "retired_no_dense_exact_interface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SameFieldPressureInterfaceCandidate {
    g27_anchor: String,
    g27_anchor_pressure: usize,
    w_anchor: usize,
    w_anchor_weight: i128,
    cross_unit_contact_count: usize,
    contact_weight_sum: i128,
    g27_priced_contact_count: usize,
    g27_unpriced_contact_count: usize,
    g27_unpriced_contact_weight_sum: i128,
}

impl G27SameFieldPressureInterfaceCandidate {
    pub fn g27_anchor(&self) -> &str {
        &self.g27_anchor
    }

    pub fn g27_anchor_pressure(&self) -> usize {
        self.g27_anchor_pressure
    }

    pub fn w_anchor(&self) -> usize {
        self.w_anchor
    }

    pub fn w_anchor_weight(&self) -> i128 {
        self.w_anchor_weight
    }

    pub fn cross_unit_contact_count(&self) -> usize {
        self.cross_unit_contact_count
    }

    pub fn contact_weight_sum(&self) -> i128 {
        self.contact_weight_sum
    }

    pub fn g27_priced_contact_count(&self) -> usize {
        self.g27_priced_contact_count
    }

    pub fn g27_unpriced_contact_count(&self) -> usize {
        self.g27_unpriced_contact_count
    }

    pub fn g27_unpriced_contact_weight_sum(&self) -> i128 {
        self.g27_unpriced_contact_weight_sum
    }

    pub fn optimistic_capacity_clears_lift_gap(&self) -> bool {
        self.contact_weight_sum >= LIFT_GAP_WEIGHT_NUMERATOR
    }

    pub fn g27_unpriced_capacity_clears_lift_gap(&self) -> bool {
        self.g27_unpriced_contact_weight_sum >= LIFT_GAP_WEIGHT_NUMERATOR
    }

    fn stable_token(&self) -> String {
        format!(
            "g{}:p{}:w{}:weight{}:contacts{}:contact_weight{}:g_priced{}:g_unpriced{}:g_unpriced_weight{}",
            self.g27_anchor,
            self.g27_anchor_pressure,
            self.w_anchor,
            self.w_anchor_weight,
            self.cross_unit_contact_count,
            self.contact_weight_sum,
            self.g27_priced_contact_count,
            self.g27_unpriced_contact_count,
            self.g27_unpriced_contact_weight_sum
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SameFieldPressureInterfaceSearchReport {
    core: HadwigerArtifactCore,
    searched_anchor_pairs: usize,
    retained_candidates: Vec<G27SameFieldPressureInterfaceCandidate>,
    lift_gap_weight_numerator: i128,
    posture: G27SameFieldPressureInterfacePosture,
    conclusion: String,
}

impl G27SameFieldPressureInterfaceSearchReport {
    pub fn searched_anchor_pairs(&self) -> usize {
        self.searched_anchor_pairs
    }

    pub fn retained_candidates(&self) -> &[G27SameFieldPressureInterfaceCandidate] {
        &self.retained_candidates
    }

    pub fn lift_gap_weight_numerator(&self) -> i128 {
        self.lift_gap_weight_numerator
    }

    pub fn posture(&self) -> G27SameFieldPressureInterfacePosture {
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

impl_hadwiger_artifact!(G27SameFieldPressureInterfaceSearchReport, core);

pub fn search_g27_w_circles_same_field_pressure_interfaces_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SameFieldPressureInterfaceSearchReport, G27GeometricFractionalError> {
    search_g27_w_circles_interfaces_checked(
        handle,
        AnchorStrategy::TopPressure,
        "g27_w_circles_same_field_pressure_interface_search",
    )
}

pub fn search_g27_w_circles_slack_halo_interfaces_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SameFieldPressureInterfaceSearchReport, G27GeometricFractionalError> {
    search_g27_w_circles_interfaces_checked(
        handle,
        AnchorStrategy::SlackHalo,
        "g27_w_circles_slack_halo_interface_search",
    )
}

#[derive(Clone, Copy)]
enum AnchorStrategy {
    TopPressure,
    SlackHalo,
}

fn search_g27_w_circles_interfaces_checked(
    handle: &HadwigerResearchHandle,
    strategy: AnchorStrategy,
    operation: &str,
) -> Result<G27SameFieldPressureInterfaceSearchReport, G27GeometricFractionalError> {
    let exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let g_pressures = g27_vertex_pressures(&retained_g27_tight_atom_masks()?);
    let w_points = parse_w_vertices()?;
    let w_weights = parse_w_integer_weights()?;
    let g_anchors = g_anchors(&g_pressures, strategy);
    let w_anchors = top_w_anchors(&w_weights);
    let mut candidates = Vec::new();
    for g_anchor in &g_anchors {
        for w_anchor in &w_anchors {
            candidates.push(score_anchor_pair(
                *g_anchor,
                *w_anchor,
                &g_points,
                &g_pressures,
                &w_points,
                &w_weights,
            ));
        }
    }
    sort_candidates(&mut candidates, strategy);
    candidates.truncate(RETAINED_CANDIDATE_LIMIT);
    let posture = if candidates
        .first()
        .is_some_and(G27SameFieldPressureInterfaceCandidate::g27_unpriced_capacity_clears_lift_gap)
    {
        G27SameFieldPressureInterfacePosture::FundFusedPricingAfterWeightedCertificateReplay
    } else {
        G27SameFieldPressureInterfacePosture::RetiredNoDenseExactInterface
    };
    let conclusion = conclusion(posture, candidates.first());
    let core = artifact_core(
        HadwigerArtifactKind::G27SameFieldPressureInterfaceSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: operation.to_string(),
        },
        vec![exact_geometry.reference()],
        payload(
            g_anchors.len() * w_anchors.len(),
            &candidates,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27SameFieldPressureInterfaceSearchReport {
        core,
        searched_anchor_pairs: g_anchors.len() * w_anchors.len(),
        retained_candidates: candidates,
        lift_gap_weight_numerator: LIFT_GAP_WEIGHT_NUMERATOR,
        posture,
        conclusion,
    })
}

fn score_anchor_pair(
    g_anchor: usize,
    w_anchor: usize,
    g_points: &[WExactPoint],
    g_pressures: &[usize; 27],
    w_points: &[WExactPoint],
    w_weights: &[i128],
) -> G27SameFieldPressureInterfaceCandidate {
    let translation = g_points[g_anchor].sub(w_points[w_anchor]);
    let mut contact_count = 0usize;
    let mut contact_weight_sum = 0i128;
    let mut g27_priced_contact_count = 0usize;
    let mut g27_unpriced_contact_count = 0usize;
    let mut g27_unpriced_contact_weight_sum = 0i128;
    for (g_index, g_point) in g_points.iter().enumerate() {
        for (w_index, w_point) in w_points.iter().enumerate() {
            if g_index == g_anchor && w_index == w_anchor {
                continue;
            }
            let translated_w = w_point.add(translation);
            if !approx_unit_distance(*g_point, translated_w) {
                continue;
            }
            if squared_distance(*g_point, translated_w).is_one() {
                contact_count += 1;
                contact_weight_sum += w_weights[w_index];
                if g_pressures[g_index] == 0 {
                    g27_unpriced_contact_count += 1;
                    g27_unpriced_contact_weight_sum += w_weights[w_index];
                } else {
                    g27_priced_contact_count += 1;
                }
            }
        }
    }
    G27SameFieldPressureInterfaceCandidate {
        g27_anchor: (g_anchor + 1).to_string(),
        g27_anchor_pressure: g_pressures[g_anchor],
        w_anchor: w_anchor + 1,
        w_anchor_weight: w_weights[w_anchor],
        cross_unit_contact_count: contact_count,
        contact_weight_sum,
        g27_priced_contact_count,
        g27_unpriced_contact_count,
        g27_unpriced_contact_weight_sum,
    }
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

fn g_anchors(pressures: &[usize; 27], strategy: AnchorStrategy) -> Vec<usize> {
    let mut anchors = (0..27).collect::<Vec<_>>();
    match strategy {
        AnchorStrategy::TopPressure => {
            anchors.sort_by(|left, right| {
                pressures[*right]
                    .cmp(&pressures[*left])
                    .then(left.cmp(right))
            });
            anchors.truncate(G_TOP_DECILE_LIMIT);
        }
        AnchorStrategy::SlackHalo => {}
    }
    anchors
}

fn top_w_anchors(weights: &[i128]) -> Vec<usize> {
    let mut anchors = (0..weights.len()).collect::<Vec<_>>();
    anchors.sort_by(|left, right| weights[*right].cmp(&weights[*left]).then(left.cmp(right)));
    anchors.truncate(W_TOP_DECILE_LIMIT);
    anchors
}

fn sort_candidates(
    candidates: &mut [G27SameFieldPressureInterfaceCandidate],
    strategy: AnchorStrategy,
) {
    candidates.sort_by(|left, right| {
        let capacity_order = right
            .g27_unpriced_contact_weight_sum
            .cmp(&left.g27_unpriced_contact_weight_sum);
        let contact_order = right
            .cross_unit_contact_count
            .cmp(&left.cross_unit_contact_count);
        match strategy {
            AnchorStrategy::TopPressure => contact_order
                .then_with(|| right.contact_weight_sum.cmp(&left.contact_weight_sum))
                .then(capacity_order),
            AnchorStrategy::SlackHalo => capacity_order
                .then_with(|| right.contact_weight_sum.cmp(&left.contact_weight_sum))
                .then(contact_order),
        }
        .then_with(|| right.g27_anchor_pressure.cmp(&left.g27_anchor_pressure))
        .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
}

fn conclusion(
    posture: G27SameFieldPressureInterfacePosture,
    top: Option<&G27SameFieldPressureInterfaceCandidate>,
) -> String {
    match (posture, top) {
        (
            G27SameFieldPressureInterfacePosture::FundFusedPricingAfterWeightedCertificateReplay,
            Some(candidate),
        ) => {
            format!(
                "fund same-field pricing after weighted-certificate replay: G27 vertex {} anchored to W vertex {} creates {} exact cross-unit contacts and {} G27-unpriced weighted capacity against the 51749 lift gap",
                candidate.g27_anchor(),
                candidate.w_anchor(),
                candidate.cross_unit_contact_count(),
                candidate.g27_unpriced_contact_weight_sum()
            )
        }
        _ => "retire top-decile same-field interface search at this certificate depth: no high-weight W anchor creates enough G27-unpriced exact-contact capacity with high-pressure G27 anchors to clear the 51749 lift numerator"
            .to_string(),
    }
}

fn payload(
    searched_anchor_pairs: usize,
    candidates: &[G27SameFieldPressureInterfaceCandidate],
    posture: G27SameFieldPressureInterfacePosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_same_field_pressure_interface.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "searched_anchor_pairs",
            searched_anchor_pairs as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "lift_gap_weight_numerator",
            LIFT_GAP_WEIGHT_NUMERATOR as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}
