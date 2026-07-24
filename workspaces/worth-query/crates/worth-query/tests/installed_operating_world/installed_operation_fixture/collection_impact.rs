use std::sync::OnceLock;

use worth_query::facade::{domain, read, runtime};

use super::conditional_workspace::{
    conditional_installation_with_change, conditional_model_graph_definition,
    ConditionalModelGraph, ConditionalModelGraphProvider,
};
use super::{
    canonical_ordered_collection_bundle, configured_runtime_without_executors,
    configured_runtime_without_executors_with_schema, operation_identity_contract,
    semantic_closure, GeometryDomain, ReadFamily,
};

mod routing_contract;

use routing_contract::{
    routing_collection_read_declaration, routing_collection_schema, routing_collection_semantics,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct ImpactCollectionRead;

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, ReadFamily>
    for ImpactCollectionRead
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}

#[derive(Clone, Copy)]
struct ImpactCollectionExecutor {
    routing_order: bool,
}

impl ImpactCollectionExecutor {
    const fn identity_ordered() -> Self {
        Self {
            routing_order: false,
        }
    }

    const fn routing_ordered() -> Self {
        Self {
            routing_order: true,
        }
    }
}

impl domain::WorthQueryDomainOperationExecutor<GeometryDomain, ImpactCollectionRead, ReadFamily>
    for ImpactCollectionExecutor
{
    const LOWERING_FAMILY: &'static str = "read-vertex-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(if self.routing_order {
            routing_collection_read_declaration()
        } else {
            collection_read_declaration()
        })
    }

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::execution_resource_support()
    }

    fn execute(
        &self,
        _: (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        Ok(domain::WorthQueryOperationExecutionMaterial::new(
            context.execute_installed_read(workspace)?,
            domain::WorthQueryOperationResultState::Ready,
        ))
    }
}

struct ImpactCollectionCompute;

impl
    domain::WorthQueryConditionalNodeComputeProvider<
        GeometryDomain,
        ImpactCollectionRead,
        ReadFamily,
    > for ImpactCollectionCompute
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}

    fn compute(
        &self,
        _: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                1,
            )]),
        ))
    }
}

pub(crate) fn conditional_collection_workspace_with_change(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
) -> Result<
    (
        runtime::WorthQueryWorkspace,
        worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
        [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 2],
    ),
    runtime::WorthQueryRuntimeError,
> {
    let dependency_contract = node.dependencies()[0].contract().clone();
    let (installation, request, snapshots) = conditional_installation_with_change(&node);
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let node_location =
        domain::WorthQueryConditionalNodeLocation::operation(installation.node_identity.clone())
            .expect("collection conditional location is valid");
    let builder = runtime::WorthQueryRuntime::builder()
        .domain_package(collection_package(node))
        .expect("collection impact package admits")
        .graph_participation(conditional_model_graph_definition())
        .graph_participation_provider(ConditionalModelGraph, ConditionalModelGraphProvider)
        .conditional_signal_graph(installation.graph)
        .conditional_node(
            GeometryDomain,
            ImpactCollectionRead,
            ReadFamily,
            ConditionalModelGraph,
            node_location,
            vec![installation.dependency],
            installation.providers,
            ImpactCollectionCompute,
        )
        .domain_operation_executor(
            GeometryDomain,
            ImpactCollectionRead,
            ReadFamily,
            ImpactCollectionExecutor::identity_ordered(),
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Continuation,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalEvaluation,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalComparator,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalTrigger,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalTemporalOrOnDemand,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Live,
            domain::WorthQueryConsumerSupportPosture::Supported,
        );
    let runtime = harness
        .configure_runtime_builder(
            builder,
            installation.bridge,
            [operation_identity_contract(1), dependency_contract],
            crate::support::public_bridge_runtime::public_graph_support_profile(),
        )
        .build_backend_from_parts()
        .build()?;
    Ok((runtime.workspace(name)?, request, snapshots))
}

pub(crate) fn impact_collection_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(plain_collection_package())
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Continuation,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .domain_operation_executor(
            GeometryDomain,
            ImpactCollectionRead,
            ReadFamily,
            ImpactCollectionExecutor::identity_ordered(),
        )
        .workspace(name)
}

pub(crate) fn impact_collection_invalidation_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors_with_schema(
        package(routing_collection_semantics()),
        routing_collection_schema(),
    )
    .consumer_support_posture(
        domain::WorthQueryConsumerSupportDimension::Continuation,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .consumer_support_posture(
        domain::WorthQueryConsumerSupportDimension::Sharing,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .consumer_support_posture(
        domain::WorthQueryConsumerSupportDimension::Invalidation,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .consumer_support_posture(
        domain::WorthQueryConsumerSupportDimension::DependencyImpact,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .domain_operation_executor(
        GeometryDomain,
        ImpactCollectionRead,
        ReadFamily,
        ImpactCollectionExecutor::routing_ordered(),
    )
    .workspace(name)
}

fn collection_package(
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let dependency = &node.dependencies()[0];
    let mut semantics = collection_semantics();
    if let domain::WorthQueryOperationGraphReadContract::Declared { roles } =
        &mut semantics.graph_reads
    {
        roles[0].semantic_reads.push(
            domain::WorthQueryOperationNativeProjectionContract::new(
                dependency.contract().clone(),
                dependency.projection_mask().clone(),
            )
            .expect("conditional projection is valid"),
        );
    }
    semantics.conditional_nodes = vec![node];
    package(semantics)
        .operation_graph_participation::<ImpactCollectionRead, ReadFamily, ConditionalModelGraph>(
            "model",
        )
}

fn plain_collection_package() -> domain::WorthQueryDomainPackage<GeometryDomain> {
    package(collection_semantics())
}

fn collection_semantics() -> domain::WorthQueryDomainOperationSemanticClosure {
    let mut semantics = semantic_closure(
        canonical_ordered_collection_bundle("Vertex", "identity", "id"),
        domain::WorthQuerySupportRequirement::Required,
        true,
    );
    semantics.collection = domain::WorthQueryOperationCollectionContract::Collection {
        row_identity_field: field(),
        ordering_fields: vec![field()],
        grouping: domain::WorthQueryOperationGroupingContract::Grouped {
            grouping_fields: vec![field()],
        },
        window: domain::WorthQueryOperationWindowPolicy::ContinuationBounded,
        continuation: domain::WorthQueryOperationContinuationPosture::SnapshotCursor,
    };
    semantics.support.continuation = domain::WorthQuerySupportRequirement::Required;
    semantics
}

fn package(
    semantics: domain::WorthQueryDomainOperationSemanticClosure,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        ImpactCollectionRead,
        ReadFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("impact-collection-read", 1),
        semantics,
    );
    domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
}

fn field() -> domain::WorthQueryOperationCollectionField {
    domain::WorthQueryOperationCollectionField::from_dotted("identity.id")
        .expect("valid collection field")
}

fn collection_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                "Vertex",
                read::QuerySchemaView::new(
                    "impact-collection",
                    [
                        read::SchemaFieldView::new(
                            read::AspectName::new("identity").unwrap(),
                            read::FieldName::new("id").unwrap(),
                            read::ScalarAspectType::String,
                        ),
                        read::SchemaFieldView::new(
                            read::AspectName::new("ordering").unwrap(),
                            read::FieldName::new("position").unwrap(),
                            read::ScalarAspectType::String,
                        ),
                    ],
                    [],
                ),
                |query| {
                    query
                        .project(read::AspectFieldSelector::new("identity", "id").unwrap())
                        .order_by(read::OrderingSelector::ascending("identity", "id").unwrap())
                },
                |shape| {
                    shape
                        .field(read::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
                },
            )
        })
        .expect("collection declaration is canonical")
    })
}
