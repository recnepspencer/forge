use super::super::region_receipt::{
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionReceipt,
};

pub(in crate::runtime::primitive::event_geometry::dispatch) fn candidates_hit_count(
    regions: &[WorthUiPrimitiveEventRegionReceipt],
    point: WorthUiPrimitiveEventHitTestPoint,
) -> usize {
    regions
        .iter()
        .filter(|region| region.contains(point))
        .count()
}
