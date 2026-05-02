use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::stale_targets::{
    ensure_entity_target_is_current, ensure_relation_target_is_current,
};
use crate::authority::mutation::MutationWorkspace;
use crate::authority::mutation::{apply_adjacency_deltas, AdjacencyDelta, AdjacencyDeltaKind};
use crate::storage::logic::state::{PartitionAccess, RelationEndpoints};
use crate::transactions::data::{
    CommitConflict, ConflictClass, EntityReference, UpdateRelationEndpointsIntent,
};

pub(super) fn apply(
    intent: &UpdateRelationEndpointsIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let slot = intent.relation_id.local_slot.0 as usize;
    let source = resolve_relation_endpoint(workspace, &intent.source, "source")?;
    let target = resolve_relation_endpoint(workspace, &intent.target, "target")?;
    let (kind_id, old_endpoints, payload) = workspace.with_context(|context| {
        ensure_relation_target_is_current(context.state, intent.relation_id)?;
        if let EntityReference::Existing(existing_source) = &intent.source {
            ensure_entity_target_is_current(context.state, *existing_source)?;
        }
        if let EntityReference::Existing(existing_target) = &intent.target {
            ensure_entity_target_is_current(context.state, *existing_target)?;
        }
        let partition = context
            .state
            .get_partition(intent.relation_id.partition_id)
            .ok_or_else(|| {
                CommitConflict::new(ConflictClass::MutationStateInconsistency {
                    detail:
                        "relation endpoint update requires an existing partition after stale-target validation"
                            .to_string(),
                    fields: serde_json::json!({
                        "record_class": "relation",
                        "relation_id": intent.relation_id,
                        "phase": "update_relation_endpoints",
                        "missing": "partition",
                    }),
                })
            })?;
        let slot_view = partition.relation_arena.get_slot(slot).ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail:
                    "relation endpoint update requires an existing slot after stale-target validation"
                        .to_string(),
                fields: serde_json::json!({
                    "record_class": "relation",
                    "relation_id": intent.relation_id,
                    "phase": "update_relation_endpoints",
                    "missing": "slot",
                }),
            })
        })?;
        let kind_id = slot_view.kind_id().ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail:
                    "relation endpoint update requires a retained kind id after stale-target validation"
                        .to_string(),
                fields: serde_json::json!({
                    "record_class": "relation",
                    "relation_id": intent.relation_id,
                    "phase": "update_relation_endpoints",
                    "missing": "kind_id",
                }),
            })
        })?;
        if kind_id != intent.kind_id {
            return Err(CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail:
                    "relation endpoint update intent kind does not match authoritative relation kind"
                        .to_string(),
                fields: serde_json::json!({
                    "record_class": "relation",
                    "relation_id": intent.relation_id,
                    "phase": "update_relation_endpoints",
                    "intent_kind_id": intent.kind_id.0,
                    "authoritative_kind_id": kind_id.0,
                }),
            }));
        }
        let old_endpoints = slot_view.extra().clone().ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail:
                    "relation endpoint update requires retained endpoints after stale-target validation"
                        .to_string(),
                fields: serde_json::json!({
                    "record_class": "relation",
                    "relation_id": intent.relation_id,
                    "phase": "update_relation_endpoints",
                    "missing": "endpoints",
                }),
            })
        })?;
        let payload = slot_view.payload().cloned();

        context
            .state
            .mark_relation_slot_touched(intent.relation_id.partition_id, slot);
        let partition = context.state.get_partition_mut(intent.relation_id.partition_id);
        partition.relation_arena.apply_extra_update(
            slot,
            Some(RelationEndpoints {
                source,
                target,
            }),
            version_id,
        );
        apply_adjacency_deltas(
            context.state,
            &[
                AdjacencyDelta {
                    relation_id: intent.relation_id,
                    kind: AdjacencyDeltaKind::Deleted {
                        source: old_endpoints.source,
                        target: old_endpoints.target,
                    },
                },
                AdjacencyDelta {
                    relation_id: intent.relation_id,
                    kind: AdjacencyDeltaKind::Created { source, target },
                },
            ],
        );
        Ok::<_, CommitConflict>((kind_id, old_endpoints, payload))
    })?;

    Ok(MutationOutcome::relation_updated(
        intent.relation_id,
        kind_id,
        old_endpoints.source,
        old_endpoints.target,
        source,
        target,
        payload,
    ))
}

fn resolve_relation_endpoint(
    workspace: &MutationWorkspace<'_>,
    entity_reference: &EntityReference,
    label: &str,
) -> Result<crate::identity::data::EntityId, CommitConflict> {
    workspace
        .resolve_entity_reference(entity_reference)
        .ok_or_else(|| {
            CommitConflict::new(ConflictClass::InvalidRelationEndpoint {
                detail: format!(
                    "relation endpoint update requires a live or same-batch created {label} entity"
                ),
            })
        })
}
