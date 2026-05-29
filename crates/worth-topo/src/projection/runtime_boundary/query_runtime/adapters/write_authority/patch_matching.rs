use std::collections::BTreeSet;
use std::sync::{Arc, RwLock};

use forge_relational::facade::identity::VersionId;
use forge_relational::facade::publication::PatchRecord;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::RecordRef;
use serde_json::Value;

use super::super::write_support::{record_identity, target_collection_for_patch};

pub(super) enum LoweredPatchMatch {
    TopologyEntityInsert {
        structure_label: String,
        persistent_name: String,
    },
    TopologyRelationInsert {
        kind_name: String,
        source_identity: String,
        target_identity: String,
    },
    ExistingTargetIdentity {
        resolved_target_identity: String,
    },
}

impl LoweredPatchMatch {
    pub(super) fn matching_patch_indexes(
        &self,
        runtime: &Arc<RwLock<RelationalRuntime>>,
        version_id: VersionId,
        patch: &[PatchRecord],
        used_indexes: &BTreeSet<usize>,
    ) -> Vec<usize> {
        let runtime = runtime
            .read()
            .expect("topology runtime write authority lock poisoned");
        patch
            .iter()
            .enumerate()
            .filter(|(index, _)| !used_indexes.contains(index))
            .filter_map(|(index, record)| {
                self.matches_record(&runtime, version_id, record)
                    .then_some(index)
            })
            .collect()
    }

    fn matches_record(
        &self,
        runtime: &RelationalRuntime,
        version_id: VersionId,
        record: &PatchRecord,
    ) -> bool {
        let projection = runtime.read_truth().project_version(version_id);
        match self {
            Self::TopologyEntityInsert {
                structure_label,
                persistent_name,
            } => {
                match target_collection_for_patch(runtime, version_id, &record.target).as_deref() {
                    Some("TopologyEntity") => match record.target {
                        RecordRef::Entity(entity_id) => {
                            projection.entity_record(entity_id).is_some_and(|entity| {
                                entity
                                    .payload
                                    .as_json()
                                    .and_then(|payload| payload.get("label"))
                                    .and_then(serde_json::Value::as_str)
                                    .is_some_and(|label| label == structure_label)
                            })
                        }
                        RecordRef::Relation(_) => false,
                    },
                    Some("PersistentName") => match record.target {
                        RecordRef::Entity(entity_id) => {
                            projection.entity_record(entity_id).is_some_and(|entity| {
                                entity
                                    .payload
                                    .as_json()
                                    .and_then(|payload| payload.get("naming"))
                                    .and_then(|payload| payload.get("persistent_name"))
                                    .and_then(serde_json::Value::as_str)
                                    .is_some_and(|name| name == persistent_name)
                            })
                        }
                        RecordRef::Relation(_) => false,
                    },
                    _ => false,
                }
            }
            Self::TopologyRelationInsert {
                kind_name,
                source_identity,
                target_identity,
            } => {
                let Some("TopologyRelation") =
                    target_collection_for_patch(runtime, version_id, &record.target).as_deref()
                else {
                    return false;
                };
                let RecordRef::Relation(relation_id) = record.target else {
                    return false;
                };
                let Some(relation) = projection.relation_record(relation_id) else {
                    return false;
                };
                schema::facade::platform::relations::RelationKind::from_kind_id(
                    relation.kind.kind_id,
                )
                .is_some_and(|kind| kind.kind_name() == kind_name)
                    && entity_matches_identity(
                        runtime,
                        version_id,
                        relation.source,
                        source_identity,
                    )
                    && entity_matches_identity(
                        runtime,
                        version_id,
                        relation.target,
                        target_identity,
                    )
            }
            Self::ExistingTargetIdentity {
                resolved_target_identity,
            } => record_identity(&record.target) == *resolved_target_identity,
        }
    }
}

fn entity_matches_identity(
    runtime: &RelationalRuntime,
    version_id: VersionId,
    entity_id: forge_relational::facade::identity::EntityId,
    identity: &str,
) -> bool {
    if identity.starts_with("entity:") {
        return record_identity(&RecordRef::Entity(entity_id)) == identity;
    }
    let Some(created_label) = identity.strip_prefix("created:") else {
        return false;
    };
    runtime
        .read_truth()
        .project_version(version_id)
        .entity_record(entity_id)
        .is_some_and(|entity| {
            entity
                .payload
                .as_json()
                .and_then(|payload| payload.get("label"))
                .and_then(Value::as_str)
                .is_some_and(|label| label == created_label)
        })
}
