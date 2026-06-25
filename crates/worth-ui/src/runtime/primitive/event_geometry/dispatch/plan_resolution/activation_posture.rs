use super::super::super::receipt::WorthUiPrimitiveEventContainment;
use super::super::region_receipt::WorthUiPrimitiveEventRegionReceipt;
use crate::runtime::WorthUiPrimitiveActivationPosture;

pub(in crate::runtime::primitive::event_geometry::dispatch) fn primary_activation_bubbles(
    primary: &WorthUiPrimitiveEventRegionReceipt,
) -> bool {
    region_activation_is_eligible(primary)
        && primary.containment() == WorthUiPrimitiveEventContainment::Bubble
}

pub(in crate::runtime::primitive::event_geometry::dispatch) fn region_activation_is_eligible(
    region: &WorthUiPrimitiveEventRegionReceipt,
) -> bool {
    region.activation_posture() == WorthUiPrimitiveActivationPosture::Eligible
}
