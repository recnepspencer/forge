use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryHostConditionalPredicateProvider, WorthQueryInstalledTemporalConditionalOperation,
    WorthQueryNamedClock, WorthQueryNamedClockSource, WorthQueryTemporalIntentProjector,
};
use worth_runtime_bridge::facade::BridgeManagedClockInstallationParts;

use super::installation::{
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind, WorthQueryPendingConditionalOperation,
};
use super::lifecycle::WorthQueryInstalledTemporalOperation;
use super::publication::ConditionalRuntimeAffinity;

pub(super) struct PendingTemporalOperation<Binding> {
    binding_identity: Arc<str>,
    binding: Binding,
}

impl<Binding> PendingTemporalOperation<Binding> {
    pub(super) fn new(binding_identity: Arc<str>, binding: Binding) -> Self {
        Self {
            binding_identity,
            binding,
        }
    }
}

impl<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        Node,
        Provider,
        Clock,
        Source,
        Query,
        Parameters,
        QueryResult,
        Scope,
        Projector,
    > WorthQueryPendingConditionalOperation<Schema>
    for PendingTemporalOperation<
        WorthQueryInstalledTemporalConditionalOperation<
            Schema,
            ApplicationOperation,
            Input,
            D,
            O,
            F,
            Node,
            Provider,
            Clock,
            Source,
            Query,
            Parameters,
            QueryResult,
            Scope,
            Projector,
        >,
    >
where
    Schema: 'static,
    ApplicationOperation: 'static,
    Input: 'static,
    D: 'static,
    O: 'static,
    F: 'static,
    Node: 'static,
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Query: 'static,
    Parameters: 'static,
    QueryResult: 'static,
    Scope: 'static,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
{
    fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    fn install(
        self: Box<Self>,
        bridge: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        affinity: &ConditionalRuntimeAffinity<'_>,
    ) -> Result<
        Box<dyn super::lifecycle::WorthQueryInstalledConditionalOperation>,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        let bounds = self.binding.bounds();
        let binding_identity = affinity.bind(&self.binding_identity);
        let clock = self.binding.clocked_node();
        let managed_clock = bridge
            .install_managed_clock(BridgeManagedClockInstallationParts {
                binding_identity,
                source_identity: Arc::from(clock.source_identity().as_str()),
                timeline_identity: Arc::from(clock.timeline_identity().as_str()),
                maximum_active_intents: bounds.maximum_reconstruction_rows(),
                maximum_due_wakes_per_observation: bounds.maximum_due_wakes_per_observation(),
            })
            .map_err(|denial| {
                WorthQueryConditionalRuntimeInstallationDenial::new(
                    WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
                    denial.detail(),
                )
            })?;
        Ok(Box::new(WorthQueryInstalledTemporalOperation {
            binding: self.binding,
            managed_clock,
        }))
    }
}

pub(super) fn temporal_binding_identity<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
    Node,
    Provider,
    Clock,
    Source,
    Query,
    Parameters,
    QueryResult,
    Scope,
    Projector,
>(
    binding: &WorthQueryInstalledTemporalConditionalOperation<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        Node,
        Provider,
        Clock,
        Source,
        Query,
        Parameters,
        QueryResult,
        Scope,
        Projector,
    >,
) -> Arc<str>
where
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
{
    let node = binding.clocked_node().provider().node();
    Arc::from(format!(
        "conditional:{}:clock={}:source={}:timeline={}:query={}:projector={}",
        node.authority_identity(),
        binding.clocked_node().clock_identity(),
        binding.clocked_node().source_identity().as_str(),
        binding.clocked_node().timeline_identity().as_str(),
        binding.query().identity().render_support_hex(),
        binding.projector_semantic_identity(),
    ))
}
