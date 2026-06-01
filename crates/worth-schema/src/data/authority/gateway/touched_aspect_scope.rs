use std::collections::BTreeSet;

use forge_relational::facade::runtime::RelationalReadView;

use crate::data::aspects::{
    entity_domain_aspect, relation_domain_aspect, Aspect, DiagnosticsAspect,
};
use crate::data::authority::{RawTopologyIntent, TopologyMutation};
use crate::data::entities::EntityKind;
use crate::data::relations::RelationKind;

use super::TopologyAuthorityError;

pub(super) fn touched_aspects_for_intent(
    read: Option<&RelationalReadView>,
    intent: &RawTopologyIntent,
) -> Result<BTreeSet<Aspect>, TopologyAuthorityError> {
    let mut aspects = BTreeSet::new();
    for mutation in &intent.mutations {
        aspects.extend(touched_aspects_for_mutation(read, mutation)?);
    }
    Ok(aspects)
}

fn touched_aspects_for_mutation(
    read: Option<&RelationalReadView>,
    mutation: &TopologyMutation,
) -> Result<BTreeSet<Aspect>, TopologyAuthorityError> {
    match mutation {
        TopologyMutation::CreateEntity { kind, .. }
        | TopologyMutation::UpsertEntity { kind, .. } => {
            Ok(entity_aspects(*kind).into_iter().collect())
        }
        TopologyMutation::CreateRelation { kind, .. }
        | TopologyMutation::UpsertRelation { kind, .. } => {
            Ok(relation_aspects(*kind).into_iter().collect())
        }
        TopologyMutation::RemoveEntity { entity_id } => {
            let read = read.ok_or_else(|| {
                TopologyAuthorityError::ReadSnapshot(
                    " authority requires a readable starting snapshot for entity removal"
                        .to_string(),
                )
            })?;
            let Some(existing) = read.get_entity(*entity_id) else {
                return Err(TopologyAuthorityError::MissingEntity(*entity_id));
            };
            let kind = EntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                TopologyAuthorityError::ReadSnapshot(format!(
                    "unknown  entity kind id `{}` for entity `{:?}`",
                    existing.kind.kind_id.0, entity_id
                ))
            })?;
            Ok(entity_aspects(kind).into_iter().collect())
        }
        TopologyMutation::RemoveRelation { relation_id } => {
            let read = read.ok_or_else(|| {
                TopologyAuthorityError::ReadSnapshot(
                    " authority requires a readable starting snapshot for relation removal"
                        .to_string(),
                )
            })?;
            let Some(existing) = read.get_relation(*relation_id) else {
                return Err(TopologyAuthorityError::MissingRelation(*relation_id));
            };
            let kind = RelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                TopologyAuthorityError::ReadSnapshot(format!(
                    "unknown  relation kind id `{}` for relation `{:?}`",
                    existing.kind.kind_id.0, relation_id
                ))
            })?;
            Ok(relation_aspects(kind).into_iter().collect())
        }
    }
}

fn entity_aspects(kind: EntityKind) -> [Aspect; 2] {
    [
        entity_domain_aspect(kind),
        Aspect::Diagnostics(DiagnosticsAspect::Decisions),
    ]
}

fn relation_aspects(kind: RelationKind) -> [Aspect; 2] {
    [
        relation_domain_aspect(kind),
        Aspect::Diagnostics(DiagnosticsAspect::Decisions),
    ]
}
