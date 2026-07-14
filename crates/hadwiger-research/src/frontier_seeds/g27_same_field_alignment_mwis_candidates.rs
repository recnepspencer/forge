use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_tight_atom_masks;
use super::g27_same_field_pressure_interface_support::g27_points;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_vertices, squared_distance, WExactPoint,
};

const UNIT_DISTANCE_FLOAT_TOLERANCE: f64 = 1e-9;
const G_TOP_DECILE_LIMIT: usize = 3;
const W_TOP_DECILE_LIMIT: usize = 61;
const RETAINED_CANDIDATE_LIMIT: usize = 8;
const RETIRED_G27_ANCHOR: usize = 23;
const RETIRED_W_ANCHOR: usize = 254;

#[derive(Clone)]
pub(super) struct AlignmentCandidate {
    pub(super) g27_anchor: usize,
    pub(super) w_anchor: usize,
    pub(super) source_rank: usize,
    pub(super) source_label: String,
}

#[derive(Clone, Copy)]
enum AnchorStrategy {
    TopPressure,
    SlackHalo,
}

#[derive(Clone)]
struct ScoredCandidate {
    g27_anchor: usize,
    w_anchor: usize,
    g27_anchor_pressure: usize,
    w_anchor_weight: i128,
    contact_count: usize,
    contact_weight_sum: i128,
    priced_contact_count: usize,
    unpriced_contact_count: usize,
    unpriced_contact_weight_sum: i128,
}

pub(super) fn retained_alternate_alignments(
) -> Result<Vec<AlignmentCandidate>, G27GeometricFractionalError> {
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let g_approx = approx_points(&g_points);
    let g_pressures = g27_vertex_pressures(&retained_g27_tight_atom_masks()?);
    let w_points = parse_w_vertices()?;
    let w_approx = approx_points(&w_points);
    let w_weights = parse_w_integer_weights()?;
    let mut retained = Vec::new();
    for (label, strategy) in [
        ("top_pressure", AnchorStrategy::TopPressure),
        ("slack_halo", AnchorStrategy::SlackHalo),
    ] {
        let mut candidates = scored_candidates(
            strategy,
            &g_points,
            &g_approx,
            &g_pressures,
            &w_points,
            &w_approx,
            &w_weights,
        );
        sort_candidates(&mut candidates, strategy);
        for (rank, candidate) in candidates
            .into_iter()
            .take(RETAINED_CANDIDATE_LIMIT)
            .enumerate()
        {
            if (candidate.g27_anchor, candidate.w_anchor) == (RETIRED_G27_ANCHOR, RETIRED_W_ANCHOR)
            {
                continue;
            }
            if retained.iter().any(|old: &AlignmentCandidate| {
                old.g27_anchor == candidate.g27_anchor && old.w_anchor == candidate.w_anchor
            }) {
                continue;
            }
            retained.push(AlignmentCandidate {
                g27_anchor: candidate.g27_anchor,
                w_anchor: candidate.w_anchor,
                source_rank: rank + 1,
                source_label: label.to_string(),
            });
        }
    }
    Ok(retained)
}

fn scored_candidates(
    strategy: AnchorStrategy,
    g_points: &[WExactPoint],
    g_approx: &[(f64, f64)],
    g_pressures: &[usize; 27],
    w_points: &[WExactPoint],
    w_approx: &[(f64, f64)],
    w_weights: &[i128],
) -> Vec<ScoredCandidate> {
    let g_anchors = g_anchors(g_pressures, strategy);
    let w_anchors = top_w_anchors(w_weights);
    let mut candidates = Vec::new();
    for g_anchor in g_anchors {
        for w_anchor in &w_anchors {
            candidates.push(score_anchor_pair(
                g_anchor,
                *w_anchor,
                g_points,
                g_approx,
                g_pressures,
                w_points,
                w_approx,
                w_weights,
            ));
        }
    }
    candidates
}

fn score_anchor_pair(
    g_anchor: usize,
    w_anchor: usize,
    g_points: &[WExactPoint],
    g_approx: &[(f64, f64)],
    g_pressures: &[usize; 27],
    w_points: &[WExactPoint],
    w_approx: &[(f64, f64)],
    w_weights: &[i128],
) -> ScoredCandidate {
    let translation = g_points[g_anchor - 1].sub(w_points[w_anchor - 1]);
    let translation_approx = (
        g_approx[g_anchor - 1].0 - w_approx[w_anchor - 1].0,
        g_approx[g_anchor - 1].1 - w_approx[w_anchor - 1].1,
    );
    let mut contact_count = 0usize;
    let mut contact_weight_sum = 0i128;
    let mut priced_contact_count = 0usize;
    let mut unpriced_contact_count = 0usize;
    let mut unpriced_contact_weight_sum = 0i128;
    for (g_index, g_point) in g_points.iter().enumerate() {
        for (w_index, w_point) in w_points.iter().enumerate() {
            if g_index + 1 == g_anchor && w_index + 1 == w_anchor {
                continue;
            }
            if !approx_translated_unit_distance(
                g_approx[g_index],
                w_approx[w_index],
                translation_approx,
            ) {
                continue;
            }
            let translated_w = w_point.add(translation);
            if squared_distance(*g_point, translated_w).is_one() {
                contact_count += 1;
                contact_weight_sum += w_weights[w_index];
                if g_pressures[g_index] == 0 {
                    unpriced_contact_count += 1;
                    unpriced_contact_weight_sum += w_weights[w_index];
                } else {
                    priced_contact_count += 1;
                }
            }
        }
    }
    ScoredCandidate {
        g27_anchor: g_anchor,
        w_anchor,
        g27_anchor_pressure: g_pressures[g_anchor - 1],
        w_anchor_weight: w_weights[w_anchor - 1],
        contact_count,
        contact_weight_sum,
        priced_contact_count,
        unpriced_contact_count,
        unpriced_contact_weight_sum,
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
    let mut anchors = (1..=27).collect::<Vec<_>>();
    if let AnchorStrategy::TopPressure = strategy {
        anchors.sort_by(|left, right| {
            pressures[*right - 1]
                .cmp(&pressures[*left - 1])
                .then(left.cmp(right))
        });
        anchors.truncate(G_TOP_DECILE_LIMIT);
    }
    anchors
}

fn top_w_anchors(weights: &[i128]) -> Vec<usize> {
    let mut anchors = (1..=weights.len()).collect::<Vec<_>>();
    anchors.sort_by(|left, right| {
        weights[*right - 1]
            .cmp(&weights[*left - 1])
            .then(left.cmp(right))
    });
    anchors.truncate(W_TOP_DECILE_LIMIT);
    anchors
}

fn sort_candidates(candidates: &mut [ScoredCandidate], strategy: AnchorStrategy) {
    candidates.sort_by(|left, right| {
        let capacity_order = right
            .unpriced_contact_weight_sum
            .cmp(&left.unpriced_contact_weight_sum);
        let contact_order = right.contact_count.cmp(&left.contact_count);
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

fn approx_points(points: &[WExactPoint]) -> Vec<(f64, f64)> {
    points.iter().map(|point| point.approx()).collect()
}

fn approx_translated_unit_distance(
    g_point: (f64, f64),
    w_point: (f64, f64),
    translation: (f64, f64),
) -> bool {
    let dx = g_point.0 - (w_point.0 + translation.0);
    let dy = g_point.1 - (w_point.1 + translation.1);
    (dx.mul_add(dx, dy * dy) - 1.0).abs() <= UNIT_DISTANCE_FLOAT_TOLERANCE
}

impl ScoredCandidate {
    fn stable_token(&self) -> String {
        format!(
            "g{}:p{}:w{}:weight{}:contacts{}:contact_weight{}:g_priced{}:g_unpriced{}:g_unpriced_weight{}",
            self.g27_anchor,
            self.g27_anchor_pressure,
            self.w_anchor,
            self.w_anchor_weight,
            self.contact_count,
            self.contact_weight_sum,
            self.priced_contact_count,
            self.unpriced_contact_count,
            self.unpriced_contact_weight_sum
        )
    }
}
