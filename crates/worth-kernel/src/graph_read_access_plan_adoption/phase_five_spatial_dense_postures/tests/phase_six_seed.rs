use super::production_phase_five_closeout;

#[test]
fn phase_six_seed_preserves_required_and_denied_postures() {
    let closeout = production_phase_five_closeout();
    let seed = closeout.phase_six_seed();

    assert_eq!(seed.posture_projections(), closeout.posture_projections());
    assert_eq!(
        seed.grouped_admission_report().report_digest(),
        closeout.grouped_admission_report().report_digest()
    );
    assert_eq!(
        seed.bounded_execution_contract().contract_digest(),
        closeout.bounded_execution_contract().contract_digest()
    );
    assert_eq!(
        seed.source_firewall_report().report_digest(),
        closeout.source_firewall_report().report_digest()
    );
    assert!(!seed.cap_rows().is_empty());
    assert_eq!(
        seed.posture_projections()
            .iter()
            .filter(|projection| projection.claims_graph_read_receipt())
            .count(),
        closeout.counters().receipt_claim_count()
    );
    assert!(!seed.claims_validator_selection());
}
