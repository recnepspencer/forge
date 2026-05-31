use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, AuthoritativeRecordAspectState, CanonicalFieldPath,
    ContractValidatedAspectValueView, FieldKey, InternedString, LocatorAuthority,
};
use forge_relational::facade::runtime::EntityReadRecord;
use forge_relational::facade::transactions::AspectFieldPatch;

use crate::data::aspects::{entity_domain_aspect, entity_domain_field, field_key};
use crate::data::entities::EntityKind;

pub fn entity_create_fields(kind: EntityKind, label: &str) -> AspectFieldPatch {
    AspectFieldPatch::from_locator(
        planned_single_field_locator(
            entity_domain_aspect(kind).aspect_key(),
            field_key(entity_domain_field(kind)),
        ),
        AspectValue::String(label.to_string().into()),
    )
}

pub fn relation_create_fields() -> AspectFieldPatch {
    AspectFieldPatch::default()
}

pub fn entity_record_label<'a>(record: &'a EntityReadRecord, kind: EntityKind) -> Option<&'a str> {
    scalar_string_aspect(
        record.authoritative_aspect_state.as_ref()?,
        &entity_domain_aspect(kind).aspect_key(),
    )
}

fn scalar_string_aspect<'a>(
    state: &'a AuthoritativeRecordAspectState,
    aspect_key: &AspectKey,
) -> Option<&'a str> {
    match state.get(aspect_key)?.view() {
        ContractValidatedAspectValueView::Scalar(AspectValue::String(value)) => match value {
            InternedString::Raw(raw) => Some(raw.as_str()),
            InternedString::Symbol(_) => None,
        },
        ContractValidatedAspectValueView::Scalar(_)
        | ContractValidatedAspectValueView::Struct(_) => None,
    }
}

fn planned_single_field_locator(aspect_key: AspectKey, field_key: FieldKey) -> AspectFieldLocator {
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    )
}
