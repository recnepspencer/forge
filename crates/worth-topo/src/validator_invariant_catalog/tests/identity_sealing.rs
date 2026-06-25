use super::production_phase_two_closeout;

#[test]
fn family_identity_is_not_the_old_rule_display_name() {
    let closeout = production_phase_two_closeout();
    let ownership = closeout
        .catalog()
        .records()
        .iter()
        .find(|record| record.identity().name() == "ownership")
        .expect("ownership family should exist");
    let identity = ownership.identity();

    assert_ne!(identity.identity_digest(), "ownership");
    assert_ne!(identity.stable_key(), "ownership");
    assert!(identity.stable_key().contains("validator:"));
    assert!(identity
        .identity_digest()
        .contains("worth-topo-legality-family-identity"));
}
