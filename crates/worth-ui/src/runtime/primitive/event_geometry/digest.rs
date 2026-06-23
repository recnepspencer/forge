use crate::runtime::{
    WorthUiEventGeometryValueDenialReceipt, WorthUiValidatedEventGeometryPropSet,
};

use super::super::{WorthUiPrimitiveFrame, WorthUiPrimitiveResolvedCursorPosture};
use super::dispatch::WorthUiPrimitiveEventRegionReceipt;
use super::schema::WorthUiEventGeometryPropSchema;
use super::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitivePointerCapture,
};

pub(super) fn event_geometry_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedEventGeometryPropSet,
) -> u64 {
    hash_text(&format!(
        "event-geometry-admission|surface:{surface_id}|authored:{authored_digest}|cursor:{:?}|hit_area:{:?}|hit_slop:{}:{}|containment:{:?}|capture:{:?}",
        prop_set.cursor(),
        prop_set.hit_area(),
        prop_set.hit_slop_token(),
        prop_set.hit_slop_edges().digest_basis(),
        prop_set.containment(),
        prop_set.capture()
    ))
}

pub(super) fn event_geometry_receipt_digest(
    admission_digest: u64,
    prop_set: &WorthUiValidatedEventGeometryPropSet,
) -> u64 {
    hash_text(&format!(
        "event-geometry-receipt|admission:{admission_digest}|cursor:{:?}|hit_area:{:?}|hit_slop:{}:{}|containment:{:?}|capture:{:?}",
        prop_set.cursor(),
        prop_set.hit_area(),
        prop_set.hit_slop_token(),
        prop_set.hit_slop_edges().digest_basis(),
        prop_set.containment(),
        prop_set.capture()
    ))
}

pub(super) fn event_geometry_denial_digest(
    surface_id: &str,
    schema: &WorthUiEventGeometryPropSchema,
    raw_value: &str,
) -> u64 {
    hash_text(&format!(
        "event-geometry-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}",
        schema.schema_id(),
        schema.prop_key(),
        schema.value_kind(),
        raw_value,
        schema.denial_code()
    ))
}

pub(super) fn event_geometry_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiEventGeometryValueDenialReceipt],
) -> u64 {
    let mut basis = format!("event-geometry-denial-set|surface:{surface_id}");
    for denial in denials {
        basis.push_str(&format!("|denial:{}", denial.denial_digest()));
    }
    hash_text(&basis)
}

pub(super) fn event_geometry_schema_digest(schemas: &[WorthUiEventGeometryPropSchema]) -> u64 {
    let mut basis = String::from("event-geometry-schema");
    for schema in schemas {
        basis.push_str(&format!(
            "|{}:{}:{:?}:{}:{}:{:?}",
            schema.schema_id(),
            schema.prop_key(),
            schema.value_kind(),
            schema.default_value(),
            schema.expected_value_syntax(),
            schema.denial_code()
        ));
    }
    hash_text(&basis)
}

pub(super) fn event_region_digest(
    surface_id: &str,
    interaction_id: &str,
    parent_surface_id: Option<&str>,
    order: WorthUiPrimitiveEventRegionOrder,
    visual_frame: WorthUiPrimitiveFrame,
    hit_frame: WorthUiPrimitiveFrame,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    can_activate: bool,
    containment: WorthUiPrimitiveEventContainment,
    capture: WorthUiPrimitivePointerCapture,
) -> u64 {
    hash_text(&format!(
        "event-region|surface:{surface_id}|interaction:{interaction_id}|parent:{}|order:{}:{}|visual:{:?}|hit:{:?}|cursor:{:?}|activate:{}|contain:{:?}|capture:{:?}",
        parent_surface_id.unwrap_or("<root>"),
        order.depth(),
        order.order(),
        visual_frame,
        hit_frame,
        cursor,
        can_activate,
        containment,
        capture
    ))
}

pub(super) fn event_plan_digest(regions: &[WorthUiPrimitiveEventRegionReceipt]) -> u64 {
    let mut basis = String::from("event-plan");
    for region in regions {
        basis.push_str(&format!("|region:{}", region.receipt_digest()));
    }
    hash_text(&basis)
}

pub(super) fn event_dispatch_digest(
    primary_surface_id: Option<&str>,
    emitted_surface_ids: &[String],
    cursor: WorthUiPrimitiveResolvedCursorPosture,
) -> u64 {
    let mut basis = format!(
        "event-dispatch|primary:{}|cursor:{:?}",
        primary_surface_id.unwrap_or("<none>"),
        cursor
    );
    for emitted in emitted_surface_ids {
        basis.push_str(&format!("|emit:{emitted}"));
    }
    hash_text(&basis)
}

pub(super) fn hash_text(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
