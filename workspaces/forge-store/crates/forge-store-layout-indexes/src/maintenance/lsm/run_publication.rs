use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmRunPublicationAdmissionRequest,
};

pub(super) fn admit(
    request: LsmRunPublicationAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmRunPublicationAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let shape = crate::access_shapes()
        .append(crate::PhysicalMutationShape::LogStructuredAppend)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_mutation_request(family, concrete_key, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmRunPublication(_) => {
            Ok(crate::BaselineLsmRunPublicationAdmission::admit(
                outcome
                    .into_lsm_run_publication()
                    .expect("view established LSM publication selection"),
            ))
        }
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
    }
}
