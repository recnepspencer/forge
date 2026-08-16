use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryHostConditionalPredicateProvider, WorthQueryInstalledTemporalConditionalOperation,
    WorthQueryNamedClock, WorthQueryNamedClockSource, WorthQueryTemporalIntentProjector,
};
use worth_runtime_bridge::facade::BridgeManagedClockInstallationParts;

use super::installation::{
    ConditionalClockLease, WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind, WorthQueryPendingConditionalOperation,
};
use super::lifecycle::WorthQueryInstalledTemporalOperation;
use super::publication::ConditionalRuntimeAffinity;

pub(super) struct PendingTemporalOperation<Binding, Reconstruction, Execution> {
    binding_identity: Arc<super::canonical_identity::WorthQueryTemporalBindingIdentity>,
    clock_lease: Arc<ConditionalClockLease>,
    binding: Binding,
    reconstruction: Reconstruction,
    execution: Execution,
}

impl<Binding, Reconstruction, Execution>
    PendingTemporalOperation<Binding, Reconstruction, Execution>
{
    pub(super) fn new(
        binding_identity: Arc<super::canonical_identity::WorthQueryTemporalBindingIdentity>,
        clock_lease: Arc<ConditionalClockLease>,
        binding: Binding,
        reconstruction: Reconstruction,
        execution: Execution,
    ) -> Self {
        Self {
            binding_identity,
            clock_lease,
            binding,
            reconstruction,
            execution,
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
        PrincipalBinding,
        PrincipalMapping,
        Principal,
        PrincipalIdentity,
        ScopeAspect,
        ScopeField,
        ScopeValue,
        ScopeWrite,
        ScopeUnit,
        PrincipalSource,
        QueryAuthorization,
        Invoker,
        IntentEntity,
        IdentityAspect,
        IdentityField,
        IdentityValue,
        IdentityWrite,
        IdentityUnit,
        RevisionAspect,
        RevisionField,
        RevisionValue,
        RevisionWrite,
        RevisionEquality,
        RevisionUnit,
        LifecycleAspect,
        LifecycleField,
        LifecycleValue,
        LifecycleWrite,
        LifecycleEquality,
        LifecycleUnit,
        Authorization,
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
        super::reconstruction_authority::WorthQueryTemporalReconstructionAccess<
            Schema,
            PrincipalBinding,
            PrincipalMapping,
            Principal,
            PrincipalIdentity,
            Scope,
            ScopeAspect,
            ScopeField,
            ScopeValue,
            ScopeWrite,
            ScopeUnit,
            PrincipalSource,
            QueryAuthorization,
        >,
        super::operation_invocation::WorthQueryTemporalOperationExecution<
            Schema,
            ApplicationOperation,
            Input,
            Scope,
            Invoker,
            IntentEntity,
            IdentityAspect,
            IdentityField,
            IdentityValue,
            IdentityWrite,
            IdentityUnit,
            RevisionAspect,
            RevisionField,
            RevisionValue,
            RevisionWrite,
            RevisionEquality,
            RevisionUnit,
            LifecycleAspect,
            LifecycleField,
            LifecycleValue,
            LifecycleWrite,
            LifecycleEquality,
            LifecycleUnit,
            Authorization,
        >,
    >
where
    Schema: worth_query_installation::facade::ApplicationSchema + 'static,
    ApplicationOperation: 'static,
    Input: Clone + Send + Sync + 'static,
    D: 'static,
    O: 'static,
    F: 'static,
    Node: 'static,
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Query: 'static,
    Parameters: 'static,
    QueryResult: crate::domain_computation::primary_graph::WorthQueryApplicationProjection<Schema, Query>
        + 'static,
    Scope: 'static,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
    PrincipalBinding: 'static,
    PrincipalMapping: 'static,
    Principal: 'static,
    PrincipalIdentity: worth_query_installation::facade::TypedApplicationIdentityValue + 'static,
    ScopeAspect: 'static,
    ScopeField: 'static,
    ScopeValue: worth_query_installation::facade::TypedApplicationValue + Clone + Send + 'static,
    ScopeWrite: worth_query_installation::facade::WritePosture + 'static,
    ScopeUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    PrincipalSource: super::reconstruction_authority::WorthQueryTemporalPrincipalSource<Schema>,
    QueryAuthorization: super::WorthQueryTemporalQueryAuthorization<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        > + 'static,
    Invoker: super::operation_invocation::WorthQueryTemporalOperationInvoker<
        Schema,
        ApplicationOperation,
        Input,
        Scope,
    >,
    IntentEntity: 'static,
    IdentityAspect: 'static,
    IdentityField: worth_query_installation::facade::OperationReads<ApplicationOperation> + 'static,
    IdentityValue:
        worth_query_installation::facade::TypedApplicationReadableValue + Clone + Send + 'static,
    IdentityWrite: worth_query_installation::facade::WritePosture + 'static,
    IdentityUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    RevisionAspect: 'static,
    RevisionField: worth_query_installation::facade::OperationReads<ApplicationOperation>
        + worth_query_installation::facade::OperationWrites<ApplicationOperation>
        + 'static,
    RevisionValue: worth_query_installation::facade::WorthQueryTemporalIntentRevisionValue
        + worth_query_installation::facade::TypedApplicationReadableValue
        + Clone
        + Send
        + 'static,
    RevisionWrite: worth_query_installation::facade::WritableCapability + 'static,
    RevisionEquality: 'static,
    RevisionUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    LifecycleAspect: 'static,
    LifecycleField: worth_query_installation::facade::OperationReads<ApplicationOperation>
        + worth_query_installation::facade::OperationWrites<ApplicationOperation>
        + 'static,
    LifecycleValue:
        worth_query_installation::facade::TypedApplicationReadableValue + Clone + Send + 'static,
    LifecycleWrite: worth_query_installation::facade::WritableCapability + 'static,
    LifecycleEquality: 'static,
    LifecycleUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    Authorization: super::WorthQueryTemporalOperationAuthorization<Schema, ApplicationOperation, Input, Scope>
        + 'static,
{
    fn binding_identity(&self) -> &str {
        self.binding_identity.support_identity()
    }

    fn install(
        self: Box<Self>,
        bridge: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        affinity: &ConditionalRuntimeAffinity,
    ) -> Result<
        Box<dyn super::lifecycle::WorthQueryInstalledConditionalOperation<Schema>>,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        let lowering = super::predicate_admission::install_temporal_predicate_lowering(
            &self.binding,
            graph,
            bridge,
        )?;
        let bounds = self.binding.bounds();
        let runtime_canonical_identity =
            Arc::new(affinity.bind(&self.binding_identity).map_err(|denial| {
                WorthQueryConditionalRuntimeInstallationDenial::new(
                    WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
                    format!("conditional runtime identity was denied: {denial:?}"),
                )
            })?);
        let runtime_binding_identity = Arc::clone(runtime_canonical_identity.bridge_identity());
        let installation_canonical_work = self
            .binding_identity
            .canonical_work()
            .combine(runtime_canonical_identity.canonical_work());
        let clock = self.binding.clocked_node();
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
        Ok(Box::new(WorthQueryInstalledTemporalOperation {
            lifecycle_token: Default::default(),
            binding_identity: self.binding_identity,
            installation_canonical_work,
            clock_lease: self.clock_lease,
            binding: self.binding,
            reconstruction: self.reconstruction,
            execution: self.execution,
            lowering,
            managed_clock,
            runtime_binding_identity,
            runtime_canonical_identity,
            runtime_capability_identity: affinity.runtime_authority(),
            retained_wakes: Vec::new(),
            reconstructed_intents: std::collections::BTreeMap::new(),
            reconstruction_work: Default::default(),
            authoritative_commit_cursor: None,
            committed_operation_count: 0,
            already_committed_operation_count: 0,
            failed_operation_count: 0,
            indeterminate_operation_count: 0,
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
    principal_source_identity: &str,
    invoker_identity: &str,
) -> Result<
    Arc<super::canonical_identity::WorthQueryTemporalBindingIdentity>,
    worth_foundational::facade::CanonicalDigestDerivationDenial,
>
where
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
{
    let node = binding.clocked_node().provider().node();
    super::canonical_identity::prepare_temporal_binding_identity(
        super::canonical_identity::TemporalBindingIdentityParts {
            node_authority: node.authority_identity(),
            clock: binding.clocked_node().clock_identity(),
            source: binding.clocked_node().source_identity().as_str(),
            timeline: binding.clocked_node().timeline_identity().as_str(),
            query: *binding.query().identity().digest(),
            projector: binding.projector_semantic_identity(),
            principal_source: principal_source_identity,
            invoker: invoker_identity,
        },
    )
    .map(Arc::new)
}
