use crate::identity::data::EntityId;

#[derive(Debug, Clone, Copy)]
pub(super) struct RelationState<'a> {
    pub(super) source: Option<EntityId>,
    pub(super) target: Option<EntityId>,
    pub(super) authoritative_state:
        Option<&'a worth_foundational::facade::AuthoritativeRecordAspectState>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct EntityAuthoritativeState<'a> {
    pub(super) authoritative_state:
        Option<&'a worth_foundational::facade::AuthoritativeRecordAspectState>,
}
