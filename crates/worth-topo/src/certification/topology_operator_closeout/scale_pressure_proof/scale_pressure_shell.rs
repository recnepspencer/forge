use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::created_ref;
use schema::facade::platform::authority::DerivedTopologyReadBasis;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{
    RelationKind, TopologyRelationKind,
};

use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditBatch, TopologyEditContract,
};

pub(super) fn high_face_count_shell_rehome_batch(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
    stem: &str,
    workload_size: usize,
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let (region_id, shell_id, face_ids) =
        seeded_solid_shell_membership(runtime, read_basis, workload_size)?;
    let shell_key = format!("{stem}.high_face_count_shell.rehome_shell.{workload_size}");
    let mut contracts = vec![
        TopologyEditContract::create_topology_entity(&shell_key, TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            format!("{shell_key}.region_owns_shell"),
            ShellOrWireMembershipKind::RegionOwnsShell,
            region_id,
            created_ref(&shell_key),
        ),
    ];
    contracts.extend(face_ids.iter().enumerate().map(|(index, face_id)| {
        TopologyEditContract::attach_shell_or_wire_membership(
            format!("{shell_key}.shell_owns_face.{index:02}"),
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(&shell_key),
            *face_id,
        )
    }));
    contracts.push(TopologyEditContract::retire_topology_entity(
        shell_id,
        TopologyEntityKind::Shell,
    ));
    TopologyEditBatch::new(contracts)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn seeded_solid_shell_membership(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
    expected_face_count: usize,
) -> Result<(EntityId, EntityId, Vec<EntityId>), TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            scale_pressure_shell_error("high-face shell seed snapshot should be readable")
        })?;
    let shell_id = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Shell))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            scale_pressure_shell_error("high-face shell seed should contain one shell")
        })?;
    let region_id = read_view
        .relations()
        .iter()
        .find(|record| {
            record.target == shell_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::RegionOwnsShell,
                    ))
        })
        .map(|record| record.source)
        .ok_or_else(|| scale_pressure_shell_error("high-face shell should have owning region"))?;
    let face_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == shell_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(TopologyRelationKind::ShellOwnsFace))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    if face_ids.len() != expected_face_count {
        return Err(scale_pressure_shell_error(&format!(
            "expected {expected_face_count} shell-owned faces but found {}",
            face_ids.len()
        )));
    }
    Ok((region_id, shell_id, face_ids))
}

fn scale_pressure_shell_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale shell failed: {reason}"))
}




