use crate::authoring::ForgeQueryGraphReadDomainOperationDeclaration;
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::identity::SchemaBasisDigest;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::planning::ExecutionPlanBundle;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::relationship_proof::{RelationshipProofAdmission, RelationshipProofSupportProfile};
use crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch;
use crate::runtime::ForgeQueryIntentExecutionProvenance;
use crate::schema_view::QuerySchemaView;

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

impl ForgeQueryReadExecutionEngine {
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
    domain_graph_operations: Vec<ForgeQueryGraphReadDomainOperationDeclaration>,
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

    pub fn domain_graph_operations(&self) -> &[ForgeQueryGraphReadDomainOperationDeclaration] {
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
        domain_graph_operations: Vec<ForgeQueryGraphReadDomainOperationDeclaration>,
        declared_traversal_clause_count: usize,
        declared_traversal_depth_limit: usize,
        relationship_proof_admission: Option<RelationshipProofAdmission>,
        declarative_request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
        execution_plan: ExecutionPlanBundle,
    ) -> Self {
        let family_label = match family {
            ForgeQueryReadGraphFamily::Detail => "detail",
            ForgeQueryReadGraphFamily::Collection => "collection",
        };
        let digest = forge_query_evidence_identity(ForgeQueryEvidenceScope::ReadGraphDigest)
            .field_shape(ForgeQueryEvidenceTag::new("family"), family_label)
            .field_shape(ForgeQueryEvidenceTag::new("scope"), scope_class.as_str())
            .field_value(
                ForgeQueryEvidenceTag::new("plan"),
                execution_plan.query().plan_digest().as_str(),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("built_in_operator"),
                built_in_operators
                    .iter()
                    .map(ForgeQueryReadBuiltInOperator::as_str),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("domain_operation"),
                domain_graph_operations
                    .iter()
                    .map(ForgeQueryGraphReadDomainOperationDeclaration::digest_part),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("declared_traversal_count"),
                declared_traversal_clause_count,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("declared_traversal_depth"),
                declared_traversal_depth_limit,
            )
            .optional_value(
                ForgeQueryEvidenceTag::new("relationship_proof_admission"),
                relationship_proof_admission
                    .as_ref()
                    .map(|admission| admission.identity().as_str()),
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
    pub(super) snapshot_identity: ForgeQuerySnapshotIdentity,
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
    pub(super) graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}
