use super::*;

#[test]
fn lookup_prefers_more_specific_match_before_widening() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "widening",
            MappingSelector::exact("user"),
            aspect("profile"),
            any_target(),
            "signal.surface-widening",
        ),
        registration(
            "exact",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.exact",
        ),
    ])
    .expect("registry freeze should succeed");

    let profile = AspectKey::new("profile").expect("valid native aspect key");
    match registry.lookup(lookup_key("user", &profile, &field("name"))) {
        crate::mapping::BridgeMappingLookup::Exact { resolved } => {
            let matched_ids = resolved
                .registrations()
                .map(|registration| registration.mapping_id().as_str())
                .collect::<Vec<_>>();
            assert_eq!(matched_ids, vec!["exact"]);
        }
        other => panic!("expected exact match, found {other:?}"),
    }

    match registry.lookup(lookup_key("user", &profile, &field("avatar"))) {
        crate::mapping::BridgeMappingLookup::Widening { resolved } => {
            let matched = resolved.registrations().collect::<Vec<_>>();
            assert_eq!(matched.len(), 1);
            assert_eq!(matched[0].mapping_id().as_str(), "widening");
            assert_eq!(
                matched[0].widening_class(),
                Some(BridgeMappingWideningClass::Surface)
            );
        }
        other => panic!("expected widening match, found {other:?}"),
    }
}

#[test]
fn lookup_returns_all_fanout_matches_at_the_most_specific_scope() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "widening",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            any_target(),
            "price:widening",
        ),
        registration(
            "bicycle",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:bicycle",
        ),
        registration(
            "wheelbarrow",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:wheelbarrow",
        ),
    ])
    .expect("fanout and specificity should coexist");

    let cost = AspectKey::new("cost").expect("valid native aspect key");
    match registry.lookup(lookup_key("component:steel", &cost, &field("usd"))) {
        crate::mapping::BridgeMappingLookup::Exact { resolved } => {
            let matched_ids = resolved
                .registrations()
                .map(|registration| registration.mapping_id().as_str())
                .collect::<Vec<_>>();
            assert_eq!(matched_ids, vec!["bicycle", "wheelbarrow"]);
        }
        other => panic!("expected exact fanout match, found {other:?}"),
    }
}

#[test]
fn freeze_rejects_empty_mapping_values() {
    let error = FrozenMappingRegistry::freeze(vec![registration(
        " ",
        MappingSelector::exact("user"),
        aspect("profile"),
        field("name"),
        "signal.valid",
    )])
    .expect_err("empty mapping identifiers must fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::AmbiguousMappingRegistration
    );
    let context = freeze_context(&error);
    assert_eq!(
        context
            .mapping_id()
            .expect("invalid mapping id should be retained")
            .as_str(),
        " "
    );
    assert_eq!(context.invalid_field(), Some("mapping id"));
}
