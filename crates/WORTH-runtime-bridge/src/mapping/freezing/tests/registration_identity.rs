use super::*;

#[test]
fn freeze_derives_digest_shaped_registration_identities_from_native_registration_truth() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "exact",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.exact",
        ),
        registration(
            "avatar",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("avatar"),
            "signal.avatar",
        ),
    ])
    .expect("registry freeze should derive registration proof identities");

    let identities = registry
        .registrations()
        .iter()
        .map(|registration| registration.registration_identity().as_str())
        .collect::<Vec<_>>();

    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0], identities[1]);
    assert!(identities
        .iter()
        .all(|identity| identity.starts_with("frozen-mapping-registration:sha256:")));
    assert!(identities.iter().all(|identity| !identity.contains("exact")
        && !identity.contains("avatar")
        && !identity.contains("user")
        && !identity.contains("profile")
        && !identity.contains("signal.")));
}
