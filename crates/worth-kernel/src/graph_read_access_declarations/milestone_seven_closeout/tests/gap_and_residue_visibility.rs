use super::common::production_closeout;

#[test]
fn closeout_preserves_gap_visibility() {
    let closeout = production_closeout();
    let seed = closeout.milestone_eight_seed();

    assert_eq!(
        seed.admission_capability_gaps(),
        closeout.admission_capability_gaps()
    );
    assert_eq!(
        seed.carried_requirement_derivation_gaps(),
        closeout.carried_requirement_derivation_gaps()
    );
    assert_eq!(
        seed.deletion_ledger_report(),
        closeout.deletion_ledger_report()
    );
    assert_eq!(
        seed.capped_residue_report(),
        closeout.capped_residue_report()
    );
    assert_eq!(
        seed.source_firewall_report(),
        closeout.source_firewall_report()
    );
    assert!(!seed.admission_capability_gaps().is_empty());
}

#[test]
fn hard_deleted_residue_stays_zero_in_milestone_eight_seed() {
    let closeout = production_closeout();
    let seed = closeout.milestone_eight_seed();

    assert_eq!(seed.capped_residue_report().residue_count(), 0);
    assert_eq!(
        seed.capped_residue_report(),
        closeout.capped_residue_report()
    );
}
