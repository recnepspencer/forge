use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::{hash_parts, SchemaBasisDigest};
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::planning::ExecutionPlanBundle;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::relationship_proof::{
    RelationshipProofAdmission, RelationshipProofSupportProfile, RelationshipProofSupportStatus,
};
use crate::runtime::ForgeQueryIntentExecutionProvenance;
use crate::schema_view::QuerySchemaView;

use super::read_receipt_support::relationship_proof_support_surface_count;
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
    QueryRuntimeBranch,
    QueryRuntimeHistorical,
    QueryRuntimePreviewDerived,
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
    declarative_request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
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

    pub fn declarative_request(&self) -> &DeclarativeLiveQueryRequest {
        &self.declarative_request
    }

    pub fn relationship_proof_admission(&self) -> Option<&RelationshipProofAdmission> {
        self.relationship_proof_admission.as_ref()
    }

    pub fn schema_view(&self) -> &QuerySchemaView {
        &self.schema_view
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
        declarative_request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
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
            declarative_request,
            schema_view,
            execution_plan,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadReceipt {
    pub(super) read_graph_digest: String,
    pub(super) graph_family: ForgeQueryReadGraphFamily,
    pub(super) query_digest: String,
    pub(super) basis_digest: String,
    pub(super) result_digest: String,
    pub(super) snapshot_token: String,
    pub(super) scope_class: ForgeQueryReadScopeClass,
    pub(super) execution_engine: ForgeQueryReadExecutionEngine,
    pub(super) fallback_class: ForgeQueryReadFallbackClass,
    pub(super) fallback_count: usize,
    pub(super) operator_families: Vec<ForgeQueryReadOperatorFamily>,
    pub(super) built_in_operator_coverage: Vec<ForgeQueryReadBuiltInOperator>,
    pub(super) relationship_proof_posture: ForgeQueryReadRelationshipProofPosture,
    pub(super) relationship_proof_admission: Option<RelationshipProofAdmission>,
    pub(super) relationship_proof_support_profile: Option<RelationshipProofSupportProfile>,
    pub(super) breadth: ForgeQueryReadBreadth,
    pub(super) materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
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

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub(in crate::runtime) fn with_materialized_fact_posture(
        mut self,
        posture: Option<ProjectionMaterializedFactPosture>,
    ) -> Self {
        self.materialized_fact_posture = posture;
        self
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }

    pub fn execution_provenance_chain_digest(&self) -> Option<&str> {
        self.execution_provenance
            .as_ref()
            .map(|provenance| provenance.execution_provenance_chain_digest())
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        read_graph_digest: impl Into<String>,
        query_digest: impl Into<String>,
        basis_digest: impl Into<String>,
        result_digest: impl Into<String>,
        execution_engine: ForgeQueryReadExecutionEngine,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            graph_family: ForgeQueryReadGraphFamily::Collection,
            query_digest: query_digest.into(),
            basis_digest: basis_digest.into(),
            result_digest: result_digest.into(),
            snapshot_token: "snapshot:test".to_string(),
            scope_class: ForgeQueryReadScopeClass::ExplicitBroadSearch,
            execution_engine,
            fallback_class: ForgeQueryReadFallbackClass::None,
            fallback_count: 0,
            operator_families: Vec::new(),
            built_in_operator_coverage: Vec::new(),
            relationship_proof_posture: ForgeQueryReadRelationshipProofPosture::NotRequired,
            relationship_proof_admission: None,
            relationship_proof_support_profile: None,
            breadth: ForgeQueryReadBreadth {
                planned_read_surface_count: 0,
                planned_traversal_clause_count: 0,
                planned_traversal_depth_limit: 0,
                execution_read_operation_count: 0,
                execution_records_examined_count: 0,
                execution_records_emitted_count: 0,
                execution_page_width: 0,
                execution_page_truncation_count: 0,
                execution_cursor_advance_count: 0,
                execution_materialized_relation_count: 0,
            },
            materialized_fact_posture: None,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_with_materialized_fact_posture(
        mut self,
        posture: ProjectionMaterializedFactPosture,
    ) -> Self {
        self.materialized_fact_posture = Some(posture);
        self
    }
}
