use super::*;

pub(super) fn retain_direct_domain_evidence_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let Some(evidence) = settled.execution_receipt().domain_evidence() else {
        return;
    };
    let work_class = execution_work_class(settled.bound_operation().commit_posture());
    rows.extend(
        evidence
            .core()
            .counters()
            .iter()
            .map(|counter| WorthQueryConsumptionCostRow {
                name: format!("domain.operation.{}", counter.schema().name().as_str()),
                work_class,
                observed_count: observed_delta(counter),
            }),
    );
}

pub(super) fn retain_workflow_domain_evidence_rows<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    rows: &mut Vec<WorthQueryConsumptionCostRow>,
) {
    let work_class = execution_work_class(settled.bound_operation().commit_posture());
    for receipt in settled.trace().stage_receipts() {
        let Some(evidence) = receipt.domain_evidence() else {
            continue;
        };
        rows.extend(evidence.core().counters().iter().map(|counter| {
            WorthQueryConsumptionCostRow {
                name: format!(
                    "domain.stage.{}.{}",
                    receipt.stage_identity(),
                    counter.schema().name().as_str()
                ),
                work_class,
                observed_count: observed_delta(counter),
            }
        }));
    }
}

fn observed_delta(
    counter: &crate::domain_installation::WorthQueryAdmittedStructuralCounter,
) -> u64 {
    match counter.schema().monotonicity() {
        worth_query_installation::facade::WorthQueryStructuralCounterMonotonicity::NonDecreasing => {
            counter.observed() - counter.initial()
        }
        worth_query_installation::facade::WorthQueryStructuralCounterMonotonicity::Unconstrained => {
            counter.observed()
        }
    }
}
