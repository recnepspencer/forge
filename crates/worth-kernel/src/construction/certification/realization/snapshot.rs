use crate::construction::outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
};
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionRequest};
use crate::construction::result::{
    prepare_primitive_construction_result, PreparedPrimitiveConstructionResult,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionRealizationSnapshot {
    family: PrimitiveConstructionFamily,
    admitted: bool,
    selected_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    canonical_artifact_digest: Option<String>,
    realization_digest: String,
}

impl PrimitiveConstructionRealizationSnapshot {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn admitted(&self) -> bool {
        self.admitted
    }

    pub(crate) fn selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.selected_strategy
    }

    pub(crate) fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub(crate) fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.conditioning_witness.as_ref()
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub(crate) fn canonical_artifact_digest(&self) -> Option<&str> {
        self.canonical_artifact_digest.as_deref()
    }

    pub(crate) fn realization_digest(&self) -> &str {
        &self.realization_digest
    }
}

pub(crate) fn prepare_realization_snapshot(
    request: PrimitiveConstructionRequest,
) -> PrimitiveConstructionRealizationSnapshot {
    let family = request.family();
    match prepare_primitive_construction_result(request.clone()) {
        Ok(result) => accepted_snapshot(family, &result),
        Err(_) => rejected_snapshot(prepare_primitive_construction_outcome(request)),
    }
}

fn accepted_snapshot(
    family: PrimitiveConstructionFamily,
    result: &PreparedPrimitiveConstructionResult,
) -> PrimitiveConstructionRealizationSnapshot {
    let artifact = result.canonical_artifact();
    PrimitiveConstructionRealizationSnapshot {
        family,
        admitted: true,
        selected_strategy: Some(artifact.realization_strategy()),
        attempted_strategies: artifact
            .realization_report()
            .attempted_strategies()
            .to_vec(),
        conditioning_witness: Some(artifact.realization_report().conditioning_witness().clone()),
        stability_class: Some(artifact.stability_class()),
        exhaustion_reason: None,
        canonical_artifact_digest: Some(artifact.artifact_digest().to_string()),
        realization_digest: artifact.realization_report().report_digest().to_string(),
    }
}

fn rejected_snapshot(
    outcome: PrimitiveConstructionPreparedOutcome,
) -> PrimitiveConstructionRealizationSnapshot {
    match outcome {
        PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
            PrimitiveConstructionRealizationSnapshot {
                family: outcome.family(),
                admitted: true,
                selected_strategy: Some(outcome.realization_strategy()),
                attempted_strategies: outcome.attempted_realization_strategies().to_vec(),
                conditioning_witness: None,
                stability_class: Some(outcome.stability_class()),
                exhaustion_reason: None,
                canonical_artifact_digest: Some(outcome.canonical_artifact_digest().to_string()),
                realization_digest: outcome.outcome_digest().to_string(),
            }
        }
        PrimitiveConstructionPreparedOutcome::Rejected(outcome) => {
            PrimitiveConstructionRealizationSnapshot {
                family: outcome.family(),
                admitted: false,
                selected_strategy: None,
                attempted_strategies: outcome.attempted_realization_strategies().to_vec(),
                conditioning_witness: outcome.conditioning_witness().cloned(),
                stability_class: outcome.stability_class(),
                exhaustion_reason: outcome.exhaustion_reason(),
                canonical_artifact_digest: None,
                realization_digest: outcome
                    .exhaustion_report_digest()
                    .unwrap_or(outcome.failure_digest())
                    .to_string(),
            }
        }
    }
}
