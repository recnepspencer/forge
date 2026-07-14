use num_bigint::BigInt;
use num_traits::Zero;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::artifact_core;
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_atom_slacks;
use super::g27_same_field_fixed_dual_pricing_payload::{conclusion, payload};
use super::g27_same_field_fixed_dual_pricing_support::{
    empty_words, set_bit, threshold_mwis_bracket, BitWords,
};
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_bound;
use super::g27_same_field_pressure_interface_support::{approx_unit_distance, g27_points};
use super::g27_w_circles_exact_geometry_audit::audit_g27_w_circles_607_exact_geometry_checked;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, parse_w_vertices, squared_distance,
    EXPECTED_VERTEX_COUNT,
};

const G27_ANCHOR_INDEX: usize = 22;
const W_ANCHOR_INDEX: usize = 253;
const W_GLOBAL_ALPHA_WEIGHT: i128 = 512_933;
const PRICED_ATOM_LIMIT: usize = 10;

pub type G27FixedDualLpSummary = (
    i128,
    i128,
    i128,
    usize,
    usize,
    i128,
    usize,
    usize,
    bool,
    usize,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27FixedDualPricingPosture {
    FundMasterDualScaleAudit,
    RetiredMwisCollapse,
    NeedsStrongerMwisCertificate,
}

impl G27FixedDualPricingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundMasterDualScaleAudit => "fund_master_dual_scale_audit",
            Self::RetiredMwisCollapse => "retired_mwis_collapse",
            Self::NeedsStrongerMwisCertificate => "needs_stronger_mwis_certificate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27FixedDualPricingChannel {
    atom_mask: u32,
    atom_size: usize,
    g27_slack_numerator: BigInt,
    compatible_w_vertex_count: usize,
    excluded_w_vertex_count: usize,
    mwis_weight_lower_bound: i128,
    mwis_upper_bound: i128,
    mwis_certified_exact: bool,
    component_count: usize,
    largest_component_size: usize,
    exact_component_count: usize,
    lp_relaxation_upper_bound: i128,
    clique_lp_relaxation_upper_bound: i128,
    lp_relaxation_edge_constraints: usize,
    clique_lp_constraint_count: usize,
    odd_cycle_lp_relaxation_upper_bound: i128,
    odd_cycle_cut_count: usize,
    odd_cycle_round_count: usize,
    best_odd_cycle_violation_ppm: i128,
    maximal_clique_count: usize,
    maximal_clique_cap_hit: bool,
    largest_clique_size: usize,
    mwis_witness_vertices: Vec<usize>,
}

impl G27FixedDualPricingChannel {
    pub fn compatibility_summary(&self) -> (usize, usize) {
        (self.compatible_w_vertex_count, self.excluded_w_vertex_count)
    }

    pub fn mwis_summary(&self) -> (i128, i128, bool, usize, usize, usize, usize) {
        (
            self.mwis_weight_lower_bound,
            self.mwis_upper_bound,
            self.mwis_certified_exact,
            self.component_count,
            self.largest_component_size,
            self.exact_component_count,
            self.mwis_witness_vertices.len(),
        )
    }

    pub fn lp_summary(&self) -> G27FixedDualLpSummary {
        (
            self.lp_relaxation_upper_bound,
            self.clique_lp_relaxation_upper_bound,
            self.odd_cycle_lp_relaxation_upper_bound,
            self.odd_cycle_cut_count,
            self.odd_cycle_round_count,
            self.best_odd_cycle_violation_ppm,
            self.clique_lp_constraint_count,
            self.maximal_clique_count,
            self.maximal_clique_cap_hit,
            self.largest_clique_size,
        )
    }

    pub(super) fn stable_token(&self) -> String {
        let witness_prefix = self
            .mwis_witness_vertices
            .iter()
            .take(16)
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("-");
        format!(
            "mask{:x}:size{}:slack{}:compatible{}:excluded{}:mwis{}:upper{}:exact{}:components{}:largest{}:exact_components{}:lp_upper{}:clique_lp_upper{}:odd_cycle_lp_upper{}:lp_edges{}:clique_cuts{}:odd_cycle_cuts{}:odd_cycle_rounds{}:best_odd_cycle_ppm{}:max_cliques{}:cap{}:max_clique_size{}:witness{}",
            self.atom_mask,
            self.atom_size,
            self.g27_slack_numerator,
            self.compatible_w_vertex_count,
            self.excluded_w_vertex_count,
            self.mwis_weight_lower_bound,
            self.mwis_upper_bound,
            self.mwis_certified_exact,
            self.component_count,
            self.largest_component_size,
            self.exact_component_count,
            self.lp_relaxation_upper_bound,
            self.clique_lp_relaxation_upper_bound,
            self.odd_cycle_lp_relaxation_upper_bound,
            self.lp_relaxation_edge_constraints,
            self.clique_lp_constraint_count,
            self.odd_cycle_cut_count,
            self.odd_cycle_round_count,
            self.best_odd_cycle_violation_ppm,
            self.maximal_clique_count,
            self.maximal_clique_cap_hit,
            self.largest_clique_size,
            witness_prefix
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27SameFieldFixedDualPricingReport {
    core: HadwigerArtifactCore,
    g27_anchor: usize,
    w_anchor: usize,
    priced_tight_atom_count: usize,
    g27_slack_denominator_digits: usize,
    w_global_alpha_weight: i128,
    top_channels: Vec<G27FixedDualPricingChannel>,
    posture: G27FixedDualPricingPosture,
    conclusion: String,
}

impl G27SameFieldFixedDualPricingReport {
    pub fn g27_anchor(&self) -> usize {
        self.g27_anchor
    }

    pub fn w_anchor(&self) -> usize {
        self.w_anchor
    }

    pub fn priced_tight_atom_count(&self) -> usize {
        self.priced_tight_atom_count
    }

    pub fn w_global_alpha_weight(&self) -> i128 {
        self.w_global_alpha_weight
    }

    pub fn top_channels(&self) -> &[G27FixedDualPricingChannel] {
        &self.top_channels
    }

    pub fn posture(&self) -> G27FixedDualPricingPosture {
        self.posture
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27SameFieldFixedDualPricingReport, core);

pub fn price_g27_w_circles_fixed_dual_channels_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27SameFieldFixedDualPricingReport, G27GeometricFractionalError> {
    let exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let w_points = parse_w_vertices()?;
    let w_weights = parse_w_integer_weights()?;
    let w_adjacency = w_adjacency()?;
    let cross_conflicts = cross_conflict_masks(&g_points, &w_points);
    let (g27_slack_denominator, atom_slacks) = retained_g27_atom_slacks()?;
    let mut tight_atoms = atom_slacks
        .into_iter()
        .filter(|(_, slack)| slack.is_zero())
        .collect::<Vec<_>>();
    tight_atoms.sort_by(|left, right| {
        contact_incidence_weight(right.0, &cross_conflicts, &w_weights)
            .cmp(&contact_incidence_weight(
                left.0,
                &cross_conflicts,
                &w_weights,
            ))
            .then_with(|| left.0.cmp(&right.0))
    });
    let mut channels = Vec::new();
    for (atom_mask, slack) in tight_atoms.iter().take(PRICED_ATOM_LIMIT) {
        let candidates = compatible_w_candidates(*atom_mask, &cross_conflicts);
        let excluded_w_vertex_count = EXPECTED_VERTEX_COUNT - 1 - candidates.len();
        let (
            mwis_weight_lower_bound,
            mwis_upper_bound,
            mwis_witness_vertices,
            mwis_certified_exact,
            component_count,
            largest_component_size,
            exact_component_count,
        ) = price_compatible_w_subgraph(&w_adjacency, &w_weights, &candidates);
        let lp_relaxation = stable_set_lp_relaxation_bound(&w_adjacency, &w_weights, &candidates)?;
        channels.push(G27FixedDualPricingChannel {
            atom_mask: *atom_mask,
            atom_size: atom_mask.count_ones() as usize,
            g27_slack_numerator: slack.clone(),
            compatible_w_vertex_count: candidates.len(),
            excluded_w_vertex_count,
            mwis_weight_lower_bound,
            mwis_upper_bound,
            mwis_certified_exact,
            component_count,
            largest_component_size,
            exact_component_count,
            lp_relaxation_upper_bound: lp_relaxation.objective_ceiling,
            clique_lp_relaxation_upper_bound: lp_relaxation.clique_objective_ceiling,
            lp_relaxation_edge_constraints: lp_relaxation.edge_constraint_count,
            clique_lp_constraint_count: lp_relaxation.clique_constraint_count,
            odd_cycle_lp_relaxation_upper_bound: lp_relaxation.odd_cycle_objective_ceiling,
            odd_cycle_cut_count: lp_relaxation.odd_cycle_cut_count,
            odd_cycle_round_count: lp_relaxation.odd_cycle_round_count,
            best_odd_cycle_violation_ppm: lp_relaxation.best_odd_cycle_violation_ppm,
            maximal_clique_count: lp_relaxation.maximal_clique_count,
            maximal_clique_cap_hit: lp_relaxation.maximal_clique_cap_hit,
            largest_clique_size: lp_relaxation.largest_clique_size,
            mwis_witness_vertices: mwis_witness_vertices
                .into_iter()
                .map(|vertex| vertex + 1)
                .collect(),
        });
    }
    channels.sort_by(|left, right| {
        right
            .mwis_weight_lower_bound
            .cmp(&left.mwis_weight_lower_bound)
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let posture = pricing_posture(&channels);
    let conclusion = conclusion(posture, channels.first());
    let core = artifact_core(
        HadwigerArtifactKind::G27SameFieldPressureInterfaceSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_fixed_dual_pricing".to_string(),
        },
        vec![exact_geometry.reference()],
        payload(
            tight_atoms.len().min(PRICED_ATOM_LIMIT),
            g27_slack_denominator.to_string().len(),
            W_GLOBAL_ALPHA_WEIGHT,
            &channels,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27SameFieldFixedDualPricingReport {
        core,
        g27_anchor: G27_ANCHOR_INDEX + 1,
        w_anchor: W_ANCHOR_INDEX + 1,
        priced_tight_atom_count: tight_atoms.len().min(PRICED_ATOM_LIMIT),
        g27_slack_denominator_digits: g27_slack_denominator.to_string().len(),
        w_global_alpha_weight: W_GLOBAL_ALPHA_WEIGHT,
        top_channels: channels,
        posture,
        conclusion,
    })
}

fn w_adjacency() -> Result<Vec<BitWords>, G27GeometricFractionalError> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in parse_w_retained_edges(EXPECTED_VERTEX_COUNT)? {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    Ok(adjacency)
}

fn cross_conflict_masks(
    g_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    w_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
) -> Vec<u32> {
    let translation = g_points[G27_ANCHOR_INDEX].sub(w_points[W_ANCHOR_INDEX]);
    w_points
        .iter()
        .enumerate()
        .map(|(w_index, w_point)| {
            let translated_w = w_point.add(translation);
            let mut mask = 0u32;
            for (g_index, g_point) in g_points.iter().enumerate() {
                if g_index == G27_ANCHOR_INDEX && w_index == W_ANCHOR_INDEX {
                    continue;
                }
                if approx_unit_distance(*g_point, translated_w)
                    && squared_distance(*g_point, translated_w).is_one()
                {
                    mask |= 1u32 << g_index;
                }
            }
            mask
        })
        .collect()
}

fn compatible_w_candidates(atom_mask: u32, cross_conflicts: &[u32]) -> Vec<usize> {
    cross_conflicts
        .iter()
        .enumerate()
        .filter_map(|(w_index, conflict_mask)| {
            if w_index != W_ANCHOR_INDEX && conflict_mask & atom_mask == 0 {
                Some(w_index)
            } else {
                None
            }
        })
        .collect()
}

fn price_compatible_w_subgraph(
    w_adjacency: &[BitWords],
    w_weights: &[i128],
    candidates: &[usize],
) -> (i128, i128, Vec<usize>, bool, usize, usize, usize) {
    let bracket = threshold_mwis_bracket(w_adjacency, w_weights, candidates, W_GLOBAL_ALPHA_WEIGHT);
    (
        bracket.lower_bound,
        bracket.upper_bound,
        bracket.witness_vertices,
        bracket.certified_exact,
        bracket.component_count,
        bracket.largest_component_size,
        bracket.exact_component_count,
    )
}

fn contact_incidence_weight(atom_mask: u32, cross_conflicts: &[u32], w_weights: &[i128]) -> i128 {
    cross_conflicts
        .iter()
        .enumerate()
        .filter(|(_, conflict_mask)| *conflict_mask & atom_mask != 0)
        .map(|(w_index, _)| w_weights[w_index])
        .sum()
}

fn pricing_posture(channels: &[G27FixedDualPricingChannel]) -> G27FixedDualPricingPosture {
    if channels
        .iter()
        .any(|channel| channel.mwis_weight_lower_bound >= W_GLOBAL_ALPHA_WEIGHT)
    {
        return G27FixedDualPricingPosture::FundMasterDualScaleAudit;
    }
    if channels
        .iter()
        .all(|channel| channel.mwis_upper_bound < W_GLOBAL_ALPHA_WEIGHT)
    {
        return G27FixedDualPricingPosture::RetiredMwisCollapse;
    }
    G27FixedDualPricingPosture::NeedsStrongerMwisCertificate
}
