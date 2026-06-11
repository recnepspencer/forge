use crate::mathematical_verification::ExactRational;

use super::replay_errors::{require_replay_non_empty, GeneratedPatternReplayShapeError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PeriodicLatticeVector {
    vector_id: String,
    dx: ExactRational,
    dy: ExactRational,
}

impl PeriodicLatticeVector {
    pub fn new(
        vector_id: impl Into<String>,
        dx: ExactRational,
        dy: ExactRational,
    ) -> Result<Self, GeneratedPatternReplayShapeError> {
        Ok(Self {
            vector_id: require_replay_non_empty(vector_id, "lattice_vector_id")?,
            dx,
            dy,
        })
    }

    pub fn vector_id(&self) -> &str {
        &self.vector_id
    }

    pub fn dx(&self) -> &ExactRational {
        &self.dx
    }

    pub fn dy(&self) -> &ExactRational {
        &self.dy
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.vector_id,
            self.dx.stable_token(),
            self.dy.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicLatticeBasis {
    vectors: Vec<PeriodicLatticeVector>,
}

impl PeriodicLatticeBasis {
    pub(crate) fn new(
        mut vectors: Vec<PeriodicLatticeVector>,
    ) -> Result<Self, GeneratedPatternReplayShapeError> {
        if vectors.is_empty() {
            return Err(GeneratedPatternReplayShapeError::EmptyField {
                field: "lattice_basis_vectors",
            });
        }
        vectors.sort();
        for window in vectors.windows(2) {
            if window[0].vector_id == window[1].vector_id {
                return Err(GeneratedPatternReplayShapeError::DuplicateIdentity {
                    field: "lattice_vector_id",
                    value: window[0].vector_id.clone(),
                });
            }
        }
        Ok(Self { vectors })
    }

    pub fn vectors(&self) -> &[PeriodicLatticeVector] {
        &self.vectors
    }

    pub(crate) fn require_vector(
        &self,
        vector_id: &str,
    ) -> Result<&PeriodicLatticeVector, GeneratedPatternReplayShapeError> {
        self.vectors
            .iter()
            .find(|vector| vector.vector_id == vector_id)
            .ok_or_else(|| GeneratedPatternReplayShapeError::UnknownLatticeVector {
                vector_id: vector_id.to_string(),
            })
    }

    pub fn stable_token(&self) -> String {
        self.vectors
            .iter()
            .map(PeriodicLatticeVector::stable_token)
            .collect::<Vec<_>>()
            .join("|")
    }
}
