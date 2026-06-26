use super::production_phase_four_closeout;

#[test]
fn phase_five_seed_carries_receipt_or_gap_without_reinterpreting_posture() {
    let closeout = production_phase_four_closeout();
    let seed = closeout.phase_five_seed();

    assert_eq!(
        closeout.closeout_digest(),
        seed.phase_four_closeout_digest()
    );
    assert_eq!(
        closeout.selected_slice().slice_digest(),
        seed.selected_slice().slice_digest()
    );
    assert_eq!(
        closeout.plan_projection().projection_digest(),
        seed.plan_projection().projection_digest()
    );
    assert_eq!(
        closeout.receipt_projection().projection_digest(),
        seed.receipt_projection().projection_digest()
    );
    assert_eq!(
        closeout.cutover_proof().cutover_digest(),
        seed.cutover_proof().cutover_digest()
    );
    assert!(!seed.claims_validator_selection());
    assert!(!seed.claims_access_plan_consumption());
    assert!(!seed.claims_graph_read_execution());
    assert!(!seed.claims_graph_read_receipt());
}
