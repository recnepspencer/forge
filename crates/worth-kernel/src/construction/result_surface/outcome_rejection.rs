use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use crate::construction::result::PrimitiveConstructionResultError;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionRejectionClass {
    InvalidRequest,
    GeometryScaffold,
    ConditioningExhaustion,
    SpatialBirth,
    ImpossibleBirthAttachment,
    TopologyExecution,
    ArtifactAssembly,
}

impl PrimitiveConstructionRejectionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::GeometryScaffold => "geometry_scaffold",
            Self::ConditioningExhaustion => "conditioning_exhaustion",
            Self::SpatialBirth => "spatial_birth",
            Self::ImpossibleBirthAttachment => "impossible_birth_attachment",
            Self::TopologyExecution => "topology_execution",
            Self::ArtifactAssembly => "artifact_assembly",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionRejectionLocality {
    Admission,
    Scaffold,
    SpatialBirth,
    Execution,
    Artifact,
}

impl PrimitiveConstructionRejectionLocality {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::Scaffold => "scaffold",
            Self::SpatialBirth => "spatial_birth",
            Self::Execution => "execution",
            Self::Artifact => "artifact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRejectedOutcome {
    family: PrimitiveConstructionFamily,
    rejection_class: PrimitiveConstructionRejectionClass,
    rejection_locality: PrimitiveConstructionRejectionLocality,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_report_digest: Option<String>,
    reason: String,
    failure_digest: String,
}

impl PrimitiveConstructionRejectedOutcome {
    pub(crate) fn new(
        family: PrimitiveConstructionFamily,
        rejection_class: PrimitiveConstructionRejectionClass,
        rejection_locality: PrimitiveConstructionRejectionLocality,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        conditioning_witness: Option<PrimitiveConditioningWitness>,
        exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
        stability_class: Option<PrimitiveStabilityClass>,
        exhaustion_report_digest: Option<String>,
        reason: String,
    ) -> Self {
        let failure_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            rejection_class.as_str().to_string(),
            rejection_locality.as_str().to_string(),
            attempted_realization_strategies
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.coordinate_magnitude().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.feature_size().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.condition_number().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.machine_epsilon_at_scale().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.precision_headroom_ratio().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| {
                    witness
                        .minimum_support_normal_magnitude()
                        .to_bits()
                        .to_string()
                })
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.normalization_scale_applied().to_bits().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.feature_size_collapsed().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.needs_local_transform().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| {
                    witness
                        .support_normal_headroom_ratio()
                        .to_bits()
                        .to_string()
                })
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.feature_conditioning_class().as_str().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.support_normal_class().as_str().to_string())
                .unwrap_or_default(),
            conditioning_witness
                .as_ref()
                .map(|witness| witness.normalization_disposition().as_str().to_string())
                .unwrap_or_default(),
            exhaustion_reason
                .map(PrimitiveRealizationExhaustionReason::as_str)
                .unwrap_or("none")
                .to_string(),
            stability_class
                .map(PrimitiveStabilityClass::as_str)
                .unwrap_or("none")
                .to_string(),
            exhaustion_report_digest.clone().unwrap_or_default(),
            reason.clone(),
        ]);
        Self {
            family,
            rejection_class,
            rejection_locality,
            attempted_realization_strategies,
            conditioning_witness,
            exhaustion_reason,
            stability_class,
            exhaustion_report_digest,
            reason,
            failure_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub fn rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.attempted_realization_strategies.len()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn selected_realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.attempted_realization_strategies.last().copied()
    }

    pub fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.conditioning_witness.as_ref()
    }

    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.feature_conditioning_class())
    }

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.support_normal_class())
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.normalization_disposition())
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn exhaustion_report_digest(&self) -> Option<&str> {
        self.exhaustion_report_digest.as_deref()
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

pub(crate) fn rejected_outcome(
    family: PrimitiveConstructionFamily,
    error: &PrimitiveConstructionResultError,
) -> PrimitiveConstructionRejectedOutcome {
    match error {
        PrimitiveConstructionResultError::Phase(phase) => match phase {
            PrimitiveConstructionPhaseError::InvalidRequest { .. } => {
                PrimitiveConstructionRejectedOutcome::new(
                    family,
                    PrimitiveConstructionRejectionClass::InvalidRequest,
                    PrimitiveConstructionRejectionLocality::Admission,
                    Vec::new(),
                    None,
                    None,
                    None,
                    None,
                    phase.to_string(),
                )
            }
            PrimitiveConstructionPhaseError::Geometry(geometry) => match geometry {
                PrimitiveConstructionGeometryError::RealizationExhausted(report) => {
                    PrimitiveConstructionRejectedOutcome::new(
                        family,
                        PrimitiveConstructionRejectionClass::ConditioningExhaustion,
                        PrimitiveConstructionRejectionLocality::Scaffold,
                        report.attempted_strategies().to_vec(),
                        Some(report.conditioning_witness().clone()),
                        Some(report.exhaustion_reason()),
                        Some(report.stability_class()),
                        Some(report.report_digest().to_string()),
                        phase.to_string(),
                    )
                }
                PrimitiveConstructionGeometryError::GeometryFailure(_) => {
                    PrimitiveConstructionRejectedOutcome::new(
                        family,
                        PrimitiveConstructionRejectionClass::GeometryScaffold,
                        PrimitiveConstructionRejectionLocality::Scaffold,
                        Vec::new(),
                        None,
                        None,
                        None,
                        None,
                        phase.to_string(),
                    )
                }
            },
            PrimitiveConstructionPhaseError::SpatialBirth(_) => {
                PrimitiveConstructionRejectedOutcome::new(
                    family,
                    PrimitiveConstructionRejectionClass::SpatialBirth,
                    PrimitiveConstructionRejectionLocality::SpatialBirth,
                    Vec::new(),
                    None,
                    None,
                    None,
                    None,
                    phase.to_string(),
                )
            }
            PrimitiveConstructionPhaseError::TopologyLowering(_) => {
                PrimitiveConstructionRejectedOutcome::new(
                    family,
                    PrimitiveConstructionRejectionClass::TopologyExecution,
                    PrimitiveConstructionRejectionLocality::Execution,
                    Vec::new(),
                    None,
                    None,
                    None,
                    None,
                    phase.to_string(),
                )
            }
        },
        PrimitiveConstructionResultError::Execution(error) => {
            PrimitiveConstructionRejectedOutcome::new(
                family,
                PrimitiveConstructionRejectionClass::TopologyExecution,
                PrimitiveConstructionRejectionLocality::Execution,
                Vec::new(),
                None,
                None,
                None,
                None,
                error.to_string(),
            )
        }
        PrimitiveConstructionResultError::BirthCompleteness(error) => {
            PrimitiveConstructionRejectedOutcome::new(
                family,
                PrimitiveConstructionRejectionClass::SpatialBirth,
                PrimitiveConstructionRejectionLocality::SpatialBirth,
                Vec::new(),
                None,
                None,
                None,
                None,
                error.to_string(),
            )
        }
        PrimitiveConstructionResultError::ImpossibleBirthAttachment(row) => {
            PrimitiveConstructionRejectedOutcome::new(
                family,
                PrimitiveConstructionRejectionClass::ImpossibleBirthAttachment,
                PrimitiveConstructionRejectionLocality::SpatialBirth,
                Vec::new(),
                None,
                None,
                None,
                None,
                row.reason().to_string(),
            )
        }
        PrimitiveConstructionResultError::Artifact(error) => {
            PrimitiveConstructionRejectedOutcome::new(
                family,
                PrimitiveConstructionRejectionClass::ArtifactAssembly,
                PrimitiveConstructionRejectionLocality::Artifact,
                Vec::new(),
                None,
                None,
                None,
                None,
                error.to_string(),
            )
        }
    }
}
