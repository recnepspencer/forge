use crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningDenialReason,
};

pub(crate) struct WorthUiAllocationPlanner;

impl WorthUiAllocationPlanner {
    pub(crate) fn plan(admission: WorthUiAllocationPlanningAdmission) -> WorthUiAllocationPlanning {
        let mut counters = WorthUiAllocationPlanningCounters::default();
        counters.record_planning_attempt();
        counters.record_measurement_basis_read();

        if let Some(denial) = admission.measurement_basis().denial_posture() {
            let basis = WorthUiAllocationPlanningBasis::from_admitted(
                admission.constraint_basis().clone(),
                admission.portal_allocation_input().cloned(),
            );
            let denial_posture = Some(WorthUiAllocationPlanningDenial::new(
                WorthUiAllocationPlanningDenialReason::MeasurementBasisDenied,
                Some(denial.clone()),
                None,
                counters,
            ));
            return WorthUiAllocationPlanning::new(basis, None, denial_posture, counters);
        }

        counters.record_candidate_projection_read();
        let basis = WorthUiAllocationPlanningBasis::from_admitted(
            admission.constraint_basis().clone(),
            admission.portal_allocation_input().cloned(),
        );
        WorthUiAllocationPlanning::new(basis, Some(admission.into_projection()), None, counters)
    }
}
