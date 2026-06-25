use super::super::candidate_receipt::WorthUiPrimitiveEventDispatchCandidateReceipt;
use super::super::region_receipt::WorthUiPrimitiveEventRegionReceipt;
use super::activation_posture::region_activation_is_eligible;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime::primitive::event_geometry::dispatch) enum CandidateSelectionMode {
    Hover,
    Press,
}

pub(in crate::runtime::primitive::event_geometry::dispatch) fn candidate_for_region(
    region: &WorthUiPrimitiveEventRegionReceipt,
    hit: bool,
    selected: Option<&WorthUiPrimitiveEventRegionReceipt>,
    mode: CandidateSelectionMode,
    bubbles_from_selected: bool,
) -> WorthUiPrimitiveEventDispatchCandidateReceipt {
    let selected_region =
        selected.is_some_and(|selected| selected.receipt_digest() == region.receipt_digest());
    if !hit && !bubbles_from_selected {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::no_hit(region);
    }
    if selected_region && !region_activation_is_eligible(region) {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::disabled_hit(region);
    }
    if mode == CandidateSelectionMode::Hover && selected_region {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::cursor_target(region);
    }
    if mode == CandidateSelectionMode::Hover {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::pass_through(region);
    }
    if selected_region {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::enabled_primary_hit(region);
    }
    if bubbles_from_selected && region_activation_is_eligible(region) {
        return WorthUiPrimitiveEventDispatchCandidateReceipt::bubbled_ancestor(region);
    }
    WorthUiPrimitiveEventDispatchCandidateReceipt::pass_through(region)
}
