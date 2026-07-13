use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmReplayAdmissionRequest,
};

pub(super) fn admit(
    request: LsmReplayAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmReplayAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let materialization = crate::access_planning()
        .admit_lsm_replay_materialization(family, request.catalog, request.source)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let frontier = crate::access_planning()
        .current_lsm_replay_materialization_frontier(request.catalog, request.source)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let current = materialization
        .clone()
        .require_current_at(frontier)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let shape = crate::access_planning()
        .rebuild_access(crate::AccessLaneClassification::Maintenance)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_recovery_request(family, concrete_key, materialization, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmReplayRecovery(_) => crate::BaselineLsmReplayAdmission::admit(
            outcome
                .into_lsm_replay_recovery()
                .expect("view established LSM replay selection"),
            request.source.clone(),
            current,
        )
        .map_err(|_| LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
    }
}
