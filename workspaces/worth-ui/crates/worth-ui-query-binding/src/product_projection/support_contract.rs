use worth_query::facade::{
    consumer_kit::{
        project_workspace_support_snapshot, support_pinning_contract,
        WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture, WorthQuerySupportPinReport,
        WorthQuerySupportPinningError,
    },
    runtime::{WorthQueryRuntimeFacadeFamily, WorthQueryWorkspace},
};

#[allow(
    clippy::result_large_err,
    reason = "cold support pinning preserves the exact Query diagnostic topology"
)]
pub(crate) fn evaluate_product_projection_support(
    workspace: &WorthQueryWorkspace,
) -> Result<WorthQuerySupportPinReport, WorthQuerySupportPinningError> {
    let snapshot = project_workspace_support_snapshot(workspace);
    let contract = [
        WorthQueryRuntimeFacadeFamily::Read,
        WorthQueryRuntimeFacadeFamily::Live,
        WorthQueryRuntimeFacadeFamily::AsyncResource,
        WorthQueryRuntimeFacadeFamily::MixedCauseDelivery,
        WorthQueryRuntimeFacadeFamily::Intent,
    ]
    .into_iter()
    .try_fold(
        support_pinning_contract("worth-ui-query-binding").against_snapshot(&snapshot)?,
        |contract, family| {
            contract.require_family(family, |row| {
                row.status(WorthQueryPinnedSupportStatus::Supported)
                    .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                    .bind_live_row_digest()
            })
        },
    )?
    .seal()?;

    let report = contract.evaluate_snapshot(&snapshot)?;
    report.assert_satisfied()?;
    Ok(report)
}
