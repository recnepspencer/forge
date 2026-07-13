use forge_store_physical_certification::{
    CounterContractKind, CounterExpectationKind, PhysicalCounterEvidenceReceipt,
};

use forge_store_test_support::harness::recovery::counter_evidence as support;

use support::{counter_receipt, lower_physical_isolation_plan, observed_trace};

#[test]
fn repeated_executed_counter_admission_converges_to_same_rows() {
    let plan = lower_physical_isolation_plan();
    let first = counter_receipt(&plan, observed_trace(&plan));
    let second = counter_receipt(&plan, observed_trace(&plan));

    assert_eq!(physical_rows(&first), physical_rows(&second));
    assert_eq!(foundational_rows(&first), foundational_rows(&second));
    assert_eq!(
        first.foundational_receipt().bundle().counter_specs().len(),
        first.foundational_receipt().counter_rows().len()
    );
}

fn physical_rows(
    receipt: &PhysicalCounterEvidenceReceipt,
) -> Vec<(CounterContractKind, CounterExpectationKind, u64)> {
    receipt
        .rows()
        .iter()
        .map(|row| (row.kind(), row.strength(), row.observed_count()))
        .collect()
}

fn foundational_rows(receipt: &PhysicalCounterEvidenceReceipt) -> Vec<(String, u64)> {
    receipt
        .foundational_receipt()
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str().to_owned(), row.observed_count()))
        .collect()
}
