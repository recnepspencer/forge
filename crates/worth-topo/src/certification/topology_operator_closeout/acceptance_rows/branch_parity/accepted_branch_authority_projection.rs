use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{created_ref, DerivedTopologyReadBasis};

use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditAction, TopologyEditContract,
};

pub(super) fn collapse_split_wire_contracts(
    collapse_wire_key: &str,
    retired_wire_id: EntityId,
    moved_half_edge_ids: &[EntityId],
) -> Vec<TopologyEditContract> {
    let mut contracts = vec![TopologyEditContract::create_topology_entity(
        collapse_wire_key,
        TopologyEntityKind::Wire,
    )];
    for (index, half_edge_id) in moved_half_edge_ids.iter().enumerate() {
        contracts.push(TopologyEditContract::attach_shell_or_wire_membership(
            format!("{collapse_wire_key}.owns_half_edge_{}", index + 1),
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(collapse_wire_key),
            *half_edge_id,
        ));
    }
    contracts.push(TopologyEditContract::retire_topology_entity(
        retired_wire_id,
        TopologyEntityKind::Wire,
    ));
    contracts
}

pub(super) fn authority_expanded_split_mutations(
    contracts: &[TopologyEditContract],
    moved_relation_ids: &[RelationId],
) -> Vec<TopologyMutation> {
    moved_relation_ids
        .iter()
        .map(|relation_id| TopologyMutation::RemoveRelation {
            relation_id: *relation_id,
        })
        .chain(contracts.iter().flat_map(|contract| {
            contract
                .lowered_mutations()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        }))
        .collect()
}

pub(super) fn authority_expanded_collapse_mutations(
    contracts: &[TopologyEditContract],
    split_relation_ids: &[RelationId],
) -> Vec<TopologyMutation> {
    let mut mutations = contracts
        .iter()
        .flat_map(|contract| contract.lowered_mutations().iter().cloned())
        .collect::<Vec<_>>();
    let retire_index = mutations
        .iter()
        .position(|mutation| matches!(mutation, TopologyMutation::RemoveEntity { .. }))
        .unwrap_or(mutations.len());
    for relation_id in split_relation_ids.iter().rev() {
        mutations.insert(
            retire_index,
            TopologyMutation::RemoveRelation {
                relation_id: *relation_id,
            },
        );
    }
    mutations
}

pub(super) fn authority_expanded_rewire_mutations(
    contracts: &[TopologyEditContract],
    create_key_prefix: &str,
) -> Result<Vec<TopologyMutation>, TopologyCertificationError> {
    let mut mutations = Vec::new();
    for (index, contract) in contracts.iter().enumerate() {
        let TopologyEditAction::RewireLoopSuccessor {
            relation_id,
            kind,
            half_edge_id,
            successor_half_edge_id,
        } = contract.action
        else {
            return Err(TopologyCertificationError::Query(
                "branch-local rewire projection expected only successor contracts".into(),
            ));
        };
        mutations.push(TopologyMutation::RemoveRelation { relation_id });
        mutations.push(TopologyMutation::CreateRelation {
            create_key: CreateKey::new(format!("{create_key_prefix}.rewire_{}", index + 1)),
            kind: RelationKind::Topology(kind.relation_kind()),
            source: EntityReference::Existing(half_edge_id),
            target: EntityReference::Existing(successor_half_edge_id),
        });
    }
    Ok(mutations)
}

pub(super) fn seeded_wire_and_half_edges(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(EntityId, Vec<EntityId>), TopologyCertificationError> {
    let read = read_snapshot(runtime, read_basis)?;
    let wire_id = read
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| TopologyCertificationError::Query("seeded wire missing".into()))?;
    Ok((wire_id, owned_half_edges(&read, wire_id)?))
}

pub(super) fn owned_half_edges(
    read: &RelationalReadView,
    wire_id: EntityId,
) -> Result<Vec<EntityId>, TopologyCertificationError> {
    let mut half_edge_ids = read
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    half_edge_ids.sort_by_key(|id| id.local_slot);
    Ok(half_edge_ids)
}

pub(super) fn owned_half_edge_relation_ids(
    read: &RelationalReadView,
    wire_id: EntityId,
    half_edge_ids: &[EntityId],
) -> Result<Vec<RelationId>, TopologyCertificationError> {
    half_edge_ids
        .iter()
        .map(|half_edge_id| {
            relation_id_by_shape(
                read,
                TopologyRelationKind::WireOwnsHalfEdge,
                wire_id,
                *half_edge_id,
            )
        })
        .collect()
}

pub(super) fn first_entity_id(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
    kind: TopologyEntityKind,
    description: &str,
) -> Result<EntityId, TopologyCertificationError> {
    read_snapshot(runtime, read_basis)?
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id) == Some(EntityKind::Topology(kind))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| TopologyCertificationError::Query(format!("{description} missing")))
}

pub(super) fn entity_id_by_label(
    read: &RelationalReadView,
    label: &str,
    kind: TopologyEntityKind,
) -> Result<EntityId, TopologyCertificationError> {
    read.entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id) == Some(EntityKind::Topology(kind))
                && record
                    .payload
                    .as_json()
                    .and_then(|json| json.get("label"))
                    .and_then(|value| value.as_str())
                    == Some(label)
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| TopologyCertificationError::Query(format!("entity `{label}` missing")))
}

pub(super) fn relation_id_by_shape(
    read: &RelationalReadView,
    kind: TopologyRelationKind,
    source: EntityId,
    target: EntityId,
) -> Result<RelationId, TopologyCertificationError> {
    read.relations()
        .iter()
        .find(|record| {
            RelationKind::from_kind_id(record.kind.kind_id) == Some(RelationKind::Topology(kind))
                && record.source == source
                && record.target == target
        })
        .map(|record| record.relation_id)
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "relation `{}` from `{:?}` to `{:?}` missing",
                kind.kind_name(),
                source,
                target
            ))
        })
}

pub(super) fn read_snapshot(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<RelationalReadView, TopologyCertificationError> {
    runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| TopologyCertificationError::Query("topology snapshot missing".into()))
}
