use crate::mapping::{SubscriptionSliceKind, TruthPatchScope};

use super::ids::BridgeAspectRegistrationId;
use super::types::{SliceFallbackPolicy, TruthDeltaSurfaceKind};

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

    pub(super) fn semantic_duplicate_of(&self, other: &Self) -> bool {
        self.truth_scope == other.truth_scope
            && self.truth_surface_kind == other.truth_surface_kind
            && self.subscription_slice_kind == other.subscription_slice_kind
            && self.fallback_policy == other.fallback_policy
    }
}
