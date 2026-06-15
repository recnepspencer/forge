use super::HarnessEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HarnessEvidenceRequirement {
    family: HarnessEvidenceFamily,
}

impl HarnessEvidenceRequirement {
    pub fn runtime_receipt() -> Self {
        Self {
            family: HarnessEvidenceFamily::RuntimeReceipt,
        }
    }

    pub fn operation_receipt() -> Self {
        Self {
            family: HarnessEvidenceFamily::OperationReceipt,
        }
    }

    pub fn active_plan_observation() -> Self {
        Self {
            family: HarnessEvidenceFamily::ActivePlanObservation,
        }
    }

    pub fn active_plan_digest() -> Self {
        Self {
            family: HarnessEvidenceFamily::ActivePlanDigest,
        }
    }

    pub fn artifact_digest() -> Self {
        Self {
            family: HarnessEvidenceFamily::ArtifactDigest,
        }
    }

    pub fn counter_family() -> Self {
        Self {
            family: HarnessEvidenceFamily::CounterFamily,
        }
    }

    pub fn state_receipt() -> Self {
        Self {
            family: HarnessEvidenceFamily::StateReceipt,
        }
    }

    pub fn command_identity() -> Self {
        Self {
            family: HarnessEvidenceFamily::CommandIdentity,
        }
    }

    pub fn visible_frame_observation() -> Self {
        Self {
            family: HarnessEvidenceFamily::VisibleFrameObservation,
        }
    }

    pub fn family(self) -> HarnessEvidenceFamily {
        self.family
    }
}
