use crate::mapping::{
    BridgeAspectRegistrationId, BridgeMappingId, SliceWideningPolicy, SubscriptionSliceKind,
    TruthDeltaSurfaceKind,
};

use super::super::coordinates::BridgePatchTargetCoordinate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePatchContext {
    patch_target_coordinate: BridgePatchTargetCoordinate,
}

impl BridgePatchContext {
    pub fn new(patch_target_coordinate: BridgePatchTargetCoordinate) -> Self {
        Self {
            patch_target_coordinate,
        }
    }

    pub fn patch_target_coordinate(&self) -> &BridgePatchTargetCoordinate {
        &self.patch_target_coordinate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutingContext {
    patch_target_coordinate: BridgePatchTargetCoordinate,
    truth_surface_kind: Option<TruthDeltaSurfaceKind>,
    mapping_id: Option<BridgeMappingId>,
    aspect_registration_id: Option<BridgeAspectRegistrationId>,
    slice_kind: Option<SubscriptionSliceKind>,
    slice_widening_policy: Option<SliceWideningPolicy>,
}

impl BridgeRoutingContext {
    pub fn new(patch_target_coordinate: BridgePatchTargetCoordinate) -> Self {
        Self {
            patch_target_coordinate,
            truth_surface_kind: None,
            mapping_id: None,
            aspect_registration_id: None,
            slice_kind: None,
            slice_widening_policy: None,
        }
    }

    pub fn with_truth_surface_kind(mut self, truth_surface_kind: TruthDeltaSurfaceKind) -> Self {
        self.truth_surface_kind = Some(truth_surface_kind);
        self
    }

    pub fn with_mapping_id(mut self, mapping_id: BridgeMappingId) -> Self {
        self.mapping_id = Some(mapping_id);
        self
    }

    pub fn with_aspect_registration_id(
        mut self,
        aspect_registration_id: BridgeAspectRegistrationId,
    ) -> Self {
        self.aspect_registration_id = Some(aspect_registration_id);
        self
    }

    pub fn with_slice_kind(mut self, slice_kind: SubscriptionSliceKind) -> Self {
        self.slice_kind = Some(slice_kind);
        self
    }

    pub fn with_slice_widening_policy(
        mut self,
        slice_widening_policy: SliceWideningPolicy,
    ) -> Self {
        self.slice_widening_policy = Some(slice_widening_policy);
        self
    }

    pub fn patch_target_coordinate(&self) -> &BridgePatchTargetCoordinate {
        &self.patch_target_coordinate
    }

    pub fn truth_surface_kind(&self) -> Option<TruthDeltaSurfaceKind> {
        self.truth_surface_kind
    }

    pub fn mapping_id(&self) -> Option<&BridgeMappingId> {
        self.mapping_id.as_ref()
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        self.aspect_registration_id.as_ref()
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.slice_kind.as_ref()
    }

    pub fn slice_widening_policy(&self) -> Option<SliceWideningPolicy> {
        self.slice_widening_policy
    }
}
