use super::*;

pub(super) fn retain_direct_resource_admission_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    retain_resource_admission_rows(settled.resources().counters(), rows);
}

pub(super) fn retain_workflow_resource_admission_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    retain_resource_admission_rows(settled.trace().resources().counters(), rows);
}

fn retain_resource_admission_rows(
    counters: crate::domain_installation::WorthQueryExecutionResourceAdmissionCounters,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    retain_rows!(
        rows,
        "query.resource_admission",
        FoundationalPerformanceWorkClass::ValidationPlanning,
        counters,
        [
            runtime_authority_checks,
            input_contract_checks,
            execution_contract_checks,
            resource_contract_lookups,
            support_snapshot_checks,
            strategy_checks,
            envelope_dimension_checks,
            provider_session_mints,
        ]
    );
}
