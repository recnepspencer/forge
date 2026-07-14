use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_escape_loop::{
    run_g27_pressure_escape_hypothesis_iterations_checked, G27EscapeHypothesisIteration,
    G27EscapeHypothesisIterationKind, G27PressureEscapeHypothesisRun,
};

const G27_ISOMETRIES: &str = include_str!("g27_geometric_fractional/g27_isometries.txt");
const G27_VERTEX_COUNT: usize = 27;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27IsometryLeadDetail {
    row_index: usize,
    domain_vertices: Vec<String>,
    image_vertices: Vec<String>,
    mapping_pairs: Vec<(String, String)>,
}

impl G27IsometryLeadDetail {
    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn domain_vertices(&self) -> &[String] {
        &self.domain_vertices
    }

    pub fn image_vertices(&self) -> &[String] {
        &self.image_vertices
    }

    pub fn mapping_pairs(&self) -> &[(String, String)] {
        &self.mapping_pairs
    }

    fn stable_token(&self) -> String {
        let pairs = self
            .mapping_pairs
            .iter()
            .map(|(source, target)| format!("{source}->{target}"))
            .collect::<Vec<_>>()
            .join(",");
        format!("g27_isometry_row{}:[{pairs}]", self.row_index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27OutsideMoserMutationObligation {
    relation_to_break: String,
    coordinate_scope: String,
    preserve_requirement: String,
    falsification_condition: String,
}

impl G27OutsideMoserMutationObligation {
    pub fn relation_to_break(&self) -> &str {
        &self.relation_to_break
    }

    pub fn coordinate_scope(&self) -> &str {
        &self.coordinate_scope
    }

    pub fn preserve_requirement(&self) -> &str {
        &self.preserve_requirement
    }

    pub fn falsification_condition(&self) -> &str {
        &self.falsification_condition
    }

    pub fn requires_outside_moser_geometry(&self) -> bool {
        self.coordinate_scope.contains("outside_moser")
    }

    fn stable_token(&self) -> String {
        format!(
            "break:{}:scope:{}:preserve:{}:falsify:{}",
            self.relation_to_break,
            self.coordinate_scope,
            self.preserve_requirement,
            self.falsification_condition
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PressureEscapeLeadReport {
    core: HadwigerArtifactCore,
    source_run: G27PressureEscapeHypothesisRun,
    best_iteration: G27EscapeHypothesisIteration,
    isometry_detail: G27IsometryLeadDetail,
    mutation_obligation: G27OutsideMoserMutationObligation,
}

impl G27PressureEscapeLeadReport {
    pub fn source_run(&self) -> &G27PressureEscapeHypothesisRun {
        &self.source_run
    }

    pub fn best_iteration(&self) -> &G27EscapeHypothesisIteration {
        &self.best_iteration
    }

    pub fn isometry_detail(&self) -> &G27IsometryLeadDetail {
        &self.isometry_detail
    }

    pub fn mutation_obligation(&self) -> &G27OutsideMoserMutationObligation {
        &self.mutation_obligation
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27PressureEscapeLeadReport, core);

pub fn materialize_g27_pressure_escape_lead_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27PressureEscapeLeadReport, G27GeometricFractionalError> {
    let source_run = run_g27_pressure_escape_hypothesis_iterations_checked(handle)?;
    let best_iteration = source_run.best_iteration().clone();
    if best_iteration.kind() != G27EscapeHypothesisIterationKind::IsometryBreaker {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_best_iteration_kind",
        });
    }
    let row_index =
        best_iteration
            .target_isometry_row()
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "g27_best_iteration_row",
            })?;
    let isometry_detail = parse_isometry_row(row_index)?;
    let mutation_obligation = mutation_obligation(&isometry_detail);
    let core = artifact_core(
        HadwigerArtifactKind::G27PressureEscapeLeadReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_pressure_escape_lead_report".to_string(),
        },
        vec![source_run.reference()],
        lead_payload(&best_iteration, &isometry_detail, &mutation_obligation),
    )?;
    Ok(G27PressureEscapeLeadReport {
        core,
        source_run,
        best_iteration,
        isometry_detail,
        mutation_obligation,
    })
}

fn parse_isometry_row(
    row_index: usize,
) -> Result<G27IsometryLeadDetail, G27GeometricFractionalError> {
    let row = G27_ISOMETRIES.lines().nth(row_index).ok_or(
        G27GeometricFractionalError::MalformedData {
            source: "g27_isometry_row_index",
        },
    )?;
    let values = row
        .split_whitespace()
        .map(|part| {
            part.parse::<isize>()
                .map_err(|_| G27GeometricFractionalError::MalformedData {
                    source: "g27_isometry_row_value",
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if values.len() != G27_VERTEX_COUNT {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_isometry_row_width",
        });
    }
    let mut mapping_pairs = Vec::new();
    for (source, target) in values.into_iter().enumerate() {
        if target >= 0 {
            mapping_pairs.push(((source + 1).to_string(), (target as usize + 1).to_string()));
        }
    }
    if mapping_pairs.is_empty() {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_empty_isometry_mapping",
        });
    }
    let domain_vertices = mapping_pairs
        .iter()
        .map(|(source, _)| source.clone())
        .collect();
    let image_vertices = mapping_pairs
        .iter()
        .map(|(_, target)| target.clone())
        .collect();
    Ok(G27IsometryLeadDetail {
        row_index,
        domain_vertices,
        image_vertices,
        mapping_pairs,
    })
}

fn mutation_obligation(row: &G27IsometryLeadDetail) -> G27OutsideMoserMutationObligation {
    G27OutsideMoserMutationObligation {
        relation_to_break: format!("retained_isometry_row_{}", row.row_index()),
        coordinate_scope: "outside_moser_lattice_or_ring_exact_algebraic_geometry".to_string(),
        preserve_requirement: format!(
            "preserve G27 unit-distance replay while changing relation {}",
            row.stable_token()
        ),
        falsification_condition: "retire this lead if bounded exact search finds no outside-Moser anchor that preserves checked unit edges and changes the row slack"
            .to_string(),
    }
}

fn lead_payload(
    best_iteration: &G27EscapeHypothesisIteration,
    row: &G27IsometryLeadDetail,
    obligation: &G27OutsideMoserMutationObligation,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_escape_lead.v1"),
        HadwigerArtifactPayloadEntry::text("best_iteration", best_iteration.stable_token()),
        HadwigerArtifactPayloadEntry::text("isometry_detail", row.stable_token()),
        HadwigerArtifactPayloadEntry::text("mutation_obligation", obligation.stable_token()),
    ]
}
