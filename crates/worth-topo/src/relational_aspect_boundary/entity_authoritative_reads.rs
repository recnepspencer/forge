use forge_foundational::facade::{
    AspectKey, AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
    InternedString,
};
use forge_relational::facade::runtime::EntityReadRecord;
use schema::facade::platform::aspects::{
    entity_domain_aspect, entity_domain_field, Aspect, NamingAspect,
};
use schema::facade::platform::entities::{EntityKind, NamingEntityKind};

pub(crate) fn entity_record_domain_label(record: &EntityReadRecord) -> Option<String> {
    let kind = EntityKind::from_kind_id(record.kind.kind_id)?;
    if kind == EntityKind::Naming(NamingEntityKind::PersistentName) {
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
    _field: &str,
) -> Option<String> {
    scalar_string_from_state(
        record.authoritative_aspect_state.as_ref()?,
        &aspect.aspect_key(),
    )
}

fn scalar_string_from_state(
    state: &AuthoritativeRecordAspectState,
    aspect_key: &AspectKey,
) -> Option<String> {
    match state.get(aspect_key)?.view() {
        ContractValidatedAspectValueView::Scalar(AspectValue::String(value)) => match value {
            InternedString::Raw(raw) => Some(raw.clone()),
            InternedString::Symbol(_) => None,
        },
        ContractValidatedAspectValueView::Scalar(_)
        | ContractValidatedAspectValueView::Struct(_) => None,
    }
}
