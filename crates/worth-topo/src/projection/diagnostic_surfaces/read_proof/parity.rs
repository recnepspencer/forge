use schema::facade::platform::authority::DerivedTopologyReadBasis;

use super::fallback::TopologyDomainQueryFallbackPosture;
use super::report::{
    TopologyDomainQueryExecutionEngine, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport,
};
use crate::projection::read_views::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyDomainQueryParityKind {
    Replay,
    BranchLocal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyDomainQueryViewParityArtifact {
    request_family: TopologyDomainQueryRequestFamily,
    authority_snapshot_id: u64,
    authority_branch_id: String,
    execution_engine: TopologyDomainQueryExecutionEngine,
    fallback_posture: TopologyDomainQueryFallbackPosture,
    query_execution_count: usize,
    canonical_query_digest: String,
    canonical_result_shape_digest: String,
    lowered_traversal_count: usize,
    relationship_proof_admission_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
    view_digest_hex: String,
}

impl TopologyDomainQueryViewParityArtifact {
    pub(crate) fn request_family(&self) -> TopologyDomainQueryRequestFamily {
        self.request_family
    }

    pub(crate) fn authority_snapshot_id(&self) -> u64 {
        self.authority_snapshot_id
    }

    pub(crate) fn authority_branch_id(&self) -> &str {
        self.authority_branch_id.as_str()
    }

    pub(crate) fn view_digest_hex(&self) -> &str {
        self.view_digest_hex.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyDomainQueryViewParityReport {
    pub(crate) parity_kind: TopologyDomainQueryParityKind,
    pub(crate) request_family: TopologyDomainQueryRequestFamily,
    pub(crate) left_branch_id: String,
    pub(crate) right_branch_id: String,
    pub(crate) left_snapshot_id: u64,
    pub(crate) right_snapshot_id: u64,
    pub(crate) branch_identity_match: bool,
    pub(crate) snapshot_identity_match: bool,
    pub(crate) fallback_posture_match: bool,
    pub(crate) execution_engine_match: bool,
    pub(crate) canonical_query_digest_match: bool,
    pub(crate) canonical_result_shape_digest_match: bool,
    pub(crate) breadth_counters_match: bool,
    pub(crate) view_digest_match: bool,
    pub(crate) parity_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryParityAggregateRow {
    pub(crate) parity_kind: TopologyDomainQueryParityKind,
    pub(crate) request_family: TopologyDomainQueryRequestFamily,
    pub(crate) checked_count: usize,
    pub(crate) verified_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryParityAggregateReport {
    pub(crate) domain_query_parity_count: usize,
    pub(crate) view_determinism_checked_count: usize,
    pub(crate) view_determinism_verified_count: usize,
    pub(crate) replay_checked_count: usize,
    pub(crate) replay_verified_count: usize,
    pub(crate) branch_local_checked_count: usize,
    pub(crate) branch_local_verified_count: usize,
    pub(crate) parity_rows: Vec<TopologyDomainQueryParityAggregateRow>,
}

impl TopologyDomainQueryParityAggregateReport {
    pub(crate) fn from_reports(reports: &[TopologyDomainQueryViewParityReport]) -> Self {
        let mut parity_rows = std::collections::BTreeMap::<
            (
                TopologyDomainQueryParityKind,
                TopologyDomainQueryRequestFamily,
            ),
            TopologyDomainQueryParityAggregateRow,
        >::new();
        let replay_checked_count = reports
            .iter()
            .filter(|report| report.parity_kind == TopologyDomainQueryParityKind::Replay)
            .count();
        let replay_verified_count = reports
            .iter()
            .filter(|report| {
                report.parity_kind == TopologyDomainQueryParityKind::Replay
                    && report.parity_verified
            })
            .count();
        let branch_local_checked_count = reports
            .iter()
            .filter(|report| report.parity_kind == TopologyDomainQueryParityKind::BranchLocal)
            .count();
        let branch_local_verified_count = reports
            .iter()
            .filter(|report| {
                report.parity_kind == TopologyDomainQueryParityKind::BranchLocal
                    && report.parity_verified
            })
            .count();
        for report in reports {
            let row = parity_rows
                .entry((report.parity_kind, report.request_family))
                .or_insert(TopologyDomainQueryParityAggregateRow {
                    parity_kind: report.parity_kind,
                    request_family: report.request_family,
                    checked_count: 0,
                    verified_count: 0,
                });
            row.checked_count += 1;
            if report.parity_verified {
                row.verified_count += 1;
            }
        }
        Self {
            domain_query_parity_count: reports.len(),
            view_determinism_checked_count: reports.len(),
            view_determinism_verified_count: replay_verified_count + branch_local_verified_count,
            replay_checked_count,
            replay_verified_count,
            branch_local_checked_count,
            branch_local_verified_count,
            parity_rows: parity_rows.into_values().collect(),
        }
    }
}

pub(crate) enum TopologyDomainQueryViewRef<'a> {
    SharedVertex(&'a TopologyHalfEdgeSharedVertexNeighborhoodView),
    Radial(&'a TopologyHalfEdgeRadialNeighborhoodView),
    LoopCycle(&'a TopologyLoopCycleView),
    LocalRewire(&'a TopologyLocalRewireNeighborhoodView),
}

pub(crate) fn build_domain_query_view_parity_artifact(
    read_basis: &DerivedTopologyReadBasis,
    view: TopologyDomainQueryViewRef<'_>,
) -> TopologyDomainQueryViewParityArtifact {
    let request_report = request_report(&view);
    let view_digest_hex = digest_parts(&view_digest_parts(&view));
    TopologyDomainQueryViewParityArtifact {
        request_family: request_report.request_family,
        authority_snapshot_id: read_basis.snapshot().snapshot_id.0,
        authority_branch_id: read_basis.branch_id().0.clone(),
        execution_engine: request_report.execution_engine,
        fallback_posture: request_report.fallback_posture,
        query_execution_count: request_report.query_execution_count,
        canonical_query_digest: parity_query_digest(request_report),
        canonical_result_shape_digest: request_report
            .lowering_artifact
            .canonical_result_shape_digest()
            .to_string(),
        lowered_traversal_count: request_report.lowered_traversal_count,
        relationship_proof_admission_count: request_report.relationship_proof_admission_count,
        row_scan_fallback_count: request_report.row_scan_fallback_count,
        whole_view_fallback_count: request_report.whole_view_fallback_count,
        repeated_rediscovery_denied_count: request_report.repeated_rediscovery_denied_count,
        view_digest_hex,
    }
}

pub(crate) fn compare_domain_query_view_parity(
    parity_kind: TopologyDomainQueryParityKind,
    left: &TopologyDomainQueryViewParityArtifact,
    right: &TopologyDomainQueryViewParityArtifact,
) -> TopologyDomainQueryViewParityReport {
    let branch_identity_match = left.authority_branch_id == right.authority_branch_id;
    let snapshot_identity_match = left.authority_snapshot_id == right.authority_snapshot_id;
    let execution_engine_match =
        execution_engines_are_parity_compatible(left.execution_engine, right.execution_engine);
    let fallback_posture_match = left.fallback_posture == right.fallback_posture;
    let canonical_query_digest_match = left.canonical_query_digest == right.canonical_query_digest;
    let canonical_result_shape_digest_match =
        left.canonical_result_shape_digest == right.canonical_result_shape_digest;
    let breadth_counters_match = left.query_execution_count == right.query_execution_count
        && left.lowered_traversal_count == right.lowered_traversal_count
        && left.relationship_proof_admission_count == right.relationship_proof_admission_count
        && left.row_scan_fallback_count == right.row_scan_fallback_count
        && left.whole_view_fallback_count == right.whole_view_fallback_count
        && left.repeated_rediscovery_denied_count == right.repeated_rediscovery_denied_count;
    let view_digest_match = left.view_digest_hex == right.view_digest_hex;
    let parity_verified = left.request_family == right.request_family
        && branch_identity_match
        && snapshot_identity_match
        && execution_engine_match
        && fallback_posture_match
        && canonical_query_digest_match
        && canonical_result_shape_digest_match
        && breadth_counters_match
        && view_digest_match;
    TopologyDomainQueryViewParityReport {
        parity_kind,
        request_family: left.request_family,
        left_branch_id: left.authority_branch_id.clone(),
        right_branch_id: right.authority_branch_id.clone(),
        left_snapshot_id: left.authority_snapshot_id,
        right_snapshot_id: right.authority_snapshot_id,
        branch_identity_match,
        snapshot_identity_match,
        execution_engine_match,
        fallback_posture_match,
        canonical_query_digest_match,
        canonical_result_shape_digest_match,
        breadth_counters_match,
        view_digest_match,
        parity_verified,
    }
}

fn request_report<'a>(
    view: &'a TopologyDomainQueryViewRef<'a>,
) -> &'a TopologyDomainQueryRequestReport {
    match view {
        TopologyDomainQueryViewRef::SharedVertex(view) => &view.request_report,
        TopologyDomainQueryViewRef::Radial(view) => &view.request_report,
        TopologyDomainQueryViewRef::LoopCycle(view) => &view.request_report,
        TopologyDomainQueryViewRef::LocalRewire(view) => &view.request_report,
    }
}

fn execution_engines_are_parity_compatible(
    left: TopologyDomainQueryExecutionEngine,
    right: TopologyDomainQueryExecutionEngine,
) -> bool {
    matches!(
        (left, right),
        (
            TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
                | TopologyDomainQueryExecutionEngine::QueryRuntimeHistorical,
            TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
                | TopologyDomainQueryExecutionEngine::QueryRuntimeHistorical,
        )
    )
}

fn parity_query_digest(report: &TopologyDomainQueryRequestReport) -> String {
    report.executed_query_digest.clone().unwrap_or_else(|| {
        report
            .lowering_artifact
            .canonical_query_digest()
            .to_string()
    })
}

fn view_digest_parts(view: &TopologyDomainQueryViewRef<'_>) -> Vec<String> {
    match view {
        TopologyDomainQueryViewRef::SharedVertex(view) => vec![
            format!("source_half_edge:{}", view.source_half_edge_identity),
            format!("source_edge:{}", view.source_edge_identity),
            format!(
                "source_vertices:{}",
                view.source_vertex_identities.join("|")
            ),
            format!(
                "adjacent_half_edges:{}",
                view.vertex_adjacent_half_edge_identities.join("|")
            ),
            format!(
                "adjacent_different_edge_half_edges:{}",
                view.vertex_adjacent_different_edge_half_edge_identities
                    .join("|")
            ),
        ],
        TopologyDomainQueryViewRef::Radial(view) => vec![
            format!("source_half_edge:{}", view.source_half_edge_identity),
            format!("source_edge:{}", view.source_edge_identity),
            format!(
                "current_target_half_edge:{}",
                view.current_target_half_edge_identity
            ),
            format!("current_target_edge:{}", view.current_target_edge_identity),
            format!(
                "same_edge_half_edges:{}",
                view.same_edge_half_edge_identities.join("|")
            ),
            format!(
                "different_edge_half_edges:{}",
                view.different_edge_half_edge_identities.join("|")
            ),
        ],
        TopologyDomainQueryViewRef::LoopCycle(view) => vec![
            format!("start_half_edge:{}", view.start_half_edge_identity),
            format!("cycle_identities:{}", view.cycle_identities.join("|")),
        ],
        TopologyDomainQueryViewRef::LocalRewire(view) => vec![
            format!("moved_half_edge:{}", view.moved_half_edge_identity),
            format!("old_successor:{}", view.old_successor_identity),
            format!("old_predecessor:{}", view.old_predecessor_identity),
            format!("cycle_identities:{}", view.cycle_identities.join("|")),
        ],
    }
}

fn digest_parts(parts: &[String]) -> String {
    let mut state: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x100000001b3);
        }
    }
    format!("{state:016x}")
}




