use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadExecutionEngine, ForgeQueryReadReceipt,
    ForgeQueryReadScopeClass, ForgeQuerySnapshotIdentity,
};

use super::fallback::TopologyReadFallbackPosture;
use super::graph_access::TopologyReadGraphAccessProof;
use crate::projection::runtime_boundary::read_lowering::{
    TopologyReadLoweringArtifact, TopologyReadRelationshipProofPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyReadExecutionEngine {
    QueryRuntimeCurrent,
    QueryRuntimeBranch,
    QueryRuntimeHistorical,
    QueryRuntimePreviewDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyReadRequestFamily {
    HalfEdgeSharedVertexNeighborhood,
    HalfEdgeRadialNeighborhood,
    LoopCycleNeighborhood,
    LocalRewireNeighborhood,
}

impl TopologyReadRequestFamily {
    pub const ALL: [Self; 4] = [
        Self::HalfEdgeSharedVertexNeighborhood,
        Self::HalfEdgeRadialNeighborhood,
        Self::LoopCycleNeighborhood,
        Self::LocalRewireNeighborhood,
    ];

    pub fn claimed_scope_class(self) -> ForgeQueryReadScopeClass {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood => ForgeQueryReadScopeClass::LocalNeighborhood,
            Self::HalfEdgeRadialNeighborhood => ForgeQueryReadScopeClass::LocalNeighborhood,
            Self::LoopCycleNeighborhood => ForgeQueryReadScopeClass::ExplicitBroadSearch,
            Self::LocalRewireNeighborhood => ForgeQueryReadScopeClass::AnchoredExpansion,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadRequestReport {
    pub(crate) request_family: TopologyReadRequestFamily,
    pub(crate) lowering_artifact: TopologyReadLoweringArtifact,
    pub(crate) execution_engine: TopologyReadExecutionEngine,
    pub(crate) executed_scope_class: Option<ForgeQueryReadScopeClass>,
    pub(crate) executed_query_digest: Option<String>,
    pub(crate) executed_basis_digest: Option<String>,
    pub(crate) executed_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
    pub(crate) executed_built_in_operator_coverage: Vec<ForgeQueryReadBuiltInOperator>,
    pub(crate) fallback_posture: TopologyReadFallbackPosture,
    pub(crate) query_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
    pub(crate) graph_access_proof: Option<TopologyReadGraphAccessProof>,
}

impl TopologyReadRequestReport {
    pub(crate) fn query_execution_without_fallback_debt(
        lowering_artifact: TopologyReadLoweringArtifact,
        receipt: &ForgeQueryReadReceipt,
    ) -> Self {
        Self {
            request_family: lowering_artifact.request_family(),
            execution_engine: topology_execution_engine_from_receipt(receipt),
            executed_scope_class: Some(receipt.scope_class().clone()),
            executed_query_digest: Some(receipt.query_digest().to_string()),
            executed_basis_digest: Some(receipt.basis_digest().to_string()),
            executed_snapshot_identity: Some(receipt.snapshot_identity().clone()),
            executed_built_in_operator_coverage: receipt.built_in_operator_coverage().to_vec(),
            lowered_traversal_count: lowering_artifact.traversal_steps().len(),
            relationship_proof_admission_count: lowering_artifact
                .relationship_proof_admission_count(),
            lowering_artifact,
            fallback_posture: TopologyReadFallbackPosture::None,
            query_execution_count: 1,
            row_scan_fallback_count: 0,
            whole_view_fallback_count: 0,
            repeated_rediscovery_denied_count: 0,
            graph_access_proof: TopologyReadGraphAccessProof::from_receipt(receipt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadFamilyAggregateRow {
    pub(crate) request_family: TopologyReadRequestFamily,
    pub(crate) request_count: usize,
    pub(crate) query_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadDebtRow {
    pub(crate) request_family: TopologyReadRequestFamily,
    pub(crate) request_count: usize,
    pub(crate) fallback_posture: TopologyReadFallbackPosture,
    pub(crate) relationship_proof_posture: TopologyReadRelationshipProofPosture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadExecutionAggregateRow {
    pub(crate) request_family: TopologyReadRequestFamily,
    pub(crate) claimed_scope_class: ForgeQueryReadScopeClass,
    pub(crate) executed_scope_class: Option<ForgeQueryReadScopeClass>,
    pub(crate) execution_engine: TopologyReadExecutionEngine,
    pub(crate) fallback_posture: TopologyReadFallbackPosture,
    pub(crate) relationship_proof_posture: TopologyReadRelationshipProofPosture,
    pub(crate) request_count: usize,
    pub(crate) query_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadAggregateReport {
    pub(crate) request_count: usize,
    pub(crate) query_runtime_current_execution_count: usize,
    pub(crate) query_runtime_historical_execution_count: usize,
    pub(crate) local_neighborhood_execution_count: usize,
    pub(crate) anchored_expansion_execution_count: usize,
    pub(crate) explicit_broad_search_execution_count: usize,
    pub(crate) locality_claim_mismatch_count: usize,
    pub(crate) query_execution_count: usize,
    pub(crate) lowered_traversal_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
    pub(crate) family_rows: Vec<TopologyReadFamilyAggregateRow>,
    pub(crate) debt_rows: Vec<TopologyReadDebtRow>,
    pub(crate) execution_rows: Vec<TopologyReadExecutionAggregateRow>,
}

impl TopologyReadAggregateReport {
    pub(crate) fn from_request_reports(reports: &[TopologyReadRequestReport]) -> Self {
        let mut family_rows =
            BTreeMap::<TopologyReadRequestFamily, TopologyReadFamilyAggregateRow>::new();
        let mut debt_rows = BTreeMap::<
            (
                TopologyReadRequestFamily,
                TopologyReadFallbackPosture,
                TopologyReadRelationshipProofPosture,
            ),
            TopologyReadDebtRow,
        >::new();
        let mut execution_rows = BTreeMap::<
            (
                TopologyReadRequestFamily,
                &'static str,
                Option<&'static str>,
                TopologyReadExecutionEngine,
                TopologyReadFallbackPosture,
                TopologyReadRelationshipProofPosture,
            ),
            TopologyReadExecutionAggregateRow,
        >::new();
        let mut aggregate = Self {
            request_count: reports.len(),
            query_runtime_current_execution_count: 0,
            query_runtime_historical_execution_count: 0,
            local_neighborhood_execution_count: 0,
            anchored_expansion_execution_count: 0,
            explicit_broad_search_execution_count: 0,
            locality_claim_mismatch_count: 0,
            query_execution_count: 0,
            lowered_traversal_count: 0,
            relationship_proof_admission_count: 0,
            row_scan_fallback_count: 0,
            whole_view_fallback_count: 0,
            repeated_rediscovery_denied_count: 0,
            family_rows: Vec::new(),
            debt_rows: Vec::new(),
            execution_rows: Vec::new(),
        };
        for report in reports {
            let claimed_scope_class = report.request_family.claimed_scope_class();
            let claimed_scope_key = scope_class_key(&claimed_scope_class);
            let executed_scope_key = report.executed_scope_class.as_ref().map(scope_class_key);
            aggregate.query_execution_count += report.query_execution_count;
            if report.execution_engine == TopologyReadExecutionEngine::QueryRuntimeCurrent {
                aggregate.query_runtime_current_execution_count += report.query_execution_count;
            }
            if report.execution_engine == TopologyReadExecutionEngine::QueryRuntimeHistorical {
                aggregate.query_runtime_historical_execution_count += report.query_execution_count;
            }
            match report.executed_scope_class {
                Some(ForgeQueryReadScopeClass::LocalNeighborhood) => {
                    aggregate.local_neighborhood_execution_count += report.query_execution_count;
                }
                Some(ForgeQueryReadScopeClass::AnchoredExpansion) => {
                    aggregate.anchored_expansion_execution_count += report.query_execution_count;
                }
                Some(ForgeQueryReadScopeClass::ExplicitBroadSearch) => {
                    aggregate.explicit_broad_search_execution_count += report.query_execution_count;
                }
                None => {}
            }
            if report.executed_scope_class != Some(claimed_scope_class.clone()) {
                aggregate.locality_claim_mismatch_count += 1;
            }
            aggregate.lowered_traversal_count += report.lowered_traversal_count;
            aggregate.relationship_proof_admission_count +=
                report.relationship_proof_admission_count;
            aggregate.row_scan_fallback_count += report.row_scan_fallback_count;
            aggregate.whole_view_fallback_count += report.whole_view_fallback_count;
            aggregate.repeated_rediscovery_denied_count += report.repeated_rediscovery_denied_count;
            let family_row = family_rows.entry(report.request_family).or_insert(
                TopologyReadFamilyAggregateRow {
                    request_family: report.request_family,
                    request_count: 0,
                    query_execution_count: 0,
                    lowered_traversal_count: 0,
                    relationship_proof_admission_count: 0,
                    row_scan_fallback_count: 0,
                    whole_view_fallback_count: 0,
                    repeated_rediscovery_denied_count: 0,
                },
            );
            family_row.request_count += 1;
            family_row.query_execution_count += report.query_execution_count;
            family_row.lowered_traversal_count += report.lowered_traversal_count;
            family_row.relationship_proof_admission_count +=
                report.relationship_proof_admission_count;
            family_row.row_scan_fallback_count += report.row_scan_fallback_count;
            family_row.whole_view_fallback_count += report.whole_view_fallback_count;
            family_row.repeated_rediscovery_denied_count +=
                report.repeated_rediscovery_denied_count;
            let execution_row = execution_rows
                .entry((
                    report.request_family,
                    claimed_scope_key,
                    executed_scope_key,
                    report.execution_engine,
                    report.fallback_posture,
                    report.lowering_artifact.relationship_proof_posture(),
                ))
                .or_insert(TopologyReadExecutionAggregateRow {
                    request_family: report.request_family,
                    claimed_scope_class,
                    executed_scope_class: report.executed_scope_class.clone(),
                    execution_engine: report.execution_engine,
                    fallback_posture: report.fallback_posture,
                    relationship_proof_posture: report
                        .lowering_artifact
                        .relationship_proof_posture(),
                    request_count: 0,
                    query_execution_count: 0,
                    lowered_traversal_count: 0,
                    relationship_proof_admission_count: 0,
                    row_scan_fallback_count: 0,
                    whole_view_fallback_count: 0,
                    repeated_rediscovery_denied_count: 0,
                });
            execution_row.request_count += 1;
            execution_row.query_execution_count += report.query_execution_count;
            execution_row.lowered_traversal_count += report.lowered_traversal_count;
            execution_row.relationship_proof_admission_count +=
                report.relationship_proof_admission_count;
            execution_row.row_scan_fallback_count += report.row_scan_fallback_count;
            execution_row.whole_view_fallback_count += report.whole_view_fallback_count;
            execution_row.repeated_rediscovery_denied_count +=
                report.repeated_rediscovery_denied_count;
            if report.fallback_posture != TopologyReadFallbackPosture::None
                || report.lowering_artifact.relationship_proof_posture()
                    == TopologyReadRelationshipProofPosture::Deferred
            {
                let debt_row = debt_rows
                    .entry((
                        report.request_family,
                        report.fallback_posture,
                        report.lowering_artifact.relationship_proof_posture(),
                    ))
                    .or_insert(TopologyReadDebtRow {
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
        aggregate.execution_rows = execution_rows.into_values().collect();
        aggregate
    }
}

fn topology_execution_engine_from_receipt(
    receipt: &ForgeQueryReadReceipt,
) -> TopologyReadExecutionEngine {
    match receipt.execution_engine() {
        ForgeQueryReadExecutionEngine::QueryRuntimeCurrent => {
            TopologyReadExecutionEngine::QueryRuntimeCurrent
        }
        ForgeQueryReadExecutionEngine::QueryRuntimeHistorical => {
            TopologyReadExecutionEngine::QueryRuntimeHistorical
        }
        ForgeQueryReadExecutionEngine::QueryRuntimeBranch => {
            TopologyReadExecutionEngine::QueryRuntimeBranch
        }
        ForgeQueryReadExecutionEngine::QueryRuntimePreviewDerived => {
            TopologyReadExecutionEngine::QueryRuntimePreviewDerived
        }
    }
}

fn scope_class_key(scope_class: &ForgeQueryReadScopeClass) -> &'static str {
    match scope_class {
        ForgeQueryReadScopeClass::LocalNeighborhood => "local_neighborhood",
        ForgeQueryReadScopeClass::AnchoredExpansion => "anchored_expansion",
        ForgeQueryReadScopeClass::ExplicitBroadSearch => "explicit_broad_search",
    }
}
