use crate::memory_workspace::ForgeQueryEntity;
use crate::planning::{FallbackDisposition, PlannedExecutionRoute};
use crate::query_context::{QueryContextExecutionArtifact, QueryContextExecutionFamily};
use crate::runtime::read_composition_relationship_proof::support_profile_for_relationship_proof;

use super::read_receipt_support::materialized_result_digest;
use super::{
    ForgeQueryReadBreadth, ForgeQueryReadExecutionEngine, ForgeQueryReadFallbackClass,
    ForgeQueryReadGraph, ForgeQueryReadReceipt, ForgeQueryReadRelationshipProofPosture,
};

impl ForgeQueryReadReceipt {
    pub(in crate::runtime) fn from_materialized_rows(
        read_graph: &ForgeQueryReadGraph,
        snapshot_token: String,
        execution: &crate::execution::ExecutionResultEnvelope,
        rows: &[ForgeQueryEntity],
    ) -> Self {
        let execution_counters = execution
            .counters()
            .clone()
            .with_materialized_row_count(rows.len());
        Self::from_parts(
            read_graph,
            execution.report().query_digest().as_str(),
            execution.report().basis_digest().as_str(),
            snapshot_token,
            execution_engine_for_planned_route(read_graph),
            fallback_class_for_planned_route(read_graph),
            execution_counters.execution_fallback_taken_count(),
            ForgeQueryReadBreadth {
                planned_read_surface_count: read_graph
                    .execution_plan()
                    .counters()
                    .planned_read_surface_count(),
                planned_traversal_clause_count: read_graph
                    .execution_plan()
                    .counters()
                    .planned_traversal_clause_count()
                    .max(read_graph.declared_traversal_clause_count()),
                planned_traversal_depth_limit: read_graph
                    .execution_plan()
                    .counters()
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
            rows,
        )
    }

    pub(in crate::runtime) fn from_query_context_execution(
        read_graph: &ForgeQueryReadGraph,
        snapshot_token: String,
        context_execution: &QueryContextExecutionArtifact,
        rows: &[ForgeQueryEntity],
    ) -> Self {
        Self::from_parts(
            read_graph,
            context_execution.query_digest(),
            context_execution.basis_digest(),
            snapshot_token,
            execution_engine_for_query_context(context_execution.family()),
            fallback_class_for_planned_route(read_graph),
            0,
            ForgeQueryReadBreadth {
                planned_read_surface_count: read_graph
                    .execution_plan()
                    .counters()
                    .planned_read_surface_count(),
                planned_traversal_clause_count: read_graph
                    .execution_plan()
                    .counters()
                    .planned_traversal_clause_count()
                    .max(read_graph.declared_traversal_clause_count()),
                planned_traversal_depth_limit: read_graph
                    .execution_plan()
                    .counters()
                    .planned_traversal_depth_limit()
                    .max(read_graph.declared_traversal_depth_limit()),
                execution_read_operation_count: context_execution
                    .counters()
                    .context_execution_count(),
                execution_records_examined_count: context_execution.counters().payload_row_count(),
                execution_records_emitted_count: rows.len(),
                execution_page_width: context_execution.counters().payload_row_count(),
                execution_page_truncation_count: 0,
                execution_cursor_advance_count: 0,
                execution_materialized_relation_count: rows.len(),
            },
            rows,
        )
    }

    fn from_parts(
        read_graph: &ForgeQueryReadGraph,
        query_digest: &str,
        basis_digest: &str,
        snapshot_token: String,
        execution_engine: ForgeQueryReadExecutionEngine,
        fallback_class: ForgeQueryReadFallbackClass,
        fallback_count: usize,
        breadth: ForgeQueryReadBreadth,
        rows: &[ForgeQueryEntity],
    ) -> Self {
        let relationship_proof_admission = read_graph.relationship_proof_admission().cloned();
        let relationship_proof_support_profile = relationship_proof_admission
            .as_ref()
            .map(support_profile_for_relationship_proof);
        Self {
            read_graph_digest: read_graph.digest().to_string(),
            graph_family: read_graph.family().clone(),
            query_digest: query_digest.to_string(),
            basis_digest: basis_digest.to_string(),
            result_digest: materialized_result_digest(query_digest, basis_digest, rows)
                .as_str()
                .to_string(),
            snapshot_token,
            scope_class: read_graph.scope_class().clone(),
            execution_engine,
            fallback_class,
            fallback_count,
            operator_families: read_graph.operator_families(),
            built_in_operator_coverage: read_graph.built_in_operators().to_vec(),
            relationship_proof_posture: if relationship_proof_admission.is_some() {
                ForgeQueryReadRelationshipProofPosture::DescriptorAdmittedSyntheticRuntime
            } else {
                ForgeQueryReadRelationshipProofPosture::NotRequired
            },
            relationship_proof_admission,
            relationship_proof_support_profile,
            breadth,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }
}

fn fallback_class_for_planned_route(
    read_graph: &ForgeQueryReadGraph,
) -> ForgeQueryReadFallbackClass {
    match read_graph.execution_plan().query().fallback() {
        FallbackDisposition::Forbidden | FallbackDisposition::AdmittedButUnused => {
            ForgeQueryReadFallbackClass::None
        }
        FallbackDisposition::AdmittedAndSelected => {
            ForgeQueryReadFallbackClass::SnapshotIndexedDebt
        }
    }
}

fn execution_engine_for_planned_route(
    read_graph: &ForgeQueryReadGraph,
) -> ForgeQueryReadExecutionEngine {
    match read_graph.execution_plan().query().route() {
        PlannedExecutionRoute::RuntimeSnapshotRead
        | PlannedExecutionRoute::RuntimeExpandedSnapshotRead
        | PlannedExecutionRoute::StoreSnapshotRead => {
            ForgeQueryReadExecutionEngine::QueryRuntimeCurrent
        }
    }
}

fn execution_engine_for_query_context(
    family: &QueryContextExecutionFamily,
) -> ForgeQueryReadExecutionEngine {
    match family {
        QueryContextExecutionFamily::RuntimeCurrent => {
            ForgeQueryReadExecutionEngine::QueryRuntimeCurrent
        }
        QueryContextExecutionFamily::RuntimeBranch => {
            ForgeQueryReadExecutionEngine::QueryRuntimeBranch
        }
        QueryContextExecutionFamily::HistoricalMaterialized => {
            ForgeQueryReadExecutionEngine::QueryRuntimeHistorical
        }
        QueryContextExecutionFamily::PreviewDerivedHistorical => {
            ForgeQueryReadExecutionEngine::QueryRuntimePreviewDerived
        }
    }
}
