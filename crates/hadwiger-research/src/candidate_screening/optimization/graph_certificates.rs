use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{ScreeningRational, ScreeningSolverTranscript};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FractionalChromaticCertificate {
    certificate_id: String,
    vertex_weights: Vec<(String, ScreeningRational)>,
    lower_bound: ScreeningRational,
    solver_transcript: ScreeningSolverTranscript,
}

impl FractionalChromaticCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        vertex_weights: Vec<(String, ScreeningRational)>,
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
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            vertex_weights: checked_weights,
            lower_bound,
            solver_transcript,
        })
    }

    pub(crate) fn vertex_weights(&self) -> &[(String, ScreeningRational)] {
        &self.vertex_weights
    }

    pub(crate) fn lower_bound(&self) -> &ScreeningRational {
        &self.lower_bound
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!(
            "{}:{}:{}",
            self.certificate_id,
            self.lower_bound.stable_token(),
            self.solver_transcript.stable_token()
        );
        for (vertex, weight) in &self.vertex_weights {
            token.push_str(&format!(":{vertex}={}", weight.stable_token()));
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreeningMatrixCertificate {
    dimension: usize,
    entries: Vec<Vec<ScreeningRational>>,
}

impl ScreeningMatrixCertificate {
    pub fn new(entries: Vec<Vec<ScreeningRational>>) -> Result<Self, HadwigerArtifactShapeError> {
        if entries.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "matrix_entries",
            });
        }
        let dimension = entries.len();
        if entries.iter().any(|row| row.len() != dimension) {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "square_matrix_entries",
            });
        }
        Ok(Self { dimension, entries })
    }

    pub(crate) fn dimension(&self) -> usize {
        self.dimension
    }

    pub(crate) fn entry(&self, row: usize, column: usize) -> &ScreeningRational {
        &self.entries[row][column]
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!("dim={}", self.dimension);
        for row in &self.entries {
            for entry in row {
                token.push_str(&format!(":{}", entry.stable_token()));
            }
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LovaszThetaCertificate {
    certificate_id: String,
    lower_bound: ScreeningRational,
    psd_witness: ScreeningMatrixCertificate,
    solver_transcript: ScreeningSolverTranscript,
}

impl LovaszThetaCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        lower_bound: ScreeningRational,
        psd_witness: ScreeningMatrixCertificate,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            lower_bound,
            psd_witness,
            solver_transcript,
        })
    }

    pub(crate) fn lower_bound(&self) -> &ScreeningRational {
        &self.lower_bound
    }

    pub(crate) fn psd_witness(&self) -> &ScreeningMatrixCertificate {
        &self.psd_witness
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.certificate_id,
            self.lower_bound.stable_token(),
            self.psd_witness.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}
