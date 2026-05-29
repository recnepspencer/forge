use forge_foundational::facade::{
    AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView, InternedString,
};
use forge_relational::facade::runtime::EntityReadRecord;
use forge_relational::facade::transactions::{AspectFieldPatch, AspectFieldPatchTarget};

use crate::data::aspects::{entity_domain_aspect, entity_domain_field, field_key};
use crate::data::entities::EntityKind;

pub fn entity_create_fields(kind: EntityKind, label: &str) -> AspectFieldPatch {
    AspectFieldPatch::single(
        entity_domain_aspect(kind).aspect_key(),
        field_key(entity_domain_field(kind)),
        AspectValue::String(label.to_string().into()),
    )
}

pub fn relation_create_fields() -> AspectFieldPatch {
    AspectFieldPatch::default()
}

pub fn entity_record_label<'a>(record: &'a EntityReadRecord, kind: EntityKind) -> Option<&'a str> {
    scalar_string_aspect(
        record.authoritative_aspect_state.as_ref()?,
        &AspectFieldPatchTarget::single(
            entity_domain_aspect(kind).aspect_key(),
            field_key(entity_domain_field(kind)),
        ),
    )
}

fn scalar_string_aspect<'a>(
    state: &'a AuthoritativeRecordAspectState,
    target: &AspectFieldPatchTarget,
) -> Option<&'a str> {
    match state.get(target.aspect_key())?.view() {
        ContractValidatedAspectValueView::Scalar(AspectValue::String(value)) => match value {
            InternedString::Raw(raw) => Some(raw.as_str()),
            InternedString::Symbol(_) => None,
        },
        ContractValidatedAspectValueView::Scalar(_)
        | ContractValidatedAspectValueView::Struct(_) => None,
    }
}
