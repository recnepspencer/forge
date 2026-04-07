use crate::error::BridgeBuildError;
use crate::mapping::SubscriptionSliceKind;

use super::ids::BridgeAspectRegistrationId;
use super::registration::BridgeAspectRegistration;
use super::types::{SliceFallbackPolicy, TruthDeltaSurfaceKind};
use super::validation::{
    canonical_aspect_registration_order, validate_registration_set, validate_registration_values,
};

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

    pub(crate) fn truth_scope(&self) -> &crate::mapping::TruthPatchScope {
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
