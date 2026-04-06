use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::identity::{AspectRegistrationIdTag, BridgeIdentity};
use crate::mapping::{MappingSelector, SubscriptionSliceKind, TruthPatchScope};

pub type BridgeAspectRegistrationId = BridgeIdentity<AspectRegistrationIdTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthDeltaSurfaceKind {
    EntityField,
    EntityRelationEndpoint,
    EntityRegion,
    EntityPartition,
    EntityFacet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SliceFallbackPolicy {
    Disallow,
    RegisteredEntityCoarseFallback,
    RegisteredPartitionFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAspectRegistration {
    registration_id: BridgeAspectRegistrationId,
    truth_scope: TruthPatchScope,
    truth_surface_kind: TruthDeltaSurfaceKind,
    subscription_slice_kind: SubscriptionSliceKind,
    fallback_policy: SliceFallbackPolicy,
}

impl BridgeAspectRegistration {
    pub fn new(
        registration_id: BridgeAspectRegistrationId,
        truth_scope: TruthPatchScope,
        truth_surface_kind: TruthDeltaSurfaceKind,
        subscription_slice_kind: SubscriptionSliceKind,
        fallback_policy: SliceFallbackPolicy,
    ) -> Self {
        Self {
            registration_id,
            truth_scope,
            truth_surface_kind,
            subscription_slice_kind,
            fallback_policy,
        }
    }

    pub fn registration_id(&self) -> &BridgeAspectRegistrationId {
        &self.registration_id
    }

    pub fn truth_scope(&self) -> &TruthPatchScope {
        &self.truth_scope
    }

    pub fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.truth_surface_kind
    }

    pub fn subscription_slice_kind(&self) -> &SubscriptionSliceKind {
        &self.subscription_slice_kind
    }

    pub fn fallback_policy(&self) -> SliceFallbackPolicy {
        self.fallback_policy
    }

    fn semantic_duplicate_of(&self, other: &Self) -> bool {
        self.truth_scope == other.truth_scope
            && self.truth_surface_kind == other.truth_surface_kind
            && self.subscription_slice_kind == other.subscription_slice_kind
            && self.fallback_policy == other.fallback_policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenAspectRegistration {
    registration: BridgeAspectRegistration,
}

impl FrozenAspectRegistration {
    fn new(registration: BridgeAspectRegistration) -> Self {
        Self { registration }
    }

    pub(crate) fn registration_id(&self) -> &BridgeAspectRegistrationId {
        self.registration.registration_id()
    }

    pub(crate) fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.registration.truth_surface_kind()
    }

    pub(crate) fn truth_scope(&self) -> &TruthPatchScope {
        self.registration.truth_scope()
    }

    pub(crate) fn subscription_slice_kind(&self) -> &SubscriptionSliceKind {
        self.registration.subscription_slice_kind()
    }

    pub(crate) fn fallback_policy(&self) -> SliceFallbackPolicy {
        self.registration.fallback_policy()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrozenAspectMappingRegistry {
    registrations: Vec<FrozenAspectRegistration>,
}

impl FrozenAspectMappingRegistry {
    pub(crate) fn freeze(
        mut registrations: Vec<BridgeAspectRegistration>,
    ) -> Result<Self, BridgeBuildError> {
        registrations.sort_by(canonical_aspect_registration_order);
        validate_registration_values(&registrations)?;
        validate_registration_set(&registrations)?;

        Ok(Self {
            registrations: registrations
                .into_iter()
                .map(FrozenAspectRegistration::new)
                .collect(),
        })
    }

    pub(crate) fn registrations(&self) -> &[FrozenAspectRegistration] {
        &self.registrations
    }
}

fn canonical_aspect_registration_order(
    left: &BridgeAspectRegistration,
    right: &BridgeAspectRegistration,
) -> std::cmp::Ordering {
    right
        .truth_scope()
        .specificity_rank()
        .cmp(&left.truth_scope().specificity_rank())
        .then_with(|| left.truth_surface_kind().cmp(&right.truth_surface_kind()))
        .then_with(|| left.truth_scope().cmp(right.truth_scope()))
        .then_with(|| left.subscription_slice_kind().cmp(right.subscription_slice_kind()))
        .then_with(|| left.fallback_policy().cmp(&right.fallback_policy()))
        .then_with(|| left.registration_id().cmp(right.registration_id()))
}

fn validate_registration_values(
    registrations: &[BridgeAspectRegistration],
) -> Result<(), BridgeBuildError> {
    for registration in registrations {
        validate_non_empty("aspect registration id", registration.registration_id().as_str())?;
        validate_selector(
            "aspect registration entity selector",
            registration.truth_scope().entity_selector(),
        )?;
        validate_selector(
            "aspect registration aspect selector",
            registration.truth_scope().aspect_selector(),
        )?;
        validate_selector(
            "aspect registration surface selector",
            registration.truth_scope().surface_selector(),
        )?;

        match registration.fallback_policy() {
            SliceFallbackPolicy::Disallow => {}
            SliceFallbackPolicy::RegisteredEntityCoarseFallback => {
                if *registration.subscription_slice_kind()
                    != SubscriptionSliceKind::RegisteredCoarseFallback
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy,
                        format!(
                            "Aspect registration `{}` uses entity coarse fallback without targeting the registered coarse fallback slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
            SliceFallbackPolicy::RegisteredPartitionFallback => {
                if *registration.subscription_slice_kind() != SubscriptionSliceKind::SignalPartition
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy,
                        format!(
                            "Aspect registration `{}` uses partition fallback without targeting the signal partition slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_registration_set(
    registrations: &[BridgeAspectRegistration],
) -> Result<(), BridgeBuildError> {
    for (index, left) in registrations.iter().enumerate() {
        for right in registrations.iter().skip(index + 1) {
            if left.registration_id() == right.registration_id() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateAspectRegistration,
                    format!(
                        "Duplicate bridge aspect registration id `{}` detected across multiple registrations.",
                        left.registration_id().as_str()
                    ),
                ));
            }

            if left.semantic_duplicate_of(right) {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateAspectRegistration,
                    format!(
                        "Duplicate bridge aspect registration detected between `{}` and `{}`.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_surface_kind() != right.truth_surface_kind() {
                continue;
            }

            if !left.truth_scope().overlaps(right.truth_scope()) {
                continue;
            }

            if left.truth_scope() == right.truth_scope()
                && left.subscription_slice_kind() == right.subscription_slice_kind()
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousAspectRegistration,
                    format!(
                        "Ambiguous bridge aspect registration detected for identical fine-grained truth scope between `{}` and `{}`.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_scope().specificity_rank() == right.truth_scope().specificity_rank() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousAspectRegistration,
                    format!(
                        "Ambiguous bridge aspect registration overlap detected between `{}` and `{}`.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }
        }
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
            BridgeBuildErrorKind::AmbiguousAspectRegistration,
            format!("Bridge aspect mapping {label} must be non-empty."),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeAspectRegistration, BridgeAspectRegistrationId, FrozenAspectMappingRegistry,
        SliceFallbackPolicy, TruthDeltaSurfaceKind,
    };
    use crate::error::BridgeBuildErrorKind;
    use crate::mapping::{
        MappingSelector, SubscriptionSliceKind, TruthPatchScope,
    };

    fn registration(
        id: &str,
        entity: MappingSelector,
        aspect: MappingSelector,
        surface: MappingSelector,
        truth_surface_kind: TruthDeltaSurfaceKind,
        subscription_slice_kind: SubscriptionSliceKind,
        fallback_policy: SliceFallbackPolicy,
    ) -> BridgeAspectRegistration {
        BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new(id),
            TruthPatchScope::new(entity, aspect, surface),
            truth_surface_kind,
            subscription_slice_kind,
            fallback_policy,
        )
    }

    #[test]
    fn freeze_accepts_empty_aspect_registry_for_incremental_rollout() {
        let registry = FrozenAspectMappingRegistry::freeze(vec![])
            .expect("empty fine-grained registry should be allowed before routing consumes it");

        assert!(registry.registrations().is_empty());
    }

    #[test]
    fn freeze_rejects_duplicate_registration_ids() {
        let error = FrozenAspectMappingRegistry::freeze(vec![
            registration(
                "shared",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
            registration(
                "shared",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("avatar"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
        ])
        .expect_err("duplicate aspect registration ids must fail");

        assert_eq!(error.kind(), BridgeBuildErrorKind::DuplicateAspectRegistration);
    }

    #[test]
    fn freeze_rejects_same_rank_overlap_for_same_surface_kind() {
        let error = FrozenAspectMappingRegistry::freeze(vec![
            registration(
                "field-by-entity",
                MappingSelector::exact("user"),
                MappingSelector::any(),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
            registration(
                "field-by-aspect",
                MappingSelector::any(),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
        ])
        .expect_err("same-rank aspect overlap must fail");

        assert_eq!(error.kind(), BridgeBuildErrorKind::AmbiguousAspectRegistration);
    }

    #[test]
    fn freeze_allows_same_scope_for_different_surface_kinds() {
        let registry = FrozenAspectMappingRegistry::freeze(vec![
            registration(
                "field",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
            registration(
                "facet",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityFacet,
                SubscriptionSliceKind::SignalFacet,
                SliceFallbackPolicy::Disallow,
            ),
        ])
        .expect("different surface kinds should coexist");

        assert_eq!(registry.registrations().len(), 2);
    }

    #[test]
    fn freeze_rejects_invalid_entity_fallback_target() {
        let error = FrozenAspectMappingRegistry::freeze(vec![registration(
            "invalid-fallback",
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::RegisteredEntityCoarseFallback,
        )])
        .expect_err("entity coarse fallback must target the coarse fallback slice kind");

        assert_eq!(
            error.kind(),
            BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy
        );
    }

    #[test]
    fn freeze_canonicalizes_aspect_registration_order() {
        let registry = FrozenAspectMappingRegistry::freeze(vec![
            registration(
                "fallback",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::any(),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::RegisteredCoarseFallback,
                SliceFallbackPolicy::RegisteredEntityCoarseFallback,
            ),
            registration(
                "exact",
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
            registration(
                "broad",
                MappingSelector::any(),
                MappingSelector::exact("profile"),
                MappingSelector::any(),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            ),
        ])
        .expect("aspect registration freeze should succeed");

        let ordered_ids: Vec<_> = registry
            .registrations()
            .iter()
            .map(|registration| registration.registration.registration_id().as_str())
            .collect();

        assert_eq!(ordered_ids, vec!["exact", "fallback", "broad"]);
    }
}
