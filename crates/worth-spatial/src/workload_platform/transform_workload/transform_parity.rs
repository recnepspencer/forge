use super::TransformSequence;
use crate::planar_contracts::motion_posture::PlanarRotationPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransformParityKind {
    EquivalentConvergence,
    SemanticDivergence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformParityRow {
    kind: TransformParityKind,
    reason: String,
}

impl TransformParityRow {
    fn equivalent(reason: impl Into<String>) -> Self {
        Self {
            kind: TransformParityKind::EquivalentConvergence,
            reason: reason.into(),
        }
    }

    fn divergent(reason: impl Into<String>) -> Self {
        Self {
            kind: TransformParityKind::SemanticDivergence,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> TransformParityKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformParityReport {
    rows: Vec<TransformParityRow>,
}

impl TransformParityReport {
    pub(crate) fn from_sequence(sequence: &TransformSequence) -> Self {
        let mut rows = Vec::new();
        if sequence.cancellation_steps() > 0 {
            rows.push(TransformParityRow::equivalent(
                "exact cancellation replay converges when transform basis is replayed",
            ));
        }
        if sequence.rotation_posture() != PlanarRotationPosture::None
            || sequence.changed_coordinate_steps() > 0
            || sequence.has_posture_change_evidence()
        {
            rows.push(TransformParityRow::divergent(
                "non-identity transform changes coordinate or posture evidence",
            ));
        }
        if rows.is_empty() {
            rows.push(TransformParityRow::divergent(
                "transform sequence has no evidence of equivalent convergence",
            ));
        }
        Self { rows }
    }

    pub fn rows(&self) -> &[TransformParityRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn has_equivalent_convergence(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.kind() == TransformParityKind::EquivalentConvergence)
    }

    pub fn has_semantic_divergence(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.kind() == TransformParityKind::SemanticDivergence)
    }
}
