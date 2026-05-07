use crate::identity::{hash_parts, SchemaBasisDigest};
use crate::planning::{ExecutionPlanBundle, FallbackDisposition, PlannedExecutionRoute};
use crate::relationship_proof::{
    RelationshipProofAdmission, RelationshipProofSupportProfile, RelationshipProofSupportStatus,
};
use crate::runtime::read_composition_relationship_proof::support_profile_for_relationship_proof;

use super::{ForgeQueryReadBreadth, ForgeQueryReadBuiltInOperator, ForgeQueryReadOperatorFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadScopeClass {
    LocalNeighborhood,
    AnchoredExpansion,
    ExplicitBroadSearch,
}

impl ForgeQueryReadScopeClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
            Self::AnchoredExpansion => "anchored_expansion",
            Self::ExplicitBroadSearch => "explicit_broad_search",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadGraphFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadExecutionEngine {
    QueryRuntimeCurrent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadFallbackClass {
    None,
    SnapshotIndexedDebt,
    WholeViewDebt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadRelationshipProofPosture {
    NotRequired,
    DescriptorAdmittedSyntheticRuntime,
}

impl ForgeQueryReadRelationshipProofPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::DescriptorAdmittedSyntheticRuntime => "descriptor_admitted_synthetic_runtime",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadGraph {
    digest: String,
    family: ForgeQueryReadGraphFamily,
    scope_class: ForgeQueryReadScopeClass,
    schema_basis: SchemaBasisDigest,
    built_in_operators: Vec<ForgeQueryReadBuiltInOperator>,
    declared_traversal_clause_count: usize,
    declared_traversal_depth_limit: usize,
    relationship_proof_admission: Option<RelationshipProofAdmission>,
    execution_plan: ExecutionPlanBundle,
}

impl ForgeQueryReadGraph {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> &ForgeQueryReadGraphFamily {
        &self.family
    }

    pub fn scope_class(&self) -> &ForgeQueryReadScopeClass {
        &self.scope_class
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn built_in_operators(&self) -> &[ForgeQueryReadBuiltInOperator] {
        &self.built_in_operators
    }

    pub fn query_digest(&self) -> &str {
        self.execution_plan
            .query()
            .validated_query_digest()
            .as_str()
    }

    pub fn declared_traversal_clause_count(&self) -> usize {
        self.declared_traversal_clause_count
    }

    pub fn declared_traversal_depth_limit(&self) -> usize {
        self.declared_traversal_depth_limit
    }

    pub fn execution_plan(&self) -> &ExecutionPlanBundle {
        &self.execution_plan
    }

    pub fn relationship_proof_admission(&self) -> Option<&RelationshipProofAdmission> {
        self.relationship_proof_admission.as_ref()
    }

    pub fn operator_families(&self) -> Vec<ForgeQueryReadOperatorFamily> {
        let query = self.execution_plan.query();
        let mut families = Vec::new();
        if query.projection_count() > 0 {
            families.push(ForgeQueryReadOperatorFamily::Projection);
        }
        if query.traversal_count() > 0 {
            families.push(ForgeQueryReadOperatorFamily::Traversal);
        }
        if query.predicate_count() > 0 {
            families.push(ForgeQueryReadOperatorFamily::Predicate);
        }
        if query.ordering_count() > 0 {
            families.push(ForgeQueryReadOperatorFamily::Ordering);
        }
        families
    }

    pub(in crate::runtime) fn new(
        family: ForgeQueryReadGraphFamily,
        scope_class: ForgeQueryReadScopeClass,
        schema_basis: SchemaBasisDigest,
        built_in_operators: Vec<ForgeQueryReadBuiltInOperator>,
        declared_traversal_clause_count: usize,
        declared_traversal_depth_limit: usize,
        relationship_proof_admission: Option<RelationshipProofAdmission>,
        execution_plan: ExecutionPlanBundle,
    ) -> Self {
        let digest = hash_parts(&[
            format!("family:{:?}", family),
            format!("scope:{}", scope_class.as_str()),
            format!("plan:{}", execution_plan.query().plan_digest().as_str()),
            format!("built_in_operators:{built_in_operators:?}"),
            format!("declared_traversal_count:{declared_traversal_clause_count}"),
            format!("declared_traversal_depth:{declared_traversal_depth_limit}"),
            format!(
                "relationship_proof_admission:{}",
                relationship_proof_admission
                    .as_ref()
                    .map(|admission| admission.identity().as_str())
                    .unwrap_or("none")
            ),
        ]);
        Self {
            digest,
            family,
            scope_class,
            schema_basis,
            built_in_operators,
            declared_traversal_clause_count,
            declared_traversal_depth_limit,
            relationship_proof_admission,
            execution_plan,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadReceipt {
    read_graph_digest: String,
    graph_family: ForgeQueryReadGraphFamily,
    query_digest: String,
    basis_digest: String,
    result_digest: String,
    snapshot_token: String,
    scope_class: ForgeQueryReadScopeClass,
    execution_engine: ForgeQueryReadExecutionEngine,
    fallback_class: ForgeQueryReadFallbackClass,
    fallback_count: usize,
    operator_families: Vec<ForgeQueryReadOperatorFamily>,
    built_in_operator_coverage: Vec<ForgeQueryReadBuiltInOperator>,
    relationship_proof_posture: ForgeQueryReadRelationshipProofPosture,
    relationship_proof_admission: Option<RelationshipProofAdmission>,
    relationship_proof_support_profile: Option<RelationshipProofSupportProfile>,
    breadth: ForgeQueryReadBreadth,
}

impl ForgeQueryReadReceipt {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn graph_family(&self) -> &ForgeQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn scope_class(&self) -> &ForgeQueryReadScopeClass {
        &self.scope_class
    }

    pub fn execution_engine(&self) -> &ForgeQueryReadExecutionEngine {
        &self.execution_engine
    }

    pub fn fallback_class(&self) -> &ForgeQueryReadFallbackClass {
        &self.fallback_class
    }

    pub fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub fn operator_families(&self) -> &[ForgeQueryReadOperatorFamily] {
        &self.operator_families
    }

    pub fn built_in_operator_coverage(&self) -> &[ForgeQueryReadBuiltInOperator] {
        &self.built_in_operator_coverage
    }

    pub fn relationship_proof_posture(&self) -> &ForgeQueryReadRelationshipProofPosture {
        &self.relationship_proof_posture
    }

    pub fn relationship_proof_admission_identity(&self) -> Option<&str> {
        self.relationship_proof_admission
            .as_ref()
            .map(|admission| admission.identity().as_str())
    }

    pub fn relationship_proof_descriptor_count(&self) -> usize {
        self.relationship_proof_admission
            .as_ref()
            .map(RelationshipProofAdmission::descriptor_count)
            .unwrap_or(0)
    }

    pub fn relationship_proof_support_profile(&self) -> Option<&RelationshipProofSupportProfile> {
        self.relationship_proof_support_profile.as_ref()
    }

    pub fn relationship_proof_support_profile_digest(&self) -> Option<&str> {
        self.relationship_proof_support_profile
            .as_ref()
            .map(RelationshipProofSupportProfile::profile_digest)
    }

    pub fn relationship_proof_verified_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Verified,
        )
    }

    pub fn relationship_proof_deferred_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Deferred,
        )
    }

    pub fn relationship_proof_forbidden_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Forbidden,
        )
    }

    pub fn breadth(&self) -> &ForgeQueryReadBreadth {
        &self.breadth
    }

    pub(in crate::runtime) fn from_execution(
        read_graph: &ForgeQueryReadGraph,
        snapshot_token: String,
        execution: &crate::execution::ExecutionResultEnvelope,
    ) -> Self {
        let query = read_graph.execution_plan().query();
        let planning = read_graph.execution_plan().counters();
        let execution_counters = execution.counters();
        let fallback_class = match query.fallback() {
            FallbackDisposition::Forbidden | FallbackDisposition::AdmittedButUnused => {
                ForgeQueryReadFallbackClass::None
            }
            FallbackDisposition::AdmittedAndSelected => {
                ForgeQueryReadFallbackClass::SnapshotIndexedDebt
            }
        };
        let execution_engine = match query.route() {
            PlannedExecutionRoute::RuntimeSnapshotRead
            | PlannedExecutionRoute::RuntimeExpandedSnapshotRead
            | PlannedExecutionRoute::StoreSnapshotRead => {
                ForgeQueryReadExecutionEngine::QueryRuntimeCurrent
            }
        };
        let relationship_proof_admission = read_graph.relationship_proof_admission().cloned();
        let relationship_proof_support_profile = relationship_proof_admission
            .as_ref()
            .map(support_profile_for_relationship_proof);
        Self {
            read_graph_digest: read_graph.digest().to_string(),
            graph_family: read_graph.family().clone(),
            query_digest: execution.report().query_digest().as_str().to_string(),
            basis_digest: execution.report().basis_digest().as_str().to_string(),
            result_digest: execution.report().result_digest().as_str().to_string(),
            snapshot_token,
            scope_class: read_graph.scope_class().clone(),
            execution_engine,
            fallback_class,
            fallback_count: execution_counters.execution_fallback_taken_count(),
            operator_families: read_graph.operator_families(),
            built_in_operator_coverage: read_graph.built_in_operators().to_vec(),
            relationship_proof_posture: if relationship_proof_admission.is_some() {
                ForgeQueryReadRelationshipProofPosture::DescriptorAdmittedSyntheticRuntime
            } else {
                ForgeQueryReadRelationshipProofPosture::NotRequired
            },
            relationship_proof_admission,
            relationship_proof_support_profile,
            breadth: ForgeQueryReadBreadth {
                planned_read_surface_count: planning.planned_read_surface_count(),
                planned_traversal_clause_count: planning
                    .planned_traversal_clause_count()
                    .max(read_graph.declared_traversal_clause_count()),
                planned_traversal_depth_limit: planning
                    .planned_traversal_depth_limit()
                    .max(read_graph.declared_traversal_depth_limit()),
                execution_read_operation_count: execution_counters.execution_read_operation_count(),
                execution_records_examined_count: execution_counters
                    .execution_records_examined_count(),
                execution_records_emitted_count: execution_counters
                    .execution_records_emitted_count(),
                execution_page_width: execution_counters.page_width(),
                execution_page_truncation_count: execution_counters.page_truncation_count(),
                execution_cursor_advance_count: execution_counters.cursor_advance_count(),
                execution_materialized_relation_count: execution_counters
                    .materialized_relation_count(),
            },
        }
    }
}

fn relationship_proof_support_surface_count(
    profile: Option<&RelationshipProofSupportProfile>,
    status: RelationshipProofSupportStatus,
) -> usize {
    profile
        .map(|profile| {
            profile
                .surfaces()
                .iter()
                .filter(|(_, surface_status)| *surface_status == status)
                .count()
        })
        .unwrap_or(0)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadResult {
    payload: Vec<String>,
    receipt: ForgeQueryReadReceipt,
}

impl ForgeQueryReadResult {
    pub fn payload(&self) -> &[String] {
        &self.payload
    }

    pub fn receipt(&self) -> &ForgeQueryReadReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(payload: Vec<String>, receipt: ForgeQueryReadReceipt) -> Self {
        Self { payload, receipt }
    }
}
