use std::sync::Arc;

use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryHostConditionalPredicateProvider,
    WorthQueryInstalledTemporalConditionalOperation, WorthQueryNamedClock,
    WorthQueryNamedClockSource, WorthQueryTemporalIntentProjector,
};
use worth_runtime_bridge::facade::{BridgeManagedClockInstallationParts, BridgeOwnedSignalRuntime};

use super::WorthQueryPreparedConditionalRuntimeBinding;
use crate::domain_computation::primary_graph::conditional_operation::{
    installation::{
        WorthQueryConditionalRuntimeInstallationDenial,
        WorthQueryConditionalRuntimeInstallationDenialKind,
    },
    publication::ConditionalRuntimeAffinity,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn prepare_temporal_runtime_binding<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
    Node: 'static,
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
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    bridge: &mut BridgeOwnedSignalRuntime,
    affinity: &ConditionalRuntimeAffinity,
    binding_identity: &crate::domain_computation::primary_graph::conditional_operation::canonical_identity::WorthQueryTemporalBindingIdentity,
) -> Result<
    WorthQueryPreparedConditionalRuntimeBinding,
    WorthQueryConditionalRuntimeInstallationDenial,
>
where
    Schema: ApplicationSchema,
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
{
    let lowering = super::super::predicate_admission::install_temporal_predicate_lowering(
        binding, graph, bridge,
    )?;
    let bounds = binding.bounds();
    let runtime_canonical_identity =
        Arc::new(affinity.bind(binding_identity).map_err(|denial| {
            WorthQueryConditionalRuntimeInstallationDenial::new(
                WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
                format!("conditional runtime identity was denied: {denial:?}"),
            )
        })?);
    let runtime_binding_identity = Arc::clone(runtime_canonical_identity.bridge_identity());
    let installation_canonical_work = binding_identity
        .canonical_work()
        .combine(runtime_canonical_identity.canonical_work());
    let clock = binding.clocked_node();
    let managed_clock = bridge
        .install_managed_clock(BridgeManagedClockInstallationParts {
            lowering: &lowering,
            binding_identity: Arc::clone(&runtime_binding_identity),
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
    Ok(WorthQueryPreparedConditionalRuntimeBinding {
        lowering,
        managed_clock,
        runtime_binding_identity,
        runtime_canonical_identity,
        installation_canonical_work,
        runtime_capability_identity: affinity.runtime_authority(),
        authoritative_reconstruction: Box::new(()),
    })
}
