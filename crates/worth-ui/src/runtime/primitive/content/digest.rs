use super::receipt::{WorthUiPrimitiveContentItem, WorthUiPrimitiveContentReceipt};
use super::report::WorthUiValidatedPrimitiveContentPropSet;
use super::schema::WorthUiPrimitiveContentPropSchema;
use super::WorthUiPrimitiveContentValueDenialReceipt;

pub(super) fn primitive_content_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedPrimitiveContentPropSet,
) -> u64 {
    hash_text(&format!(
        "content-admission|surface:{surface_id}|authored:{authored_digest}|kind:{:?}|order:{:?}|text:{}|icon:{:?}|text_size:{}:{}|icon_size:{}:{}|stroke:{}:{}|spacer:{}:{}|badge:{:?}|divider:{}:{}|a11y:{:?}",
        prop_set.kind(),
        prop_set.order(),
        prop_set.text(),
        prop_set.icon_id().map(|id| id.as_str()),
        prop_set.text_size_token(),
        prop_set.text_size_points(),
        prop_set.icon_size_token(),
        prop_set.icon_size_points(),
        prop_set.icon_stroke_token(),
        prop_set.icon_stroke_width_points(),
        prop_set.spacer_size_token(),
        prop_set.spacer_size_points(),
        prop_set.badge_text(),
        prop_set.divider_thickness_token(),
        prop_set.divider_thickness_points(),
        prop_set.accessibility_name()
    ))
}

pub(super) fn primitive_content_receipt_digest(
    _admission_digest: u64,
    receipt: &WorthUiPrimitiveContentReceipt,
) -> u64 {
    let mut basis = format!(
        "content-receipt|kind:{:?}|a11y:{:?}",
        receipt.kind(),
        receipt.accessibility_name()
    );
    for item in receipt.items() {
        basis.push_str(&format!("|item:{}", content_item_digest_basis(item)));
    }
    hash_text(&basis)
}

pub(super) fn primitive_content_denial_digest(
    surface_id: &str,
    schema: &WorthUiPrimitiveContentPropSchema,
    raw_value: &str,
) -> u64 {
    hash_text(&format!(
        "content-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}",
        schema.schema_id(),
        schema.prop_key(),
        schema.value_kind(),
        raw_value,
        schema.denial_code()
    ))
}

pub(super) fn primitive_content_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiPrimitiveContentValueDenialReceipt],
) -> u64 {
    let mut basis = format!("content-denial-set|surface:{surface_id}");
    for denial in denials {
        basis.push_str(&format!("|denial:{}", denial.denial_digest()));
    }
    hash_text(&basis)
}

pub(super) fn primitive_content_schema_digest(
    schemas: &[WorthUiPrimitiveContentPropSchema],
) -> u64 {
    let mut basis = String::from("content-schema");
    for schema in schemas {
        basis.push_str(&format!(
            "|{}:{}:{:?}:{:?}:{}:{:?}",
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

pub(super) fn hash_text(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn content_item_digest_basis(item: &WorthUiPrimitiveContentItem) -> String {
    match item {
        WorthUiPrimitiveContentItem::Text(item) => {
            format!(
                "text:{}:{}:{}",
                item.text(),
                item.size_token(),
                item.size_points()
            )
        }
        WorthUiPrimitiveContentItem::Icon(item) => format!(
            "icon:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            item.icon_id(),
            item.source_kind(),
            item.provider(),
            item.source_key(),
            item.paint_command().token(),
            item.size_token(),
            item.size_points(),
            item.stroke_token(),
            item.stroke_width_points()
        ),
        WorthUiPrimitiveContentItem::Spacer(item) => {
            format!("spacer:{}:{}", item.size_token(), item.size_points())
        }
        WorthUiPrimitiveContentItem::Badge(item) => {
            format!(
                "badge:{}:{}:{}",
                item.text(),
                item.size_token(),
                item.size_points()
            )
        }
        WorthUiPrimitiveContentItem::Divider(item) => {
            format!(
                "divider:{}:{}",
                item.thickness_token(),
                item.thickness_points()
            )
        }
    }
}
