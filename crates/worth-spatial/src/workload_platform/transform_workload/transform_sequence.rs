use crate::planar_contracts::motion_posture::{
    PlanarMotionCancellation, PlanarReorientation, PlanarRotationPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorDelta {
    x_units: i64,
    y_units: i64,
}

impl VectorDelta {
    pub fn xy(x_units: i64, y_units: i64) -> Self {
        Self { x_units, y_units }
    }

    pub fn identity(&self) -> String {
        format!("delta:x={}:y={}", self.x_units, self.y_units)
    }

    pub fn changes_coordinates(&self) -> bool {
        self.x_units != 0 || self.y_units != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RotationTurn {
    QuarterTurnClockwise,
    HalfTurn,
    QuarterTurnCounterClockwise,
}

impl RotationTurn {
    pub const fn quarter_turn_clockwise() -> Self {
        Self::QuarterTurnClockwise
    }

    pub const fn identity(self) -> &'static str {
        match self {
            Self::QuarterTurnClockwise => "quarter-turn-clockwise",
            Self::HalfTurn => "half-turn",
            Self::QuarterTurnCounterClockwise => "quarter-turn-counter-clockwise",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransformReorientation {
    PreservesHandedness,
    ReversesHandedness,
}

impl TransformReorientation {
    pub const fn preserves_handedness() -> Self {
        Self::PreservesHandedness
    }

    pub(crate) const fn as_planar_reorientation(self) -> PlanarReorientation {
        match self {
            Self::PreservesHandedness => PlanarReorientation::PreservesHandedness,
            Self::ReversesHandedness => PlanarReorientation::ReversesHandedness,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransformStep {
    Translation(VectorDelta),
    Rotation(RotationTurn),
    Reorientation(TransformReorientation),
    ExactCancellationReplay { steps: usize },
    LabelOnlyMotion { label: String },
}

impl TransformStep {
    pub fn identity(&self) -> String {
        match self {
            Self::Translation(delta) => format!("translation:{}", delta.identity()),
            Self::Rotation(turn) => format!("rotation:{}", turn.identity()),
            Self::Reorientation(posture) => {
                format!(
                    "reorientation:{}",
                    posture.as_planar_reorientation().as_str()
                )
            }
            Self::ExactCancellationReplay { steps } => {
                format!("exact-cancellation-replay:steps={steps}")
            }
            Self::LabelOnlyMotion { label } => format!("label-only:{label}"),
        }
    }

    pub fn changes_coordinates(&self) -> bool {
        match self {
            Self::Translation(delta) => delta.changes_coordinates(),
            Self::Rotation(_) => true,
            Self::Reorientation(_) => false,
            Self::ExactCancellationReplay { .. } | Self::LabelOnlyMotion { .. } => false,
        }
    }

    pub fn carries_transform_evidence(&self) -> bool {
        match self {
            Self::Translation(delta) => delta.changes_coordinates(),
            Self::Rotation(_) | Self::Reorientation(_) => true,
            Self::ExactCancellationReplay { steps } => *steps > 0,
            Self::LabelOnlyMotion { .. } => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformSequence {
    steps: Vec<TransformStep>,
}

impl TransformSequence {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn identity_label_only(label: impl Into<String>) -> Self {
        Self {
            steps: vec![TransformStep::LabelOnlyMotion {
                label: label.into(),
            }],
        }
    }

    pub fn translate(mut self, delta: VectorDelta) -> Self {
        self.steps.push(TransformStep::Translation(delta));
        self
    }

    pub fn rotate(mut self, turn: RotationTurn) -> Self {
        self.steps.push(TransformStep::Rotation(turn));
        self
    }

    pub fn reorient(mut self, posture: TransformReorientation) -> Self {
        self.steps.push(TransformStep::Reorientation(posture));
        self
    }

    pub fn cancel_with_exact_replay(mut self, steps: usize) -> Self {
        self.steps
            .push(TransformStep::ExactCancellationReplay { steps });
        self
    }

    pub fn steps(&self) -> &[TransformStep] {
        &self.steps
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn changed_coordinate_steps(&self) -> usize {
        self.steps
            .iter()
            .filter(|step| step.changes_coordinates())
            .count()
    }

    pub fn cancellation_steps(&self) -> usize {
        self.steps
            .iter()
            .filter_map(|step| match step {
                TransformStep::ExactCancellationReplay { steps } => Some(*steps),
                _ => None,
            })
            .sum()
    }

    pub(crate) fn cancellation_replay_counts(&self) -> impl Iterator<Item = usize> + '_ {
        self.steps.iter().filter_map(|step| match step {
            TransformStep::ExactCancellationReplay { steps } => Some(*steps),
            _ => None,
        })
    }

    pub(crate) fn cancellation_replay_count(&self) -> usize {
        self.cancellation_replay_counts().count()
    }

    pub fn has_label_only_motion(&self) -> bool {
        self.steps
            .iter()
            .any(|step| matches!(step, TransformStep::LabelOnlyMotion { .. }))
    }

    pub fn has_real_transform_evidence(&self) -> bool {
        self.steps
            .iter()
            .any(TransformStep::carries_transform_evidence)
    }

    pub(crate) fn has_posture_change_evidence(&self) -> bool {
        self.steps
            .iter()
            .any(|step| matches!(step, TransformStep::Reorientation(_)))
    }

    pub(crate) fn rotation_posture(&self) -> PlanarRotationPosture {
        if self.cancellation_steps() > 0 {
            PlanarRotationPosture::ExactCancellation
        } else if self
            .steps
            .iter()
            .any(|step| matches!(step, TransformStep::Rotation(_)))
        {
            PlanarRotationPosture::ExactRotation
        } else {
            PlanarRotationPosture::None
        }
    }

    pub(crate) fn reorientation(&self) -> PlanarReorientation {
        self.steps
            .iter()
            .rev()
            .find_map(|step| match step {
                TransformStep::Reorientation(posture) => Some(posture.as_planar_reorientation()),
                _ => None,
            })
            .unwrap_or(PlanarReorientation::PreservesHandedness)
    }

    pub(crate) fn cancellation_policy(&self) -> PlanarMotionCancellation {
        if self.cancellation_steps() > 0 {
            PlanarMotionCancellation::ExactBasisReplay
        } else {
            PlanarMotionCancellation::None
        }
    }
}

impl Default for TransformSequence {
    fn default() -> Self {
        Self::new()
    }
}
