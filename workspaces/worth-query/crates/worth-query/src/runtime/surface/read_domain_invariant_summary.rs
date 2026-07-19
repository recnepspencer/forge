use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::{
    WorthQueryReadBuiltInOperator, WorthQueryReadGraph, WorthQueryReadGraphFamily,
    WorthQueryReadOperatorFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadDomainInvariantSummary {
    graph_family: WorthQueryReadGraphFamily,
    scope_class: String,
    operator_families: Vec<WorthQueryReadOperatorFamily>,
    built_in_operator_coverage: Vec<WorthQueryReadBuiltInOperator>,
    schema_basis_digest: String,
    query_digest: String,
    declared_traversal_clause_count: usize,
    declared_traversal_depth_limit: usize,
    planned_read_surface_count: usize,
    summary_digest: String,
}

impl WorthQueryReadDomainInvariantSummary {
    pub(crate) fn derive(read_graph: &WorthQueryReadGraph) -> Self {
        let graph_family = read_graph.family().clone();
        let scope_class = read_graph.scope_class().as_str().to_string();
        let operator_families = read_graph.operator_families();
        let built_in_operator_coverage = read_graph.built_in_operators().to_vec();
        let schema_basis_digest = read_graph.schema_basis().as_str().to_string();
        let query_digest = read_graph.query_digest().to_string();
        let declared_traversal_clause_count = read_graph.declared_traversal_clause_count();
        let declared_traversal_depth_limit = read_graph.declared_traversal_depth_limit();
        let planned_read_surface_count = read_graph
            .execution_plan()
            .counters()
            .planned_read_surface_count();
        let graph_family_label = match graph_family {
            WorthQueryReadGraphFamily::Detail => "detail",
            WorthQueryReadGraphFamily::Collection => "collection",
        };
        let summary_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::ReadInvariantViolation)
                .field_shape(
                    WorthQueryEvidenceTag::new("role"),
                    "read-domain-invariant-summary",
                )
                .field_shape(WorthQueryEvidenceTag::new("family"), graph_family_label)
                .field_shape(WorthQueryEvidenceTag::new("scope"), &scope_class)
                .field_value(WorthQueryEvidenceTag::new("schema"), &schema_basis_digest)
                .field_value(WorthQueryEvidenceTag::new("query"), &query_digest)
                .field_usize(
                    WorthQueryEvidenceTag::new("declared_traversal_count"),
                    declared_traversal_clause_count,
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("declared_traversal_depth"),
                    declared_traversal_depth_limit,
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("planned_read_surface_count"),
                    planned_read_surface_count,
                )
                .field_value_sequence(
                    WorthQueryEvidenceTag::new("operator_family"),
                    operator_families
                        .iter()
                        .map(WorthQueryReadOperatorFamily::as_str),
                )
                .field_value_sequence(
                    WorthQueryEvidenceTag::new("built_in_operator"),
                    built_in_operator_coverage
                        .iter()
                        .map(WorthQueryReadBuiltInOperator::as_str),
                )
                .seal()
                .as_str()
                .to_string();
        Self {
            graph_family,
            scope_class,
            operator_families,
            built_in_operator_coverage,
            schema_basis_digest,
            query_digest,
            declared_traversal_clause_count,
            declared_traversal_depth_limit,
            planned_read_surface_count,
            summary_digest,
        }
    }

    pub fn graph_family(&self) -> &WorthQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn scope_class(&self) -> &str {
        &self.scope_class
    }

    pub fn operator_families(&self) -> &[WorthQueryReadOperatorFamily] {
        &self.operator_families
    }

    pub fn built_in_operator_coverage(&self) -> &[WorthQueryReadBuiltInOperator] {
        &self.built_in_operator_coverage
    }

    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn declared_traversal_clause_count(&self) -> usize {
        self.declared_traversal_clause_count
    }

    pub fn declared_traversal_depth_limit(&self) -> usize {
        self.declared_traversal_depth_limit
    }

    pub fn planned_read_surface_count(&self) -> usize {
        self.planned_read_surface_count
    }

    pub fn summary_digest(&self) -> &str {
        &self.summary_digest
    }
}
