use crate::runtime::allocation_planning::{
    WorthUiAllocationPlanningAdmission, WorthUiAllocationPlanningLowering,
};
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputWitness;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningDenialReason,
    WorthUiAllocationPlanningLoweringMismatch, WorthUiExecutionPlanInput,
    WorthUiPlanLoweringDenial,
};

pub(crate) struct WorthUiAllocationPlanner;

impl WorthUiAllocationPlanner {
    pub(crate) fn plan_from_lowered_input(
        admission: WorthUiAllocationPlanningAdmission,
        lowered_input: WorthUiExecutionPlanInput,
    ) -> WorthUiAllocationPlanning {
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
                None,
                None,
                counters,
            ));
            return WorthUiAllocationPlanning::new(basis, None, denial_posture, counters);
        }

        counters.record_lowering_read();
        if !admission.lowered_input_matches(&lowered_input) {
            let basis = WorthUiAllocationPlanningBasis::from_admitted(
                admission.constraint_basis().clone(),
                admission.portal_allocation_input().cloned(),
            );
            let denial_posture = Some(WorthUiAllocationPlanningDenial::new(
                WorthUiAllocationPlanningDenialReason::LoweringAdmissionMismatch,
                None,
                None,
                Some(WorthUiAllocationPlanningLoweringMismatch::new(
                    admission.expected_lowering_basis().clone(),
                    lowered_input.basis().clone(),
                    admission.expected_lowered_witness_digest(),
                    WorthUiExecutionPlanInputWitness::from_execution_plan_input(&lowered_input)
                        .digest(),
                )),
                None,
                counters,
            ));
            return WorthUiAllocationPlanning::new(basis, None, denial_posture, counters);
        }

        let basis = WorthUiAllocationPlanningBasis::from_admitted(
            admission.constraint_basis().clone(),
            admission.portal_allocation_input().cloned(),
        );
        let lowering =
            Some(WorthUiAllocationPlanningLowering::from_execution_plan_input(lowered_input));
        WorthUiAllocationPlanning::new(basis, lowering, None, counters)
    }

    pub(crate) fn deny_from_plan_lowering(
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
        plan_lowering_denial: WorthUiPlanLoweringDenial,
    ) -> WorthUiAllocationPlanning {
        let mut counters = WorthUiAllocationPlanningCounters::default();
        counters.record_planning_attempt();
        counters.record_measurement_basis_read();
        let basis = WorthUiAllocationPlanningBasis::from_admitted(
            measurement_basis
                .admit_allocation_constraint_basis(allocation_neighborhood)
                .expect("constraint basis should already admit before plan-lowering denial"),
            None,
        );
        let denial_posture = Some(WorthUiAllocationPlanningDenial::new(
            WorthUiAllocationPlanningDenialReason::PlanLoweringDenied,
            None,
            None,
            None,
            Some(plan_lowering_denial),
            counters,
        ));
        WorthUiAllocationPlanning::new(basis, None, denial_posture, counters)
    }
}
