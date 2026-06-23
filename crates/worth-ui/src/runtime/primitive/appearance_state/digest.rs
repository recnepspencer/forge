use super::receipt::WorthUiAppearanceStateFieldSet;
use super::report::WorthUiValidatedAppearanceStatePropSet;
use super::schema::WorthUiAppearanceStatePropSchema;
use super::WorthUiAppearanceStateValueDenialReceipt;

pub(super) fn appearance_state_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedAppearanceStatePropSet,
) -> u64 {
    hash_text(&format!(
        "appearance-state-admission|surface:{surface_id}|authored:{authored_digest}|{}",
        prop_set.digest_basis()
    ))
}

pub(super) fn appearance_state_receipt_digest(
    admission_digest: u64,
    prop_set: &WorthUiValidatedAppearanceStatePropSet,
) -> u64 {
    hash_text(&format!(
        "appearance-state-receipt|admission:{admission_digest}|{}",
        prop_set.digest_basis()
    ))
}

pub(super) fn appearance_state_denial_digest(
    surface_id: &str,
    schema: &WorthUiAppearanceStatePropSchema,
    raw_value: &str,
) -> u64 {
    hash_text(&format!(
        "appearance-state-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}",
        schema.schema_id(),
        schema.prop_key(),
        schema.value_kind(),
        raw_value,
        schema.denial_code()
    ))
}

pub(super) fn appearance_state_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiAppearanceStateValueDenialReceipt],
) -> u64 {
    let mut basis = format!("appearance-state-denial-set|surface:{surface_id}");
    for denial in denials {
        basis.push_str(&format!("|denial:{}", denial.denial_digest()));
    }
    hash_text(&basis)
}

pub(super) fn appearance_state_schema_digest(schemas: &[WorthUiAppearanceStatePropSchema]) -> u64 {
    let mut basis = String::from("appearance-state-schema");
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

pub(super) fn state_digest_basis(label: &str, fields: &WorthUiAppearanceStateFieldSet) -> String {
    format!("{label}:{}", fields.digest_basis())
}

pub(super) fn hash_text(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
