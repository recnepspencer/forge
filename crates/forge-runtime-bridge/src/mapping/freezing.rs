use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::fallback::BridgeMappingFallbackClass;
use crate::mapping::lookup::BridgeMappingLookupKey;
use crate::mapping::registration::{BridgeMappingRegistration, MappingSelector};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenBridgeMappingRegistration {
    registration: BridgeMappingRegistration,
}

impl FrozenBridgeMappingRegistration {
    fn new(registration: BridgeMappingRegistration) -> Self {
        Self { registration }
    }

    pub fn mapping_id(&self) -> &crate::mapping::registration::BridgeMappingId {
        self.registration.mapping_id()
    }

    pub fn truth_scope(&self) -> &crate::mapping::registration::TruthPatchScope {
        self.registration.truth_scope()
    }

    pub fn signal_scope(&self) -> &crate::mapping::registration::SignalInvalidationScope {
        self.registration.signal_scope()
    }

    pub fn routing_mode(&self) -> crate::mapping::registration::CoarseRoutingMode {
        self.registration.routing_mode()
    }

    pub fn fallback_class(&self) -> Option<BridgeMappingFallbackClass> {
        self.registration.truth_scope().fallback_class()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenMappingRegistry {
    pub(crate) registrations: Vec<FrozenBridgeMappingRegistration>,
}

impl FrozenMappingRegistry {
    pub(crate) fn freeze(mut registrations: Vec<BridgeMappingRegistration>) -> Result<Self, BridgeBuildError> {
        if registrations.is_empty() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MissingMappingRegistrations,
                "RuntimeBridgeBuilder requires at least one bridge mapping registration before build.",
            ));
        }

        registrations.sort_by(canonical_registration_order);
        validate_registration_values(&registrations)?;
        validate_registration_set(&registrations)?;

        Ok(Self {
            registrations: registrations
                .into_iter()
                .map(FrozenBridgeMappingRegistration::new)
                .collect(),
        })
    }

    pub(crate) fn registrations(&self) -> &[FrozenBridgeMappingRegistration] {
        &self.registrations
    }
}

fn canonical_registration_order(
    left: &BridgeMappingRegistration,
    right: &BridgeMappingRegistration,
) -> std::cmp::Ordering {
    right
        .truth_scope()
        .specificity_rank()
        .cmp(&left.truth_scope().specificity_rank())
        .then_with(|| left.truth_scope().cmp(right.truth_scope()))
        .then_with(|| left.signal_scope().cmp(right.signal_scope()))
        .then_with(|| left.routing_mode().cmp(&right.routing_mode()))
        .then_with(|| left.mapping_id().cmp(right.mapping_id()))
}

fn validate_registration_set(
    registrations: &[BridgeMappingRegistration],
) -> Result<(), BridgeBuildError> {
    for (index, left) in registrations.iter().enumerate() {
        for right in registrations.iter().skip(index + 1) {
            if left.mapping_id() == right.mapping_id() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateMappingRegistration,
                    format!(
                        "Duplicate bridge mapping id `{}` detected across multiple registrations.",
                        left.mapping_id().as_str()
                    ),
                ));
            }

            if !left.truth_scope().overlaps(right.truth_scope()) {
                continue;
            }

            if left.semantic_duplicate_of(right) {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateMappingRegistration,
                    format!(
                        "Duplicate bridge mapping registration detected for truth scope between `{}` and `{}`.",
                        left.mapping_id().as_str(),
                        right.mapping_id().as_str()
                    ),
                ));
            }

            if left.truth_scope() == right.truth_scope() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousMappingRegistration,
                    format!(
                        "Ambiguous bridge mapping registration detected for identical truth scope between `{}` and `{}`.",
                        left.mapping_id().as_str(),
                        right.mapping_id().as_str()
                    ),
                ));
            }

            if left.truth_scope().specificity_rank() == right.truth_scope().specificity_rank() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousMappingRegistration,
                    format!(
                        "Ambiguous bridge mapping registration overlap detected between `{}` and `{}`.",
                        left.mapping_id().as_str(),
                        right.mapping_id().as_str()
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn validate_registration_values(
    registrations: &[BridgeMappingRegistration],
) -> Result<(), BridgeBuildError> {
    for registration in registrations {
        validate_non_empty("mapping id", registration.mapping_id().as_str())?;
        validate_selector("entity selector", registration.truth_scope().entity_selector())?;
        validate_selector("aspect selector", registration.truth_scope().aspect_selector())?;
        validate_selector("surface selector", registration.truth_scope().surface_selector())?;
        validate_non_empty("signal scope", registration.signal_scope().as_str())?;
    }

    Ok(())
}

fn validate_selector(label: &str, selector: &MappingSelector) -> Result<(), BridgeBuildError> {
    match selector {
        MappingSelector::Any => Ok(()),
        MappingSelector::Exact(value) => validate_non_empty(label, value),
    }
}

fn validate_non_empty(label: &str, value: &str) -> Result<(), BridgeBuildError> {
    if value.trim().is_empty() {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::AmbiguousMappingRegistration,
            format!("Bridge mapping {label} must be non-empty."),
        ));
    }

    Ok(())
}

impl crate::mapping::registration::TruthPatchScope {
    pub(crate) fn matches_key(&self, key: BridgeMappingLookupKey<'_>) -> bool {
        self.entity_selector().matches(key.entity_identity())
            && self.aspect_selector().matches(key.aspect_label())
            && self.surface_selector().matches(key.surface_label())
    }
}

#[cfg(test)]
mod tests {
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
}
