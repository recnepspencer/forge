use super::super::RadialRingMigrationError;
use crate::projection::read_views::domain::{
    TopologyReadFallbackPosture, TopologyReadRequestFamily, TopologyReadRequestReport,
};
use crate::projection::read_views::TopologyHalfEdgeRadialNeighborhoodView;
use forge_query::facade::ForgeQueryReadScopeClass;

pub(super) fn query_report_digests_for_radial_views(
    radial_views: &[TopologyHalfEdgeRadialNeighborhoodView],
) -> Result<Vec<String>, RadialRingMigrationError> {
    radial_views
        .iter()
        .map(|view| query_report_digest(view.request_report()))
        .collect()
}

fn query_report_digest(
    report: &TopologyReadRequestReport,
) -> Result<String, RadialRingMigrationError> {
    require_query_radial_report(report)?;
    Ok(super::super::super::super::catalog::catalog_digest([
        "worth-topo:radial-ring-query-report:v1".to_string(),
        format!("request-family:{}", report.request_family().as_str()),
        format!("execution-engine:{}", report.execution_engine().as_str()),
        format!(
            "executed-query:{}",
            report.executed_query_digest().unwrap_or("missing")
        ),
        format!(
            "executed-basis:{}",
            report.executed_basis_digest().unwrap_or("missing")
        ),
        format!("canonical-query:{}", report.canonical_query_digest()),
        format!(
            "canonical-result:{}",
            report.canonical_result_shape_digest()
        ),
        format!("root-entity:{}", report.root_entity()),
        format!("query-executions:{}", report.query_execution_count()),
        format!(
            "relationship-proof-admissions:{}",
            report.relationship_proof_admission_count()
        ),
        format!("row-scan-fallbacks:{}", report.row_scan_fallback_count()),
        format!(
            "whole-view-fallbacks:{}",
            report.whole_view_fallback_count()
        ),
        format!(
            "repeated-rediscovery-denials:{}",
            report.repeated_rediscovery_denied_count()
        ),
    ]))
}

fn require_query_radial_report(
    report: &TopologyReadRequestReport,
) -> Result<(), RadialRingMigrationError> {
    if report.request_family() != TopologyReadRequestFamily::HalfEdgeRadialNeighborhood {
        return Err(RadialRingMigrationError::ReadStageQueryProofInvalid);
    }
    if report.claimed_scope_class() != ForgeQueryReadScopeClass::LocalNeighborhood
        || report.executed_scope_class() != Some(ForgeQueryReadScopeClass::LocalNeighborhood)
    {
        return Err(RadialRingMigrationError::ReadStageQueryProofInvalid);
    }
    if report.fallback_posture() != TopologyReadFallbackPosture::None
        || report.query_execution_count() != 1
        || report.relationship_proof_admission_count() == 0
        || report.row_scan_fallback_count() != 0
        || report.whole_view_fallback_count() != 0
        || report.repeated_rediscovery_denied_count() != 0
    {
        return Err(RadialRingMigrationError::ReadStageQueryProofInvalid);
    }
    let graph_access_proof = report
        .graph_access_proof()
        .ok_or(RadialRingMigrationError::ReadStageQueryProofInvalid)?;
    if !graph_access_proof.no_caller_owned_graph_work() {
        return Err(RadialRingMigrationError::ReadStageQueryProofInvalid);
    }
    Ok(())
}
