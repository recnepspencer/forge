use super::error::WorthTopologyDomainQueryError;
use super::report::WorthTopologyDomainQueryRequestFamily;
use super::request::{WorthTopologyDomainQueryRequest, WorthTopologyDomainQueryTraversalStep};
use super::schema::worth_topology_domain_query_schema_view;
use forge_query::facade::{
    admit_policy_tenant_context, admit_relationship_proofs, plan_validated_bundle,
    planning_request_context_for_direct, validate_canonical_bundle, AspectFieldSelector,
    AuthoredResultShapeField, BranchAccessGrant, CollectionQueryBuilder,
    CollectionResultShapeBuilder, EqualityPredicate, ExecutionBasisIntent, GuidedAuthoringPath,
    LiveQueryFamily, PlannedExecutionRoute, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, QueryFamily, RelationshipProofBudget, RelationshipProofDescriptor,
    RelationshipProofDescriptorSet, RelationshipProofTopologyClass, RootEntityKey,
    ScalarPredicateValue, SchemaVariantSnapshot, SnapshotLineageClass, TenantBasisEpoch,
    TenantBindingSnapshot, TraversalSelector,
};

const TOPOLOGY_ENTITY_ROOT: &str = "WorthTopologyEntity";
const IDENTITY_ASPECT: &str = "identity";
const IDENTITY_FIELD: &str = "id";
const TOPOLOGY_ASPECT: &str = "topology";
const TOPOLOGY_KIND_FIELD: &str = "kind";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorthTopologyDomainQueryLoweringPosture {
    CanonicalTraversalLowered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthTopologyDomainQueryRelationshipProofPosture {
    Admitted,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryLoweringArtifact {
    request_family: WorthTopologyDomainQueryRequestFamily,
    lowering_posture: WorthTopologyDomainQueryLoweringPosture,
    query_family: QueryFamily,
    root_entity: String,
    traversal_steps: Vec<WorthTopologyDomainQueryTraversalStep>,
    canonical_query_digest: String,
    canonical_result_shape_digest: String,
    planned_execution_route: PlannedExecutionRoute,
    live_query_family: LiveQueryFamily,
    live_performance_status: String,
    planned_traversal_depth_limit: usize,
    planned_aggregate_input_breadth: usize,
    relationship_proof_posture: WorthTopologyDomainQueryRelationshipProofPosture,
    relationship_proof_admission_identity: Option<String>,
    relationship_proof_topology_classes: Vec<RelationshipProofTopologyClass>,
    relationship_proof_admission_count: usize,
    relationship_proof_topology_width: usize,
    relationship_proof_support_profile_digest: String,
}

impl WorthTopologyDomainQueryLoweringArtifact {
    #[allow(dead_code)]
    pub(crate) fn request_family(&self) -> WorthTopologyDomainQueryRequestFamily {
        self.request_family
    }

    #[allow(dead_code)]
    pub(crate) fn lowering_posture(&self) -> WorthTopologyDomainQueryLoweringPosture {
        self.lowering_posture
    }

    #[allow(dead_code)]
    pub(crate) fn query_family(&self) -> QueryFamily {
        self.query_family.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn root_entity(&self) -> &str {
        self.root_entity.as_str()
    }

    pub(crate) fn traversal_steps(&self) -> &[WorthTopologyDomainQueryTraversalStep] {
        &self.traversal_steps
    }

    #[allow(dead_code)]
    pub(crate) fn canonical_query_digest(&self) -> &str {
        self.canonical_query_digest.as_str()
    }

    #[allow(dead_code)]
    pub(crate) fn canonical_result_shape_digest(&self) -> &str {
        self.canonical_result_shape_digest.as_str()
    }

    #[allow(dead_code)]
    pub(crate) fn planned_execution_route(&self) -> &PlannedExecutionRoute {
        &self.planned_execution_route
    }

    #[allow(dead_code)]
    pub(crate) fn live_query_family(&self) -> &LiveQueryFamily {
        &self.live_query_family
    }

    #[allow(dead_code)]
    pub(crate) fn live_performance_status(&self) -> &str {
        self.live_performance_status.as_str()
    }

    #[allow(dead_code)]
    pub(crate) fn planned_traversal_depth_limit(&self) -> usize {
        self.planned_traversal_depth_limit
    }

    #[allow(dead_code)]
    pub(crate) fn planned_aggregate_input_breadth(&self) -> usize {
        self.planned_aggregate_input_breadth
    }

    #[allow(dead_code)]
    pub(crate) fn relationship_proof_posture(
        &self,
    ) -> WorthTopologyDomainQueryRelationshipProofPosture {
        self.relationship_proof_posture
    }

    #[allow(dead_code)]
    pub(crate) fn relationship_proof_admission_identity(&self) -> Option<&str> {
        self.relationship_proof_admission_identity.as_deref()
    }

    #[allow(dead_code)]
    pub(crate) fn relationship_proof_topology_classes(&self) -> &[RelationshipProofTopologyClass] {
        &self.relationship_proof_topology_classes
    }

    pub(crate) fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    #[allow(dead_code)]
    pub(crate) fn relationship_proof_topology_width(&self) -> usize {
        self.relationship_proof_topology_width
    }

    #[allow(dead_code)]
    pub(crate) fn relationship_proof_support_profile_digest(&self) -> &str {
        self.relationship_proof_support_profile_digest.as_str()
    }
}

pub(crate) fn lower_topology_domain_query(
    request: &WorthTopologyDomainQueryRequest,
) -> Result<WorthTopologyDomainQueryLoweringArtifact, WorthTopologyDomainQueryError> {
    request.validate()?;
    let root = RootEntityKey::new(TOPOLOGY_ENTITY_ROOT).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let traversal_steps = request.traversal_steps();
    let mut query = CollectionQueryBuilder::new(root)
        .project(project(IDENTITY_ASPECT, IDENTITY_FIELD)?)
        .project(project(TOPOLOGY_ASPECT, TOPOLOGY_KIND_FIELD)?)
        .where_equal(identity_anchor_predicate(request.anchor_identity())?)
        .order_by(ordering_selector(IDENTITY_ASPECT, IDENTITY_FIELD)?);
    for step in &traversal_steps {
        query = query.traverse(traversal_selector(*step)?);
    }
    let query = query.build().map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let result_shape = CollectionResultShapeBuilder::new()
        .field(result_field(
            IDENTITY_ASPECT,
            IDENTITY_FIELD,
            IDENTITY_FIELD,
        )?)
        .field(result_field(
            TOPOLOGY_ASPECT,
            TOPOLOGY_KIND_FIELD,
            TOPOLOGY_KIND_FIELD,
        )?)
        .build()
        .map_err(|error| {
            WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
        })?;
    let canonical =
        GuidedAuthoringPath::canonicalize_collection(query, result_shape).map_err(|error| {
            WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
        })?;
    let schema_view = worth_topology_domain_query_schema_view().map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(error.to_string())
    })?;
    let validated = validate_canonical_bundle(canonical.clone(), schema_view).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let request_context = planning_request_context_for_direct(&validated, runtime_basis_intent())
        .map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let plan = plan_validated_bundle(&validated, request_context).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let live_promotion = plan.live_promotion().clone();
    let relationship_proof_support_profile =
        forge_query::facade::runtime_backed_relationship_proof_support_profile();
    let policy = PolicyRuleSnapshot::synthetic_authority(
        "worth-topology-domain-query",
        "worth-topology-domain-query-rules",
        PolicyEpoch::Synthetic(1),
    );
    let admitted_policy_context = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            "branch-a",
            "schema-a",
            TenantBasisEpoch::Synthetic(1),
        ),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible"),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let descriptor_set = relationship_proof_descriptor_set(
        request,
        admitted_policy_context.bundle().policy_digest(),
    );
    let (relationship_proof_admission, relationship_proof_counters) =
        admit_relationship_proofs(canonical.query(), &admitted_policy_context, &descriptor_set)
            .map_err(|error| {
                WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
            })?;
    Ok(WorthTopologyDomainQueryLoweringArtifact {
        request_family: request.family(),
        lowering_posture: WorthTopologyDomainQueryLoweringPosture::CanonicalTraversalLowered,
        query_family: canonical.query().family().clone(),
        root_entity: canonical.query().root().as_str().to_string(),
        traversal_steps,
        canonical_query_digest: canonical.query().digest().as_str().to_string(),
        canonical_result_shape_digest: canonical.result_shape().digest().as_str().to_string(),
        planned_execution_route: plan.query().route().clone(),
        live_query_family: live_promotion.family().clone(),
        live_performance_status: live_promotion
            .performance_report()
            .performance_status()
            .to_string(),
        planned_traversal_depth_limit: plan.counters().planned_traversal_depth_limit(),
        planned_aggregate_input_breadth: plan.counters().planned_aggregate_input_breadth(),
        relationship_proof_posture: WorthTopologyDomainQueryRelationshipProofPosture::Admitted,
        relationship_proof_admission_identity: Some(
            relationship_proof_admission.identity().as_str().to_string(),
        ),
        relationship_proof_topology_classes: relationship_proof_admission
            .topology_classes()
            .to_vec(),
        relationship_proof_admission_count: relationship_proof_counters
            .relationship_proof_admission_count(),
        relationship_proof_topology_width: relationship_proof_counters
            .relationship_proof_topology_width(),
        relationship_proof_support_profile_digest: relationship_proof_support_profile
            .profile_digest()
            .to_string(),
    })
}

fn relationship_proof_descriptor_set(
    request: &WorthTopologyDomainQueryRequest,
    policy_digest: &str,
) -> RelationshipProofDescriptorSet {
    let descriptors = request
        .traversal_steps()
        .into_iter()
        .map(|step| {
            if step.depth() == 1 {
                RelationshipProofDescriptor::direct_edge_relation_name(
                    step.relation_name(),
                    policy_digest.to_string(),
                )
            } else {
                RelationshipProofDescriptor::bounded_ancestor_relation_name(
                    step.relation_name(),
                    step.depth(),
                    policy_digest.to_string(),
                )
                .expect("validated traversal steps must admit bounded-ancestor descriptors")
            }
        })
        .collect::<Vec<_>>();
    let topology_width = descriptors
        .iter()
        .map(|descriptor| match descriptor {
            RelationshipProofDescriptor::DirectEdge { .. } => 1,
            RelationshipProofDescriptor::BoundedAncestor { max_depth, .. }
            | RelationshipProofDescriptor::BoundedDescendant { max_depth, .. } => {
                usize::from(*max_depth)
            }
            RelationshipProofDescriptor::TenantMembership { .. }
            | RelationshipProofDescriptor::QueryShapeMismatch { .. }
            | RelationshipProofDescriptor::UnboundedRecursiveWalk { .. }
            | RelationshipProofDescriptor::HostCallbackForbidden { .. } => 0,
        })
        .sum();
    let descriptor_count = descriptors.len();
    RelationshipProofDescriptorSet::new(
        descriptors,
        RelationshipProofBudget::bounded(descriptor_count, topology_width),
    )
}

fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        forge_query::facade::BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn project(
    aspect: impl Into<String>,
    field: impl Into<String>,
) -> Result<AspectFieldSelector, WorthTopologyDomainQueryError> {
    AspectFieldSelector::new(aspect, field).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

fn ordering_selector(
    aspect: impl Into<String>,
    field: impl Into<String>,
) -> Result<forge_query::facade::OrderingSelector, WorthTopologyDomainQueryError> {
    forge_query::facade::OrderingSelector::ascending(aspect, field).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

fn result_field(
    aspect: impl Into<String>,
    field: impl Into<String>,
    delivered_name: impl Into<String>,
) -> Result<AuthoredResultShapeField, WorthTopologyDomainQueryError> {
    AuthoredResultShapeField::new(aspect, field, delivered_name).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

fn identity_anchor_predicate(
    identity: &str,
) -> Result<EqualityPredicate, WorthTopologyDomainQueryError> {
    EqualityPredicate::new(
        IDENTITY_ASPECT,
        IDENTITY_FIELD,
        ScalarPredicateValue::String(identity.to_string()),
    )
    .map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

fn traversal_selector(
    step: WorthTopologyDomainQueryTraversalStep,
) -> Result<TraversalSelector, WorthTopologyDomainQueryError> {
    TraversalSelector::bounded_relation_name(step.relation_name(), step.depth()).map_err(|error| {
        WorthTopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}
