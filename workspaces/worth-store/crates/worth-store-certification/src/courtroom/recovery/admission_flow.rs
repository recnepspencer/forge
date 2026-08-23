use super::integrity_fixtures::{damaged_integrity_model_input, intact_integrity_model_input};

#[test]
fn intact_integrity_handoff_produces_a_stable_recovery_identity() {
    let first = intact_integrity_model_input("stable-handoff");
    let second = intact_integrity_model_input("stable-handoff");

    assert_eq!(first.payload().identity(), second.payload().identity());
    assert_eq!(first.counters().vetted_record_count(), 5);
    assert_eq!(first.counters().recovery_blocking_count(), 0);
    assert!(first.proves_no_raw_bytes_crossed());
    assert!(!first.claims_recovery());
}

#[test]
fn recovery_blocking_integrity_input_does_not_claim_recovery() {
    let input = damaged_integrity_model_input();

    assert_eq!(input.counters().recovery_blocking_count(), 1);
    assert!(!input.claims_recovery());
    assert!(input.proves_no_raw_bytes_crossed());
}

#[test]
fn corruption_readmission_preserves_the_primary_damage_case() {
    let input = damaged_integrity_model_input();
    let handoff = input
        .payload()
        .corruption_readmission_handoffs()
        .into_iter()
        .next()
        .expect("blocking damage should produce a corruption readmission handoff");

    assert_eq!(
        handoff.primary_damage_case(),
        worth_store_contracts::CorruptionHandoffDamageCase::ChecksumMismatch
    );
    assert_eq!(
        handoff.repair_capability(),
        worth_store_physical_integrity::RecoveryCorruptionRepairCapability::ClassifyGenerationPosture
    );
}
