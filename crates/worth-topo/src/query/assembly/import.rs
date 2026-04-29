use super::*;

pub(super) fn import_topology_entity_records(
    workspace: &mut ForgeQueryWorkspace,
    entities: &[EntityReadRecord],
    evidence: &WorthTopologyQueryMutationEvidence,
) -> Result<
    (
        std::collections::BTreeMap<forge_relational::facade::identity::EntityId, String>,
        Vec<ForgeQueryWriteReceipt>,
    ),
    WorthTopologyQueryImportError,
> {
    let mut entity_map = std::collections::BTreeMap::new();
    let mut receipts = Vec::new();
    for entity in entities {
        let Some(kind) = WorthEntityKind::from_kind_id(entity.kind.kind_id) else {
            return Err(WorthTopologyQueryImportError::UnsupportedEntityKind {
                entity_id: format!("{:?}", entity.entity_id),
                kind_name: entity.kind.kind_name.clone(),
            });
        };
        if !kind.is_topological() {
            continue;
        }
        let payload = entity.payload.as_json();
        let structure = payload
            .and_then(|value| value.get("topology"))
            .and_then(|value| value.get("structure"))
            .cloned()
            .unwrap_or_else(|| Value::String(kind.kind_name().to_string()));
        let persistent_name = payload
            .and_then(|value| value.get("naming"))
            .and_then(|value| value.get("persistent_name"))
            .cloned()
            .unwrap_or(Value::Null);
        let receipt = workspace.insert("WorthTopologyEntity", |builder| {
            builder
                .metadata(WorthTopologyQueryMutationEvidence::metadata_key(), evidence)
                .aspect("topology.kind", kind.kind_name())
                .aspect("lineage.provenance", entity.entity_id)
                .aspect("topology.structure", structure.clone())
                .aspect("naming.persistent_name", persistent_name.clone())
        })?;
        entity_map.insert(
            entity.entity_id,
            receipt.deltas()[0].entity_identity.clone(),
        );
        receipts.push(receipt);
    }
    Ok((entity_map, receipts))
}

pub(super) fn index_persistent_name_targets(
    relations: &[RelationReadRecord],
) -> Result<
    std::collections::BTreeMap<
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    >,
    WorthTopologyQueryImportError,
> {
    let mut targets = std::collections::BTreeMap::new();
    for relation in relations {
        let Some(kind) = WorthRelationKind::from_kind_id(relation.kind.kind_id) else {
            return Err(WorthTopologyQueryImportError::UnsupportedRelationKind {
                relation_id: format!("{:?}", relation.relation_id),
                kind_name: relation.kind.kind_name.clone(),
            });
        };
        if kind == WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity) {
            targets.insert(relation.source, relation.target);
        }
    }
    Ok(targets)
}

pub(super) fn import_persistent_name_records(
    workspace: &mut ForgeQueryWorkspace,
    entities: &[EntityReadRecord],
    entity_map: &std::collections::BTreeMap<forge_relational::facade::identity::EntityId, String>,
    naming_targets: &std::collections::BTreeMap<
        forge_relational::facade::identity::EntityId,
        forge_relational::facade::identity::EntityId,
    >,
    evidence: &WorthTopologyQueryMutationEvidence,
) -> Result<Vec<ForgeQueryWriteReceipt>, WorthTopologyQueryImportError> {
    let mut receipts = Vec::new();
    let persistent_name_kind = WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName);
    for entity in entities {
        let Some(kind) = WorthEntityKind::from_kind_id(entity.kind.kind_id) else {
            return Err(WorthTopologyQueryImportError::UnsupportedEntityKind {
                entity_id: format!("{:?}", entity.entity_id),
                kind_name: entity.kind.kind_name.clone(),
            });
        };
        if kind != persistent_name_kind {
            continue;
        }
        let payload = entity.payload.as_json();
        let persistent_name = payload
            .and_then(|value| value.get("naming"))
            .and_then(|value| value.get("persistent_name"))
            .cloned()
            .unwrap_or(Value::Null);
        let target_identity = match naming_targets.get(&entity.entity_id) {
            Some(target_entity_id) => {
                Some(entity_map.get(target_entity_id).cloned().ok_or_else(|| {
                    WorthTopologyQueryImportError::MissingEntityMapping {
                        relation_id: format!(
                            "persistent-name-target:{:?}->{:?}",
                            entity.entity_id, target_entity_id
                        ),
                        endpoint: "target",
                        entity_id: format!("{:?}", target_entity_id),
                    }
                })?)
            }
            None => None,
        };
        receipts.push(workspace.insert("WorthPersistentName", |builder| {
            let builder = builder
                .metadata(WorthTopologyQueryMutationEvidence::metadata_key(), evidence)
                .aspect("topology.kind", kind.kind_name())
                .aspect("lineage.provenance", entity.entity_id)
                .aspect("naming.persistent_name", persistent_name.clone());
            if let Some(target_identity) = target_identity.clone() {
                builder.aspect("naming.target_identity", target_identity)
            } else {
                builder
            }
        })?);
    }
    Ok(receipts)
}

pub(super) fn import_topology_relation_records(
    workspace: &mut ForgeQueryWorkspace,
    relations: &[RelationReadRecord],
    entity_map: &std::collections::BTreeMap<forge_relational::facade::identity::EntityId, String>,
    evidence: &WorthTopologyQueryMutationEvidence,
) -> Result<Vec<ForgeQueryWriteReceipt>, WorthTopologyQueryImportError> {
    let mut receipts = Vec::new();
    for relation in relations {
        let Some(kind) = WorthRelationKind::from_kind_id(relation.kind.kind_id) else {
            return Err(WorthTopologyQueryImportError::UnsupportedRelationKind {
                relation_id: format!("{:?}", relation.relation_id),
                kind_name: relation.kind.kind_name.clone(),
            });
        };
        let WorthRelationKind::Topology(kind) = kind else {
            continue;
        };
        let Some(source_identity) = entity_map.get(&relation.source) else {
            return Err(WorthTopologyQueryImportError::MissingEntityMapping {
                relation_id: format!("{:?}", relation.relation_id),
                endpoint: "source",
                entity_id: format!("{:?}", relation.source),
            });
        };
        let Some(target_identity) = entity_map.get(&relation.target) else {
            return Err(WorthTopologyQueryImportError::MissingEntityMapping {
                relation_id: format!("{:?}", relation.relation_id),
                endpoint: "target",
                entity_id: format!("{:?}", relation.target),
            });
        };
        receipts.push(workspace.insert("WorthTopologyRelation", |builder| {
            let builder = builder
                .metadata(WorthTopologyQueryMutationEvidence::metadata_key(), evidence)
                .aspect(
                    "topology.kind",
                    WorthRelationKind::Topology(kind).kind_name(),
                )
                .aspect("lineage.provenance", relation.relation_id)
                .aspect("topology.source_identity", source_identity.clone())
                .aspect("topology.target_identity", target_identity.clone());
            if let Some(path) = topology_relation_dependency_path(WorthRelationKind::Topology(kind))
            {
                builder.aspect(path, WorthRelationKind::Topology(kind).kind_name())
            } else {
                builder
            }
        })?);
    }
    if receipts.is_empty() {
        Err(WorthTopologyQueryImportError::Runtime(
            ForgeQueryRuntimeError::Workspace(forge_query::facade::ForgeQueryWorkspaceError::new(
                "worth topology query import requires at least one relation record",
            )),
        ))
    } else {
        Ok(receipts)
    }
}
