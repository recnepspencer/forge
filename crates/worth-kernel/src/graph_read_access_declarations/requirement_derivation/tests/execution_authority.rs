use crate::graph_read_access_declarations::current_worth_graph_read_requirement_derivation_closeout;

use super::common::{phase_two_closeout_from_seed, production_seed};

#[test]
fn requirement_derivation_does_not_claim_execution_or_receipts() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 should close without execution authority");

    assert!(!phase_four.claims_graph_read_execution());
    assert!(!phase_four.claims_access_plan_consumption());
    assert!(!phase_four.claims_graph_read_receipts_complete());
    assert_eq!(phase_four.derivation_summary().execution_claim_count(), 0);
    assert_eq!(phase_four.derivation_summary().receipt_claim_count(), 0);
    assert!(!phase_four.phase_five_seed().claims_graph_read_execution());
    assert!(!phase_four
        .phase_five_seed()
        .claims_access_plan_consumption());
    assert!(phase_four.requirement_records().iter().all(|record| !record
        .claims_graph_read_execution()
        && !record.claims_access_plan_consumption()
        && !record.derivation_outcome().claims_graph_read_execution()
        && !record.derivation_outcome().claims_access_plan_consumption()));
}
