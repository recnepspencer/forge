use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{ForgeQueryEntity, ForgeQueryRuntimeError, ForgeQueryWorkspaceError};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::query_aspect_path_strings;
use schema::facade::platform::aspects::{Aspect, TopologyAspect};
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::{
    RelationKind, TopologyRelationKind,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use super::authority::{ImportedTopologyEntity, ImportedTopologyRelation, TopologyQueryApplyError};
use crate::projection::TopologyQueryMutationEvidence;

pub fn index_imported_entities(
    rows: Vec<ForgeQueryEntity>,
) -> Result<BTreeMap<EntityId, ImportedTopologyEntity>, TopologyQueryApplyError> {
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
            TopologyQueryApplyError::Query(ForgeQueryRuntimeError::Workspace(
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
            .ok_or(TopologyQueryApplyError::MissingExistingEntityKind(
                entity_id,
            ))?;
        let kind = EntityKind::ALL
            .into_iter()
            .find(|kind| kind.kind_name() == kind_name)
            .ok_or(TopologyQueryApplyError::MissingExistingEntityKind(
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
) -> Result<BTreeMap<RelationId, ImportedTopologyRelation>, TopologyQueryApplyError> {
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
                TopologyQueryApplyError::Query(ForgeQueryRuntimeError::Workspace(
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
            .ok_or(TopologyQueryApplyError::MissingExistingRelationKind(
                relation_id,
            ))?;
        let kind = RelationKind::ALL
            .into_iter()
            .find(|kind| kind.kind_name() == kind_name)
            .ok_or(TopologyQueryApplyError::MissingExistingRelationKind(
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
                    .ok_or(TopologyQueryApplyError::MissingExistingRelationBinding(
                        relation_id,
                    ))?
                    .to_string(),
                target_query_identity: row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("target_identity"))
                    .and_then(Value::as_str)
                    .ok_or(TopologyQueryApplyError::MissingExistingRelationBinding(
                        relation_id,
                    ))?
                    .to_string(),
            },
        );
    }
    Ok(relations)
}

pub fn mutation_evidence_for_intent(
    read_basis: &DerivedTopologyReadBasis,
    intent: &RawTopologyIntent,
    entities: &BTreeMap<EntityId, ImportedTopologyEntity>,
    relations: &BTreeMap<RelationId, ImportedTopologyRelation>,
) -> Result<TopologyQueryMutationEvidence, TopologyQueryApplyError> {
    let mut evidence = TopologyQueryMutationEvidence::from_read_basis(read_basis);
    let mut touched = BTreeSet::new();
    for mutation in &intent.mutations {
        match mutation {
            TopologyMutation::CreateEntity { kind, .. } => {
                touched.extend(entity_touched_aspects(*kind));
            }
            TopologyMutation::CreateRelation { kind, .. } => {
                touched.extend(relation_touched_aspects(*kind));
            }
            TopologyMutation::RemoveEntity { entity_id } => {
                let kind = entities
                    .get(entity_id)
                    .ok_or(TopologyQueryApplyError::MissingExistingEntityBinding(
                        *entity_id,
                    ))?
                    .kind;
                touched.extend(entity_touched_aspects(kind));
            }
            TopologyMutation::RemoveRelation { relation_id } => {
                let kind = relations
                    .get(relation_id)
                    .ok_or(TopologyQueryApplyError::MissingExistingRelationBinding(
                        *relation_id,
                    ))?
                    .kind;
                touched.extend(relation_touched_aspects(kind));
            }
            TopologyMutation::UpsertEntity { kind, .. } => {
                touched.extend(entity_touched_aspects(*kind));
            }
            TopologyMutation::UpsertRelation {
                relation_id, kind, ..
            } => {
                if let Some(imported) = relations.get(relation_id) {
                    touched.extend(relation_touched_aspects(imported.kind));
                }
                touched.extend(relation_touched_aspects(*kind));
            }
        }
    }
    evidence.touched_aspect_paths = query_aspect_path_strings(touched);
    evidence.derivation_origin = MutationOrigin::LocalEdit;
    Ok(evidence)
}

fn entity_touched_aspects(kind: EntityKind) -> [Aspect; 2] {
    [
        match kind {
            EntityKind::Topology(_) => Aspect::Topology(TopologyAspect::Structure),
            EntityKind::Geometry(_) => Aspect::Geometry(schema::facade::platform::aspects::GeometryAspect::Binding),
            EntityKind::Naming(_) => Aspect::Naming(schema::facade::platform::aspects::NamingAspect::PersistentName),
            EntityKind::Diagnostics(_) => {
                Aspect::Diagnostics(schema::facade::platform::aspects::DiagnosticsAspect::Interpretations)
            }
        },
        Aspect::Diagnostics(schema::facade::platform::aspects::DiagnosticsAspect::Decisions),
    ]
}

pub fn relation_touched_aspects(kind: RelationKind) -> [Aspect; 2] {
    [
        match kind {
            RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
            | RelationKind::Topology(TopologyRelationKind::BodyOwnsLump)
            | RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion)
            | RelationKind::Topology(TopologyRelationKind::RegionOwnsShell)
            | RelationKind::Topology(TopologyRelationKind::ShellOwnsFace)
            | RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => {
                Aspect::Topology(TopologyAspect::Ownership)
            }
            RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
                Aspect::Topology(TopologyAspect::Radial)
            }
            RelationKind::Topology(_) => Aspect::Topology(TopologyAspect::Boundary),
            RelationKind::Geometry(_) => Aspect::Geometry(schema::facade::platform::aspects::GeometryAspect::Binding),
            RelationKind::Naming(_) => Aspect::Naming(schema::facade::platform::aspects::NamingAspect::PersistentName),
            RelationKind::Diagnostics(_) => {
                Aspect::Diagnostics(schema::facade::platform::aspects::DiagnosticsAspect::Interpretations)
            }
        },
        Aspect::Diagnostics(schema::facade::platform::aspects::DiagnosticsAspect::Decisions),
    ]
}




