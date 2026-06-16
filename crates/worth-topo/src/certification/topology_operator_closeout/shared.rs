use crate::certification::error::TopologyCertificationError;
use crate::certification::topology_operator_closeout::report::{
    MilestoneThreeMutationReplayParityReport, MilestoneThreeMutationReplayStepRow,
};
use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::interpret_topology_view;
use crate::validation::{validate_interpreted_topology, DerivedTopologyValidationReport};
use forge_query::facade::{ForgeQueryEntity, ForgeQueryEntityIdentity};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;
use crate::projection::runtime_boundary::query_support::query_entity_identity_reporting_label;
use schema::facade::platform::relations::TopologyRelationKind;

pub(super) fn first_source_identity_for_relation_kind(
    relation_rows: &[ForgeQueryEntity],
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyCertificationError> {
    relation_rows
        .iter()
        .find_map(|row| {
            (row.external_row()
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                == Some(relation_kind.kind_name()))
            .then(|| {
                row.external_row()
                    .get("topology")
                    .and_then(|value| value.get("source_identity"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
            })
            .flatten()
        })
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "query rows should expose `{}` source identities",
                relation_kind.kind_name()
            ))
        })
}

pub(super) fn entity_id_from_query_identity(
    identity: &str,
) -> Result<EntityId, TopologyCertificationError> {
    let [partition_id, local_slot, generation] = query_identity_parts(identity, "entity")?;
    Ok(EntityId::new(
        PartitionId(partition_id as u32),
        local_slot,
        generation as u32,
    ))
}

pub(super) fn relation_id_from_query_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<RelationId, TopologyCertificationError> {
    let parts = identity.relational_record_parts().ok_or_else(|| {
        TopologyCertificationError::Query(format!(
            "expected relational relation query identity, got `{}`",
            query_entity_identity_reporting_label(identity)
        ))
    })?;
    if parts.kind() != RelationalBridgeRecordIdentityKind::Relation {
        return Err(TopologyCertificationError::Query(format!(
            "expected relation query identity, got `{}`",
            query_entity_identity_reporting_label(identity)
        )));
    }
    Ok(RelationId::new(
        PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}

pub(super) fn relation_id_from_query_identity_label(
    identity: &str,
) -> Result<RelationId, TopologyCertificationError> {
    let [partition_id, local_slot, generation] = query_identity_parts(identity, "relation")?;
    Ok(RelationId::new(
        PartitionId(partition_id as u32),
        local_slot,
        generation as u32,
    ))
}

fn query_identity_parts(
    identity: &str,
    expected_kind: &str,
) -> Result<[u64; 3], TopologyCertificationError> {
    let mut parts = identity.split(':');
    let kind = parts.next().unwrap_or_default();
    if kind != expected_kind {
        return Err(TopologyCertificationError::Query(format!(
            "expected `{expected_kind}` query identity, got `{identity}`"
        )));
    }
    let partition_id = parse_query_identity_part(parts.next(), identity, "partition")?;
    let local_slot = parse_query_identity_part(parts.next(), identity, "local slot")?;
    let generation = parse_query_identity_part(parts.next(), identity, "generation")?;
    if parts.next().is_some() {
        return Err(TopologyCertificationError::Query(format!(
            "query identity `{identity}` had too many fields"
        )));
    }
    Ok([partition_id, local_slot, generation])
}

fn parse_query_identity_part(
    part: Option<&str>,
    identity: &str,
    label: &str,
) -> Result<u64, TopologyCertificationError> {
    part.ok_or_else(|| {
        TopologyCertificationError::Query(format!("query identity `{identity}` is missing {label}"))
    })?
    .parse()
    .map_err(|_| {
        TopologyCertificationError::Query(format!(
            "query identity `{identity}` has invalid {label}"
        ))
    })
}

pub(super) fn find_loop_id_by_label(
    materialized: &MaterializedTopologyView,
    label: &str,
) -> Result<forge_relational::facade::identity::EntityId, TopologyCertificationError> {
    materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.label == label)
        .map(|loop_record| loop_record.entity_id)
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "materialized topology should expose loop `{label}`"
            ))
        })
}

pub(super) fn derived_validation_report_from_materialized(
    materialized: &MaterializedTopologyView,
) -> Result<DerivedTopologyValidationReport, TopologyCertificationError> {
    let interpreted = interpret_topology_view(materialized);
    validate_interpreted_topology(materialized, &interpreted)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

pub(super) fn replay_checked(
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    replay_step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: DeterministicDigest,
    final_materialized_topology_digest: DeterministicDigest,
    replay_final_materialized_topology_digest: DeterministicDigest,
) -> MilestoneThreeMutationReplayParityReport {
    let returned_to_baseline =
        final_materialized_topology_digest == baseline_materialized_topology_digest;
    let mut mismatch_count = 0usize;
    if step_rows != replay_step_rows {
        mismatch_count += 1;
    }
    if final_materialized_topology_digest != replay_final_materialized_topology_digest {
        mismatch_count += 1;
    }
    let parity_status = if mismatch_count == 0 {
        ReplayParityStatus::Match
    } else {
        ReplayParityStatus::Mismatch
    };
    MilestoneThreeMutationReplayParityReport {
        replay_checked: true,
        parity_status,
        mismatch_count,
        step_rows,
        replay_step_rows,
        baseline_materialized_topology_digest: Some(baseline_materialized_topology_digest),
        final_materialized_topology_digest: Some(final_materialized_topology_digest),
        replay_final_materialized_topology_digest: Some(replay_final_materialized_topology_digest),
        returned_to_baseline: Some(returned_to_baseline),
    }
}

pub(super) fn replay_checked_rejected(
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    replay_step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: DeterministicDigest,
) -> MilestoneThreeMutationReplayParityReport {
    let mismatch_count = usize::from(step_rows != replay_step_rows);
    let parity_status = if mismatch_count == 0 {
        ReplayParityStatus::Match
    } else {
        ReplayParityStatus::Mismatch
    };
    MilestoneThreeMutationReplayParityReport {
        replay_checked: true,
        parity_status,
        mismatch_count,
        step_rows,
        replay_step_rows,
        baseline_materialized_topology_digest: Some(baseline_materialized_topology_digest.clone()),
        final_materialized_topology_digest: Some(baseline_materialized_topology_digest.clone()),
        replay_final_materialized_topology_digest: Some(baseline_materialized_topology_digest),
        returned_to_baseline: Some(true),
    }
}
