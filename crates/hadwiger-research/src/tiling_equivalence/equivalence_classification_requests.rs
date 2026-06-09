use crate::conflict_graphs::{ConflictCoreExtractionReport, TilingConflictGraph};
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerCanonicalArtifact};
use crate::motif_language::TerminalForcingRelation;
use crate::periodic_patterns::{GeneratedPatternReplayChecked, PeriodicQuotientCell};
use crate::research_cockpit::TileEquivalenceWitness;

use super::equivalence_errors::{require_equivalence_non_empty, TilingEquivalenceError};
use super::equivalence_scopes::TilingEquivalenceScope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingCandidateEquivalenceRequest {
    equivalence_id: String,
    scope: TilingEquivalenceScope,
    left_reference: HadwigerArtifactReference,
    right_reference: HadwigerArtifactReference,
    left_basis_token: String,
    right_basis_token: String,
}

impl TilingCandidateEquivalenceRequest {
    pub fn from_conflict_graphs(
        equivalence_id: impl Into<String>,
        left: &TilingConflictGraph,
        right: &TilingConflictGraph,
        scope: TilingEquivalenceScope,
    ) -> Result<Self, TilingEquivalenceError> {
        if !matches!(
            scope,
            TilingEquivalenceScope::ExactConflictGraph | TilingEquivalenceScope::CheckerInputReuse
        ) {
            return Err(TilingEquivalenceError::ScopeInputMismatch {
                scope: scope.as_str(),
            });
        }
        Self::new(
            equivalence_id,
            scope,
            left.reference(),
            right.reference(),
            conflict_graph_equivalence_token(left, scope),
            conflict_graph_equivalence_token(right, scope),
        )
    }

    pub fn from_conflict_cores(
        equivalence_id: impl Into<String>,
        left: &ConflictCoreExtractionReport,
        right: &ConflictCoreExtractionReport,
    ) -> Result<Self, TilingEquivalenceError> {
        Self::new(
            equivalence_id,
            TilingEquivalenceScope::ConflictCore,
            left.reference(),
            right.reference(),
            left.artifact_digest().stable_token(),
            right.artifact_digest().stable_token(),
        )
    }

    pub fn from_tile_equivalence_witness(
        equivalence_id: impl Into<String>,
        witness: TileEquivalenceWitness,
    ) -> Result<Self, TilingEquivalenceError> {
        let scope = tile_witness_equivalence_scope(&witness);
        let reference = witness.reference();
        let left_basis = witness.equivalence_token();
        let right_basis = if witness.blocks_duplicate_checker_work() {
            left_basis.clone()
        } else {
            format!("unsupported:{left_basis}")
        };
        Self::new(
            equivalence_id,
            scope,
            reference.clone(),
            reference,
            left_basis,
            right_basis,
        )
    }

    pub fn from_terminal_forcing_relations(
        equivalence_id: impl Into<String>,
        left: &TerminalForcingRelation,
        right: &TerminalForcingRelation,
    ) -> Result<Self, TilingEquivalenceError> {
        Self::new(
            equivalence_id,
            TilingEquivalenceScope::MotifTerminalBehavior,
            left.reference(),
            right.reference(),
            terminal_relation_behavior_token(left),
            terminal_relation_behavior_token(right),
        )
    }

    pub fn from_periodic_quotient_cells(
        equivalence_id: impl Into<String>,
        left: &PeriodicQuotientCell,
        right: &PeriodicQuotientCell,
    ) -> Result<Self, TilingEquivalenceError> {
        Self::new(
            equivalence_id,
            TilingEquivalenceScope::PeriodicQuotientConstraints,
            left.reference(),
            right.reference(),
            left.artifact_digest().stable_token(),
            right.artifact_digest().stable_token(),
        )
    }

    pub fn from_generated_replays(
        equivalence_id: impl Into<String>,
        left: &GeneratedPatternReplayChecked,
        right: &GeneratedPatternReplayChecked,
    ) -> Result<Self, TilingEquivalenceError> {
        Self::new(
            equivalence_id,
            TilingEquivalenceScope::GeneratedClosure,
            left.report().reference(),
            right.report().reference(),
            left.report().artifact_digest().stable_token(),
            right.report().artifact_digest().stable_token(),
        )
    }

    fn new(
        equivalence_id: impl Into<String>,
        scope: TilingEquivalenceScope,
        left_reference: HadwigerArtifactReference,
        right_reference: HadwigerArtifactReference,
        left_basis_token: impl Into<String>,
        right_basis_token: impl Into<String>,
    ) -> Result<Self, TilingEquivalenceError> {
        Ok(Self {
            equivalence_id: require_equivalence_non_empty(equivalence_id, "equivalence_id")?,
            scope,
            left_reference,
            right_reference,
            left_basis_token: require_equivalence_non_empty(left_basis_token, "left_basis_token")?,
            right_basis_token: require_equivalence_non_empty(
                right_basis_token,
                "right_basis_token",
            )?,
        })
    }

    pub(crate) fn equivalence_id(&self) -> &str {
        &self.equivalence_id
    }

    pub(crate) fn scope(&self) -> TilingEquivalenceScope {
        self.scope
    }

    pub(crate) fn left_reference(&self) -> &HadwigerArtifactReference {
        &self.left_reference
    }

    pub(crate) fn right_reference(&self) -> &HadwigerArtifactReference {
        &self.right_reference
    }

    pub(crate) fn basis_tokens_match(&self) -> bool {
        self.left_basis_token == self.right_basis_token
    }

    pub(crate) fn equivalence_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.scope.as_str(),
            self.left_basis_token,
            self.right_basis_token
        )
    }
}

fn tile_witness_equivalence_scope(witness: &TileEquivalenceWitness) -> TilingEquivalenceScope {
    match witness.scope() {
        crate::research_cockpit::TileEquivalenceScope::ContactConstraint => {
            TilingEquivalenceScope::TileContactGraph
        }
        crate::research_cockpit::TileEquivalenceScope::MetricThreshold => {
            TilingEquivalenceScope::MetricThresholdClass
        }
        crate::research_cockpit::TileEquivalenceScope::PeriodicColorRule => {
            TilingEquivalenceScope::PeriodicColorRule
        }
    }
}

fn terminal_relation_behavior_token(relation: &TerminalForcingRelation) -> String {
    format!(
        "motif={};kind={};terminals={};color_count={}",
        relation.motif_reference().stable_token(),
        relation.relation_kind().as_str(),
        relation.terminal_labels().join("|"),
        relation.color_count()
    )
}

fn conflict_graph_equivalence_token(
    graph: &TilingConflictGraph,
    scope: TilingEquivalenceScope,
) -> String {
    let vertices = graph
        .graph_version()
        .vertices()
        .iter()
        .map(|vertex| vertex.vertex_label().to_string())
        .collect::<Vec<_>>()
        .join("|");
    let edges = graph
        .conflict_edges()
        .iter()
        .map(|edge| {
            format!(
                "{}:{}:{}:{}",
                edge.left_vertex_label(),
                edge.right_vertex_label(),
                edge.basis().as_str(),
                edge.translated_boundary().unwrap_or("none")
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let color_target = match scope {
        TilingEquivalenceScope::CheckerInputReuse => graph
            .required_color_count()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        _ => "not-applicable".to_string(),
    };
    format!("vertices={vertices};edges={edges};color_target={color_target}")
}
