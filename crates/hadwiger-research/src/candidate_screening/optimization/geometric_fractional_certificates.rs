use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{ScreeningRational, ScreeningSolverTranscript};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeometricFractionalSearchScope {
    Unspecified,
    MoserLatticeReproduction,
    MoserRingReproduction,
    EscapeEvidence { evidence_reference: String },
}

impl GeometricFractionalSearchScope {
    pub(crate) fn suppresses_improvement_without_escape(&self) -> bool {
        matches!(
            self,
            Self::MoserLatticeReproduction | Self::MoserRingReproduction
        )
    }

    pub fn stable_token(&self) -> String {
        match self {
            Self::Unspecified => "unspecified".to_string(),
            Self::MoserLatticeReproduction => "moser_lattice_reproduction".to_string(),
            Self::MoserRingReproduction => "moser_ring_reproduction".to_string(),
            Self::EscapeEvidence { evidence_reference } => {
                format!("escape_evidence:{evidence_reference}")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometricPairwiseSquaredDistance {
    left_pair: (String, String),
    right_pair: (String, String),
    left_squared_distance: ScreeningRational,
    right_squared_distance: ScreeningRational,
}

impl GeometricPairwiseSquaredDistance {
    pub fn new(
        left_a: impl Into<String>,
        left_b: impl Into<String>,
        right_a: impl Into<String>,
        right_b: impl Into<String>,
        left_squared_distance: ScreeningRational,
        right_squared_distance: ScreeningRational,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if left_squared_distance.is_negative() || right_squared_distance.is_negative() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "nonnegative_squared_distance",
            });
        }
        Ok(Self {
            left_pair: normalized_pair(left_a, left_b, "left_distance_pair")?,
            right_pair: normalized_pair(right_a, right_b, "right_distance_pair")?,
            left_squared_distance,
            right_squared_distance,
        })
    }

    pub(crate) fn left_pair(&self) -> &(String, String) {
        &self.left_pair
    }

    pub(crate) fn right_pair(&self) -> &(String, String) {
        &self.right_pair
    }

    pub(crate) fn left_squared_distance(&self) -> &ScreeningRational {
        &self.left_squared_distance
    }

    pub(crate) fn right_squared_distance(&self) -> &ScreeningRational {
        &self.right_squared_distance
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}-{}={}~{}-{}={}",
            self.left_pair.0,
            self.left_pair.1,
            self.left_squared_distance.stable_token(),
            self.right_pair.0,
            self.right_pair.1,
            self.right_squared_distance.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometricSubsetIsometryWitness {
    witness_id: String,
    mapping: Vec<(String, String)>,
    pairwise_squared_distances: Vec<GeometricPairwiseSquaredDistance>,
}

impl GeometricSubsetIsometryWitness {
    pub fn new(
        witness_id: impl Into<String>,
        mapping: Vec<(String, String)>,
        pairwise_squared_distances: Vec<GeometricPairwiseSquaredDistance>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if mapping.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "isometry_mapping",
            });
        }
        let mut checked_mapping = Vec::new();
        for (left, right) in mapping {
            checked_mapping.push((
                require_non_empty(left, "left_isometry_vertex")?,
                require_non_empty(right, "right_isometry_vertex")?,
            ));
        }
        checked_mapping.sort();
        let mut pairwise_squared_distances = pairwise_squared_distances;
        pairwise_squared_distances.sort_by_key(GeometricPairwiseSquaredDistance::stable_token);
        Ok(Self {
            witness_id: require_non_empty(witness_id, "isometry_witness_id")?,
            mapping: checked_mapping,
            pairwise_squared_distances,
        })
    }

    pub(crate) fn mapping(&self) -> &[(String, String)] {
        &self.mapping
    }

    pub(crate) fn pairwise_squared_distances(&self) -> &[GeometricPairwiseSquaredDistance] {
        &self.pairwise_squared_distances
    }

    pub fn stable_token(&self) -> String {
        let mut token = self.witness_id.clone();
        for (left, right) in &self.mapping {
            token.push_str(&format!(":{left}->{right}"));
        }
        for distance in &self.pairwise_squared_distances {
            token.push_str(&format!(":{}", distance.stable_token()));
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometricFractionalEqualityAdjustment {
    left_subset: Vec<String>,
    right_subset: Vec<String>,
    multiplier: ScreeningRational,
    isometry_witness: GeometricSubsetIsometryWitness,
}

impl GeometricFractionalEqualityAdjustment {
    pub fn new(
        left_subset: Vec<String>,
        right_subset: Vec<String>,
        multiplier: ScreeningRational,
        isometry_witness: GeometricSubsetIsometryWitness,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            left_subset: checked_subset(left_subset, "left_geometric_subset")?,
            right_subset: checked_subset(right_subset, "right_geometric_subset")?,
            multiplier,
            isometry_witness,
        })
    }

    pub(crate) fn left_subset(&self) -> &[String] {
        &self.left_subset
    }

    pub(crate) fn right_subset(&self) -> &[String] {
        &self.right_subset
    }

    pub(crate) fn multiplier(&self) -> &ScreeningRational {
        &self.multiplier
    }

    pub(crate) fn isometry_witness(&self) -> &GeometricSubsetIsometryWitness {
        &self.isometry_witness
    }

    pub fn stable_token(&self) -> String {
        format!(
            "left={}|right={}|multiplier={}|witness={}",
            self.left_subset.join(","),
            self.right_subset.join(","),
            self.multiplier.stable_token(),
            self.isometry_witness.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometricFractionalChromaticCertificate {
    certificate_id: String,
    target_lower_bound: ScreeningRational,
    vertex_weights: Vec<(String, ScreeningRational)>,
    equality_adjustments: Vec<GeometricFractionalEqualityAdjustment>,
    lower_bound: ScreeningRational,
    solver_transcript: ScreeningSolverTranscript,
    search_scope: GeometricFractionalSearchScope,
}

impl GeometricFractionalChromaticCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        target_lower_bound: ScreeningRational,
        vertex_weights: Vec<(String, ScreeningRational)>,
        equality_adjustments: Vec<GeometricFractionalEqualityAdjustment>,
        lower_bound: ScreeningRational,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if vertex_weights.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "vertex_weights",
            });
        }
        let mut checked_weights = Vec::new();
        for (vertex, weight) in vertex_weights {
            checked_weights.push((require_non_empty(vertex, "vertex_label")?, weight));
        }
        checked_weights.sort_by_key(|(vertex, _)| vertex.clone());
        let mut equality_adjustments = equality_adjustments;
        equality_adjustments.sort_by_key(GeometricFractionalEqualityAdjustment::stable_token);
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            target_lower_bound,
            vertex_weights: checked_weights,
            equality_adjustments,
            lower_bound,
            solver_transcript,
            search_scope: GeometricFractionalSearchScope::Unspecified,
        })
    }

    pub fn with_moser_lattice_reproduction_scope(mut self) -> Self {
        self.search_scope = GeometricFractionalSearchScope::MoserLatticeReproduction;
        self
    }

    pub fn with_moser_ring_reproduction_scope(mut self) -> Self {
        self.search_scope = GeometricFractionalSearchScope::MoserRingReproduction;
        self
    }

    pub fn with_escape_evidence_scope(
        mut self,
        evidence_reference: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.search_scope = GeometricFractionalSearchScope::EscapeEvidence {
            evidence_reference: require_non_empty(evidence_reference, "escape_evidence_reference")?,
        };
        Ok(self)
    }

    pub(crate) fn target_lower_bound(&self) -> &ScreeningRational {
        &self.target_lower_bound
    }

    pub(crate) fn vertex_weights(&self) -> &[(String, ScreeningRational)] {
        &self.vertex_weights
    }

    pub(crate) fn equality_adjustments(&self) -> &[GeometricFractionalEqualityAdjustment] {
        &self.equality_adjustments
    }

    pub(crate) fn lower_bound(&self) -> &ScreeningRational {
        &self.lower_bound
    }

    pub(crate) fn search_scope(&self) -> &GeometricFractionalSearchScope {
        &self.search_scope
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!(
            "{}:target={}:lower={}:scope={}:{}",
            self.certificate_id,
            self.target_lower_bound.stable_token(),
            self.lower_bound.stable_token(),
            self.search_scope.stable_token(),
            self.solver_transcript.stable_token()
        );
        for (vertex, weight) in &self.vertex_weights {
            token.push_str(&format!(":{vertex}={}", weight.stable_token()));
        }
        for adjustment in &self.equality_adjustments {
            token.push_str(&format!(":{}", adjustment.stable_token()));
        }
        token
    }
}

fn checked_subset(
    subset: Vec<String>,
    field: &'static str,
) -> Result<Vec<String>, HadwigerArtifactShapeError> {
    if subset.is_empty() {
        return Err(HadwigerArtifactShapeError::EmptyField { field });
    }
    let mut checked = subset
        .into_iter()
        .map(|vertex| require_non_empty(vertex, field))
        .collect::<Result<Vec<_>, _>>()?;
    checked.sort();
    let original_len = checked.len();
    checked.dedup();
    if checked.len() != original_len {
        return Err(HadwigerArtifactShapeError::EmptyField { field });
    }
    Ok(checked)
}

fn normalized_pair(
    left: impl Into<String>,
    right: impl Into<String>,
    field: &'static str,
) -> Result<(String, String), HadwigerArtifactShapeError> {
    let left = require_non_empty(left, field)?;
    let right = require_non_empty(right, field)?;
    if left <= right {
        Ok((left, right))
    } else {
        Ok((right, left))
    }
}
