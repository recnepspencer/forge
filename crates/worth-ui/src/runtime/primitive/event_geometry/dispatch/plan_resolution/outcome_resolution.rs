use super::super::outcome_receipt::WorthUiPrimitiveEventDispatchOutcome;
use super::super::region_receipt::WorthUiPrimitiveEventRegionReceipt;
use super::activation_posture::region_activation_is_eligible;

pub(in crate::runtime::primitive::event_geometry::dispatch) fn dispatch_outcome(
    primary: &WorthUiPrimitiveEventRegionReceipt,
    emitted: Vec<String>,
) -> WorthUiPrimitiveEventDispatchOutcome {
    if !region_activation_is_eligible(primary) {
        return WorthUiPrimitiveEventDispatchOutcome::disabled(primary);
    }
    if emitted.len() > 1 {
        WorthUiPrimitiveEventDispatchOutcome::bubbled(primary, emitted)
    } else {
        WorthUiPrimitiveEventDispatchOutcome::enabled(primary)
    }
}
