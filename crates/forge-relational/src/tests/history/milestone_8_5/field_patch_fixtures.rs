use super::*;

pub(super) fn strategy_field_locator(
    aspect_key: AspectKey,
    field_key: FieldKey,
) -> AspectFieldLocator {
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    )
}

pub(super) fn strategy_name_and_replicas_patch(name: &str, replicas: u64) -> AspectFieldPatch {
    AspectFieldPatch::from(std::collections::BTreeMap::from([
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("name").expect("valid name aspect key"),
                FieldKey::new("name").expect("valid name field key"),
            ),
            AspectValue::String(InternedString::Raw(name.to_string())),
        ),
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("replicas").expect("valid replicas aspect key"),
                FieldKey::new("replicas").expect("valid replicas field key"),
            ),
            AspectValue::UInt64(replicas),
        ),
    ]))
}
