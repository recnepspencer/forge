use crate::basis_lifecycle::BasisOperationLane;

use super::super::WorthQueryBoundDomainOperation;
use super::denial::{WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial};

pub(super) fn compare_portable_operation<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<
    worth_query_installation::facade::WorthQueryPortableOperationComparisonEquivalent,
    WorthQueryCompatibilityDenial,
> {
    use worth_query_installation::facade::WorthQueryPortableOperationComparisonOutcome;

    match worth_query_installation::facade::compare_portable_domain_operations(
        subject.definition(),
        candidate.definition(),
    ) {
        WorthQueryPortableOperationComparisonOutcome::Equivalent(evidence) => {
            record_work(evidence.work(), counters);
            Ok(evidence)
        }
        WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) => {
            record_work(mismatch.work(), counters);
            Err(WorthQueryCompatibilityDenial::portable_operation_mismatch(
                mismatch, *counters,
            ))
        }
        WorthQueryPortableOperationComparisonOutcome::Unsupported(unsupported) => {
            record_work(unsupported.work(), counters);
            Err(
                WorthQueryCompatibilityDenial::portable_operation_unsupported(
                    unsupported,
                    *counters,
                ),
            )
        }
    }
}

fn record_work(
    work: worth_query_installation::facade::WorthQueryPortableOperationComparisonWork,
    counters: &mut WorthQueryCompatibilityCounters,
) {
    counters.portable_contract_comparisons += work.owner_dimensions_inspected() as usize;
    counters.portable_variable_items_submitted += work.variable_items_submitted() as usize;
    counters.canonical_comparisons += (work.direct_foundational_comparison_requests()
        + work.canonical_export_comparison_requests()
        + work.delegated_conditional_foundational_comparison_requests())
        as usize;
    counters.portable_conditional_nodes_submitted += (work.subject_conditional_nodes_submitted()
        + work.candidate_conditional_nodes_submitted())
        as usize;
}
