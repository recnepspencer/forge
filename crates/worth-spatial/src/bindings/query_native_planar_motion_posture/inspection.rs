use crate::planar_contracts::motion_posture::{
    PlanarMotionCancellation, PlanarMotionPostureBasis, PlanarMotionStep,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarMotionPostureInspectionKind {
    MotionStep,
    Rotation,
    Cancellation,
    SignalCompatibility,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarMotionPostureInspectionRow {
    kind: PlanarMotionPostureInspectionKind,
    locus: String,
    value: String,
}

impl PlanarMotionPostureInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarMotionPostureBasis) -> Vec<Self> {
        let mut rows = basis
            .steps()
            .iter()
            .enumerate()
            .map(|(index, step)| {
                row(
                    PlanarMotionPostureInspectionKind::MotionStep,
                    format!("motion.step.{index}.{}", step.kind()),
                    step.authority_value(),
                )
            })
            .collect::<Vec<_>>();
        rows.push(row(
            PlanarMotionPostureInspectionKind::Rotation,
            "motion.rotation.posture",
            basis.rotation_posture().as_str(),
        ));
        rows.push(row(
            PlanarMotionPostureInspectionKind::Cancellation,
            "motion.cancellation.policy",
            basis.cancellation().as_str(),
        ));
        rows.push(row(
            PlanarMotionPostureInspectionKind::SignalCompatibility,
            "motion.signal.compatibility",
            signal_compatibility_value(basis),
        ));
        rows
    }

    pub fn kind(&self) -> PlanarMotionPostureInspectionKind {
        self.kind
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn signal_compatibility_value(basis: &PlanarMotionPostureBasis) -> &'static str {
    if basis.cancellation() == PlanarMotionCancellation::ExactBasisReplay {
        "compatible-after-exact-cancellation"
    } else if basis
        .steps()
        .iter()
        .any(|step| matches!(step, PlanarMotionStep::Reorientation { .. }))
    {
        "compatible-after-explicit-reorientation"
    } else {
        "compatible-after-retained-motion"
    }
}

fn row(
    kind: PlanarMotionPostureInspectionKind,
    locus: impl Into<String>,
    value: impl Into<String>,
) -> PlanarMotionPostureInspectionRow {
    PlanarMotionPostureInspectionRow {
        kind,
        locus: locus.into(),
        value: value.into(),
    }
}
