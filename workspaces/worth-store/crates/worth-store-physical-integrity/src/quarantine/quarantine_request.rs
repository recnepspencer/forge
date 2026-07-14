use crate::{
    ExecutedQuarantineFinding, QuarantineHandoffPosture, QuarantineLifecyclePosture,
    QuarantineSealDenial, QuarantineSealDenialKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineSealRequest {
    finding: ExecutedQuarantineFinding,
    initial_posture: QuarantineLifecyclePosture,
    handoff_posture: QuarantineHandoffPosture,
}

impl QuarantineSealRequest {
    pub fn from_executed_finding(finding: ExecutedQuarantineFinding) -> Self {
        Self {
            finding,
            initial_posture: QuarantineLifecyclePosture::Proposed,
            handoff_posture: QuarantineHandoffPosture::RecoveryOwnerRequired,
        }
    }

    pub const fn with_initial_posture(mut self, posture: QuarantineLifecyclePosture) -> Self {
        self.initial_posture = posture;
        self
    }

    pub const fn with_handoff_posture(mut self, posture: QuarantineHandoffPosture) -> Self {
        self.handoff_posture = posture;
        self
    }

    pub(crate) fn into_checked_parts(
        self,
    ) -> Result<
        (
            ExecutedQuarantineFinding,
            QuarantineLifecyclePosture,
            QuarantineHandoffPosture,
        ),
        QuarantineSealDenial,
    > {
        if !self.initial_posture.is_physical_integrity_mintable() {
            return Err(QuarantineSealDenial::new(
                QuarantineSealDenialKind::LaterLifecycleOwnerRequired,
            ));
        }
        Ok((
            self.finding,
            self.initial_posture.sealed_after_physical_integrity_mint(),
            self.handoff_posture,
        ))
    }
}
