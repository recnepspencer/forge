use crate::authoring::WorthQueryGraphReadDomainOperationDeclaration;
use crate::basis::{BasisAuthorityFamily, ExecutionBasisIntent, SnapshotLineageClass};
use crate::canonicalization::CanonicalQueryBundle;
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::identity::{CanonicalQueryDigest, SchemaBasisDigest};
use crate::planning::{plan_validated_bundle, planning_request_context_for_direct};
use crate::relationship_proof::RelationshipProofAdmission;
use crate::runtime::{
    WorthQueryReadBuiltInOperator, WorthQueryReadDenial, WorthQueryReadDenialKind,
    WorthQueryReadGraph, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};
use crate::schema_view::QuerySchemaView;
use crate::validation::ValidatedQueryBundle;

/// Query-owned read meaning after canonicalization and validation but before
/// authority admission or planning.
///
/// This type is present in the inferred builder signature only. Consumers
/// cannot inspect or manufacture its canonical, validated, or schema-bearing
/// internals, and only Query's context handoff may plan it.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryDeclaredReadIntent {
    digest: String,
    family: WorthQueryReadGraphFamily,
    scope_class: WorthQueryReadScopeClass,
    schema_basis: SchemaBasisDigest,
    built_in_operators: Vec<WorthQueryReadBuiltInOperator>,
    domain_graph_operations: Vec<WorthQueryGraphReadDomainOperationDeclaration>,
    declared_traversal_clause_count: usize,
    declared_traversal_depth_limit: usize,
    declarative_request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    canonical: CanonicalQueryBundle,
    validated: ValidatedQueryBundle,
}

impl WorthQueryDeclaredReadIntent {
    pub(crate) fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        self.canonical.query().digest()
    }

    pub(crate) fn canonical(&self) -> &CanonicalQueryBundle {
        &self.canonical
    }

    pub(crate) fn validated(&self) -> &ValidatedQueryBundle {
        &self.validated
    }

    pub(crate) fn built_in_operators(&self) -> &[WorthQueryReadBuiltInOperator] {
        &self.built_in_operators
    }

    pub(crate) fn requires_relationship_proof(&self) -> bool {
        self.declared_traversal_clause_count > 0
    }

    pub(crate) fn plan(
        self,
        relationship_proof_admission: Option<RelationshipProofAdmission>,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        let basis_intent = ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        );
        let request_context = planning_request_context_for_direct(&self.validated, basis_intent)
            .map_err(planning_denial)?;
        let execution_plan =
            plan_validated_bundle(&self.validated, request_context).map_err(planning_denial)?;
        Ok(WorthQueryReadGraph::new(
            self.family,
            self.scope_class,
            self.schema_basis,
            self.built_in_operators,
            self.domain_graph_operations,
            self.declared_traversal_clause_count,
            self.declared_traversal_depth_limit,
            relationship_proof_admission,
            self.declarative_request,
            self.schema_view,
            execution_plan,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family: WorthQueryReadGraphFamily,
        scope_class: WorthQueryReadScopeClass,
        schema_basis: SchemaBasisDigest,
        built_in_operators: Vec<WorthQueryReadBuiltInOperator>,
        domain_graph_operations: Vec<WorthQueryGraphReadDomainOperationDeclaration>,
        declared_traversal_clause_count: usize,
        declared_traversal_depth_limit: usize,
        declarative_request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
        canonical: CanonicalQueryBundle,
        validated: ValidatedQueryBundle,
    ) -> Self {
        let digest = worth_query_evidence_identity(WorthQueryEvidenceScope::ReadGraphDigest)
            .field_shape(WorthQueryEvidenceTag::new("stage"), "declared_read_intent")
            .field_value(
                WorthQueryEvidenceTag::new("canonical_query"),
                canonical.query().digest().as_str(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("canonical_result_shape"),
                canonical.result_shape().digest().as_str(),
            )
            .field_shape(WorthQueryEvidenceTag::new("scope"), scope_class.as_str())
            .field_value_sequence(
                WorthQueryEvidenceTag::new("built_in_operator"),
                built_in_operators
                    .iter()
                    .map(WorthQueryReadBuiltInOperator::as_str),
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
            declarative_request,
            schema_view,
            canonical,
            validated,
        }
    }
}

fn planning_denial(error: impl std::fmt::Debug) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new(
        WorthQueryReadDenialKind::PlanningDenied,
        format!("{error:?}"),
    )
}
