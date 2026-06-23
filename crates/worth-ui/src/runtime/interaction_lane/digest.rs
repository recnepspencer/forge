use super::denial_receipt::WorthUiInteractionValueDenialReceipt;
use super::report::WorthUiValidatedInteractionPropSet;
use super::schema::WorthUiInteractionPropSchema;

pub(super) fn interaction_schema_digest(schemas: &[WorthUiInteractionPropSchema]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    for schema in schemas {
        digest = fold(digest, schema.schema_id().as_bytes());
        digest = fold(digest, schema.prop_key().as_bytes());
        digest = fold(digest, schema.expected_value_syntax().as_bytes());
    }
    digest
}

pub(super) fn interaction_denial_set_digest(
    surface_id: &str,
    denials: &[WorthUiInteractionValueDenialReceipt],
) -> u64 {
    let mut digest = fold(0xcbf2_9ce4_8422_2325, surface_id.as_bytes());
    for denial in denials {
        digest = fold(digest, denial.prop_key().as_bytes());
        digest = fold(digest, denial.raw_value().as_bytes());
        digest = fold(digest, format!("{:?}", denial.denial_code()).as_bytes());
    }
    digest
}

pub(super) fn interaction_admission_digest(
    surface_id: &str,
    authored_digest: u64,
    prop_set: &WorthUiValidatedInteractionPropSet,
) -> u64 {
    let basis = format!(
        "interaction-admission|surface:{surface_id}|authored:{authored_digest}|kind:{:?}|id:{}|target:{:?}|readiness:{:?}|payload:{}",
        prop_set.kind(),
        prop_set.interaction_id(),
        prop_set.target(),
        prop_set.readiness(),
        prop_set.payload_value().as_text()
    );
    fold(0xcbf2_9ce4_8422_2325, basis.as_bytes())
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}
