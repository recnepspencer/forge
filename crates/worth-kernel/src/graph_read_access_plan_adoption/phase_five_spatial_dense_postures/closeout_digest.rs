use crate::graph_read_access_plan_adoption::WorthGraphReadAccessFirstVerticalSliceSeed;

use super::batch_admission::WorthGraphReadAccessGroupedAdmissionReport;
use super::bounded_execution::WorthGraphReadAccessBoundedExecutionContract;
use super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureProjection;
use super::source_firewall::WorthGraphReadAccessSpatialDenseSourceFirewallReport;
use super::stable_digest;

pub(crate) fn spatial_dense_closeout_digest(
    seed: &WorthGraphReadAccessFirstVerticalSliceSeed,
    projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
    grouped_admission_report: &WorthGraphReadAccessGroupedAdmissionReport,
    bounded_execution_contract: &WorthGraphReadAccessBoundedExecutionContract,
    source_firewall_report: &WorthGraphReadAccessSpatialDenseSourceFirewallReport,
) -> String {
    stable_digest(
        &std::iter::once("worth_graph_read_access_spatial_dense_posture_closeout_v1".to_string())
            .chain([
                format!("seed:{}", seed.seed_digest()),
                format!(
                    "phase_four_receipt:{}",
                    seed.receipt_projection().projection_digest()
                ),
                format!("grouped:{}", grouped_admission_report.report_digest()),
                format!("bounded:{}", bounded_execution_contract.contract_digest()),
                format!("firewall:{}", source_firewall_report.report_digest()),
            ])
            .chain(
                projections
                    .iter()
                    .map(|projection| format!("projection:{}", projection.projection_digest())),
            )
            .collect::<Vec<_>>(),
    )
}
