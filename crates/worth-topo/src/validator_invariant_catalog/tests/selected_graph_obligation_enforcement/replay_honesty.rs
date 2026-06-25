use super::super::relational_invariant_catalog::execution_inputs::relational_invariant_query_execution_input;
use crate::validator_invariant_catalog::WorthTopologySelectedGraphObligationEnforcementCloseout;

#[test]
fn same_query_authority_replays_to_same_phase_six_closeout_digest() {
    let (left_relational_closeout, left_execution_input) =
        relational_invariant_query_execution_input();
    let left =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &left_relational_closeout,
            left_execution_input,
        )
        .expect("left replay should close");

    let (right_relational_closeout, right_execution_input) =
        relational_invariant_query_execution_input();
    let right =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &right_relational_closeout,
            right_execution_input,
        )
        .expect("right replay should close");

    assert_eq!(left.closeout_digest(), right.closeout_digest());
    assert_eq!(
        left.phase_seven_seed().seed_digest(),
        right.phase_seven_seed().seed_digest()
    );
    assert_eq!(
        left.enforcement_receipts()
            .iter()
            .map(|receipt| receipt.enforcement_receipt_digest())
            .collect::<Vec<_>>(),
        right
            .enforcement_receipts()
            .iter()
            .map(|receipt| receipt.enforcement_receipt_digest())
            .collect::<Vec<_>>()
    );
}
