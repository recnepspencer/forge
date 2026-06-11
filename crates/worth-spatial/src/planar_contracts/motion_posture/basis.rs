use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;

use super::validation::validate_planar_motion_posture_basis;
use super::{
    PlanarMotionCancellation, PlanarMotionPostureDenial, PlanarMotionStep, PlanarReorientation,
    PlanarRotationPosture,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarMotionPostureBasis {
    boolean_readiness_receipt: PlanarContractBundleValidationReceipt,
    steps: Vec<PlanarMotionStep>,
    rotation_posture: PlanarRotationPosture,
    cancellation: PlanarMotionCancellation,
    final_coordinate_digest: Option<String>,
}

impl PlanarMotionPostureBasis {
    pub fn builder() -> PlanarMotionPostureBuilder {
        PlanarMotionPostureBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: PlanarMotionPostureBuilder,
    ) -> Result<Self, PlanarMotionPostureDenial> {
        let basis = Self {
            boolean_readiness_receipt: builder.boolean_readiness_receipt.ok_or_else(|| {
                super::PlanarMotionPostureDenial::new(
                    super::PlanarMotionPostureDenialKind::MissingBooleanReadinessReceipt,
                    "planar motion posture requires a boolean-readiness receipt",
                )
            })?,
            steps: builder.steps,
            rotation_posture: builder.rotation_posture,
            cancellation: builder.cancellation,
            final_coordinate_digest: builder.final_coordinate_digest,
        };
        validate_planar_motion_posture_basis(&basis)?;
        Ok(basis)
    }

    pub fn boolean_readiness_receipt(&self) -> &PlanarContractBundleValidationReceipt {
        &self.boolean_readiness_receipt
    }

    pub fn steps(&self) -> &[PlanarMotionStep] {
        &self.steps
    }

    pub fn rotation_posture(&self) -> PlanarRotationPosture {
        self.rotation_posture
    }

    pub fn cancellation(&self) -> PlanarMotionCancellation {
        self.cancellation
    }

    pub fn final_coordinate_digest(&self) -> Option<&str> {
        self.final_coordinate_digest.as_deref()
    }

    pub(crate) fn reorientation_steps(&self) -> impl Iterator<Item = PlanarReorientation> + '_ {
        self.steps.iter().filter_map(|step| match step {
            PlanarMotionStep::Reorientation { posture } => Some(*posture),
            _ => None,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarMotionPostureBuilder {
    boolean_readiness_receipt: Option<PlanarContractBundleValidationReceipt>,
    steps: Vec<PlanarMotionStep>,
    rotation_posture: PlanarRotationPosture,
    cancellation: PlanarMotionCancellation,
    final_coordinate_digest: Option<String>,
}

impl Default for PlanarMotionPostureBuilder {
    fn default() -> Self {
        Self {
            boolean_readiness_receipt: None,
            steps: Vec::new(),
            rotation_posture: PlanarRotationPosture::None,
            cancellation: PlanarMotionCancellation::None,
            final_coordinate_digest: None,
        }
    }
}

impl PlanarMotionPostureBuilder {
    pub fn boolean_readiness_receipt(
        mut self,
        receipt: PlanarContractBundleValidationReceipt,
    ) -> Self {
        self.boolean_readiness_receipt = Some(receipt);
        self
    }

    pub fn exact_translation(mut self, step_identity: impl Into<String>) -> Self {
        self.steps
            .push(PlanarMotionStep::exact_translation(step_identity));
        self
    }

    pub fn exact_rotation(mut self, step_identity: impl Into<String>) -> Self {
        self.rotation_posture = PlanarRotationPosture::ExactRotation;
        self.steps
            .push(PlanarMotionStep::exact_rotation(step_identity));
        self
    }

    pub fn reorientation(mut self, posture: PlanarReorientation) -> Self {
        self.steps.push(PlanarMotionStep::reorientation(posture));
        self
    }

    pub fn cancellation_policy(mut self, cancellation: PlanarMotionCancellation) -> Self {
        if cancellation == PlanarMotionCancellation::ExactBasisReplay {
            self.rotation_posture = PlanarRotationPosture::ExactCancellation;
        }
        self.cancellation = cancellation;
        self
    }

    pub fn final_coordinate_digest_only(mut self, digest: impl Into<String>) -> Self {
        self.final_coordinate_digest = Some(digest.into());
        self
    }

    pub fn build(self) -> Result<PlanarMotionPostureBasis, PlanarMotionPostureDenial> {
        PlanarMotionPostureBasis::from_builder(self)
    }
}
