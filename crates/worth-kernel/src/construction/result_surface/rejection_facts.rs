use super::super::digest::digest_owned_parts;
use super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use super::super::result::PrimitiveConstructionResultError;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use super::outcome_rejection::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionRejectedFacts {
    rejection_class: PrimitiveConstructionRejectionClass,
    rejection_locality: PrimitiveConstructionRejectionLocality,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_fact_digest: Option<String>,
    reason: String,
    failure_digest: String,
}

impl PrimitiveConstructionRejectedFacts {
    fn new(
        family: PrimitiveConstructionFamily,
        rejection_class: PrimitiveConstructionRejectionClass,
        rejection_locality: PrimitiveConstructionRejectionLocality,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        conditioning_witness: Option<PrimitiveConditioningWitness>,
        exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
        stability_class: Option<PrimitiveStabilityClass>,
        exhaustion_fact_digest: Option<String>,
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
            exhaustion_fact_digest.clone().unwrap_or_default(),
            reason.clone(),
        ]);
        Self {
            rejection_class,
            rejection_locality,
            attempted_realization_strategies,
            conditioning_witness,
            exhaustion_reason,
            stability_class,
            exhaustion_fact_digest,
            reason,
            failure_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    #[allow(dead_code)]
    pub(crate) fn rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub(crate) fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.conditioning_witness.as_ref()
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    #[allow(dead_code)]
    pub(crate) fn feature_conditioning_class(
        &self,
    ) -> Option<worth_geom::facade::PrimitiveFeatureConditioningClass> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.feature_conditioning_class())
    }

    #[allow(dead_code)]
    pub(crate) fn support_normal_class(
        &self,
    ) -> Option<worth_geom::facade::PrimitiveSupportNormalClass> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.support_normal_class())
    }

    #[allow(dead_code)]
    pub(crate) fn normalization_disposition(
        &self,
    ) -> Option<worth_geom::facade::PrimitiveNormalizationDisposition> {
        self.conditioning_witness
            .as_ref()
            .map(|witness| witness.normalization_disposition())
    }

    #[cfg(test)]
    pub(crate) fn exhaustion_fact_digest(&self) -> Option<&str> {
        self.exhaustion_fact_digest.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }

    #[allow(dead_code)]
    pub(crate) fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

pub(crate) fn prepare_primitive_construction_rejected_facts(
    family: PrimitiveConstructionFamily,
    error: &PrimitiveConstructionResultError,
) -> PrimitiveConstructionRejectedFacts {
    match error {
        PrimitiveConstructionResultError::MissingExecutedGraphAuthorityEvidence => {
            PrimitiveConstructionRejectedFacts::new(
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
        PrimitiveConstructionResultError::Phase(phase) => match phase {
            PrimitiveConstructionPhaseError::InvalidRequest { .. } => {
                PrimitiveConstructionRejectedFacts::new(
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
                    PrimitiveConstructionRejectedFacts::new(
                        family,
                        PrimitiveConstructionRejectionClass::ConditioningExhaustion,
                        PrimitiveConstructionRejectionLocality::Scaffold,
                        report.attempted_strategies().to_vec(),
                        Some(report.conditioning_witness().clone()),
                        Some(report.exhaustion_reason()),
                        Some(report.stability_class()),
                        Some(report.fact_digest().to_string()),
                        phase.to_string(),
                    )
                }
                PrimitiveConstructionGeometryError::GeometryFailure(_) => {
                    PrimitiveConstructionRejectedFacts::new(
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
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(error) => match error {
                topology::facade::TopologyConstructionQueryAdmittedHandoffError::Handoff(_) => {
                    PrimitiveConstructionRejectedFacts::new(
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
                topology::facade::TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                    _,
                ) => PrimitiveConstructionRejectedFacts::new(
                    family,
                    PrimitiveConstructionRejectionClass::SpatialBirth,
                    PrimitiveConstructionRejectionLocality::SpatialBirth,
                    Vec::new(),
                    None,
                    None,
                    None,
                    None,
                    error.to_string(),
                ),
                topology::facade::TopologyConstructionQueryAdmittedHandoffError::ImpossibleBirthAttachment(
                    reason,
                ) => PrimitiveConstructionRejectedFacts::new(
                    family,
                    PrimitiveConstructionRejectionClass::ImpossibleBirthAttachment,
                    PrimitiveConstructionRejectionLocality::SpatialBirth,
                    Vec::new(),
                    None,
                    None,
                    None,
                    None,
                    reason.to_string(),
                ),
            },
            PrimitiveConstructionPhaseError::TopologyBirthCompose(error) => {
                PrimitiveConstructionRejectedFacts::new(
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
        },
    }
}
