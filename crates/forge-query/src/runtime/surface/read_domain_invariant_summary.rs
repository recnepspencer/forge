use crate::identity::hash_parts;

use super::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadGraph, ForgeQueryReadGraphFamily,
    ForgeQueryReadOperatorFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadDomainInvariantSummary {
    graph_family: ForgeQueryReadGraphFamily,
    scope_class: String,
    operator_families: Vec<ForgeQueryReadOperatorFamily>,
    built_in_operator_coverage: Vec<ForgeQueryReadBuiltInOperator>,
    schema_basis_digest: String,
    query_digest: String,
    declared_traversal_clause_count: usize,
    declared_traversal_depth_limit: usize,
    planned_read_surface_count: usize,
    summary_digest: String,
}

impl ForgeQueryReadDomainInvariantSummary {
    pub(crate) fn derive(read_graph: &ForgeQueryReadGraph) -> Self {
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
        let summary_digest = hash_parts(&[
            "forge_query_read_domain_invariant_summary_v1".to_string(),
            format!("family:{graph_family:?}"),
            format!("scope:{scope_class}"),
            format!("schema:{schema_basis_digest}"),
            format!("query:{query_digest}"),
            format!("traversal_count:{declared_traversal_clause_count}"),
            format!("traversal_depth:{declared_traversal_depth_limit}"),
            format!("planned_surfaces:{planned_read_surface_count}"),
            format!("operators:{operator_families:?}"),
            format!("built_in_coverage:{built_in_operator_coverage:?}"),
        ]);
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

    pub fn graph_family(&self) -> &ForgeQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn scope_class(&self) -> &str {
        &self.scope_class
    }

    pub fn operator_families(&self) -> &[ForgeQueryReadOperatorFamily] {
        &self.operator_families
    }

    pub fn built_in_operator_coverage(&self) -> &[ForgeQueryReadBuiltInOperator] {
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
