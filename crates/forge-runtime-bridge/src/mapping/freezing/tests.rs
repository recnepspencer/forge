    use super::FrozenMappingRegistry;
    use crate::error::BridgeBuildErrorKind;
    use crate::mapping::lookup::BridgeMappingLookupKey;
    use crate::mapping::{
        BridgeMappingFallbackClass, BridgeMappingId, BridgeMappingRegistration,
        CoarseRoutingMode, MappingSelector, SignalInvalidationScope, TruthPatchScope,
    };

    fn registration(
        mapping_id: &str,
        entity: MappingSelector,
        aspect: MappingSelector,
        surface: MappingSelector,
        signal_scope: &str,
    ) -> BridgeMappingRegistration {
        BridgeMappingRegistration::new(
            BridgeMappingId::new(mapping_id),
            TruthPatchScope::new(entity, aspect, surface),
            SignalInvalidationScope::new(signal_scope),
            CoarseRoutingMode::Direct,
        )
    }

    #[test]
    fn freeze_rejects_missing_registrations() {
        let error = FrozenMappingRegistry::freeze(vec![])
            .expect_err("registry freeze should fail without registrations");

        assert_eq!(
            error.kind(),
            BridgeBuildErrorKind::MissingMappingRegistrations
        );
    }

    #[test]
    fn freeze_rejects_duplicate_semantic_registrations() {
        let error = FrozenMappingRegistry::freeze(
            vec![
                registration(
                    "alpha",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.user.profile",
                ),
                registration(
                    "beta",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.user.profile",
                ),
            ],
        )
        .expect_err("duplicate semantic registrations must fail");

        assert_eq!(
            error.kind(),
            BridgeBuildErrorKind::DuplicateMappingRegistration
        );
    }

    #[test]
    fn freeze_rejects_duplicate_mapping_ids_even_when_scopes_differ() {
        let error = FrozenMappingRegistry::freeze(
            vec![
                registration(
                    "shared-id",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.user.profile.name",
                ),
                registration(
                    "shared-id",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("avatar"),
                    "signal.user.profile.avatar",
                ),
            ],
        )
        .expect_err("duplicate mapping ids must fail even when scopes differ");

        assert_eq!(
            error.kind(),
            BridgeBuildErrorKind::DuplicateMappingRegistration
        );
        assert!(error.to_string().contains("shared-id"));
    }

    #[test]
    fn freeze_rejects_same_rank_overlap() {
        let error = FrozenMappingRegistry::freeze(
            vec![
                registration(
                    "entity-wide",
                    MappingSelector::exact("user"),
                    MappingSelector::any(),
                    MappingSelector::exact("name"),
                    "signal.entity-wide",
                ),
                registration(
                    "aspect-wide",
                    MappingSelector::any(),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.aspect-wide",
                ),
            ],
        )
        .expect_err("same-rank overlapping registrations must fail");

        assert_eq!(
            error.kind(),
            BridgeBuildErrorKind::AmbiguousMappingRegistration
        );
    }

    #[test]
    fn freeze_canonicalizes_iteration_order() {
        let registry = FrozenMappingRegistry::freeze(
            vec![
                registration(
                    "fallback",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    "signal.surface-fallback",
                ),
                registration(
                    "exact",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.exact",
                ),
                registration(
                    "broad",
                    MappingSelector::any(),
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    "signal.broad",
                ),
            ],
        )
        .expect("registry freeze should succeed");

        let ordered_ids: Vec<_> = registry
            .registrations()
            .iter()
            .map(|registration| registration.mapping_id().as_str())
            .collect();

        assert_eq!(ordered_ids, vec!["exact", "fallback", "broad"]);
    }

    #[test]
    fn lookup_prefers_more_specific_match_before_fallback() {
        let registry = FrozenMappingRegistry::freeze(
            vec![
                registration(
                    "fallback",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    "signal.surface-fallback",
                ),
                registration(
                    "exact",
                    MappingSelector::exact("user"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                    "signal.exact",
                ),
            ],
        )
        .expect("registry freeze should succeed");

        match registry.lookup(BridgeMappingLookupKey::new("user", "profile", "name")) {
            crate::mapping::BridgeMappingLookup::Exact { resolved } => {
                assert_eq!(resolved.registration().mapping_id().as_str(), "exact");
            }
            other => panic!("expected exact match, found {other:?}"),
        }

        match registry.lookup(BridgeMappingLookupKey::new("user", "profile", "avatar")) {
            crate::mapping::BridgeMappingLookup::Fallback { resolved } => {
                assert_eq!(resolved.registration().mapping_id().as_str(), "fallback");
                assert_eq!(
                    resolved.registration().fallback_class(),
                    Some(BridgeMappingFallbackClass::Surface)
                );
            }
            other => panic!("expected fallback match, found {other:?}"),
        }
    }

    #[test]
    fn freeze_rejects_empty_mapping_values() {
        let error = FrozenMappingRegistry::freeze(vec![registration(
            " ",
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
            "signal.valid",
        )])
        .expect_err("empty mapping identifiers must fail");

        assert_eq!(error.kind(), BridgeBuildErrorKind::AmbiguousMappingRegistration);
        assert!(error.to_string().contains("mapping id"));
    }
