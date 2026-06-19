#[cfg(test)]
use super::super::digest::digest_owned_parts;
#[cfg(test)]
use super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
#[cfg(test)]
use super::super::result::PrimitiveConstructionResultError;
#[cfg(test)]
use super::geometry_recovery::{
    geometry_recovery_actions_for_rejection_class, geometry_recovery_receipts_for_rejection_class,
    GeometryRecoveryActionFactReceipt, PrimitiveConstructionRecoveryAction,
};
#[cfg(test)]
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionRejectionClass {
    InvalidRequest,
    GeometryScaffold,
    ConditioningExhaustion,
    SpatialBirth,
    ImpossibleBirthAttachment,
    TopologyExecution,
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
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionRejectionLocality {
    Admission,
    Scaffold,
    SpatialBirth,
    Execution,
}

impl PrimitiveConstructionRejectionLocality {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::Scaffold => "scaffold",
            Self::SpatialBirth => "spatial_birth",
            Self::Execution => "execution",
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRejectedOutcome {
    family: PrimitiveConstructionFamily,
    rejection_class: PrimitiveConstructionRejectionClass,
    rejection_locality: PrimitiveConstructionRejectionLocality,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_fact_digest: Option<String>,
    recovery_fact_receipts: Vec<GeometryRecoveryActionFactReceipt>,
    reason: String,
    failure_digest: String,
}

#[cfg(test)]
impl PrimitiveConstructionRejectedOutcome {
    pub(crate) fn new(
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
        let recovery_fact_receipts =
            geometry_recovery_receipts_for_rejection_class(family, rejection_class);
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
            family,
            rejection_class,
            rejection_locality,
            attempted_realization_strategies,
            conditioning_witness,
            exhaustion_reason,
            stability_class,
            exhaustion_fact_digest,
            recovery_fact_receipts,
            reason,
            failure_digest,
        }
    }

    #[cfg(test)]
    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub fn rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    #[cfg(test)]
    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn recovery_actions(&self) -> &'static [PrimitiveConstructionRecoveryAction] {
        geometry_recovery_actions_for_rejection_class(self.rejection_class)
    }

    #[cfg(test)]
    pub fn recovery_fact_receipts(&self) -> &[GeometryRecoveryActionFactReceipt] {
        &self.recovery_fact_receipts
    }
}

#[cfg(test)]
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
                        Some(report.fact_digest().to_string()),
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
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(error) => match error {
                topology::facade::TopologyConstructionQueryAdmittedHandoffError::Handoff(_) => {
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
                topology::facade::TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                    _,
                ) => PrimitiveConstructionRejectedOutcome::new(
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
                ) => PrimitiveConstructionRejectedOutcome::new(
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
        },
    }
}
