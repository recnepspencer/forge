#[path = "outcome_rejection.rs"]
mod outcome_rejection;

use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::PrimitiveConstructionIntent;
pub(crate) use outcome_rejection::rejected_outcome;
pub use outcome_rejection::{
    PrimitiveConstructionRejectedOutcome, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionAcceptedOutcome {
    family: PrimitiveConstructionFamily,
    topology_birth_class: String,
    canonical_artifact_digest: String,
    result_digest: String,
    birth_truth_digest: String,
    birth_completeness_digest: String,
    topology_fact_digest: String,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    feature_conditioning_class: PrimitiveFeatureConditioningClass,
    support_normal_class: PrimitiveSupportNormalClass,
    normalization_disposition: PrimitiveNormalizationDisposition,
    outcome_digest: String,
}

impl PrimitiveConstructionAcceptedOutcome {
    fn from_prepared_result(
        family: PrimitiveConstructionFamily,
        prepared: &crate::construction::result::PreparedPrimitiveConstructionResult,
    ) -> Self {
        let canonical_artifact = prepared.canonical_artifact();
        let outcome_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            canonical_artifact.topology_birth_class().to_string(),
            canonical_artifact.artifact_digest().to_string(),
            prepared.result_digest().to_string(),
            canonical_artifact
                .realization_strategy()
                .as_str()
                .to_string(),
            canonical_artifact
                .attempted_realization_strategies()
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            canonical_artifact.stability_class().as_str().to_string(),
            canonical_artifact
                .feature_conditioning_class()
                .as_str()
                .to_string(),
            canonical_artifact
                .support_normal_class()
                .as_str()
                .to_string(),
            canonical_artifact
                .normalization_disposition()
                .as_str()
                .to_string(),
        ]);
        Self {
            family,
            topology_birth_class: canonical_artifact.topology_birth_class().to_string(),
            canonical_artifact_digest: canonical_artifact.artifact_digest().to_string(),
            result_digest: prepared.result_digest().to_string(),
            birth_truth_digest: canonical_artifact.birth_truth_digest().to_string(),
            birth_completeness_digest: canonical_artifact.birth_completeness_digest().to_string(),
            topology_fact_digest: canonical_artifact.topology_fact_digest().to_string(),
            realization_strategy: canonical_artifact.realization_strategy(),
            attempted_realization_strategies: canonical_artifact
                .attempted_realization_strategies()
                .to_vec(),
            stability_class: canonical_artifact.stability_class(),
            feature_conditioning_class: canonical_artifact.feature_conditioning_class(),
            support_normal_class: canonical_artifact.support_normal_class(),
            normalization_disposition: canonical_artifact.normalization_disposition(),
            outcome_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn canonical_artifact_digest(&self) -> &str {
        &self.canonical_artifact_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn birth_truth_digest(&self) -> &str {
        &self.birth_truth_digest
    }

    pub fn birth_completeness_digest(&self) -> &str {
        &self.birth_completeness_digest
    }

    pub fn topology_fact_digest(&self) -> &str {
        &self.topology_fact_digest
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_strategy
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.attempted_realization_strategies.len()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.feature_conditioning_class
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.normalization_disposition
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreparedOutcome {
    Accepted(PrimitiveConstructionAcceptedOutcome),
    Rejected(PrimitiveConstructionRejectedOutcome),
}

impl PrimitiveConstructionPreparedOutcome {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        match self {
            Self::Accepted(outcome) => outcome.family(),
            Self::Rejected(outcome) => outcome.family(),
        }
    }

    pub fn outcome_digest(&self) -> &str {
        match self {
            Self::Accepted(outcome) => outcome.outcome_digest(),
            Self::Rejected(outcome) => outcome.failure_digest(),
        }
    }

    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        match self {
            Self::Accepted(outcome) => Some(outcome.realization_strategy()),
            Self::Rejected(outcome) => outcome.selected_realization_strategy(),
        }
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        match self {
            Self::Accepted(outcome) => outcome.attempted_realization_strategy_count(),
            Self::Rejected(outcome) => outcome.attempted_realization_strategy_count(),
        }
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        match self {
            Self::Accepted(outcome) => outcome.attempted_realization_strategies(),
            Self::Rejected(outcome) => outcome.attempted_realization_strategies(),
        }
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        match self {
            Self::Accepted(outcome) => Some(outcome.stability_class()),
            Self::Rejected(outcome) => outcome.stability_class(),
        }
    }

    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        match self {
            Self::Accepted(outcome) => Some(outcome.feature_conditioning_class()),
            Self::Rejected(outcome) => outcome.feature_conditioning_class(),
        }
    }

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        match self {
            Self::Accepted(outcome) => Some(outcome.support_normal_class()),
            Self::Rejected(outcome) => outcome.support_normal_class(),
        }
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        match self {
            Self::Accepted(outcome) => Some(outcome.normalization_disposition()),
            Self::Rejected(outcome) => outcome.normalization_disposition(),
        }
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(outcome) => outcome.exhaustion_reason(),
        }
    }
}

pub fn prepare_primitive_construction_outcome<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> PrimitiveConstructionPreparedOutcome {
    let intent = intent.into();
    let family = intent.family();
    match prepare_primitive_construction_result(intent) {
        Ok(prepared) => PrimitiveConstructionPreparedOutcome::Accepted(
            PrimitiveConstructionAcceptedOutcome::from_prepared_result(family, &prepared),
        ),
        Err(error) => {
            PrimitiveConstructionPreparedOutcome::Rejected(rejected_outcome(family, &error))
        }
    }
}

#[cfg(test)]
#[path = "../tests/outcome.rs"]
mod tests;
