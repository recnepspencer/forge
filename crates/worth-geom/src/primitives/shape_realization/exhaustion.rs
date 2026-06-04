use worth_math::MathError;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::primitives::shape_realization::schema::{
    PrimitiveConditioningWitness, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRealizationExhaustionReason {
    DegenerateSupportNormals,
}

impl PrimitiveRealizationExhaustionReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DegenerateSupportNormals => "degenerate_support_normals",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRealizationExhaustionReport {
    family: &'static str,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: PrimitiveConditioningWitness,
    stability_class: PrimitiveStabilityClass,
    exhaustion_reason: PrimitiveRealizationExhaustionReason,
    report_digest: String,
}

impl PrimitiveRealizationExhaustionReport {
    pub(crate) fn new(
        family: &'static str,
        attempted_strategies: Vec<PrimitiveRealizationStrategy>,
        conditioning_witness: PrimitiveConditioningWitness,
        exhaustion_reason: PrimitiveRealizationExhaustionReason,
    ) -> Self {
        let report_digest = digest_parts(&[
            family.to_string(),
            attempted_strategies
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            PrimitiveStabilityClass::RejectedBelowConditioningFloor
                .as_str()
                .to_string(),
            exhaustion_reason.as_str().to_string(),
            conditioning_witness
                .coordinate_magnitude()
                .to_bits()
                .to_string(),
            conditioning_witness.feature_size().to_bits().to_string(),
            conditioning_witness
                .condition_number()
                .to_bits()
                .to_string(),
            conditioning_witness
                .machine_epsilon_at_scale()
                .to_bits()
                .to_string(),
            conditioning_witness
                .precision_headroom_ratio()
                .to_bits()
                .to_string(),
            conditioning_witness
                .minimum_support_normal_magnitude()
                .to_bits()
                .to_string(),
            conditioning_witness
                .support_normal_headroom_ratio()
                .to_bits()
                .to_string(),
            conditioning_witness
                .normalization_scale_applied()
                .to_bits()
                .to_string(),
            conditioning_witness.feature_size_collapsed().to_string(),
            conditioning_witness.needs_local_transform().to_string(),
            conditioning_witness
                .feature_conditioning_class()
                .as_str()
                .to_string(),
            conditioning_witness
                .support_normal_class()
                .as_str()
                .to_string(),
            conditioning_witness
                .normalization_disposition()
                .as_str()
                .to_string(),
        ]);
        Self {
            family,
            attempted_strategies,
            conditioning_witness,
            stability_class: PrimitiveStabilityClass::RejectedBelowConditioningFloor,
            exhaustion_reason,
            report_digest,
        }
    }

    pub fn family(&self) -> &'static str {
        self.family
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_reason
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

impl std::fmt::Display for PrimitiveRealizationExhaustionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} realization exhausted after {}: {}",
            self.family,
            self.attempted_strategies
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join(" -> "),
            self.exhaustion_reason.as_str()
        )
    }
}

#[derive(Debug)]
pub enum PrimitiveRealizationError {
    Exhausted(PrimitiveRealizationExhaustionReport),
    Geometry(MathError),
}

impl std::fmt::Display for PrimitiveRealizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhausted(report) => write!(f, "{report}"),
            Self::Geometry(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveRealizationError {}

impl From<MathError> for PrimitiveRealizationError {
    fn from(value: MathError) -> Self {
        Self::Geometry(value)
    }
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
