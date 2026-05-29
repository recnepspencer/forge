use super::*;

pub(crate) fn read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    read_entity_field(record, "name")
}

pub(crate) fn read_entity_field<'record>(
    record: &'record EntityReadRecord,
    field_name: &str,
) -> Option<&'record str> {
    let field_key = forge_foundational::facade::FieldKey::new(field_name).expect("valid field key");
    record.authoritative_field_display_value(&field_key)
}

pub(crate) fn read_relation_field<'record>(
    record: &'record crate::facade::runtime::RelationReadRecord,
    field_name: &str,
) -> Option<&'record str> {
    let field_key = forge_foundational::facade::FieldKey::new(field_name).expect("valid field key");
    record.authoritative_field_display_value(&field_key)
}

pub(crate) fn all_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::All,
        aspects: CanonicalAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(crate) fn any_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::Any,
        aspects: CanonicalAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(crate) fn entity_aspect_history_digest(
    runtime: &RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    entity_aspect_history_digest_on_branch(
        runtime,
        &BranchId("main".to_string()),
        entity_id,
        filter,
    )
}

pub(crate) fn entity_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    runtime
        .history()
        .entity_aspect_history_with_trace(branch_id, entity_id, filter)
        .aspect_history_digest()
}

pub(crate) fn relation_aspect_history_digest(
    runtime: &RelationalRuntime,
    relation_id: RelationId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    relation_aspect_history_digest_on_branch(
        runtime,
        &BranchId("main".to_string()),
        relation_id,
        filter,
    )
}

pub(crate) fn relation_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    relation_id: RelationId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    runtime
        .history()
        .relation_aspect_history_with_trace(branch_id, relation_id, filter)
        .aspect_history_digest()
}

pub(crate) fn lineage_aspect_history_digest(
    runtime: &RelationalRuntime,
    lineage_id: LineageId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::LineageAspectResolutionDigest {
    lineage_aspect_history_digest_on_branch(
        runtime,
        &BranchId("main".to_string()),
        lineage_id,
        filter,
    )
}

pub(crate) fn lineage_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    lineage_id: LineageId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::LineageAspectResolutionDigest {
    runtime
        .lineage_access()
        .entity_aspect_history_with_trace(
            crate::facade::lineage::HistoricalResolutionRequest {
                branch_id: branch_id.clone(),
                lineage_id,
                boundedness_basis:
                    crate::facade::lineage::HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            },
            filter,
        )
        .lineage_aspect_resolution_digest()
}
