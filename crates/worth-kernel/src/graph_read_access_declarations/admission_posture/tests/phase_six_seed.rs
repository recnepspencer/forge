use super::common::{
    production_admission_posture_closeout, reversed_admission_posture_closeout_pair,
};

#[test]
fn phase_six_seed_preserves_gap_and_posture_visibility() {
    let closeout = production_admission_posture_closeout();
    let seed = closeout.phase_six_seed();

    assert_eq!(seed.posture_records(), closeout.posture_records());
    assert_eq!(
        seed.admission_capability_gaps(),
        closeout.admission_capability_gaps()
    );
    assert_eq!(
        seed.carried_requirement_derivation_gaps(),
        closeout.carried_requirement_derivation_gaps()
    );
    assert_eq!(seed.gap_cap_report(), closeout.gap_cap_report());
    assert!(!seed.deletion_items().is_empty());
    assert_eq!(seed.admission_closeout_digest(), closeout.closeout_digest());
    assert!(seed
        .posture_records()
        .iter()
        .all(|record| record.posture_outcome().admission_gap().is_some()));
}

#[test]
fn phase_six_seed_identity_is_stable_under_record_ordering() {
    let (forward, reversed) = reversed_admission_posture_closeout_pair();

    assert_eq!(forward.closeout_digest(), reversed.closeout_digest());
    assert_eq!(
        forward.phase_six_seed().admission_closeout_digest(),
        reversed.phase_six_seed().admission_closeout_digest()
    );
}
