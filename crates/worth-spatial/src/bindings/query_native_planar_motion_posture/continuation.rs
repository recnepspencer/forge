use crate::planar_contracts::motion_posture::{
    PlanarMotionCancellation, PlanarMotionPostureReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarMotionContinuationKind {
    ExactCancellationReplay,
    RetainedMotionNextStep,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarMotionContinuation {
    kind: PlanarMotionContinuationKind,
    retained_motion_digest: String,
}

impl PlanarMotionContinuation {
    pub fn from_receipt(receipt: &PlanarMotionPostureReceipt) -> Self {
        let kind = if receipt.basis().cancellation() == PlanarMotionCancellation::ExactBasisReplay {
            PlanarMotionContinuationKind::ExactCancellationReplay
        } else {
            PlanarMotionContinuationKind::RetainedMotionNextStep
        };
        Self {
            kind,
            retained_motion_digest: receipt.retained_motion_digest().to_string(),
        }
    }

    pub fn kind(&self) -> PlanarMotionContinuationKind {
        self.kind
    }

    pub fn retained_motion_digest(&self) -> &str {
        &self.retained_motion_digest
    }
}
