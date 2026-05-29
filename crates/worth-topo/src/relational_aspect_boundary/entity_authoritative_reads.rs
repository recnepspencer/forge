use forge_foundational::facade::{
    AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView, InternedString,
};
use forge_relational::facade::runtime::EntityReadRecord;
use forge_relational::facade::transactions::AspectFieldPatchTarget;
use schema::facade::{entity_domain_aspect, entity_domain_field, Aspect, EntityKind, NamingAspect};

use super::field_key;

pub(crate) fn entity_record_domain_label(record: &EntityReadRecord) -> Option<String> {
    let kind = EntityKind::from_kind_id(record.kind.kind_id)?;
    if kind == EntityKind::Naming(schema::facade::NamingEntityKind::PersistentName) {
        return entity_record_string_aspect(
            record,
            &Aspect::Naming(NamingAspect::PersistentName),
            "persistent_name",
        );
    }
    entity_record_string_aspect(
        record,
        &entity_domain_aspect(kind),
        entity_domain_field(kind),
    )
}

pub(crate) fn entity_record_string_aspect(
    record: &EntityReadRecord,
    aspect: &Aspect,
    field: &str,
) -> Option<String> {
    scalar_string_from_state(
        record.authoritative_aspect_state.as_ref()?,
        &AspectFieldPatchTarget::single(aspect.aspect_key(), field_key(field)),
    )
}

fn scalar_string_from_state(
    state: &AuthoritativeRecordAspectState,
    target: &AspectFieldPatchTarget,
) -> Option<String> {
    match state.get(target.aspect_key())?.view() {
        ContractValidatedAspectValueView::Scalar(AspectValue::String(value)) => match value {
            InternedString::Raw(raw) => Some(raw.clone()),
            InternedString::Symbol(_) => None,
        },
        ContractValidatedAspectValueView::Scalar(_)
        | ContractValidatedAspectValueView::Struct(_) => None,
    }
}
