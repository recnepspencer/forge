use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryHostConditionalOutputComparatorProvider,
    WorthQueryHostConditionalOutputVersionProvider, WorthQueryHostConditionalPredicateProvider,
    WorthQueryInstalledApplicationConditionalNode, WorthQueryInstalledGraphParticipationAuthority,
    WorthQueryInstalledTemporalConditionalOperation, WorthQueryNamedClock,
    WorthQueryNamedClockSource, WorthQueryOutputEquivalenceRequirement,
    WorthQueryPortableConditionalNodeDeclaration, WorthQuerySemanticLocality,
    WorthQueryTemporalIntentProjector,
};
use worth_runtime_bridge::facade::{
    BridgeConditionalComputeProvider, BridgeConditionalCondition, BridgeConditionalContract,
    BridgeConditionalContractParts, BridgeConditionalLocation, BridgeConditionalProviderSemantics,
    BridgeConditionalProviderSet, BridgeInstalledConditionalLowering,
    BridgeOwnedConditionalInstallationRequest, BridgeOwnedSignalRuntime,
    BridgeSemanticDependencyCandidate, BridgeSemanticDependencyCandidateParts,
    BridgeSemanticLocality,
};
use worth_signal::facade::{SignalConditionalArtifactReuse, SignalConditionalVersionComparator};

use super::installation::{
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind,
};
use super::predicate_observation::QueryTemporalPredicateProvider;

pub(in crate::domain_computation::primary_graph) struct QueryConditionalComputeContext {
    pub(in crate::domain_computation::primary_graph) output_version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct QueryConditionalComputeSemanticContract(Arc<str>);

struct QueryConditionalComputeProvider<Node> {
    semantics: QueryConditionalComputeSemanticContract,
    output_version: Option<Arc<dyn WorthQueryHostConditionalOutputVersionProvider<Node>>>,
}

impl<Node: 'static> BridgeConditionalProviderSemantics for QueryConditionalComputeProvider<Node> {
    type SemanticContract = QueryConditionalComputeSemanticContract;

    fn semantic_contract(&self) -> Self::SemanticContract {
        self.semantics.clone()
    }
}

impl<Node: 'static> BridgeConditionalComputeProvider for QueryConditionalComputeProvider<Node> {
    fn compute(
        &self,
        context: &mut dyn std::any::Any,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        let context = context
            .downcast_ref::<QueryConditionalComputeContext>()
            .ok_or_else(|| {
                "conditional execution lacked Query's governed re-entry context".to_string()
            })?;
        let output_version = self
            .output_version
            .as_ref()
            .map(|provider| provider.output_version(context.output_version))
            .transpose()
            .map_err(|failure| failure.detail().to_owned())?
            .unwrap_or(context.output_version);
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                output_version,
            )]),
        ))
    }
}

struct QueryHostOutputComparator<Node> {
    identity: Arc<str>,
    provider: Arc<dyn WorthQueryHostConditionalOutputComparatorProvider<Node>>,
}

impl<Node: 'static> BridgeConditionalProviderSemantics for QueryHostOutputComparator<Node> {
    type SemanticContract = Arc<str>;

    fn semantic_contract(&self) -> Self::SemanticContract {
        Arc::clone(&self.identity)
    }
}

impl<Node: 'static> worth_runtime_bridge::facade::BridgeConditionalComparatorProvider
    for QueryHostOutputComparator<Node>
{
    fn has_meaningful_change(
        &self,
        _aspect: worth_signal::facade::Aspect,
        cached: u64,
        current: u64,
    ) -> Result<bool, String> {
        self.provider
            .has_meaningful_change(cached, current)
            .map_err(|failure| failure.detail().to_owned())
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn install_temporal_predicate_lowering<
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
    graph: &WorthQueryInstalledGraphParticipationAuthority,
    bridge: &mut BridgeOwnedSignalRuntime,
) -> Result<Arc<BridgeInstalledConditionalLowering>, WorthQueryConditionalRuntimeInstallationDenial>
where
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
    Node: 'static,
{
    let installed_provider = binding.clocked_node().provider();
    let node = installed_provider.node();
    let contract = lower_temporal_contract(node.declaration())?;
    let location = lower_location(node.location());
    let dependencies = dependency_candidates(node, graph)?;
    let node_authority: Arc<str> = Arc::from(node.authority_identity());
    let output_version = installed_provider.retain_output_version_for_runtime();
    let compute_identity: Arc<str> = Arc::from(format!(
        "{}|output-version={}",
        node_authority,
        output_version
            .as_ref()
            .map(|(identity, _)| *identity)
            .unwrap_or("query-attempt"),
    ));
    let output_comparator = installed_provider.retain_output_comparator_for_runtime();
    validate_output_comparator_identity(node.declaration(), output_comparator.as_ref())?;
    let providers = BridgeConditionalProviderSet::new()
        .wake(QueryTemporalPredicateProvider::<Node, Provider>::new(
            installed_provider.retain_provider_for_runtime(),
            Arc::clone(&node_authority),
        ))
        .compute(QueryConditionalComputeProvider::<Node> {
            semantics: QueryConditionalComputeSemanticContract(compute_identity),
            output_version: output_version.map(|(_, provider)| provider),
        });
    let providers = match output_comparator {
        Some((identity, provider)) => {
            providers.output_comparator(QueryHostOutputComparator::<Node> {
                identity: Arc::from(identity),
                provider,
            })
        }
        None => providers,
    };
    bridge
        .install_owned_conditional(BridgeOwnedConditionalInstallationRequest {
            contract,
            location,
            dependencies,
            providers,
        })
        .map_err(|denial| bridge_denial(format!("{:?}: {}", denial.kind(), denial.detail())))
}

fn validate_output_comparator_identity<Node>(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    installed: Option<&(
        &'static str,
        Arc<dyn WorthQueryHostConditionalOutputComparatorProvider<Node>>,
    )>,
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    let WorthQueryOutputEquivalenceRequirement::Registered(required) =
        declaration.output_equivalence()
    else {
        return Ok(());
    };
    let Some((installed_identity, _)) = installed else {
        return Err(bridge_denial(format!(
            "registered output comparator `{}` has no bound host provider",
            required.as_str()
        )));
    };
    if *installed_identity != required.as_str() {
        return Err(bridge_denial(format!(
            "registered output comparator `{}` was bound to provider `{installed_identity}`",
            required.as_str()
        )));
    }
    Ok(())
}

fn dependency_candidates<Schema, ApplicationOperation, Input, D, O, F, Node>(
    node: &WorthQueryInstalledApplicationConditionalNode<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        Node,
    >,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
) -> Result<Vec<BridgeSemanticDependencyCandidate>, WorthQueryConditionalRuntimeInstallationDenial>
{
    let operation = node.operation().domain_operation();
    node.declaration()
        .dependencies()
        .iter()
        .enumerate()
        .map(|(ordinal, _)| {
            let authority = operation
                .conditional_dependency(node.location().clone(), ordinal)
                .map_err(|_| bridge_denial("installed conditional dependency was not retained"))?;
            candidate_from_authority(&authority, graph)
        })
        .collect()
}

fn candidate_from_authority(
    authority: &worth_query_installation::facade::WorthQueryInstalledConditionalDependencyAuthority,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
) -> Result<BridgeSemanticDependencyCandidate, WorthQueryConditionalRuntimeInstallationDenial> {
    let dependency = authority.dependency();
    if authority.runtime_ordinal() != graph.runtime_ordinal()
        || dependency.graph_read_role().as_str() != graph.role()
    {
        return Err(WorthQueryConditionalRuntimeInstallationDenial::new(
            WorthQueryConditionalRuntimeInstallationDenialKind::ForeignBinding,
            dependency.graph_read_role().as_str(),
        ));
    }
    let locality = match dependency.locality() {
        WorthQuerySemanticLocality::SourceRecord => BridgeSemanticLocality::ManagedSourceRecord,
        WorthQuerySemanticLocality::SourcePartition(role)
            if graph.truth_partition_role() == Some(role) =>
        {
            BridgeSemanticLocality::SourcePartition(role.clone())
        }
        WorthQuerySemanticLocality::SourcePartition(role) => {
            return Err(bridge_denial(format!(
                "primary application conditional runtime has no matching source-partition binding for `{}`",
                role.as_str()
            )))
        }
        WorthQuerySemanticLocality::WholeLogicalGraph => BridgeSemanticLocality::WholeLogicalGraph,
    };
    BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
        source_installation_identity: Arc::from(format!(
            "{}|generation={}|operation={}|node={}|dependency={}",
            authority.owner(),
            authority.generation().ordinal(),
            authority.operation_slot(),
            authority.location().node_identity(),
            authority.dependency_ordinal(),
        )),
        source_basis: Arc::from(authority.operation_canonical_identity()),
        source_runtime_authority: authority.runtime_ordinal(),
        source_installation_generation: authority.generation().ordinal(),
        source_authority_binding_identity: Arc::from(authority.authority_binding_identity()),
        source_stage_identity: authority.location().stage_identity().map(Arc::from),
        source_node_identity: Arc::from(authority.location().node_identity()),
        dependency_ordinal: authority.dependency_ordinal(),
        declared_graph_role: Arc::from(dependency.graph_read_role().as_str()),
        graph_participation_identity: Arc::from(graph.authority_identity()),
        graph_adapter_identity: Arc::from(graph.provider_identity()),
        source_record_identity: None,
        observation_record_identity: None,
        contract: dependency.contract().clone(),
        projection_mask: dependency.projection_mask().clone(),
        binding: dependency.binding().clone(),
        locality,
        relevant_changes: dependency.relevant_changes().to_vec(),
    })
    .map_err(|denial| bridge_denial(format!("semantic dependency denied: {:?}", denial.kind())))
}

fn lower_temporal_contract(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<BridgeConditionalContract, WorthQueryConditionalRuntimeInstallationDenial> {
    if declaration.condition().class() != WorthQueryConditionalConditionClass::Temporal {
        return Err(bridge_denial(
            "named clock binding did not retain a temporal condition",
        ));
    }
    Ok(BridgeConditionalContract::new(
        BridgeConditionalContractParts {
            identity: Arc::from(declaration.identity()),
            dependency_count: declaration.dependencies().len(),
            condition_dependency_ordinals: (0..declaration.dependencies().len()).collect(),
            condition: BridgeConditionalCondition::TemporalWake,
            dependency_comparator: lower_dependency_comparator(declaration.dependency_comparator()),
            output_comparator: lower_output_comparator(declaration.output_equivalence()),
            artifact_reuse: lower_artifact_reuse(declaration.artifact_reuse_equivalence()),
        },
    ))
}

fn lower_location(
    location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
) -> BridgeConditionalLocation {
    match location.stage_identity() {
        Some(stage) => BridgeConditionalLocation::workflow_stage(
            Arc::from(stage),
            Arc::from(location.node_identity()),
        ),
        None => BridgeConditionalLocation::operation(Arc::from(location.node_identity())),
    }
}

fn lower_dependency_comparator(
    requirement: &WorthQueryComparatorRequirement,
) -> SignalConditionalVersionComparator {
    match requirement {
        WorthQueryComparatorRequirement::ExactCanonicalValue
        | WorthQueryComparatorRequirement::FoundationalContractEquivalence => {
            SignalConditionalVersionComparator::Exact
        }
        WorthQueryComparatorRequirement::Registered(_) => {
            SignalConditionalVersionComparator::RuntimeResolved
        }
    }
}

fn lower_output_comparator(
    requirement: &WorthQueryOutputEquivalenceRequirement,
) -> SignalConditionalVersionComparator {
    match requirement {
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue
        | WorthQueryOutputEquivalenceRequirement::FoundationalContractEquivalence => {
            SignalConditionalVersionComparator::Exact
        }
        WorthQueryOutputEquivalenceRequirement::OutputIdentity => {
            SignalConditionalVersionComparator::OutputIdentity
        }
        WorthQueryOutputEquivalenceRequirement::Registered(_) => {
            SignalConditionalVersionComparator::RuntimeResolved
        }
    }
}

fn lower_artifact_reuse(
    requirement: &WorthQueryArtifactReuseEquivalence,
) -> SignalConditionalArtifactReuse {
    match requirement {
        WorthQueryArtifactReuseEquivalence::NotReusable => {
            SignalConditionalArtifactReuse::NotReusable
        }
        WorthQueryArtifactReuseEquivalence::DependencyAndOutputEquivalent => {
            SignalConditionalArtifactReuse::DependencyAndOutputEquivalent
        }
        WorthQueryArtifactReuseEquivalence::OutputEquivalent => {
            SignalConditionalArtifactReuse::OutputEquivalent
        }
        WorthQueryArtifactReuseEquivalence::Registered(_) => {
            SignalConditionalArtifactReuse::RuntimeResolved
        }
    }
}

fn bridge_denial(subject: impl Into<String>) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(
        WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
        subject,
    )
}
