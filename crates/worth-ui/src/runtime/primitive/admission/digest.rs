use crate::runtime::{WorthUiPrimitiveValueDenialReceipt, WorthUiValidatedPrimitivePropSet};

use super::super::WorthUiPrimitiveAuthoredPropSchema;

pub(super) fn primitive_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedPrimitivePropSet,
) -> u64 {
    hash_text(&format!(
        "primitive-admission|surface:{surface_id}|authored:{authored_digest}|text:{}|align:{:?}|padding:{}|radius:{}|bg:{}|fg:{}|interaction:{:?}|cursor:{:?}|focus:{:?}|interaction_id:{}|submit_payload:{}|motion:{:?}|motion_target:{:?}|motion_duration:{}|motion_easing:{:?}",
        prop_set.text(),
        prop_set.align(),
        prop_set.padding_token(),
        prop_set.radius_token(),
        prop_set.background_color().hex_triplet(),
        prop_set.foreground_color().hex_triplet(),
        prop_set.interaction_kind(),
        prop_set.cursor(),
        prop_set.focus(),
        prop_set.interaction_id(),
        prop_set.submit_payload(),
        prop_set.motion_kind(),
        prop_set.motion_target(),
        prop_set.motion_duration_token(),
        prop_set.motion_easing()
    ))
}

pub(super) fn primitive_denial_digest(
    surface_id: &str,
    schema: &WorthUiPrimitiveAuthoredPropSchema,
    raw_value: &str,
) -> u64 {
    hash_text(&format!(
        "primitive-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}",
        schema.schema_id(),
        schema.prop_key(),
        schema.value_kind(),
        raw_value,
        schema.denial_code()
    ))
}

pub(super) fn primitive_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiPrimitiveValueDenialReceipt],
) -> u64 {
    let mut basis = format!("primitive-denial-set|surface:{surface_id}");
    for denial in denials {
        basis.push_str(&format!("|denial:{}", denial.denial_digest()));
    }
    hash_text(&basis)
}

pub(super) fn primitive_schema_digest(schemas: &[WorthUiPrimitiveAuthoredPropSchema]) -> u64 {
    let mut basis = String::from("primitive-schema");
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
