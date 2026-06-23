use crate::runtime::{WorthUiFlowLayoutValueDenialReceipt, WorthUiValidatedFlowLayoutPropSet};

use super::schema::WorthUiFlowLayoutPropSchema;

pub(super) fn flow_layout_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedFlowLayoutPropSet,
) -> u64 {
    hash_text(&format!(
        "flow-admission|surface:{surface_id}|authored:{authored_digest}|kind:{:?}|gap:{}:{}|padding:{}:{}|align:{:?}|cross:{:?}|fit:{:?}|fill:{:?}",
        prop_set.kind(),
        prop_set.gap_token(),
        prop_set.gap_points(),
        prop_set.padding_token(),
        prop_set.padding_edges().digest_basis(),
        prop_set.align(),
        prop_set.cross_align(),
        prop_set.fit(),
        prop_set.fill()
    ))
}

pub(super) fn flow_layout_receipt_digest(
    admission_digest: u64,
    prop_set: &WorthUiValidatedFlowLayoutPropSet,
) -> u64 {
    hash_text(&format!(
        "flow-receipt|admission:{admission_digest}|kind:{:?}|gap:{}:{}|padding:{}:{}|align:{:?}|cross:{:?}|fit:{:?}|fill:{:?}",
        prop_set.kind(),
        prop_set.gap_token(),
        prop_set.gap_points(),
        prop_set.padding_token(),
        prop_set.padding_edges().digest_basis(),
        prop_set.align(),
        prop_set.cross_align(),
        prop_set.fit(),
        prop_set.fill()
    ))
}

pub(super) fn flow_layout_denial_digest(
    surface_id: &str,
    schema: &WorthUiFlowLayoutPropSchema,
    raw_value: &str,
) -> u64 {
    hash_text(&format!(
        "flow-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}",
        schema.schema_id(),
        schema.prop_key(),
        schema.value_kind(),
        raw_value,
        schema.denial_code()
    ))
}

pub(super) fn flow_layout_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiFlowLayoutValueDenialReceipt],
) -> u64 {
    let mut basis = format!("flow-denial-set|surface:{surface_id}");
    for denial in denials {
        basis.push_str(&format!("|denial:{}", denial.denial_digest()));
    }
    hash_text(&basis)
}

pub(super) fn flow_layout_schema_digest(schemas: &[WorthUiFlowLayoutPropSchema]) -> u64 {
    let mut basis = String::from("flow-schema");
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

pub(super) fn hash_text(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
