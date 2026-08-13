use std::{collections::BTreeSet, marker::PhantomData, sync::Arc};

use crate::domain_computation::primary_graph::application_runtime::installation::ApplicationRuntimePublication;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryHostConditionalPredicateProvider,
    WorthQueryInstalledTemporalConditionalOperation, WorthQueryNamedClock,
    WorthQueryNamedClockSource, WorthQueryTemporalIntentProjector,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalRuntimeInstallationDenialKind {
    PrimaryGraphPublication,
    ForeignBinding,
    DuplicateBinding,
    IncompleteBindingInventory,
    BridgeRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalRuntimeInstallationDenial {
    kind: WorthQueryConditionalRuntimeInstallationDenialKind,
    subject: String,
}

impl WorthQueryConditionalRuntimeInstallationDenial {
    pub(super) fn new(
        kind: WorthQueryConditionalRuntimeInstallationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryConditionalRuntimeInstallationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

pub struct WorthQueryConditionalClockHandle<Schema, Node, Clock> {
    binding_identity: Arc<str>,
    marker: PhantomData<fn() -> (Schema, Node, Clock)>,
}

impl<Schema, Node, Clock> WorthQueryConditionalClockHandle<Schema, Node, Clock> {
    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }
}

pub struct WorthQueryConditionalApplicationRuntimeInstallation<Schema> {
    publication: ApplicationRuntimePublication<Schema>,
    binding_identities: BTreeSet<Arc<str>>,
    bindings: Vec<Box<dyn WorthQueryPendingConditionalOperation<Schema>>>,
}

impl<Schema> WorthQueryConditionalApplicationRuntimeInstallation<Schema>
where
    Schema: ApplicationSchema + 'static,
{
    pub(in crate::domain_computation::primary_graph) fn new(
        publication: ApplicationRuntimePublication<Schema>,
    ) -> Result<Self, WorthQueryConditionalRuntimeInstallationDenial> {
        publication
            .runtime
            .installed_packages()
            .validate_application_schema(&publication.installed_schema)
            .map_err(|denial| {
                WorthQueryConditionalRuntimeInstallationDenial::new(
                    WorthQueryConditionalRuntimeInstallationDenialKind::PrimaryGraphPublication,
                    denial.subject(),
                )
            })?;
        Ok(Self {
            publication,
            binding_identities: BTreeSet::new(),
            bindings: Vec::new(),
        })
    }

    pub fn bind_temporal_operation<
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
        &mut self,
        binding: WorthQueryInstalledTemporalConditionalOperation<
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
    ) -> Result<
        WorthQueryConditionalClockHandle<Schema, Node, Clock>,
        WorthQueryConditionalRuntimeInstallationDenial,
    >
    where
        Input: 'static,
        Provider: WorthQueryHostConditionalPredicateProvider<Node>,
        Clock: WorthQueryNamedClock,
        Source: WorthQueryNamedClockSource<Clock>,
        Query: 'static,
        Parameters: 'static,
        QueryResult: 'static,
        Scope: 'static,
        Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
        ApplicationOperation: 'static,
        D: 'static,
        O: 'static,
        F: 'static,
        Node: 'static,
    {
        self.validate_temporal_binding(&binding)?;
        let identity = super::pending_binding::temporal_binding_identity(&binding);
        if !self.binding_identities.insert(Arc::clone(&identity)) {
            return Err(WorthQueryConditionalRuntimeInstallationDenial::new(
                WorthQueryConditionalRuntimeInstallationDenialKind::DuplicateBinding,
                identity.as_ref(),
            ));
        }
        self.bindings.push(Box::new(
            super::pending_binding::PendingTemporalOperation::new(Arc::clone(&identity), binding),
        ));
        Ok(WorthQueryConditionalClockHandle {
            binding_identity: identity,
            marker: PhantomData,
        })
    }

    pub fn publish(
        self,
    ) -> Result<
        WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        super::super::application_runtime::installation::publish_application_runtime_with_conditionals(
            self.publication,
            self.bindings,
        )
    }

    fn validate_temporal_binding<
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
        &self,
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
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial>
    where
        Provider: WorthQueryHostConditionalPredicateProvider<Node>,
        Clock: WorthQueryNamedClock,
        Source: WorthQueryNamedClockSource<Clock>,
        Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
    {
        let index = self.publication.runtime.installed_packages();
        index
            .validate_conditional_application_node(binding.clocked_node().provider().node())
            .map_err(|denial| foreign_binding_denial(denial.subject()))?;
        self.publication
            .installed_schema
            .validate_installed_query(binding.query())
            .map_err(|denial| foreign_binding_denial(denial.subject()))
    }
}

pub(in crate::domain_computation::primary_graph) trait WorthQueryPendingConditionalOperation<Schema>
{
    fn binding_identity(&self) -> &str;

    fn install(
        self: Box<Self>,
        bridge: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        affinity: &super::publication::ConditionalRuntimeAffinity<'_>,
    ) -> Result<
        Box<dyn super::lifecycle::WorthQueryInstalledConditionalOperation>,
        WorthQueryConditionalRuntimeInstallationDenial,
    >;
}

fn foreign_binding_denial(
    subject: impl Into<String>,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(
        WorthQueryConditionalRuntimeInstallationDenialKind::ForeignBinding,
        subject,
    )
}
