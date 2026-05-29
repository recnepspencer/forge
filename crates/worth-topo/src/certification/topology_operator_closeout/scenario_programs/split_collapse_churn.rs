use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, DerivedTopologyReadBasis, MilestoneOnePrimitiveCase,
};

use super::super::report::{
    MilestoneThreeEditReplayStepRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeSplitCollapseChurnWitness,
};
use super::super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    derived_validation_report_from_materialized, replay_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditApplicationMode, TopologyEditBatch,
    TopologyEditContract, TopologyEditDigest, TopologyEditFamily,
};

struct MilestoneThreeSplitCollapseRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    edit_families: Vec<TopologyEditFamily>,
    topology_edit_digest: TopologyEditDigest,
    naming_edit_continuity_matrix: crate::topology_operators::NamingEditContinuityMatrix,
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    derived_validation_report: crate::validation::DerivedTopologyValidationReport,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeSplitCollapseChurnWitness,
}

pub(crate) fn certify_milestone_three_split_collapse_churn_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_split_collapse_churn(&mut runtime_factory, stem)?;
    let replay = execute_split_collapse_churn(&mut runtime_factory, stem)?;
    let replay_report = replay_checked(
        left.step_rows.clone(),
        replay.step_rows.clone(),
        left.baseline_materialized_topology_digest.clone(),
        left.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    );
    let continuity_outcome_class = left.naming_edit_continuity_matrix.outcome_class();
    let continuity_rejection_class = left.naming_edit_continuity_matrix.rejection_class();

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::SplitCollapseChurn,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        edit_families: left.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: Some(left.witness),
        broken_radial_witness: None,
        topology_edit_digest: left.topology_edit_digest,
        naming_edit_continuity_matrix: left.naming_edit_continuity_matrix,
        continuity_outcome_class,
        continuity_rejection_class,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        derived_validation_report: Some(left.derived_validation_report),
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        edit_replay_parity_report: replay_report,
        detail: format!(
            "connected wire split and owner-retiring collapse churn both executed through query-native membership programs with continuity `{continuity_outcome_class:?}`"
        ),
    })
}

fn execute_split_collapse_churn<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeSplitCollapseRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.split_collapse_churn"),
        &primitive,
    )?;
    let (original_wire_id, half_edge_ids) =
        seeded_wire_and_half_edges(&runtime, &verified.read_basis())?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.split_collapse_churn.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let original_wire_identity = wire_identity(
        &baseline_snapshot.materialized,
        original_wire_id,
        "original split-collapse wire",
    )?;
    let moved_half_edge_ids = vec![half_edge_ids[2], half_edge_ids[3]];
    let retained_half_edge_ids = vec![half_edge_ids[0], half_edge_ids[1]];
    let moved_half_edge_identities =
        half_edge_identities(&baseline_snapshot.materialized, &moved_half_edge_ids)?;
    let retained_half_edge_identities =
        half_edge_identities(&baseline_snapshot.materialized, &retained_half_edge_ids)?;

    let split_wire_key = CreateKey::new(&format!("{stem}.split_collapse_churn.split_wire"));
    let split_batch = split_wire_batch(split_wire_key.as_str(), &moved_half_edge_ids)?;
    let split_execution = assembly
        .apply_edit(
            &mut workspace,
            split_batch.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let split_wire = split_execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.label == split_wire_key.as_str())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse churn should materialize the split wire".to_string(),
            )
        })?;
    let split_wire_id = split_wire.entity_id;
    let split_wire_half_edge_ids = split_wire.half_edge_ids.clone();
    let split_step_wire_count = split_execution.materialized.topology().wires.len();

    let collapse_wire_key = CreateKey::new(&format!("{stem}.split_collapse_churn.collapse_wire"));
    let collapse_batch = collapse_split_wire_batch(
        collapse_wire_key.as_str(),
        split_wire_id,
        &split_wire_half_edge_ids,
    )?;
    let collapse_execution = assembly
        .apply_edit(
            &mut workspace,
            collapse_batch.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let final_materialized_topology_digest =
        digest_materialized_topology_view(&collapse_execution.materialized);
    let derived_validation_report =
        derived_validation_report_from_materialized(&collapse_execution.materialized)?;
    let batches = vec![split_batch.clone(), collapse_batch.clone()];
    let step_rows = vec![
        accepted_step_row(0, &split_batch, &split_execution),
        accepted_step_row(1, &collapse_batch, &collapse_execution),
    ];

    Ok(MilestoneThreeSplitCollapseRun {
        primitive_family,
        primitive,
        edit_families: batches.iter().flat_map(|batch| batch.families()).collect(),
        topology_edit_digest: aggregate_topology_edit_digest(&batches),
        naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest,
        derived_validation_report,
        derived_materialization_fallback_class: collapse_execution
            .materialized
            .report()
            .fallback_class,
        witness: MilestoneThreeSplitCollapseChurnWitness {
            original_wire_identity,
            split_wire_identity: split_wire_key.as_str().to_string(),
            collapse_wire_identity: collapse_wire_key.as_str().to_string(),
            moved_half_edge_identities,
            retained_half_edge_identities,
            split_step_wire_count,
            final_wire_count: collapse_execution.materialized.topology().wires.len(),
        },
    })
}

fn split_wire_batch(
    split_wire_key: &str,
    moved_half_edge_ids: &[EntityId],
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let mut contracts = vec![TopologyEditContract::create_topology_entity(
        split_wire_key,
        TopologyEntityKind::Wire,
    )];
    for (index, half_edge_id) in moved_half_edge_ids.iter().enumerate() {
        contracts.push(TopologyEditContract::attach_shell_or_wire_membership(
            &format!("{split_wire_key}.owns_half_edge_{}", index + 1),
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(split_wire_key),
            *half_edge_id,
        ));
    }
    TopologyEditBatch::new(contracts)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn collapse_split_wire_batch(
    collapse_wire_key: &str,
    retired_wire_id: EntityId,
    moved_half_edge_ids: &[EntityId],
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let mut contracts = vec![TopologyEditContract::create_topology_entity(
        collapse_wire_key,
        TopologyEntityKind::Wire,
    )];
    for (index, half_edge_id) in moved_half_edge_ids.iter().enumerate() {
        contracts.push(TopologyEditContract::attach_shell_or_wire_membership(
            &format!("{collapse_wire_key}.owns_half_edge_{}", index + 1),
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(collapse_wire_key),
            *half_edge_id,
        ));
    }
    contracts.push(TopologyEditContract::retire_topology_entity(
        retired_wire_id,
        TopologyEntityKind::Wire,
    ));
    TopologyEditBatch::new(contracts)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn seeded_wire_and_half_edges(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(EntityId, Vec<EntityId>), TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse seeded snapshot should remain readable".to_string(),
            )
        })?;
    let wire = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse primitive should contain one wire".to_string(),
            )
        })?;
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    Ok((wire, half_edge_ids))
}

fn wire_identity(
    materialized: &crate::derived_topology::materialized_graph::MaterializedTopologyView,
    wire_id: EntityId,
    description: &str,
) -> Result<String, TopologyCertificationError> {
    materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.entity_id == wire_id)
        .map(|wire| wire.label.clone())
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "materialized topology should expose {description}"
            ))
        })
}

fn half_edge_identities(
    materialized: &crate::derived_topology::materialized_graph::MaterializedTopologyView,
    half_edge_ids: &[EntityId],
) -> Result<Vec<String>, TopologyCertificationError> {
    half_edge_ids
        .iter()
        .map(|half_edge_id| {
            materialized
                .topology()
                .half_edges
                .iter()
                .find(|half_edge| half_edge.entity_id == *half_edge_id)
                .map(|half_edge| half_edge.label.clone())
                .ok_or_else(|| {
                    TopologyCertificationError::Query(format!(
                        "materialized topology should expose half-edge `{:?}`",
                        half_edge_id
                    ))
                })
        })
        .collect()
}
