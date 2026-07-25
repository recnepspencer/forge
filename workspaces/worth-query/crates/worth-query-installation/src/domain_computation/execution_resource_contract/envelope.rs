use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionDegradation, WorthQueryExecutionMode,
    WorthQueryExecutionResourceRequest, WorthQueryPartialEffectPosture,
    WorthQueryResourceDimension, WorthQueryResourceLimitRequest, WorthQuerySemanticScaleAxis,
    WorthQuerySemanticScaleRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceEnvelope {
    scale_ceilings: WorthQuerySemanticScaleRequest,
    resource_ceilings: WorthQueryResourceLimitRequest,
    mode: WorthQueryExecutionMode,
    degradation: Option<WorthQueryExecutionDegradation>,
    partial_effect_posture: WorthQueryPartialEffectPosture,
    cancellation_safe_point: WorthQueryCancellationSafePointFamily,
}

impl WorthQueryExecutionResourceEnvelope {
    pub fn new(
        scale_ceilings: WorthQuerySemanticScaleRequest,
        resource_ceilings: WorthQueryResourceLimitRequest,
        mode: WorthQueryExecutionMode,
        degradation: Option<WorthQueryExecutionDegradation>,
        cancellation_safe_point: WorthQueryCancellationSafePointFamily,
    ) -> Self {
        Self {
            scale_ceilings,
            resource_ceilings,
            mode,
            degradation,
            partial_effect_posture: WorthQueryPartialEffectPosture::EffectFree,
            cancellation_safe_point,
        }
    }

    pub fn bounded(
        scale_ceiling: u64,
        resource_ceiling: u64,
        mode: WorthQueryExecutionMode,
        cancellation_safe_point: WorthQueryCancellationSafePointFamily,
    ) -> Self {
        Self::new(
            WorthQuerySemanticScaleRequest::bounded(scale_ceiling),
            WorthQueryResourceLimitRequest::bounded(resource_ceiling),
            mode,
            None,
            cancellation_safe_point,
        )
    }

    pub fn scale_ceiling(&self, axis: WorthQuerySemanticScaleAxis) -> u64 {
        self.scale_ceilings
            .get(axis)
            .expect("installed resource envelope has every semantic scale axis")
    }

    pub fn resource_ceiling(&self, dimension: WorthQueryResourceDimension) -> u64 {
        self.resource_ceilings
            .get(dimension)
            .expect("installed resource envelope has every resource dimension")
    }

    pub fn queue_depth_ceiling(&self) -> u64 {
        self.resource_ceiling(WorthQueryResourceDimension::QueueDepth)
    }

    pub fn concurrency_width_ceiling(&self) -> u64 {
        self.resource_ceiling(WorthQueryResourceDimension::ConcurrencyWidth)
    }

    pub fn scale_ceilings(&self) -> &WorthQuerySemanticScaleRequest {
        &self.scale_ceilings
    }

    pub fn resource_ceilings(&self) -> &WorthQueryResourceLimitRequest {
        &self.resource_ceilings
    }

    pub fn mode(&self) -> WorthQueryExecutionMode {
        self.mode
    }

    pub fn degradation(&self) -> Option<WorthQueryExecutionDegradation> {
        self.degradation
    }

    pub const fn partial_effect_posture(&self) -> WorthQueryPartialEffectPosture {
        self.partial_effect_posture
    }

    pub fn with_partial_effect_posture(mut self, posture: WorthQueryPartialEffectPosture) -> Self {
        self.partial_effect_posture = posture;
        self
    }

    pub fn cancellation_safe_point(&self) -> &WorthQueryCancellationSafePointFamily {
        &self.cancellation_safe_point
    }

    pub fn admits(&self, request: &WorthQueryExecutionResourceRequest) -> bool {
        request
            .scale()
            .iter()
            .all(|(axis, value)| value <= self.scale_ceiling(axis))
            && request
                .limits()
                .iter()
                .all(|(dimension, value)| value <= self.resource_ceiling(dimension))
            && request.modes().contains(&self.mode)
            && self
                .degradation
                .is_none_or(|degradation| request.degradations().contains(&degradation))
            && request
                .partial_effect_postures()
                .contains(&self.partial_effect_posture)
            && request.cancellation_safe_point() == self.cancellation_safe_point()
    }
}
