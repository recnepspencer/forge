use worth_runtime_bridge::facade::{
    BridgeConditionalComputeProvider, BridgeConditionalProviderSet,
    BridgeSemanticCorrespondenceRegistration, BridgeSignalAspectTargetDeclaration,
    RelationalBridgeRecordIdentityParts,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthQueryConditionalComputeContext {
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    operation_identity: String,
    binding_identity: String,
    basis_identity: String,
    workflow_run_identity: Option<String>,
    snapshot_identity: String,
    attempt: u64,
}

impl WorthQueryConditionalComputeContext {
    pub fn location(&self) -> &worth_query_installation::facade::WorthQueryConditionalNodeLocation {
        &self.location
    }
    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }
    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }
    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }
    pub fn workflow_run_identity(&self) -> Option<&str> {
        self.workflow_run_identity.as_deref()
    }
    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }
    pub const fn attempt(&self) -> u64 {
        self.attempt
    }
    pub(crate) fn new(
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        operation_identity: String,
        binding_identity: String,
        basis_identity: String,
        workflow_run_identity: Option<String>,
        snapshot_identity: String,
        attempt: u64,
    ) -> Self {
        Self {
            location,
            operation_identity,
            binding_identity,
            basis_identity,
            workflow_run_identity,
            snapshot_identity,
            attempt,
        }
    }
}

pub trait WorthQueryConditionalNodeComputeProvider<D, O, F>: Send + Sync + 'static {
    fn compute(
        &self,
        context: &WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String>;
}

pub struct WorthQueryConditionalDependencyInstallation {
    source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    targets: Vec<BridgeSignalAspectTargetDeclaration>,
}

impl WorthQueryConditionalDependencyInstallation {
    pub fn new(
        source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        targets: Vec<BridgeSignalAspectTargetDeclaration>,
    ) -> Self {
        Self {
            source_record_identity,
            targets,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthQueryConditionalNodeInstallationDenial {
    DomainNotInstalled,
    OperationNotInstalled,
    GraphNotInstalled,
    LocationNotDeclared,
    DeclarationLookupDrift,
    DependencyShape,
    Correspondence(worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind),
    Bridge {
        kind: worth_runtime_bridge::facade::BridgeConditionalDenialKind,
        detail: String,
    },
    DuplicateInstallation,
}

pub(crate) struct QueryComputeProvider<D, O, F, P> {
    provider: P,
    _marker: std::marker::PhantomData<fn() -> (D, O, F)>,
}

impl<D, O, F, P> QueryComputeProvider<D, O, F, P> {
    pub(crate) fn new(provider: P) -> Self {
        Self {
            provider,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D: 'static, O: 'static, F: 'static, P> BridgeConditionalComputeProvider
    for QueryComputeProvider<D, O, F, P>
where
    P: WorthQueryConditionalNodeComputeProvider<D, O, F>,
{
    fn compute(
        &self,
        context: &mut dyn std::any::Any,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        let context = context
            .downcast_ref::<WorthQueryConditionalComputeContext>()
            .ok_or_else(|| {
                "conditional compute context belongs to another Query entry".to_string()
            })?;
        self.provider.compute(context)
    }
}

pub(crate) fn build_correspondence_registrations<D: 'static, O: 'static, F: 'static, G: 'static>(
    operation: &super::super::WorthQueryInstalledDomainOperation<D, O, F>,
    graph: &super::super::WorthQueryInstalledGraphParticipation<G>,
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    dependencies: Vec<WorthQueryConditionalDependencyInstallation>,
) -> Result<
    Vec<BridgeSemanticCorrespondenceRegistration>,
    WorthQueryConditionalNodeInstallationDenial,
> {
    let declaration = declared_node(operation.definition(), &location)
        .ok_or(WorthQueryConditionalNodeInstallationDenial::LocationNotDeclared)?;
    if dependencies.len() != declaration.dependencies().len() {
        return Err(WorthQueryConditionalNodeInstallationDenial::DependencyShape);
    }
    dependencies
        .into_iter()
        .enumerate()
        .map(|(ordinal, dependency)| {
            operation
                .semantic_correspondence_registration(
                    location.clone(),
                    ordinal,
                    graph,
                    dependency.source_record_identity,
                    dependency.targets,
                )
                .map_err(|denial| {
                    WorthQueryConditionalNodeInstallationDenial::Correspondence(denial.kind())
                })
        })
        .collect()
}

pub(crate) fn declared_node<'a>(
    definition: &'a worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition,
    location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
) -> Option<&'a worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration> {
    match location {
        worth_query_installation::facade::WorthQueryConditionalNodeLocation::Operation {
            ..
        } => definition
            .semantics()
            .conditional_nodes
            .iter()
            .find(|node| location_matches(location, node)),
        worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
            stage_identity,
            ..
        } => match &definition.semantics().workflow {
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
                workflow,
            ) => workflow
                .stages()
                .iter()
                .find(|stage| stage.identity() == stage_identity)
                .and_then(|stage| {
                    stage
                        .semantics()
                        .conditional_nodes
                        .iter()
                        .find(|node| location_matches(location, node))
                }),
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => {
                None
            }
        },
    }
}

fn location_matches(
    location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    node: &worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
) -> bool {
    location.node_identity() == node.identity()
}

pub(crate) fn with_compute_provider<D: 'static, O: 'static, F: 'static, P>(
    providers: BridgeConditionalProviderSet,
    provider: P,
) -> BridgeConditionalProviderSet
where
    P: WorthQueryConditionalNodeComputeProvider<D, O, F>,
{
    providers.compute(QueryComputeProvider::<D, O, F, P>::new(provider))
}

pub(crate) trait PendingConditionalInstallation: Send {
    fn install(
        self: Box<Self>,
        domains: &super::super::WorthQueryDomainInstallationRegistry,
        graphs: &super::super::WorthQueryInstalledGraphParticipationRegistry,
        signal: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        registry: &mut super::WorthQueryConditionalExecutionRegistry,
    ) -> Result<(), WorthQueryConditionalNodeInstallationDenial>;
}

pub(crate) struct PendingConditionalNode<D, O, F, G, P> {
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    dependencies: Vec<WorthQueryConditionalDependencyInstallation>,
    providers: BridgeConditionalProviderSet,
    compute: P,
    _marker: std::marker::PhantomData<fn() -> (D, O, F, G)>,
}

impl<D, O, F, G, P> PendingConditionalNode<D, O, F, G, P> {
    pub(crate) fn new(
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependencies: Vec<WorthQueryConditionalDependencyInstallation>,
        providers: BridgeConditionalProviderSet,
        compute: P,
    ) -> Self {
        Self {
            location,
            dependencies,
            providers,
            compute,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D: 'static, O: 'static, F: 'static, G: 'static, P> PendingConditionalInstallation
    for PendingConditionalNode<D, O, F, G, P>
where
    P: WorthQueryConditionalNodeComputeProvider<D, O, F>,
{
    fn install(
        self: Box<Self>,
        domains: &super::super::WorthQueryDomainInstallationRegistry,
        graphs: &super::super::WorthQueryInstalledGraphParticipationRegistry,
        signal: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        registry: &mut super::WorthQueryConditionalExecutionRegistry,
    ) -> Result<(), WorthQueryConditionalNodeInstallationDenial> {
        let domain = domains
            .domain::<D>()
            .map_err(|_| WorthQueryConditionalNodeInstallationDenial::DomainNotInstalled)?;
        let (authority, workflow) = domains
            .execution_index()
            .domain_operation_authority(
                std::any::TypeId::of::<D>(),
                std::any::TypeId::of::<O>(),
                std::any::TypeId::of::<F>(),
            )
            .ok_or(WorthQueryConditionalNodeInstallationDenial::OperationNotInstalled)?;
        let bindings = domains
            .execution_index()
            .domain_operation_graph_bindings(
                std::any::TypeId::of::<D>(),
                std::any::TypeId::of::<O>(),
                std::any::TypeId::of::<F>(),
            )
            .to_vec();
        let operation = super::super::WorthQueryInstalledDomainOperation::<D, O, F>::mint(
            domain.authority_arc(),
            authority,
            workflow,
            bindings,
        );
        let graph: super::super::WorthQueryInstalledGraphParticipation<G> = graphs
            .get::<G>()
            .map(super::super::WorthQueryInstalledGraphParticipation::new)
            .map_err(|_| WorthQueryConditionalNodeInstallationDenial::GraphNotInstalled)?;
        let declaration = declared_node(operation.definition(), &self.location)
            .cloned()
            .ok_or(WorthQueryConditionalNodeInstallationDenial::DeclarationLookupDrift)?;
        if self.providers.has_compute_provider() {
            return Err(WorthQueryConditionalNodeInstallationDenial::Bridge {
                kind:
                    worth_runtime_bridge::facade::BridgeConditionalDenialKind::ExtraComputeProvider,
                detail: "Query conditional registrations own the sole compute provider".into(),
            });
        }
        let registrations = build_correspondence_registrations(
            &operation,
            &graph,
            self.location.clone(),
            self.dependencies,
        )?;
        let lowering = signal
            .install(
                worth_runtime_bridge::facade::BridgeConditionalInstallationRequest {
                    declaration,
                    location: self.location,
                    registrations,
                    providers: with_compute_provider::<D, O, F, P>(self.providers, self.compute),
                },
            )
            .map_err(
                |denial| WorthQueryConditionalNodeInstallationDenial::Bridge {
                    kind: denial.kind(),
                    detail: denial.detail().to_string(),
                },
            )?;
        registry
            .install::<D, O, F>(super::WorthQueryInstalledConditionalNode {
                lowering,
                operation_identity: operation.definition().canonical_identity().to_string(),
                runtime_authority: operation.domain_authority().runtime_authority().as_u64(),
                installation_generation: operation.installation_generation().ordinal(),
            })
            .map_err(|_| WorthQueryConditionalNodeInstallationDenial::DuplicateInstallation)
    }
}
