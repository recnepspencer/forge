use crate::authoring::WorthQueryGraphReadDomainOperationDeclaration;
use crate::basis::QuerySchemaBasisAuthority;
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::identity::SchemaBasisDigest;
use crate::intent_admission::WorthQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::planning::ExecutionPlanBundle;
use crate::policy_plan::PolicyAwareCurrentPlan;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::relationship_proof::{RelationshipProofAdmission, RelationshipProofSupportProfile};
use crate::runtime::WorthQueryAuthoritativeMutationObligationDispatch;
use crate::runtime::WorthQueryIntentExecutionProvenance;
use crate::schema_view::QuerySchemaView;

use super::{WorthQueryReadBreadth, WorthQueryReadBuiltInOperator, WorthQueryReadOperatorFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadScopeClass {
    LocalNeighborhood,
    AnchoredExpansion,
    ExplicitBroadSearch,
}

impl WorthQueryReadScopeClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
            Self::AnchoredExpansion => "anchored_expansion",
            Self::ExplicitBroadSearch => "explicit_broad_search",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadGraphFamily {
    Detail,
    Collection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadExecutionEngine {
    QueryRuntimeCurrent,
    QueryRuntimeBranch,
    QueryRuntimeHistorical,
    QueryRuntimePreviewDerived,
}

impl WorthQueryReadExecutionEngine {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryRuntimeCurrent => "query-runtime-current",
            Self::QueryRuntimeBranch => "query-runtime-branch",
            Self::QueryRuntimeHistorical => "query-runtime-historical",
            Self::QueryRuntimePreviewDerived => "query-runtime-preview-derived",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadFallbackClass {
    None,
    SnapshotIndexedDebt,
    WholeViewDebt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadRelationshipProofPosture {
    NotRequired,
    DescriptorAdmittedSyntheticRuntime,
}

impl WorthQueryReadRelationshipProofPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::DescriptorAdmittedSyntheticRuntime => "descriptor_admitted_synthetic_runtime",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadGraph {
    digest: String,
    family: WorthQueryReadGraphFamily,
    scope_class: WorthQueryReadScopeClass,
    schema_basis: SchemaBasisDigest,
    built_in_operators: Vec<WorthQueryReadBuiltInOperator>,
    domain_graph_operations: Vec<WorthQueryGraphReadDomainOperationDeclaration>,
    declared_traversal_clause_count: usize,
    declared_traversal_depth_limit: usize,
    relationship_proof_admission: Option<RelationshipProofAdmission>,
    policy_aware_plan: Option<PolicyAwareCurrentPlan>,
    declarative_request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    execution_plan: ExecutionPlanBundle,
}

impl WorthQueryReadGraph {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> &WorthQueryReadGraphFamily {
        &self.family
    }

    pub fn scope_class(&self) -> &WorthQueryReadScopeClass {
        &self.scope_class
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn schema_basis_authority(&self) -> QuerySchemaBasisAuthority {
        QuerySchemaBasisAuthority::from_query_artifact(&self.schema_basis)
    }

    pub fn built_in_operators(&self) -> &[WorthQueryReadBuiltInOperator] {
        &self.built_in_operators
    }

    pub fn domain_graph_operations(&self) -> &[WorthQueryGraphReadDomainOperationDeclaration] {
        &self.domain_graph_operations
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

    pub(crate) fn policy_aware_plan(&self) -> Option<&PolicyAwareCurrentPlan> {
        self.policy_aware_plan.as_ref()
    }

    pub fn schema_view(&self) -> &QuerySchemaView {
        &self.schema_view
    }

    pub fn operator_families(&self) -> Vec<WorthQueryReadOperatorFamily> {
        let query = self.execution_plan.query();
        let mut families = Vec::new();
        if query.projection_count() > 0 {
            families.push(WorthQueryReadOperatorFamily::Projection);
        }
        if query.traversal_count() > 0 {
            families.push(WorthQueryReadOperatorFamily::Traversal);
        }
        if query.predicate_count() > 0 {
            families.push(WorthQueryReadOperatorFamily::Predicate);
        }
        if query.ordering_count() > 0 {
            families.push(WorthQueryReadOperatorFamily::Ordering);
        }
        families
    }

    pub(crate) fn new(
        family: WorthQueryReadGraphFamily,
        scope_class: WorthQueryReadScopeClass,
        schema_basis: SchemaBasisDigest,
        built_in_operators: Vec<WorthQueryReadBuiltInOperator>,
        domain_graph_operations: Vec<WorthQueryGraphReadDomainOperationDeclaration>,
        declared_traversal_clause_count: usize,
        declared_traversal_depth_limit: usize,
        relationship_proof_admission: Option<RelationshipProofAdmission>,
        policy_aware_plan: Option<PolicyAwareCurrentPlan>,
        declarative_request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
        execution_plan: ExecutionPlanBundle,
    ) -> Self {
        let family_label = match family {
            WorthQueryReadGraphFamily::Detail => "detail",
            WorthQueryReadGraphFamily::Collection => "collection",
        };
        let digest = worth_query_evidence_identity(WorthQueryEvidenceScope::ReadGraphDigest)
            .field_shape(WorthQueryEvidenceTag::new("family"), family_label)
            .field_shape(WorthQueryEvidenceTag::new("scope"), scope_class.as_str())
            .field_value(
                WorthQueryEvidenceTag::new("plan"),
                execution_plan.query().plan_digest().as_str(),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("built_in_operator"),
                built_in_operators
                    .iter()
                    .map(WorthQueryReadBuiltInOperator::as_str),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("domain_operation"),
                domain_graph_operations
                    .iter()
                    .map(WorthQueryGraphReadDomainOperationDeclaration::digest_part),
            )
            .field_usize(
                WorthQueryEvidenceTag::new("declared_traversal_count"),
                declared_traversal_clause_count,
            )
            .field_usize(
                WorthQueryEvidenceTag::new("declared_traversal_depth"),
                declared_traversal_depth_limit,
            )
            .optional_value(
                WorthQueryEvidenceTag::new("relationship_proof_admission"),
                relationship_proof_admission
                    .as_ref()
                    .map(|admission| admission.identity().as_str()),
            )
            .optional_value(
                WorthQueryEvidenceTag::new("policy_aware_plan"),
                policy_aware_plan
                    .as_ref()
                    .map(|plan| plan.core().digest().as_str()),
            )
            .seal()
            .as_str()
            .to_string();
        Self {
            digest,
            family,
            scope_class,
            schema_basis,
            built_in_operators,
            domain_graph_operations,
            declared_traversal_clause_count,
            declared_traversal_depth_limit,
            relationship_proof_admission,
            policy_aware_plan,
            declarative_request,
            schema_view,
            execution_plan,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadReceipt {
    pub(super) read_graph_digest: String,
    pub(super) graph_family: WorthQueryReadGraphFamily,
    pub(super) collection_result_family: Option<crate::collection::CollectionResultFamily>,
    pub(super) execution_plan_digest: String,
    pub(super) query_digest: String,
    pub(super) basis_digest: String,
    pub(super) result_digest: String,
    pub(super) snapshot_identity: WorthQuerySnapshotIdentity,
    pub(super) scope_class: WorthQueryReadScopeClass,
    pub(super) execution_engine: WorthQueryReadExecutionEngine,
    pub(super) fallback_class: WorthQueryReadFallbackClass,
    pub(super) fallback_count: usize,
    pub(super) operator_families: Vec<WorthQueryReadOperatorFamily>,
    pub(super) built_in_operator_coverage: Vec<WorthQueryReadBuiltInOperator>,
    pub(super) relationship_proof_posture: WorthQueryReadRelationshipProofPosture,
    pub(super) relationship_proof_admission: Option<RelationshipProofAdmission>,
    pub(super) relationship_proof_support_profile: Option<RelationshipProofSupportProfile>,
    pub(super) policy_narrowing_digest: Option<String>,
    pub(super) policy_aware_plan_digest: Option<String>,
    pub(super) policy_execution_seam_identity: Option<String>,
    pub(super) policy_executor_semantic_rediscovery_count: usize,
    pub(super) breadth: WorthQueryReadBreadth,
    pub(super) materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    pub(super) graph_read_access_plan:
        Option<crate::runtime::WorthQueryAdmittedGraphReadAccessPlan>,
    pub(super) graph_read_access_plan_consumption:
        Option<crate::runtime::WorthQueryGraphReadAccessPlanConsumption>,
    pub(super) ephemeral_graph_index_receipt:
        Option<crate::runtime::WorthQueryEphemeralGraphIndexReceipt>,
    pub(super) graph_read_streaming_receipt:
        Option<crate::runtime::WorthQueryGraphReadStreamingReceipt>,
    pub(super) graph_read_access_summary:
        Option<crate::runtime::WorthQueryGraphReadAccessReceiptSummary>,
    pub(super) graph_read_access_complexity_counters:
        Option<crate::runtime::WorthQueryGraphReadAccessComplexityCounters>,
    pub(super) graph_obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    pub(super) decision_trace_envelope: Option<WorthQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<WorthQueryIntentExecutionProvenance>,
}
