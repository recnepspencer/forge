use super::*;

pub(crate) fn read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record
        .payload
        .as_json()
        .and_then(|value| value.get("name"))
        .and_then(|value| value.as_str())
}

pub(crate) fn all_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::All,
        aspects: RequestedAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(crate) fn any_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::Any,
        aspects: RequestedAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(crate) fn entity_aspect_history_digest(
    runtime: &RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    entity_aspect_history_digest_on_branch(runtime, &BranchId("main".to_string()), entity_id, filter)
}

pub(crate) fn entity_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    runtime
        .history_access()
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
        .history_access()
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
        .entity_aspect_history_with_trace(branch_id, lineage_id, filter)
        .lineage_aspect_resolution_digest()
}
