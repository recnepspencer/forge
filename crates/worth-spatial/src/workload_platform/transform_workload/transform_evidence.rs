use super::{TransformSequence, TransformStep};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransformEvidenceKind {
    CoordinateChange,
    PostureChange,
    CancellationReplay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformEvidenceRow {
    step_identity: String,
    kind: TransformEvidenceKind,
    posture_evidence: String,
}

impl TransformEvidenceRow {
    fn from_step(step: &TransformStep) -> Option<Self> {
        Some(Self {
            step_identity: step.identity(),
            kind: evidence_kind_for_step(step)?,
            posture_evidence: posture_evidence_for_step(step),
        })
    }

    pub fn step_identity(&self) -> &str {
        &self.step_identity
    }

    pub fn changed_coordinates(&self) -> bool {
        self.kind == TransformEvidenceKind::CoordinateChange
    }

    pub fn kind(&self) -> TransformEvidenceKind {
        self.kind
    }

    pub fn posture_evidence(&self) -> &str {
        &self.posture_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformEvidenceSet {
    rows: Vec<TransformEvidenceRow>,
}

impl TransformEvidenceSet {
    pub(crate) fn from_sequence(sequence: &TransformSequence) -> Self {
        let rows = sequence
            .steps()
            .iter()
            .filter_map(TransformEvidenceRow::from_step)
            .collect();
        Self { rows }
    }

    pub fn rows(&self) -> &[TransformEvidenceRow] {
        &self.rows
    }

    pub fn changed_coordinate_rows(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.changed_coordinates())
            .count()
    }

    pub fn evidence_rows(&self) -> usize {
        self.rows.len()
    }
}

fn evidence_kind_for_step(step: &TransformStep) -> Option<TransformEvidenceKind> {
    match step {
        TransformStep::Translation(delta) if delta.changes_coordinates() => {
            Some(TransformEvidenceKind::CoordinateChange)
        }
        TransformStep::Rotation(_) => Some(TransformEvidenceKind::CoordinateChange),
        TransformStep::Reorientation(_) => Some(TransformEvidenceKind::PostureChange),
        TransformStep::ExactCancellationReplay { .. } => {
            Some(TransformEvidenceKind::CancellationReplay)
        }
        TransformStep::Translation(_) | TransformStep::LabelOnlyMotion { .. } => None,
    }
}

fn posture_evidence_for_step(step: &TransformStep) -> String {
    match step {
        TransformStep::Translation(_) => "typed translation evidence".to_string(),
        TransformStep::Rotation(_) => "typed rotation evidence".to_string(),
        TransformStep::Reorientation(posture) => {
            format!(
                "typed reorientation evidence:{}",
                posture.as_planar_reorientation().as_str()
            )
        }
        TransformStep::ExactCancellationReplay { steps } => {
            format!("typed exact cancellation replay evidence:steps={steps}")
        }
        TransformStep::LabelOnlyMotion { .. } => "label-only motion is not evidence".to_string(),
    }
}
