use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::{
    reproduce_g27_geometric_fractional_witness_checked, G27GeometricFractionalError,
};
use super::g27_geometric_fractional_slack_analysis::G27GeometricFractionalPressureReport;

const REQUIRED_ITERATIONS: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum G27EscapeHypothesisIterationKind {
    OutsideFieldClamp,
    TightPairBridge,
    IsometryBreaker,
    TightAtomTransversal,
    NonMoserCoreGraft,
}

impl G27EscapeHypothesisIterationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OutsideFieldClamp => "outside_field_clamp",
            Self::TightPairBridge => "tight_pair_bridge",
            Self::IsometryBreaker => "isometry_breaker",
            Self::TightAtomTransversal => "tight_atom_transversal",
            Self::NonMoserCoreGraft => "non_moser_core_graft",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27EscapeHypothesisIteration {
    iteration_index: usize,
    kind: G27EscapeHypothesisIterationKind,
    target_vertices: Vec<String>,
    target_isometry_row: Option<usize>,
    expected_information_gain: String,
    escape_requirement: String,
    suppression_basis: String,
    score: usize,
}

impl G27EscapeHypothesisIteration {
    pub fn iteration_index(&self) -> usize {
        self.iteration_index
    }

    pub fn kind(&self) -> G27EscapeHypothesisIterationKind {
        self.kind
    }

    pub fn target_vertices(&self) -> &[String] {
        &self.target_vertices
    }

    pub fn target_isometry_row(&self) -> Option<usize> {
        self.target_isometry_row
    }

    pub fn expected_information_gain(&self) -> &str {
        &self.expected_information_gain
    }

    pub fn escape_requirement(&self) -> &str {
        &self.escape_requirement
    }

    pub fn suppression_basis(&self) -> &str {
        &self.suppression_basis
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "iteration{}:{}:vertices[{}]:row{:?}:score{}:{}:{}:{}",
            self.iteration_index,
            self.kind.as_str(),
            self.target_vertices.join(","),
            self.target_isometry_row,
            self.score,
            self.expected_information_gain,
            self.escape_requirement,
            self.suppression_basis
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PressureEscapeHypothesisRun {
    core: HadwigerArtifactCore,
    iterations: Vec<G27EscapeHypothesisIteration>,
    best_iteration_index: usize,
    falsification_report: Option<String>,
}

impl G27PressureEscapeHypothesisRun {
    pub fn iterations(&self) -> &[G27EscapeHypothesisIteration] {
        &self.iterations
    }

    pub fn best_iteration(&self) -> &G27EscapeHypothesisIteration {
        &self.iterations[self.best_iteration_index]
    }

    pub fn falsification_report(&self) -> Option<&str> {
        self.falsification_report.as_deref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27PressureEscapeHypothesisRun, core);

pub fn run_g27_pressure_escape_hypothesis_iterations_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27PressureEscapeHypothesisRun, G27GeometricFractionalError> {
    let reproduction = reproduce_g27_geometric_fractional_witness_checked(handle)?;
    let pressure = reproduction.dual_replay().pressure_report();
    let iterations = build_iterations(pressure)?;
    let best_iteration_index = iterations
        .iter()
        .enumerate()
        .max_by_key(|(_, iteration)| iteration.score())
        .map(|(index, _)| index)
        .unwrap_or(0);
    let core = artifact_core(
        HadwigerArtifactKind::FrontierExplorationRunReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_pressure_escape_hypothesis_run".to_string(),
        },
        vec![
            reproduction.seed_import().seed_artifact().reference(),
            reproduction.evaluation().reference(),
        ],
        run_payload(pressure, &iterations, best_iteration_index),
    )?;
    Ok(G27PressureEscapeHypothesisRun {
        core,
        iterations,
        best_iteration_index,
        falsification_report: None,
    })
}

fn build_iterations(
    pressure: &G27GeometricFractionalPressureReport,
) -> Result<Vec<G27EscapeHypothesisIteration>, G27GeometricFractionalError> {
    if pressure.top_vertices().len() < 5 || pressure.top_vertex_pairs().len() < 2 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "pressure_report",
        });
    }
    let top_pair = &pressure.top_vertex_pairs()[0];
    let second_pair = &pressure.top_vertex_pairs()[1];
    let top_row = pressure.top_non_singleton_isometry_rows().first().ok_or(
        G27GeometricFractionalError::MalformedData {
            source: "pressure_non_singleton_rows",
        },
    )?;
    let iterations = vec![
        iteration(
            1,
            G27EscapeHypothesisIterationKind::OutsideFieldClamp,
            vec![top_pair.left_vertex_label(), top_pair.right_vertex_label()],
            None,
            "test whether a new exact point outside the Moser basis can cut the most reused tight pair",
            "requires algebraic coordinate replay outside the retained Moser-lattice basis",
            top_pair.tight_atom_co_participation() * pressure.tight_atom_count(),
        ),
        iteration(
            2,
            G27EscapeHypothesisIterationKind::TightPairBridge,
            vec![second_pair.left_vertex_label(), second_pair.right_vertex_label()],
            None,
            "bridge a second tight pair to detect whether pressure is localized or distributed",
            "requires non-Moser bridge geometry; Moser-only bridges are suppressed",
            second_pair.tight_atom_co_participation() * pressure.tight_atom_count(),
        ),
        iteration(
            3,
            G27EscapeHypothesisIterationKind::IsometryBreaker,
            vec![pressure.top_vertices()[0].vertex_label(), pressure.top_vertices()[1].vertex_label()],
            Some(top_row.row_index()),
            "break the highest tight-touch non-singleton isometry relation and measure geometric-fractional slack response",
            "requires an outside-Moser mutation that exits a nontrivial congruent-subset family rather than relabeling it",
            top_row.tight_atom_touches() * top_row.sparse_touches() * top_row.mapping_size(),
        ),
        iteration(
            4,
            G27EscapeHypothesisIterationKind::TightAtomTransversal,
            pressure
                .top_vertices()
                .iter()
                .take(5)
                .map(|row| row.vertex_label())
                .collect(),
            None,
            "target a transversal over the five highest tight-atom participation vertices",
            "requires exact outside-field anchor with unit constraints to all selected pressure vertices",
            pressure
                .top_vertices()
                .iter()
                .take(5)
                .map(|row| row.tight_atom_participation())
                .sum::<usize>()
                * 5,
        ),
        iteration(
            5,
            G27EscapeHypothesisIterationKind::NonMoserCoreGraft,
            vec![top_pair.left_vertex_label(), top_pair.right_vertex_label(), second_pair.left_vertex_label()],
            None,
            "graft a retained non-Moser high-fractional core onto the tight skeleton",
            "requires a retained outside-Moser core certificate before checker execution",
            pressure.tight_atom_count() + top_pair.tight_atom_co_participation()
                + second_pair.tight_atom_co_participation(),
        ),
    ];
    if iterations.len() == REQUIRED_ITERATIONS {
        Ok(iterations)
    } else {
        Err(G27GeometricFractionalError::MalformedData {
            source: "escape_iteration_count",
        })
    }
}

fn iteration(
    iteration_index: usize,
    kind: G27EscapeHypothesisIterationKind,
    vertices: Vec<&str>,
    target_isometry_row: Option<usize>,
    information_gain: &str,
    escape_requirement: &str,
    score: usize,
) -> G27EscapeHypothesisIteration {
    G27EscapeHypothesisIteration {
        iteration_index,
        kind,
        target_vertices: vertices.into_iter().map(ToOwned::to_owned).collect(),
        target_isometry_row,
        expected_information_gain: information_gain.to_string(),
        escape_requirement: escape_requirement.to_string(),
        suppression_basis: "suppressed if retained geometry remains inside Moser lattice/ring cap"
            .to_string(),
        score,
    }
}

fn run_payload(
    pressure: &G27GeometricFractionalPressureReport,
    iterations: &[G27EscapeHypothesisIteration],
    best_iteration_index: usize,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_escape_loop.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "tight_atom_count",
            pressure.tight_atom_count() as u128,
        ),
        HadwigerArtifactPayloadEntry::text("pressure_report", pressure.stable_token()),
        HadwigerArtifactPayloadEntry::unsigned("iteration_count", iterations.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "best_iteration_index",
            (best_iteration_index + 1) as u128,
        ),
    ];
    for iteration in iterations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "iteration",
            iteration.stable_token(),
        ));
    }
    payload
}
