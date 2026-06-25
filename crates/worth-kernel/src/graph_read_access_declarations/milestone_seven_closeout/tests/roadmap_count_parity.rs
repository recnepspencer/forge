use super::common::{production_closeout, production_phase_seven_seed};

#[test]
fn closeout_counters_match_phase_seven_source_obligations() {
    let seed = production_phase_seven_seed();
    let closeout = production_closeout();
    let counters = closeout.closeout_counters();

    assert_eq!(
        counters.declaration_catalog_record_count(),
        seed.posture_records().len()
    );
    assert_eq!(
        counters.read_family_identity_count(),
        seed.posture_records().len()
    );
    assert_eq!(
        counters.requirement_evidence_row_count(),
        seed.posture_records().len()
    );
    assert_eq!(
        counters.admission_capability_gap_count(),
        seed.admission_capability_gaps().len()
    );
    assert_eq!(
        counters.carried_requirement_derivation_gap_count(),
        seed.carried_requirement_derivation_gaps().len()
    );
    assert_eq!(
        counters.deletion_ledger_row_count(),
        seed.deletion_ledger_report().rows().len()
    );
    assert_eq!(
        counters.capped_residue_row_count(),
        seed.capped_residue_report().rows().len()
    );
    assert_eq!(
        counters.source_firewall_scanned_region_count(),
        seed.source_firewall_report().region_reports().len()
    );
}
