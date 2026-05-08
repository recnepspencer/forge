use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::TopologyRelationKind;
use serde_json::Value;

use super::local_successor_rewire::successor_relocation_batch;
use super::report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeEditReplayStepRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    derived_validation_report_from_materialized, first_source_identity_for_relation_kind,
    replay_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::TopologyDomainQuery;
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditDigest, TopologyEditFamily,
};

struct MilestoneThreeAmbiguousLocalRewireRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    edit_families: Vec<TopologyEditFamily>,
    topology_edit_digest: TopologyEditDigest,
    naming_edit_continuity_matrix: crate::topology_operators::NamingEditContinuityMatrix,
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    derived_validation_report: crate::validation::DerivedTopologyValidationReport,
    moved_half_edge_identity: String,
    old_successor_identity: String,
    chosen_successor_identity: String,
}

pub(crate) fn certify_milestone_three_ambiguous_local_rewire_continuity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let chosen = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 3)?;
    let alternate = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 4)?;
    let replay = execute_ambiguous_local_rewire(&mut runtime_factory, stem, 3)?;
    let replay_report = replay_checked(
        chosen.step_rows.clone(),
        replay.step_rows.clone(),
        chosen.baseline_materialized_topology_digest.clone(),
        chosen.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    );
    let continuity_outcome_class = chosen.naming_edit_continuity_matrix.outcome_class();
    let continuity_rejection_class = chosen.naming_edit_continuity_matrix.rejection_class();

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
        primitive_family: chosen.primitive_family,
        primitive: chosen.primitive,
        edit_families: chosen.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: Some(MilestoneThreeAmbiguousLocalRewireWitness {
            moved_half_edge_identity: chosen.moved_half_edge_identity.clone(),
            alternate_moved_half_edge_identity: alternate.moved_half_edge_identity.clone(),
            old_successor_identity: chosen.old_successor_identity.clone(),
            alternate_old_successor_identity: alternate.old_successor_identity.clone(),
            chosen_successor_identity: chosen.chosen_successor_identity.clone(),
            alternate_successor_identity: alternate.chosen_successor_identity.clone(),
            chosen_materialized_topology_digest: chosen.final_materialized_topology_digest.clone(),
            alternate_materialized_topology_digest: alternate
                .final_materialized_topology_digest
                .clone(),
        }),
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: chosen.topology_edit_digest,
        naming_edit_continuity_matrix: chosen.naming_edit_continuity_matrix,
        continuity_outcome_class,
        continuity_rejection_class,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        derived_validation_report: Some(chosen.derived_validation_report),
        edit_replay_parity_report: replay_report,
        detail: format!(
            "local successor rewire remained admitted while continuity stayed `{continuity_outcome_class:?}` because more than one accepted successor placement exists from the same basis"
        ),
    })
}

fn execute_ambiguous_local_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
    candidate_offset: usize,
) -> Result<MilestoneThreeAmbiguousLocalRewireRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        &format!("{stem}.ambiguous_local_rewire.{candidate_offset}.runtime"),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let domain_query = TopologyDomainQuery::load();
    let relation_rows = workspace.read::<Value>(assembly.relations());
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let neighborhood = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let old_successor_identity = neighborhood.old_successor_identity.clone();
    let chosen_successor_identity = neighborhood
        .cycle_identities
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| cycle_query_error("requested successor candidate should exist in cycle"))?;
    let batch = successor_relocation_batch(&neighborhood, &chosen_successor_identity)?;
    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_rows = vec![accepted_step_row(0, &batch, &execution)];
    let derived_validation_report =
        derived_validation_report_from_materialized(&execution.materialized)?;
    let batches = vec![batch];

    Ok(MilestoneThreeAmbiguousLocalRewireRun {
        primitive_family,
        primitive,
        edit_families: batches.iter().flat_map(|batch| batch.families()).collect(),
        topology_edit_digest: aggregate_topology_edit_digest(&batches),
        naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution.materialized,
        ),
        derived_validation_report,
        moved_half_edge_identity,
        old_successor_identity,
        chosen_successor_identity,
    })
}

fn cycle_query_error(detail: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(detail.to_string())
}
