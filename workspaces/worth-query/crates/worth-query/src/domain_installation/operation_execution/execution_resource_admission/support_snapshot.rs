use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceSupport {
    provider: WorthQueryExecutionProviderFamily,
    access_product: WorthQueryExecutionAccessProductFamily,
    allocator: WorthQueryExecutionAllocatorFamily,
    envelope: WorthQueryExecutionResourceEnvelope,
    identity: Arc<str>,
}

impl WorthQueryExecutionResourceSupport {
    pub fn new(
        provider: WorthQueryExecutionProviderFamily,
        access_product: WorthQueryExecutionAccessProductFamily,
        allocator: WorthQueryExecutionAllocatorFamily,
        envelope: WorthQueryExecutionResourceEnvelope,
    ) -> Self {
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_execution_resource_support_v1".into(),
            format!("provider:{}", provider.as_str()),
            format!("access:{}", access_product.as_str()),
            format!("allocator:{}", allocator.as_str()),
            format!("mode:{}", envelope.mode().as_str()),
            format!("safe-point:{}", envelope.cancellation_safe_point().as_str()),
            format!(
                "degradation:{}",
                envelope
                    .degradation()
                    .map_or("complete", |degradation| degradation.as_str())
            ),
            format!(
                "scale:{}",
                envelope
                    .scale_ceilings()
                    .iter()
                    .map(|(axis, value)| format!("{}={value}", axis.as_str()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "resources:{}",
                envelope
                    .resource_ceilings()
                    .iter()
                    .map(|(dimension, value)| format!("{}={value}", dimension.as_str()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]));
        Self {
            provider,
            access_product,
            allocator,
            envelope,
            identity,
        }
    }

    pub fn provider(&self) -> &WorthQueryExecutionProviderFamily {
        &self.provider
    }

    pub fn access_product(&self) -> &WorthQueryExecutionAccessProductFamily {
        &self.access_product
    }

    pub fn allocator(&self) -> &WorthQueryExecutionAllocatorFamily {
        &self.allocator
    }

    pub fn envelope(&self) -> &WorthQueryExecutionResourceEnvelope {
        &self.envelope
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn supports(&self, strategy: &WorthQueryExecutionStrategyContract) -> bool {
        let required = strategy.provider_requirements();
        self.provider == *required.provider()
            && self.access_product == *required.access_product()
            && self.allocator == *required.allocator()
            && covers(&self.envelope, strategy.envelope())
    }
}

fn covers(
    support: &WorthQueryExecutionResourceEnvelope,
    admitted: &WorthQueryExecutionResourceEnvelope,
) -> bool {
    admitted
        .scale_ceilings()
        .iter()
        .all(|(axis, value)| value <= support.scale_ceiling(axis))
        && admitted
            .resource_ceilings()
            .iter()
            .all(|(dimension, value)| value <= support.resource_ceiling(dimension))
        && admitted.mode() == support.mode()
        && admitted.cancellation_safe_point() == support.cancellation_safe_point()
        && match admitted.degradation() {
            None => true,
            Some(degradation) => support.degradation() == Some(degradation),
        }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceSupportSnapshot {
    executor: WorthQueryExecutionResourceSupport,
    graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    identity: Arc<str>,
}

impl WorthQueryExecutionResourceSupportSnapshot {
    pub(crate) fn new(
        executor: WorthQueryExecutionResourceSupport,
        mut graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    ) -> Self {
        graph_providers.sort_by(|left, right| left.0.cmp(&right.0));
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_execution_resource_support_snapshot_v1".into(),
            format!("executor:{}", executor.identity()),
            format!(
                "graphs:{}",
                graph_providers
                    .iter()
                    .map(|(role, support)| format!("{role}:{}", support.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]));
        Self {
            executor,
            graph_providers,
            identity,
        }
    }

    pub fn executor(&self) -> &WorthQueryExecutionResourceSupport {
        &self.executor
    }

    pub fn graph_providers(&self) -> &[(String, WorthQueryExecutionResourceSupport)] {
        &self.graph_providers
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn supports(&self, strategy: &WorthQueryExecutionStrategyContract) -> bool {
        self.executor.supports(strategy)
            && self
                .graph_providers
                .iter()
                .all(|(_, support)| support.supports(strategy))
    }
}
