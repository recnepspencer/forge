use crate::certification::error::TopologyCertificationError;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::topology_operator_closeout::report::{
    MilestoneThreeEditReplayParityReport, MilestoneThreeEditReplayStepRow,
    MilestoneThreeHostileOutcomeClass,
};
use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::interpret_topology_view;
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditBatch, TopologyEditContract, TopologyEditNamingOutcome,
    TopologyOperatorDigest, TopologyOperatorExecution, TopologyOperatorExecutionError,
};
use crate::validation::{validate_interpreted_topology, DerivedTopologyValidationReport};
use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::TopologyRelationKind;

pub(super) fn first_source_identity_for_relation_kind(
    relation_rows: &[ForgeQueryEntity],
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyCertificationError> {
    relation_rows
        .iter()
        .find_map(|row| {
            (row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                == Some(relation_kind.kind_name()))
            .then(|| {
                row.payload
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

pub(super) fn aggregate_topology_edit_digest(
    batches: &[TopologyEditBatch],
) -> DeterministicDigestBackedEditDigest {
    let rows = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter().map(contract_digest_row));
    let contract_count = batches.iter().map(|batch| batch.contracts().len()).sum();
    let family_count = batches.iter().map(|batch| batch.families().len()).sum();
    let changed_scope_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.changed_scopes().len())
        .sum();
    let naming_scope_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.naming_scopes().len())
        .sum();
    let derived_region_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.derived_regions().len())
        .sum();
    DeterministicDigestBackedEditDigest {
        digest: digest_rows(rows),
        contract_count,
        family_count,
        changed_scope_count,
        naming_scope_count,
        derived_region_count,
    }
}

pub(super) type DeterministicDigestBackedEditDigest = crate::topology_operators::TopologyEditDigest;

pub(super) fn aggregate_naming_edit_continuity_matrix(
    batches: &[TopologyEditBatch],
) -> NamingEditContinuityMatrix {
    let rows = batches
        .iter()
        .flat_map(|batch| batch.naming_edit_continuity_matrix().rows.into_iter())
        .collect::<Vec<_>>();
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Rejected)
        .count();
    NamingEditContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

pub(super) fn accepted_step_row(
    step_index: usize,
    batch: &TopologyEditBatch,
    execution: &TopologyOperatorExecution,
) -> MilestoneThreeEditReplayStepRow {
    MilestoneThreeEditReplayStepRow {
        step_index,
        edit_families: batch.families(),
        topology_edit_digest: execution.topology_edit_digest.clone(),
        naming_edit_continuity_matrix: execution.naming_continuity_matrix.clone(),
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        resulting_materialized_topology_digest: Some(digest_materialized_topology_view(
            &execution.materialized,
        )),
    }
}

pub(super) fn derived_validation_report_from_materialized(
    materialized: &MaterializedTopologyView,
) -> Result<DerivedTopologyValidationReport, TopologyCertificationError> {
    let interpreted = interpret_topology_view(materialized);
    validate_interpreted_topology(materialized, &interpreted)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

pub(super) fn rejected_step_row(
    step_index: usize,
    batch: &TopologyEditBatch,
    error: &TopologyOperatorExecutionError,
) -> MilestoneThreeEditReplayStepRow {
    MilestoneThreeEditReplayStepRow {
        step_index,
        edit_families: batch.families(),
        topology_edit_digest: batch.topology_edit_digest(),
        naming_edit_continuity_matrix: batch.naming_edit_continuity_matrix(),
        outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
        rejection_class: error.rejection_class(),
        resulting_materialized_topology_digest: None,
    }
}

pub(super) fn replay_checked(
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    replay_step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: DeterministicDigest,
    final_materialized_topology_digest: DeterministicDigest,
    replay_final_materialized_topology_digest: DeterministicDigest,
) -> MilestoneThreeEditReplayParityReport {
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
    MilestoneThreeEditReplayParityReport {
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
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    replay_step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: DeterministicDigest,
) -> MilestoneThreeEditReplayParityReport {
    let mismatch_count = usize::from(step_rows != replay_step_rows);
    let parity_status = if mismatch_count == 0 {
        ReplayParityStatus::Match
    } else {
        ReplayParityStatus::Mismatch
    };
    MilestoneThreeEditReplayParityReport {
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

fn contract_digest_row(contract: &TopologyEditContract) -> String {
    serde_json::to_string(contract).expect(" topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyOperatorDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyOperatorDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
