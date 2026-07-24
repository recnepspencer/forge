use worth_runtime_bridge::facade::{
    BridgeConditionalProviderSet, BridgeSemanticCorrespondenceRegistration,
    BridgeSignalAspectTargetDeclaration, RelationalBridgeRecordIdentityParts,
};

mod authority_resolution;

use super::QueryComputeProvider;
use authority_resolution::{installed_conditional_graph, installed_conditional_operation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthQueryConditionalComputeContext {
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    operation_identity: String,
    binding_identity: String,
    basis_identity: String,
    workflow_run_identity: Option<String>,
    snapshot_identity: String,
    attempt: u64,
    execution_resources: crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence,
    resource_envelope:
        std::sync::Arc<worth_query_installation::facade::WorthQueryExecutionResourceEnvelope>,
}

pub(crate) struct WorthQueryConditionalComputeContextParts {
    pub(crate) location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    pub(crate) operation_identity: String,
    pub(crate) binding_identity: String,
    pub(crate) basis_identity: String,
    pub(crate) workflow_run_identity: Option<String>,
    pub(crate) snapshot_identity: String,
    pub(crate) attempt: u64,
    pub(crate) execution_resources:
        crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence,
    pub(crate) resource_envelope:
        std::sync::Arc<worth_query_installation::facade::WorthQueryExecutionResourceEnvelope>,
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
    pub fn execution_resources(
        &self,
    ) -> &crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence {
        &self.execution_resources
    }
    pub fn resource_envelope(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryExecutionResourceEnvelope {
        &self.resource_envelope
    }
    pub(crate) fn new(parts: WorthQueryConditionalComputeContextParts) -> Self {
        let WorthQueryConditionalComputeContextParts {
            location,
            operation_identity,
            binding_identity,
            basis_identity,
            workflow_run_identity,
            snapshot_identity,
            attempt,
            execution_resources,
            resource_envelope,
        } = parts;
        Self {
            location,
            operation_identity,
            binding_identity,
            basis_identity,
            workflow_run_identity,
            snapshot_identity,
            attempt,
            execution_resources,
            resource_envelope,
        }
    }
}

pub trait WorthQueryConditionalNodeComputeProvider<D, O, F>: Send + Sync + 'static {
    /// Complete owner-native compute meaning used when comparing a reinstalled
    /// provider. Runtime counters and observation state do not belong here.
    type SemanticContract: Eq + Send + Sync + 'static;

    fn semantic_contract(&self) -> Self::SemanticContract;

    fn execution_resource_support(
        &self,
    ) -> crate::domain_installation::WorthQueryExecutionResourceSupport;

    fn compute(
        &self,
        context: &WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String>;
}

#[derive(Clone)]
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

    fn rebound_for(
        &self,
        signal: &worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
    ) -> Result<Self, WorthQueryConditionalNodeInstallationDenial> {
        let targets = self
            .targets
            .iter()
            .map(|target| signal.rebind_signal_target(target))
            .collect::<Result<Vec<_>, _>>()
            .map_err(
                |denial| WorthQueryConditionalNodeInstallationDenial::Bridge {
                    kind: denial.kind(),
                    detail: denial.detail().to_string(),
                },
            )?;
        Ok(Self {
            source_record_identity: self.source_record_identity,
            targets,
        })
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
    provider: std::sync::Arc<P>,
) -> BridgeConditionalProviderSet
where
    P: WorthQueryConditionalNodeComputeProvider<D, O, F>,
{
    providers.compute(QueryComputeProvider::<D, O, F, P>::new(provider))
}

pub(crate) trait PendingConditionalInstallation: Send {
    fn install(
        &self,
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
    compute: std::sync::Arc<P>,
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
            compute: std::sync::Arc::new(compute),
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
        &self,
        domains: &super::super::WorthQueryDomainInstallationRegistry,
        graphs: &super::super::WorthQueryInstalledGraphParticipationRegistry,
        signal: &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        registry: &mut super::WorthQueryConditionalExecutionRegistry,
    ) -> Result<(), WorthQueryConditionalNodeInstallationDenial> {
        let operation = installed_conditional_operation::<D, O, F>(domains)?;
        let graph = installed_conditional_graph::<G>(graphs)?;
        let request = self.installation_request(signal, &operation, &graph)?;
        let lowering = signal.install(request).map_err(|denial| {
            WorthQueryConditionalNodeInstallationDenial::Bridge {
                kind: denial.kind(),
                detail: denial.detail().to_string(),
            }
        })?;
        registry
            .install::<D, O, F>(super::WorthQueryInstalledConditionalNode {
                lowering,
                operation_identity: operation.definition().canonical_identity().to_string(),
                runtime_authority: operation.domain_authority().runtime_authority().as_u64(),
                installation_generation: operation.installation_generation().ordinal(),
                resource_support: self.compute.execution_resource_support(),
            })
            .map_err(|_| WorthQueryConditionalNodeInstallationDenial::DuplicateInstallation)
    }
}

impl<D: 'static, O: 'static, F: 'static, G: 'static, P> PendingConditionalNode<D, O, F, G, P>
where
    P: WorthQueryConditionalNodeComputeProvider<D, O, F>,
{
    fn installation_request(
        &self,
        signal: &worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
        operation: &super::super::WorthQueryInstalledDomainOperation<D, O, F>,
        graph: &super::super::WorthQueryInstalledGraphParticipation<G>,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeConditionalInstallationRequest,
        WorthQueryConditionalNodeInstallationDenial,
    > {
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
        let dependencies = self
            .dependencies
            .iter()
            .map(|dependency| dependency.rebound_for(signal))
            .collect::<Result<Vec<_>, _>>()?;
        let registrations = build_correspondence_registrations(
            operation,
            graph,
            self.location.clone(),
            dependencies,
        )?;
        Ok(
            worth_runtime_bridge::facade::BridgeConditionalInstallationRequest {
                declaration,
                location: self.location.clone(),
                registrations,
                providers: with_compute_provider::<D, O, F, P>(
                    self.providers.clone(),
                    std::sync::Arc::clone(&self.compute),
                ),
            },
        )
    }
}
