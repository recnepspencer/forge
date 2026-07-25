use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract,
};

use crate::admission_digest::hash_parts;

pub trait WorthQueryExecutionCapacityReservation: Send {}

impl<T: Send> WorthQueryExecutionCapacityReservation for T {}

pub trait WorthQueryExecutionCapacityPort: Send + Sync {
    fn capacity_subject_identity(&self) -> &str;

    fn try_reserve(&self) -> Option<Box<dyn WorthQueryExecutionCapacityReservation>>;
}

#[derive(Clone)]
pub struct WorthQueryExecutionResourceSupport {
    provider: WorthQueryExecutionProviderFamily,
    access_product: WorthQueryExecutionAccessProductFamily,
    allocator: WorthQueryExecutionAllocatorFamily,
    envelope: WorthQueryExecutionResourceEnvelope,
    capacity: Arc<dyn WorthQueryExecutionCapacityPort>,
    identity: Arc<str>,
}

impl WorthQueryExecutionResourceSupport {
    pub fn new(
        provider: WorthQueryExecutionProviderFamily,
        access_product: WorthQueryExecutionAccessProductFamily,
        allocator: WorthQueryExecutionAllocatorFamily,
        envelope: WorthQueryExecutionResourceEnvelope,
        capacity: Arc<dyn WorthQueryExecutionCapacityPort>,
    ) -> Self {
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_resource_support_v1".into(),
            format!("provider:{}", provider.as_str()),
            format!("access:{}", access_product.as_str()),
            format!("allocator:{}", allocator.as_str()),
            format!("capacity:{}", capacity.capacity_subject_identity()),
            format!("mode:{}", envelope.mode().as_str()),
            format!("safe-point:{}", envelope.cancellation_safe_point().as_str()),
            format!(
                "degradation:{}",
                envelope
                    .degradation()
                    .map_or("complete", |degradation| degradation.as_str())
            ),
            format!(
                "partial-effect:{}",
                envelope.partial_effect_posture().as_str()
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
            capacity,
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

    pub fn capacity_subject_identity(&self) -> &str {
        self.capacity.capacity_subject_identity()
    }

    pub(super) fn capacity(&self) -> &Arc<dyn WorthQueryExecutionCapacityPort> {
        &self.capacity
    }

    pub(super) fn has_same_capacity_authority(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.capacity, &other.capacity)
    }

    fn supports(&self, strategy: &WorthQueryExecutionStrategyContract) -> bool {
        let required = strategy.provider_requirements();
        self.provider == *required.provider()
            && self.access_product == *required.access_product()
            && self.allocator == *required.allocator()
            && covers(&self.envelope, strategy.envelope())
    }
}

impl std::fmt::Debug for WorthQueryExecutionResourceSupport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryExecutionResourceSupport")
            .field("provider", &self.provider)
            .field("access_product", &self.access_product)
            .field("allocator", &self.allocator)
            .field("envelope", &self.envelope)
            .field("capacity_subject", &self.capacity_subject_identity())
            .finish()
    }
}

impl PartialEq for WorthQueryExecutionResourceSupport {
    fn eq(&self, other: &Self) -> bool {
        self.provider == other.provider
            && self.access_product == other.access_product
            && self.allocator == other.allocator
            && self.envelope == other.envelope
            && self.capacity_subject_identity() == other.capacity_subject_identity()
            && self.has_same_capacity_authority(other)
    }
}

impl Eq for WorthQueryExecutionResourceSupport {}

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
        && admitted.degradation() == support.degradation()
        && admitted.partial_effect_posture() == support.partial_effect_posture()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceSupportSnapshot {
    executor: WorthQueryExecutionResourceSupport,
    conditional_nodes: Vec<(String, WorthQueryExecutionResourceSupport)>,
    graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    commit_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    parallel_admission: Option<WorthQueryExecutionResourceSupport>,
    identity: Arc<str>,
}

impl WorthQueryExecutionResourceSupportSnapshot {
    pub fn new(
        executor: WorthQueryExecutionResourceSupport,
        mut conditional_nodes: Vec<(String, WorthQueryExecutionResourceSupport)>,
        mut graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
        mut commit_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
        parallel_admission: Option<WorthQueryExecutionResourceSupport>,
    ) -> Self {
        conditional_nodes.sort_by(|left, right| left.0.cmp(&right.0));
        graph_providers.sort_by(|left, right| left.0.cmp(&right.0));
        commit_providers.sort_by(|left, right| left.0.cmp(&right.0));
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_resource_support_snapshot_v1".into(),
            format!("executor:{}", executor.identity()),
            format!(
                "conditionals:{}",
                conditional_nodes
                    .iter()
                    .map(|(location, support)| format!("{location}:{}", support.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "graphs:{}",
                graph_providers
                    .iter()
                    .map(|(role, support)| format!("{role}:{}", support.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "commits:{}",
                commit_providers
                    .iter()
                    .map(|(group, support)| format!("{group}:{}", support.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "parallel:{}",
                parallel_admission
                    .as_ref()
                    .map_or("none", WorthQueryExecutionResourceSupport::identity)
            ),
        ]));
        Self {
            executor,
            conditional_nodes,
            graph_providers,
            commit_providers,
            parallel_admission,
            identity,
        }
    }

    pub fn executor(&self) -> &WorthQueryExecutionResourceSupport {
        &self.executor
    }

    pub fn conditional_nodes(&self) -> &[(String, WorthQueryExecutionResourceSupport)] {
        &self.conditional_nodes
    }

    pub fn graph_providers(&self) -> &[(String, WorthQueryExecutionResourceSupport)] {
        &self.graph_providers
    }

    pub fn commit_providers(&self) -> &[(String, WorthQueryExecutionResourceSupport)] {
        &self.commit_providers
    }

    pub fn parallel_admission(&self) -> Option<&WorthQueryExecutionResourceSupport> {
        self.parallel_admission.as_ref()
    }

    pub(super) fn all_supports(&self) -> impl Iterator<Item = &WorthQueryExecutionResourceSupport> {
        std::iter::once(&self.executor)
            .chain(self.conditional_nodes.iter().map(|(_, support)| support))
            .chain(self.graph_providers.iter().map(|(_, support)| support))
            .chain(self.commit_providers.iter().map(|(_, support)| support))
            .chain(self.parallel_admission.iter())
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) fn supports(&self, strategy: &WorthQueryExecutionStrategyContract) -> bool {
        self.executor.supports(strategy)
            && self
                .conditional_nodes
                .iter()
                .all(|(_, support)| support.supports(strategy))
            && self
                .graph_providers
                .iter()
                .all(|(_, support)| support.supports(strategy))
            && self
                .commit_providers
                .iter()
                .all(|(_, support)| support.supports(strategy))
            && self
                .parallel_admission
                .as_ref()
                .is_none_or(|support| support.supports(strategy))
    }

    pub(super) fn first_mismatch(
        &self,
        strategy: &WorthQueryExecutionStrategyContract,
    ) -> Option<(String, &WorthQueryExecutionResourceSupport)> {
        if !self.executor.supports(strategy) {
            return Some(("executor".into(), &self.executor));
        }
        if let Some((location, support)) = self
            .conditional_nodes
            .iter()
            .find(|(_, support)| !support.supports(strategy))
        {
            return Some((format!("conditional node `{location}`"), support));
        }
        if let Some((role, support)) = self
            .graph_providers
            .iter()
            .find(|(_, support)| !support.supports(strategy))
        {
            return Some((format!("graph role `{role}`"), support));
        }
        if let Some((group, support)) = self
            .commit_providers
            .iter()
            .find(|(_, support)| !support.supports(strategy))
        {
            return Some((format!("commit group `{group}`"), support));
        }
        self.parallel_admission
            .as_ref()
            .filter(|support| !support.supports(strategy))
            .map(|support| ("parallel admission provider".into(), support))
    }
}
