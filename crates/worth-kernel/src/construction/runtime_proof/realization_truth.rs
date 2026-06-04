use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
#[cfg(test)]
use crate::construction::result::PreparedPrimitiveConstructionResult;
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionRuntimeRealizationTruth {
    selected_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    truth_digest: String,
}

impl PrimitiveConstructionRuntimeRealizationTruth {
    pub(crate) fn from_outcome(outcome: &PrimitiveConstructionPreparedOutcome) -> Self {
        Self::new(
            outcome.realization_strategy(),
            outcome.attempted_realization_strategies().to_vec(),
            outcome.stability_class(),
            outcome.feature_conditioning_class(),
            outcome.support_normal_class(),
            outcome.normalization_disposition(),
            outcome.exhaustion_reason(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_prepared_result(prepared: &PreparedPrimitiveConstructionResult) -> Self {
        Self::new(
            Some(prepared.realization_strategy()),
            prepared.attempted_realization_strategies().to_vec(),
            Some(prepared.stability_class()),
            Some(prepared.feature_conditioning_class()),
            Some(prepared.support_normal_class()),
            Some(prepared.normalization_disposition()),
            None,
        )
    }

    fn new(
        selected_strategy: Option<PrimitiveRealizationStrategy>,
        attempted_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: Option<PrimitiveStabilityClass>,
        feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
        support_normal_class: Option<PrimitiveSupportNormalClass>,
        normalization_disposition: Option<PrimitiveNormalizationDisposition>,
        exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    ) -> Self {
        let truth_digest = digest_owned_parts(&[
            selected_strategy
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            attempted_strategies
                .iter()
                .map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            stability_class
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            feature_conditioning_class
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            support_normal_class
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            normalization_disposition
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            exhaustion_reason
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
        ]);
        Self {
            selected_strategy,
            attempted_strategies,
            stability_class,
            feature_conditioning_class,
            support_normal_class,
            normalization_disposition,
            exhaustion_reason,
            truth_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.selected_strategy
    }

    #[cfg(test)]
    pub(crate) fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    #[cfg(test)]
    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    #[cfg(test)]
    pub(crate) fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.feature_conditioning_class
    }

    #[cfg(test)]
    pub(crate) fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }

    #[cfg(test)]
    pub(crate) fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    #[cfg(test)]
    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub(crate) fn truth_digest(&self) -> &str {
        &self.truth_digest
    }
}
