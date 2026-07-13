use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmCompactionAdmissionRequest,
};

pub(super) fn admit(
    request: LsmCompactionAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmCompactionAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let shape = crate::access_shapes()
        .compaction_read(crate::PhysicalMutationShape::CompactionRewrite)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_mutation_request(family, concrete_key, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmCompaction(_) => {
            Ok(crate::BaselineLsmCompactionAdmission::admit(
                outcome
                    .into_lsm_compaction()
                    .expect("view established LSM compaction selection"),
            ))
        }
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
    }
}
