use forge_query::facade::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadReceipt, ForgeQueryReadScopeClass,
};

use super::fallback::WorthTopologyDomainQueryFallbackPosture;
use super::lowering::{
    WorthTopologyDomainQueryLoweringArtifact, WorthTopologyDomainQueryRelationshipProofPosture,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthTopologyDomainQueryExecutionEngine {
    SnapshotIndexPrimary,
    QueryRuntimeCurrent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthTopologyDomainQueryRequestFamily {
    HalfEdgeSharedVertexNeighborhood,
    HalfEdgeRadialNeighborhood,
    LoopCycleNeighborhood,
    LocalRewireNeighborhood,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryRequestReport {
    pub(crate) request_family: WorthTopologyDomainQueryRequestFamily,
    pub(crate) lowering_artifact: WorthTopologyDomainQueryLoweringArtifact,
    pub(crate) execution_engine: WorthTopologyDomainQueryExecutionEngine,
    pub(crate) executed_scope_class: Option<ForgeQueryReadScopeClass>,
    pub(crate) executed_query_digest: Option<String>,
    pub(crate) executed_built_in_operator_coverage: Vec<ForgeQueryReadBuiltInOperator>,
    pub(crate) fallback_posture: WorthTopologyDomainQueryFallbackPosture,
    pub(crate) query_native_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
}

impl WorthTopologyDomainQueryRequestReport {
    pub(crate) fn snapshot_indexed_fallback(
        lowering_artifact: WorthTopologyDomainQueryLoweringArtifact,
    ) -> Self {
        Self {
            request_family: lowering_artifact.request_family(),
            execution_engine: WorthTopologyDomainQueryExecutionEngine::SnapshotIndexPrimary,
            executed_scope_class: None,
            executed_query_digest: None,
            executed_built_in_operator_coverage: Vec::new(),
            lowered_traversal_count: lowering_artifact.traversal_steps().len(),
            relationship_proof_admission_count: lowering_artifact
                .relationship_proof_admission_count(),
            lowering_artifact,
            fallback_posture: WorthTopologyDomainQueryFallbackPosture::SnapshotIndexedFallback,
            query_native_execution_count: 0,
            row_scan_fallback_count: 1,
            whole_view_fallback_count: 0,
            repeated_rediscovery_denied_count: 0,
        }
    }

    pub(crate) fn query_runtime_current_whole_view_debt(
        lowering_artifact: WorthTopologyDomainQueryLoweringArtifact,
        receipt: &ForgeQueryReadReceipt,
    ) -> Self {
        Self {
            request_family: lowering_artifact.request_family(),
            execution_engine: WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent,
            executed_scope_class: Some(receipt.scope_class().clone()),
            executed_query_digest: Some(receipt.query_digest().to_string()),
            executed_built_in_operator_coverage: receipt.built_in_operator_coverage().to_vec(),
            lowered_traversal_count: lowering_artifact.traversal_steps().len(),
            relationship_proof_admission_count: lowering_artifact
                .relationship_proof_admission_count(),
            lowering_artifact,
            fallback_posture: WorthTopologyDomainQueryFallbackPosture::WholeViewDebt,
            query_native_execution_count: 1,
            row_scan_fallback_count: 0,
            whole_view_fallback_count: 1,
            repeated_rediscovery_denied_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryFamilyAggregateRow {
    pub(crate) request_family: WorthTopologyDomainQueryRequestFamily,
    pub(crate) request_count: usize,
    pub(crate) query_native_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryDebtRow {
    pub(crate) request_family: WorthTopologyDomainQueryRequestFamily,
    pub(crate) request_count: usize,
    pub(crate) fallback_posture: WorthTopologyDomainQueryFallbackPosture,
    pub(crate) relationship_proof_posture: WorthTopologyDomainQueryRelationshipProofPosture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryAggregateReport {
    pub(crate) request_count: usize,
    pub(crate) query_native_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
    pub(crate) family_rows: Vec<WorthTopologyDomainQueryFamilyAggregateRow>,
    pub(crate) debt_rows: Vec<WorthTopologyDomainQueryDebtRow>,
}

impl WorthTopologyDomainQueryAggregateReport {
    pub(crate) fn from_request_reports(reports: &[WorthTopologyDomainQueryRequestReport]) -> Self {
        let mut family_rows = BTreeMap::<
            WorthTopologyDomainQueryRequestFamily,
            WorthTopologyDomainQueryFamilyAggregateRow,
        >::new();
        let mut debt_rows = BTreeMap::<
            (
                WorthTopologyDomainQueryRequestFamily,
                WorthTopologyDomainQueryFallbackPosture,
                WorthTopologyDomainQueryRelationshipProofPosture,
            ),
            WorthTopologyDomainQueryDebtRow,
        >::new();
        let mut aggregate = Self {
            request_count: reports.len(),
            query_native_execution_count: 0,
            lowered_traversal_count: 0,
            relationship_proof_admission_count: 0,
            row_scan_fallback_count: 0,
            whole_view_fallback_count: 0,
            repeated_rediscovery_denied_count: 0,
            family_rows: Vec::new(),
            debt_rows: Vec::new(),
        };
        for report in reports {
            aggregate.query_native_execution_count += report.query_native_execution_count;
            aggregate.lowered_traversal_count += report.lowered_traversal_count;
            aggregate.relationship_proof_admission_count +=
                report.relationship_proof_admission_count;
            aggregate.row_scan_fallback_count += report.row_scan_fallback_count;
            aggregate.whole_view_fallback_count += report.whole_view_fallback_count;
            aggregate.repeated_rediscovery_denied_count += report.repeated_rediscovery_denied_count;
            let family_row = family_rows.entry(report.request_family).or_insert(
                WorthTopologyDomainQueryFamilyAggregateRow {
                    request_family: report.request_family,
                    request_count: 0,
                    query_native_execution_count: 0,
                    lowered_traversal_count: 0,
                    relationship_proof_admission_count: 0,
                    row_scan_fallback_count: 0,
                    whole_view_fallback_count: 0,
                    repeated_rediscovery_denied_count: 0,
                },
            );
            family_row.request_count += 1;
            family_row.query_native_execution_count += report.query_native_execution_count;
            family_row.lowered_traversal_count += report.lowered_traversal_count;
            family_row.relationship_proof_admission_count +=
                report.relationship_proof_admission_count;
            family_row.row_scan_fallback_count += report.row_scan_fallback_count;
            family_row.whole_view_fallback_count += report.whole_view_fallback_count;
            family_row.repeated_rediscovery_denied_count +=
                report.repeated_rediscovery_denied_count;
            if report.fallback_posture != WorthTopologyDomainQueryFallbackPosture::None
                || report.lowering_artifact.relationship_proof_posture()
                    == WorthTopologyDomainQueryRelationshipProofPosture::Deferred
            {
                let debt_row = debt_rows
                    .entry((
                        report.request_family,
                        report.fallback_posture,
                        report.lowering_artifact.relationship_proof_posture(),
                    ))
                    .or_insert(WorthTopologyDomainQueryDebtRow {
                        request_family: report.request_family,
                        request_count: 0,
                        fallback_posture: report.fallback_posture,
                        relationship_proof_posture: report
                            .lowering_artifact
                            .relationship_proof_posture(),
                    });
                debt_row.request_count += 1;
            }
        }
        aggregate.family_rows = family_rows.into_values().collect();
        aggregate.debt_rows = debt_rows.into_values().collect();
        aggregate
    }
}
