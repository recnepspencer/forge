use crate::mapping::{
    BridgeAspectRegistrationId, FrozenAspectMappingRegistry, SliceFallbackPolicy,
    SubscriptionSliceKind, TruthDeltaSurfaceKind,
};

use super::surfaces::TruthDeltaSurface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FineGrainedMatchStatus {
    Matched,
    FallbackAdmitted,
    SuppressedByRegistrationPolicy,
    UnsupportedSurfaceCategory,
    AmbiguousRegistration,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FineGrainedMatchOutcome {
    Matched {
        truth_surface_kind: TruthDeltaSurfaceKind,
        aspect_registration_id: BridgeAspectRegistrationId,
        subscription_slice_kind: SubscriptionSliceKind,
    },
    FallbackAdmitted {
        truth_surface_kind: TruthDeltaSurfaceKind,
        aspect_registration_id: BridgeAspectRegistrationId,
        subscription_slice_kind: SubscriptionSliceKind,
        fallback_policy: SliceFallbackPolicy,
    },
    SuppressedByRegistrationPolicy {
        truth_surface_kind: TruthDeltaSurfaceKind,
    },
    UnsupportedSurfaceCategory {
        truth_surface_kind: TruthDeltaSurfaceKind,
    },
    AmbiguousRegistration {
        truth_surface_kind: TruthDeltaSurfaceKind,
    },
}

impl FineGrainedMatchOutcome {
    pub fn status(&self) -> FineGrainedMatchStatus {
        match self {
            Self::Matched { .. } => FineGrainedMatchStatus::Matched,
            Self::FallbackAdmitted { .. } => FineGrainedMatchStatus::FallbackAdmitted,
            Self::SuppressedByRegistrationPolicy { .. } => {
                FineGrainedMatchStatus::SuppressedByRegistrationPolicy
            }
            Self::UnsupportedSurfaceCategory { .. } => {
                FineGrainedMatchStatus::UnsupportedSurfaceCategory
            }
            Self::AmbiguousRegistration { .. } => FineGrainedMatchStatus::AmbiguousRegistration,
        }
    }

    pub fn subscription_slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        match self {
            Self::Matched {
                subscription_slice_kind,
                ..
            }
            | Self::FallbackAdmitted {
                subscription_slice_kind,
                ..
            } => Some(subscription_slice_kind),
            Self::SuppressedByRegistrationPolicy { .. }
            | Self::UnsupportedSurfaceCategory { .. }
            | Self::AmbiguousRegistration { .. } => None,
        }
    }
}

pub(crate) type FineGrainedSurfaceMatch = FineGrainedMatchOutcome;

pub(crate) fn classify_truth_delta_surface(
    surface: &TruthDeltaSurface,
    aspect_registry: &FrozenAspectMappingRegistry,
) -> FineGrainedSurfaceMatch {
    let Some(registration) = aspect_registry.lookup(
        surface.entity_identity(),
        surface.aspect_label(),
        surface.surface_label(),
        surface.surface_kind(),
    ) else {
        return FineGrainedSurfaceMatch::SuppressedByRegistrationPolicy {
            truth_surface_kind: surface.surface_kind(),
        };
    };

    match registration.fallback_policy() {
        SliceFallbackPolicy::Disallow => FineGrainedSurfaceMatch::Matched {
            truth_surface_kind: surface.surface_kind(),
            aspect_registration_id: registration.registration_id().clone(),
            subscription_slice_kind: registration.subscription_slice_kind().clone(),
        },
        fallback_policy => FineGrainedSurfaceMatch::FallbackAdmitted {
            truth_surface_kind: surface.surface_kind(),
            aspect_registration_id: registration.registration_id().clone(),
            subscription_slice_kind: registration.subscription_slice_kind().clone(),
            fallback_policy,
        },
    }
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::AspectKey;

    use crate::mapping::{
        BridgeAspectRegistration, BridgeAspectRegistrationId, FrozenAspectMappingRegistry,
        MappingSelector, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
        TruthPatchScope,
    };

    use super::{classify_truth_delta_surface, FineGrainedMatchStatus};
    use crate::routing::surfaces::TruthDeltaSurface;

    fn registry(registrations: Vec<BridgeAspectRegistration>) -> FrozenAspectMappingRegistry {
        FrozenAspectMappingRegistry::freeze(registrations).expect("aspect registry should freeze")
    }

    #[test]
    fn classify_surface_as_suppressed_when_no_registration_matches() {
        let surface = TruthDeltaSurface::new(
            "user",
            aspect_key("profile"),
            "name",
            TruthDeltaSurfaceKind::EntityField,
        );

        let classification =
            classify_truth_delta_surface(&surface, &FrozenAspectMappingRegistry::default());

        assert_eq!(
            classification.status(),
            FineGrainedMatchStatus::SuppressedByRegistrationPolicy
        );
        assert!(classification.subscription_slice_kind().is_none());
    }

    #[test]
    fn classify_surface_as_matched_when_direct_registration_exists() {
        let surface = TruthDeltaSurface::new(
            "user",
            aspect_key("profile"),
            "name",
            TruthDeltaSurfaceKind::EntityField,
        );
        let registry = registry(vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("field"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        )]);

        let classification = classify_truth_delta_surface(&surface, &registry);

        assert_eq!(classification.status(), FineGrainedMatchStatus::Matched);
        assert_eq!(
            classification.subscription_slice_kind(),
            Some(&SubscriptionSliceKind::SignalField)
        );
    }

    #[test]
    fn classify_surface_as_fallback_when_registration_admits_widening() {
        let surface = TruthDeltaSurface::new(
            "user",
            aspect_key("profile"),
            "name",
            TruthDeltaSurfaceKind::EntityField,
        );
        let registry = registry(vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("field-fallback"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::RegisteredCoarseFallback,
            SliceFallbackPolicy::RegisteredEntityCoarseFallback,
        )]);

        let classification = classify_truth_delta_surface(&surface, &registry);

        assert_eq!(
            classification.status(),
            FineGrainedMatchStatus::FallbackAdmitted
        );
        assert_eq!(
            classification.subscription_slice_kind(),
            Some(&SubscriptionSliceKind::RegisteredCoarseFallback)
        );
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid truth delta surface aspect key")
    }
}
