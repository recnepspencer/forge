use std::collections::BTreeSet;

use super::{
    canonical_identity::canonical_resource_request_identity, validation::validate_resource_request,
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionDegradation, WorthQueryExecutionMode,
    WorthQueryPartialEffectPosture, WorthQueryResourceLimitRequest, WorthQuerySemanticScaleRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceRequest {
    scale: WorthQuerySemanticScaleRequest,
    limits: WorthQueryResourceLimitRequest,
    modes: BTreeSet<WorthQueryExecutionMode>,
    degradations: BTreeSet<WorthQueryExecutionDegradation>,
    partial_effect_postures: BTreeSet<WorthQueryPartialEffectPosture>,
    cancellation_safe_point: WorthQueryCancellationSafePointFamily,
}

impl WorthQueryExecutionResourceRequest {
    pub fn new(
        scale: WorthQuerySemanticScaleRequest,
        limits: WorthQueryResourceLimitRequest,
        cancellation_safe_point: WorthQueryCancellationSafePointFamily,
    ) -> Result<Self, &'static str> {
        let request = Self {
            scale,
            limits,
            modes: [WorthQueryExecutionMode::Synchronous].into_iter().collect(),
            degradations: BTreeSet::new(),
            partial_effect_postures: [WorthQueryPartialEffectPosture::EffectFree]
                .into_iter()
                .collect(),
            cancellation_safe_point,
        };
        validate_resource_request(&request)?;
        Ok(request)
    }

    pub fn bounded(
        scale_ceiling: u64,
        resource_ceiling: u64,
        cancellation_safe_point: WorthQueryCancellationSafePointFamily,
    ) -> Self {
        Self::new(
            WorthQuerySemanticScaleRequest::bounded(scale_ceiling),
            WorthQueryResourceLimitRequest::bounded(resource_ceiling),
            cancellation_safe_point,
        )
        .expect("complete bounded resource request is valid")
    }

    pub fn allow_mode(mut self, mode: WorthQueryExecutionMode) -> Self {
        self.modes.insert(mode);
        self
    }

    pub fn allow_degradation(mut self, degradation: WorthQueryExecutionDegradation) -> Self {
        self.degradations.insert(degradation);
        self
    }

    pub fn allow_partial_effect_posture(mut self, posture: WorthQueryPartialEffectPosture) -> Self {
        self.partial_effect_postures.insert(posture);
        self
    }

    pub fn scale(&self) -> &WorthQuerySemanticScaleRequest {
        &self.scale
    }

    pub fn limits(&self) -> &WorthQueryResourceLimitRequest {
        &self.limits
    }

    pub fn modes(&self) -> &BTreeSet<WorthQueryExecutionMode> {
        &self.modes
    }

    pub fn degradations(&self) -> &BTreeSet<WorthQueryExecutionDegradation> {
        &self.degradations
    }

    pub fn partial_effect_postures(&self) -> &BTreeSet<WorthQueryPartialEffectPosture> {
        &self.partial_effect_postures
    }

    pub fn cancellation_safe_point(&self) -> &WorthQueryCancellationSafePointFamily {
        &self.cancellation_safe_point
    }

    pub fn canonical_identity(&self) -> String {
        canonical_resource_request_identity(self)
    }
}
