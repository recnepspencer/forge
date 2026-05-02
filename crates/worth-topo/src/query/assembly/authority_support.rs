use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{ForgeQueryEntity, ForgeQueryRuntimeError, ForgeQueryWorkspaceError};
use forge_relational::facade::identity::{EntityId, RelationId};
use serde_json::Value;
use worth_schema::facade::{
    worth_query_aspect_path_strings, DerivedTopologyReadBasis, RawWorthTopologyIntent, WorthAspect,
    WorthEntityKind, WorthMutationOrigin, WorthRelationKind, WorthTopologyAspect,
    WorthTopologyMutation, WorthTopologyRelationKind,
};

use super::authority::{
    ImportedTopologyEntity, ImportedTopologyRelation, WorthTopologyQueryApplyError,
};
use crate::query::WorthTopologyQueryMutationEvidence;

pub fn index_imported_entities(
    rows: Vec<ForgeQueryEntity>,
) -> Result<BTreeMap<EntityId, ImportedTopologyEntity>, WorthTopologyQueryApplyError> {
    let mut entities = BTreeMap::new();
    for row in rows {
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let entity_id: EntityId = serde_json::from_value(provenance.clone()).map_err(|error| {
            WorthTopologyQueryApplyError::Query(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(format!(
                    "entity provenance failed to decode: {error}"
                )),
            ))
        })?;
        let kind_name = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(Value::as_str)
            .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityKind(
                entity_id,
            ))?;
        let kind = WorthEntityKind::ALL
            .into_iter()
            .find(|kind| kind.kind_name() == kind_name)
            .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityKind(
                entity_id,
            ))?;
        entities.insert(
            entity_id,
            ImportedTopologyEntity {
                query_identity: row.identity,
                kind,
            },
        );
    }
    Ok(entities)
}

pub fn index_imported_relations(
    rows: Vec<ForgeQueryEntity>,
) -> Result<BTreeMap<RelationId, ImportedTopologyRelation>, WorthTopologyQueryApplyError> {
    let mut relations = BTreeMap::new();
    for row in rows {
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let relation_id: RelationId =
            serde_json::from_value(provenance.clone()).map_err(|error| {
                WorthTopologyQueryApplyError::Query(ForgeQueryRuntimeError::Workspace(
                    ForgeQueryWorkspaceError::new(format!(
                        "relation provenance failed to decode: {error}"
                    )),
                ))
            })?;
        let kind_name = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(Value::as_str)
            .ok_or(WorthTopologyQueryApplyError::MissingExistingRelationKind(
                relation_id,
            ))?;
        let kind = WorthRelationKind::ALL
            .into_iter()
            .find(|kind| kind.kind_name() == kind_name)
            .ok_or(WorthTopologyQueryApplyError::MissingExistingRelationKind(
                relation_id,
            ))?;
        relations.insert(
            relation_id,
            ImportedTopologyRelation {
                query_identity: row.identity,
                kind,
                source_query_identity: row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("source_identity"))
                    .and_then(Value::as_str)
                    .ok_or(
                        WorthTopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?
                    .to_string(),
                target_query_identity: row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("target_identity"))
                    .and_then(Value::as_str)
                    .ok_or(
                        WorthTopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?
                    .to_string(),
            },
        );
    }
    Ok(relations)
}

pub fn mutation_evidence_for_intent(
    read_basis: &DerivedTopologyReadBasis,
    intent: &RawWorthTopologyIntent,
    entities: &BTreeMap<EntityId, ImportedTopologyEntity>,
    relations: &BTreeMap<RelationId, ImportedTopologyRelation>,
) -> Result<WorthTopologyQueryMutationEvidence, WorthTopologyQueryApplyError> {
    let mut evidence = WorthTopologyQueryMutationEvidence::from_read_basis(read_basis);
    let mut touched = BTreeSet::new();
    for mutation in &intent.mutations {
        match mutation {
            WorthTopologyMutation::CreateEntity { kind, .. } => {
                touched.extend(entity_touched_aspects(*kind));
            }
            WorthTopologyMutation::CreateRelation { kind, .. } => {
                touched.extend(relation_touched_aspects(*kind));
            }
            WorthTopologyMutation::RemoveEntity { entity_id } => {
                let kind = entities
                    .get(entity_id)
                    .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityBinding(
                        *entity_id,
                    ))?
                    .kind;
                touched.extend(entity_touched_aspects(kind));
            }
            WorthTopologyMutation::RemoveRelation { relation_id } => {
                let kind = relations
                    .get(relation_id)
                    .ok_or(
                        WorthTopologyQueryApplyError::MissingExistingRelationBinding(*relation_id),
                    )?
                    .kind;
                touched.extend(relation_touched_aspects(kind));
            }
            WorthTopologyMutation::UpsertEntity { kind, .. } => {
                touched.extend(entity_touched_aspects(*kind));
            }
            WorthTopologyMutation::UpsertRelation {
                relation_id, kind, ..
            } => {
                if let Some(imported) = relations.get(relation_id) {
                    touched.extend(relation_touched_aspects(imported.kind));
                }
                touched.extend(relation_touched_aspects(*kind));
            }
        }
    }
    evidence.touched_aspect_paths = worth_query_aspect_path_strings(touched);
    evidence.derivation_origin = WorthMutationOrigin::LocalEdit;
    Ok(evidence)
}

fn entity_touched_aspects(kind: WorthEntityKind) -> [WorthAspect; 2] {
    [
        match kind {
            WorthEntityKind::Topology(_) => WorthAspect::Topology(WorthTopologyAspect::Structure),
            WorthEntityKind::Geometry(_) => {
                WorthAspect::Geometry(worth_schema::facade::WorthGeometryAspect::Binding)
            }
            WorthEntityKind::Naming(_) => {
                WorthAspect::Naming(worth_schema::facade::WorthNamingAspect::PersistentName)
            }
            WorthEntityKind::Diagnostics(_) => WorthAspect::Diagnostics(
                worth_schema::facade::WorthDiagnosticsAspect::Interpretations,
            ),
        },
        WorthAspect::Diagnostics(worth_schema::facade::WorthDiagnosticsAspect::Decisions),
    ]
}

pub fn relation_touched_aspects(kind: WorthRelationKind) -> [WorthAspect; 2] {
    [
        match kind {
            WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge) => {
                WorthAspect::Topology(WorthTopologyAspect::Ownership)
            }
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext) => {
                WorthAspect::Topology(WorthTopologyAspect::Radial)
            }
            WorthRelationKind::Topology(_) => WorthAspect::Topology(WorthTopologyAspect::Boundary),
            WorthRelationKind::Geometry(_) => {
                WorthAspect::Geometry(worth_schema::facade::WorthGeometryAspect::Binding)
            }
            WorthRelationKind::Naming(_) => {
                WorthAspect::Naming(worth_schema::facade::WorthNamingAspect::PersistentName)
            }
            WorthRelationKind::Diagnostics(_) => WorthAspect::Diagnostics(
                worth_schema::facade::WorthDiagnosticsAspect::Interpretations,
            ),
        },
        WorthAspect::Diagnostics(worth_schema::facade::WorthDiagnosticsAspect::Decisions),
    ]
}
