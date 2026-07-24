use std::collections::BTreeSet;
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
        && admitted.degradation() == support.degradation()
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

pub(crate) struct WorthQueryExecutionResourceSupportSnapshotParts {
    pub(crate) executor: WorthQueryExecutionResourceSupport,
    pub(crate) conditional_nodes: Vec<(String, WorthQueryExecutionResourceSupport)>,
    pub(crate) graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    pub(crate) commit_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
    pub(crate) parallel_admission: Option<WorthQueryExecutionResourceSupport>,
}

impl WorthQueryExecutionResourceSupportSnapshot {
    pub(crate) fn new(mut parts: WorthQueryExecutionResourceSupportSnapshotParts) -> Self {
        parts
            .conditional_nodes
            .sort_by(|left, right| left.0.cmp(&right.0));
        parts
            .graph_providers
            .sort_by(|left, right| left.0.cmp(&right.0));
        parts
            .commit_providers
            .sort_by(|left, right| left.0.cmp(&right.0));
        let executor = parts.executor;
        let conditional_nodes = parts.conditional_nodes;
        let graph_providers = parts.graph_providers;
        let commit_providers = parts.commit_providers;
        let parallel_admission = parts.parallel_admission;
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
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

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn supports(&self, strategy: &WorthQueryExecutionStrategyContract) -> bool {
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

    pub(crate) fn first_mismatch(
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

pub(crate) fn operation_conditional_supports<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    bound
        .conditional_nodes()
        .iter()
        .filter_map(|node| {
            match node.lowering.location() {
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::Operation {
                node_identity,
            } => Some((
                format!("operation:{node_identity}"),
                node.resource_support.clone(),
            )),
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                ..
            } => None,
        }
        })
        .collect()
}

pub(crate) fn stage_conditional_supports<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    expected_stage: &str,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    bound
        .conditional_nodes()
        .iter()
        .filter_map(|node| {
            match node.lowering.location() {
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                stage_identity,
                node_identity,
            } if stage_identity == expected_stage => Some((
                format!("stage:{stage_identity}:{node_identity}"),
                node.resource_support.clone(),
            )),
            _ => None,
        }
        })
        .collect()
}

pub(crate) fn commit_supports_for_roles<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    roles: &BTreeSet<&str>,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    if bound.commit_posture() != crate::domain_installation::WorthQueryBoundCommitPosture::Atomic {
        return Vec::new();
    }
    let mut groups = Vec::<(
        Arc<crate::domain_installation::graph_participation::WorthQueryInstalledGraphCommitAuthority>,
        Vec<String>,
    )>::new();
    for participation in bound
        .graph_participations()
        .iter()
        .filter(|participation| roles.contains(participation.role.as_str()))
    {
        let Some(authority) = &participation.record.commit_authority else {
            continue;
        };
        match groups.iter_mut().find(|(candidate, _)| {
            Arc::ptr_eq(candidate, authority) && candidate.identity() == authority.identity()
        }) {
            Some((_, group_roles)) => group_roles.push(participation.role.clone()),
            None => groups.push((Arc::clone(authority), vec![participation.role.clone()])),
        }
    }
    groups
        .into_iter()
        .map(|(authority, mut group_roles)| {
            group_roles.sort();
            (group_roles.join(","), authority.resource_support.clone())
        })
        .collect()
}
