use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::{
    TopologyRehomeAllOwnedFacesToNewShellDeclaration, TopologyShellRehomeFaceMember,
};

pub(super) fn high_face_count_shell_rehome_declaration(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
    stem: &str,
    workload_size: usize,
) -> Result<TopologyRehomeAllOwnedFacesToNewShellDeclaration, TopologyCertificationError> {
    let (region_id, shell_id, face_ids) =
        seeded_solid_shell_membership(runtime, read_basis, workload_size)?;
    let shell_key = format!("{stem}.high_face_count_shell.rehome_shell.{workload_size}");
    Ok(TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        shell_key.clone(),
        format!("{shell_key}.region_owns_shell"),
        region_id,
        shell_id,
        face_ids
            .into_iter()
            .enumerate()
            .map(|(index, face_id)| {
                TopologyShellRehomeFaceMember::new(
                    format!("{shell_key}.shell_owns_face.{index:02}"),
                    face_id,
                )
            })
            .collect(),
    ))
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
