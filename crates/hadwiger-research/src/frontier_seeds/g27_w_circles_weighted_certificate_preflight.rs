use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{
    clique_cover_weight_upper_bound, empty_words, greedy_independent_witness, set_bit, BitWords,
};
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_bound;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};

const TARGET_WEIGHT: i128 = 512_933;
const RETIRE_MARGIN: i128 = 1_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesWeightedCertificatePreflightStatus {
    RootCertificateAtPublishedBound,
    RootCutsNearMissNeedsRationalDualOrBranchProof,
    RetiredCheapRootCertificate,
    RetiredCliqueEnumerationCap,
}

impl G27WCirclesWeightedCertificatePreflightStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RootCertificateAtPublishedBound => "root_certificate_at_published_bound",
            Self::RootCutsNearMissNeedsRationalDualOrBranchProof => {
                "root_cuts_near_miss_needs_rational_dual_or_branch_proof"
            }
            Self::RetiredCheapRootCertificate => "retired_cheap_root_certificate",
            Self::RetiredCliqueEnumerationCap => "retired_clique_enumeration_cap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesWeightedCertificatePreflightReport {
    core: HadwigerArtifactCore,
    vertex_count: usize,
    edge_count: usize,
    weight_count: usize,
    weight_sum: i128,
    target_weight: i128,
    greedy_witness_weight: i128,
    greedy_witness_size: usize,
    clique_cover_upper_bound: i128,
    edge_lp_upper_bound: i128,
    clique_lp_upper_bound: i128,
    odd_cycle_lp_upper_bound: i128,
    odd_cycle_cut_count: usize,
    odd_cycle_round_count: usize,
    best_odd_cycle_violation_ppm: i128,
    maximal_clique_count: usize,
    maximal_clique_cap_hit: bool,
    largest_clique_size: usize,
    status: G27WCirclesWeightedCertificatePreflightStatus,
    conclusion: String,
}

impl G27WCirclesWeightedCertificatePreflightReport {
    pub fn shape_summary(&self) -> (usize, usize, usize, i128, i128) {
        (
            self.vertex_count,
            self.edge_count,
            self.weight_count,
            self.weight_sum,
            self.target_weight,
        )
    }

    pub fn bound_summary(&self) -> (i128, usize, i128, i128, i128, i128) {
        (
            self.greedy_witness_weight,
            self.greedy_witness_size,
            self.clique_cover_upper_bound,
            self.edge_lp_upper_bound,
            self.clique_lp_upper_bound,
            self.odd_cycle_lp_upper_bound,
        )
    }

    pub fn cut_summary(&self) -> (usize, usize, i128, usize, bool, usize) {
        (
            self.odd_cycle_cut_count,
            self.odd_cycle_round_count,
            self.best_odd_cycle_violation_ppm,
            self.maximal_clique_count,
            self.maximal_clique_cap_hit,
            self.largest_clique_size,
        )
    }

    pub fn status(&self) -> G27WCirclesWeightedCertificatePreflightStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        self.status
            == G27WCirclesWeightedCertificatePreflightStatus::RootCertificateAtPublishedBound
    }
}

impl_hadwiger_artifact!(G27WCirclesWeightedCertificatePreflightReport, core);

pub fn preflight_g27_w_circles_weighted_certificate_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesWeightedCertificatePreflightReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w607_certificate_shape",
        });
    }
    let adjacency = adjacency_from_edges(EXPECTED_VERTEX_COUNT, &edges);
    let candidates = (0..EXPECTED_VERTEX_COUNT).collect::<Vec<_>>();
    let (greedy_witness_weight, greedy_witness) =
        greedy_independent_witness(&adjacency, &weights, &candidates);
    let clique_cover_upper_bound =
        clique_cover_weight_upper_bound(&adjacency, &weights, &candidates);
    let lp = stable_set_lp_relaxation_bound(&adjacency, &weights, &candidates)?;
    let status = status(&lp, clique_cover_upper_bound);
    let conclusion = conclusion(status, lp.odd_cycle_objective_ceiling);
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesWeightedCertificatePreflightReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_weighted_certificate_preflight".to_string(),
        },
        vec![source.reference()],
        payload(
            weights.iter().sum(),
            greedy_witness_weight,
            greedy_witness.len(),
            clique_cover_upper_bound,
            &lp,
            status,
            &conclusion,
        ),
    )?;
    Ok(G27WCirclesWeightedCertificatePreflightReport {
        core,
        vertex_count: EXPECTED_VERTEX_COUNT,
        edge_count: EXPECTED_EDGE_COUNT,
        weight_count: weights.len(),
        weight_sum: weights.iter().sum(),
        target_weight: TARGET_WEIGHT,
        greedy_witness_weight,
        greedy_witness_size: greedy_witness.len(),
        clique_cover_upper_bound,
        edge_lp_upper_bound: lp.objective_ceiling,
        clique_lp_upper_bound: lp.clique_objective_ceiling,
        odd_cycle_lp_upper_bound: lp.odd_cycle_objective_ceiling,
        odd_cycle_cut_count: lp.odd_cycle_cut_count,
        odd_cycle_round_count: lp.odd_cycle_round_count,
        best_odd_cycle_violation_ppm: lp.best_odd_cycle_violation_ppm,
        maximal_clique_count: lp.maximal_clique_count,
        maximal_clique_cap_hit: lp.maximal_clique_cap_hit,
        largest_clique_size: lp.largest_clique_size,
        status,
        conclusion,
    })
}

fn adjacency_from_edges(
    vertex_count: usize,
    edges: &std::collections::BTreeSet<(usize, usize)>,
) -> Vec<BitWords> {
    let mut adjacency = vec![empty_words(); vertex_count];
    for (left, right) in edges {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    adjacency
}

fn status(
    lp: &super::g27_same_field_lp_relaxation::StableSetLpRelaxationBound,
    clique_cover_upper_bound: i128,
) -> G27WCirclesWeightedCertificatePreflightStatus {
    if lp.maximal_clique_cap_hit {
        return G27WCirclesWeightedCertificatePreflightStatus::RetiredCliqueEnumerationCap;
    }
    if clique_cover_upper_bound <= TARGET_WEIGHT || lp.odd_cycle_objective_ceiling <= TARGET_WEIGHT
    {
        return G27WCirclesWeightedCertificatePreflightStatus::RootCertificateAtPublishedBound;
    }
    if lp.odd_cycle_objective_ceiling <= TARGET_WEIGHT + RETIRE_MARGIN {
        G27WCirclesWeightedCertificatePreflightStatus::RootCutsNearMissNeedsRationalDualOrBranchProof
    } else {
        G27WCirclesWeightedCertificatePreflightStatus::RetiredCheapRootCertificate
    }
}

fn conclusion(
    status: G27WCirclesWeightedCertificatePreflightStatus,
    odd_cycle_upper_bound: i128,
) -> String {
    match status {
        G27WCirclesWeightedCertificatePreflightStatus::RootCertificateAtPublishedBound => {
            "cheap root certificate reaches the published W_circles_607 weighted alpha bound; rational replay is required before theorem authority".to_string()
        }
        G27WCirclesWeightedCertificatePreflightStatus::RootCutsNearMissNeedsRationalDualOrBranchProof => {
            format!("cheap root cuts are within {RETIRE_MARGIN} of target but stop at {odd_cycle_upper_bound}; require rational dual replay or a branch proof")
        }
        G27WCirclesWeightedCertificatePreflightStatus::RetiredCliqueEnumerationCap => {
            "maximal clique enumeration hit the predeclared cap; retire clique-complete root certification".to_string()
        }
        G27WCirclesWeightedCertificatePreflightStatus::RetiredCheapRootCertificate => {
            format!("cheap root cuts stop at {odd_cycle_upper_bound}, more than {RETIRE_MARGIN} above {TARGET_WEIGHT}; require imported proof artifacts or branch certificates")
        }
    }
}

fn payload(
    weight_sum: i128,
    greedy_weight: i128,
    greedy_size: usize,
    clique_cover_upper: i128,
    lp: &super::g27_same_field_lp_relaxation::StableSetLpRelaxationBound,
    status: G27WCirclesWeightedCertificatePreflightStatus,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.w607_cert_preflight.v1"),
        HadwigerArtifactPayloadEntry::unsigned("vertex_count", EXPECTED_VERTEX_COUNT as u128),
        HadwigerArtifactPayloadEntry::unsigned("edge_count", EXPECTED_EDGE_COUNT as u128),
        HadwigerArtifactPayloadEntry::unsigned("weight_sum", weight_sum as u128),
        HadwigerArtifactPayloadEntry::unsigned("target_weight", TARGET_WEIGHT as u128),
        HadwigerArtifactPayloadEntry::unsigned("greedy_witness_weight", greedy_weight as u128),
        HadwigerArtifactPayloadEntry::unsigned("greedy_witness_size", greedy_size as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "clique_cover_upper_bound",
            clique_cover_upper as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("edge_lp_upper_bound", lp.objective_ceiling as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "clique_lp_upper_bound",
            lp.clique_objective_ceiling as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "odd_cycle_lp_upper_bound",
            lp.odd_cycle_objective_ceiling as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "odd_cycle_cut_count",
            lp.odd_cycle_cut_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "odd_cycle_round_count",
            lp.odd_cycle_round_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("status", status.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}
