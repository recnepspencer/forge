use super::production_phase_three_closeout;

#[test]
fn phase_four_seed_carries_resolved_postures_and_caps_without_execution() {
    let phase_three = production_phase_three_closeout();
    let seed = phase_three.phase_four_seed();

    assert_eq!(
        phase_three.closeout_digest(),
        seed.phase_three_closeout_digest_seed()
    );
    assert_eq!(
        phase_three.posture_map().map_digest(),
        seed.posture_map_digest()
    );
    assert_eq!(
        phase_three.cap_report().report_digest(),
        seed.cap_report_digest()
    );
    assert_eq!(
        phase_three.posture_map().resolved_postures().len(),
        seed.resolved_posture_count()
    );
    assert_eq!(
        phase_three.posture_map().resolved_postures(),
        seed.resolved_postures()
    );
    assert_eq!(
        phase_three.cap_report().ledger().rows().len(),
        seed.cap_family_count()
    );
    assert_eq!(phase_three.cap_report().ledger().rows(), seed.cap_rows());
    assert!(!seed.claims_access_plan_consumption());
    assert!(!seed.claims_graph_read_execution());
    assert!(!seed.claims_graph_read_receipt());
}
