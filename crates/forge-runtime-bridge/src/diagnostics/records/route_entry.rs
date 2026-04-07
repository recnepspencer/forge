use crate::mapping::{
    BridgeAspectRegistrationId, BridgeMappingFallbackClass, BridgeMappingId, CoarseRoutingMode,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
};
use crate::routing::{FineGrainedMatchOutcome, FineGrainedMatchStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRecordEntry {
    entity_identity: String,
    aspect_label: String,
    surface_label: String,
    raw_patch_surface_label: String,
    truth_surface_identity: String,
    mapping_id: BridgeMappingId,
    signal_scope: String,
    routing_mode: CoarseRoutingMode,
    fallback_class: Option<BridgeMappingFallbackClass>,
    match_detail: FineGrainedMatchOutcome,
}

pub type BridgeRouteRecordMatch = FineGrainedMatchOutcome;

impl BridgeRouteRecordEntry {
    pub(crate) fn new(
        entity_identity: impl Into<String>,
        aspect_label: impl Into<String>,
        surface_label: impl Into<String>,
        raw_patch_surface_label: impl Into<String>,
        truth_surface_identity: impl Into<String>,
        mapping_id: BridgeMappingId,
        signal_scope: impl Into<String>,
        routing_mode: CoarseRoutingMode,
        fallback_class: Option<BridgeMappingFallbackClass>,
        match_detail: BridgeRouteRecordMatch,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
            raw_patch_surface_label: raw_patch_surface_label.into(),
            truth_surface_identity: truth_surface_identity.into(),
            mapping_id,
            signal_scope: signal_scope.into(),
            routing_mode,
            fallback_class,
            match_detail,
        }
    }

    pub fn entity_identity(&self) -> &str {
        &self.entity_identity
    }

    pub fn aspect_label(&self) -> &str {
        &self.aspect_label
    }

    pub fn surface_label(&self) -> &str {
        &self.surface_label
    }

    pub fn raw_patch_surface_label(&self) -> &str {
        &self.raw_patch_surface_label
    }

    pub fn truth_surface_identity(&self) -> &str {
        &self.truth_surface_identity
    }

    pub fn mapping_id(&self) -> &BridgeMappingId {
        &self.mapping_id
    }

    pub fn signal_scope(&self) -> &str {
        &self.signal_scope
    }

    pub fn routing_mode(&self) -> CoarseRoutingMode {
        self.routing_mode
    }

    pub fn fallback_class(&self) -> Option<&BridgeMappingFallbackClass> {
        self.fallback_class.as_ref()
    }

    pub fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        match &self.match_detail {
            FineGrainedMatchOutcome::Matched { truth_surface_kind, .. }
            | FineGrainedMatchOutcome::FallbackAdmitted { truth_surface_kind, .. }
            | FineGrainedMatchOutcome::SuppressedByRegistrationPolicy { truth_surface_kind }
            | FineGrainedMatchOutcome::UnsupportedSurfaceCategory { truth_surface_kind }
            | FineGrainedMatchOutcome::AmbiguousRegistration { truth_surface_kind } => {
                *truth_surface_kind
            }
        }
    }

    pub fn fine_grained_match_status(&self) -> FineGrainedMatchStatus {
        self.match_detail.status()
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        match &self.match_detail {
            FineGrainedMatchOutcome::Matched {
                aspect_registration_id,
                ..
            }
            | FineGrainedMatchOutcome::FallbackAdmitted {
                aspect_registration_id,
                ..
            } => Some(aspect_registration_id),
            FineGrainedMatchOutcome::SuppressedByRegistrationPolicy { .. }
            | FineGrainedMatchOutcome::UnsupportedSurfaceCategory { .. }
            | FineGrainedMatchOutcome::AmbiguousRegistration { .. } => None,
        }
    }

    pub fn subscription_slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.match_detail.subscription_slice_kind()
    }

    pub fn slice_fallback_policy(&self) -> Option<SliceFallbackPolicy> {
        match &self.match_detail {
            FineGrainedMatchOutcome::FallbackAdmitted { fallback_policy, .. } => {
                Some(*fallback_policy)
            }
            FineGrainedMatchOutcome::Matched { .. } => Some(SliceFallbackPolicy::Disallow),
            FineGrainedMatchOutcome::SuppressedByRegistrationPolicy { .. }
            | FineGrainedMatchOutcome::UnsupportedSurfaceCategory { .. }
            | FineGrainedMatchOutcome::AmbiguousRegistration { .. } => None,
        }
    }

    pub fn match_detail(&self) -> &BridgeRouteRecordMatch {
        &self.match_detail
    }
}
