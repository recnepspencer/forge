use self::relationship_proof::{admit_topology_read_relationship_proofs, runtime_basis_intent};
use self::schema::topology_read_schema_view;
use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;
use crate::projection::read_views::domain::request::{
    TopologyReadRequest, TopologyReadTraversalStep,
};
use crate::projection::runtime_boundary::read_execution::query_shape::{
    identity_anchor_predicate, identity_ordering, identity_result_field, identity_selector,
    topology_entity_root, topology_kind_result_field, topology_kind_selector,
};
use forge_query::facade::{
    plan_validated_bundle, planning_request_context_for_direct, validate_canonical_bundle,
    CollectionQueryBuilder, CollectionResultShapeBuilder, GuidedAuthoringPath, LiveQueryFamily,
    PlannedExecutionRoute, QueryFamily, RelationshipProofTopologyClass, TraversalSelector,
};

pub(crate) mod relationship_proof;
pub(crate) mod schema;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyReadLoweringPosture {
    CanonicalTraversalLowered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyReadRelationshipProofPosture {
    Admitted,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyReadLoweringArtifact {
    request_family: TopologyReadRequestFamily,
    lowering_posture: TopologyReadLoweringPosture,
    query_family: QueryFamily,
    root_entity: String,
    traversal_steps: Vec<TopologyReadTraversalStep>,
    canonical_query_digest: String,
    canonical_result_shape_digest: String,
    planned_execution_route: PlannedExecutionRoute,
    live_query_family: LiveQueryFamily,
    live_performance_status: String,
    planned_traversal_depth_limit: usize,
    planned_aggregate_input_breadth: usize,
    relationship_proof_posture: TopologyReadRelationshipProofPosture,
    relationship_proof_admission_identity: Option<String>,
    relationship_proof_topology_classes: Vec<RelationshipProofTopologyClass>,
    relationship_proof_admission_count: usize,
    relationship_proof_topology_width: usize,
    relationship_proof_support_profile_digest: String,
}

impl TopologyReadLoweringArtifact {
    #[allow(dead_code)]
    pub(crate) fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    #[allow(dead_code)]
    pub(crate) fn lowering_posture(&self) -> TopologyReadLoweringPosture {
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

    pub(crate) fn traversal_steps(&self) -> &[TopologyReadTraversalStep] {
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
    pub(crate) fn relationship_proof_posture(&self) -> TopologyReadRelationshipProofPosture {
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

pub(crate) fn lower_topology_read(
    request: &TopologyReadRequest,
) -> Result<TopologyReadLoweringArtifact, TopologyReadError> {
    request.validate()?;
    let root = topology_entity_root()?;
    let traversal_steps = request.traversal_steps();
    let mut query = CollectionQueryBuilder::new(root)
        .project(identity_selector()?)
        .project(topology_kind_selector()?)
        .where_equal(identity_anchor_predicate(request.anchor_identity())?)
        .order_by(identity_ordering()?);
    for step in &traversal_steps {
        query = query.traverse(traversal_selector(*step)?);
    }
    let query = query
        .build()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let result_shape = CollectionResultShapeBuilder::new()
        .field(identity_result_field()?)
        .field(topology_kind_result_field()?)
        .build()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let canonical = GuidedAuthoringPath::canonicalize_collection(query, result_shape)
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let schema_view = topology_read_schema_view()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(error.to_string()))?;
    let validated = validate_canonical_bundle(canonical.clone(), schema_view)
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let request_context = planning_request_context_for_direct(&validated, runtime_basis_intent())
        .map_err(|error| {
        TopologyReadError::canonical_lowering_resolution(format!("{error:?}"))
    })?;
    let plan = plan_validated_bundle(&validated, request_context)
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))?;
    let live_promotion = plan.live_promotion().clone();
    let relationship_proof = admit_topology_read_relationship_proofs(request, canonical.query())?;
    Ok(TopologyReadLoweringArtifact {
        request_family: request.family(),
        lowering_posture: TopologyReadLoweringPosture::CanonicalTraversalLowered,
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
        relationship_proof_posture: relationship_proof.posture,
        relationship_proof_admission_identity: relationship_proof.admission_identity,
        relationship_proof_topology_classes: relationship_proof.topology_classes,
        relationship_proof_admission_count: relationship_proof.admission_count,
        relationship_proof_topology_width: relationship_proof.topology_width,
        relationship_proof_support_profile_digest: relationship_proof.support_profile_digest,
    })
}

fn traversal_selector(
    step: TopologyReadTraversalStep,
) -> Result<TraversalSelector, TopologyReadError> {
    TraversalSelector::bounded_relation_name(step.relation_name(), step.depth())
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(format!("{error:?}")))
}
