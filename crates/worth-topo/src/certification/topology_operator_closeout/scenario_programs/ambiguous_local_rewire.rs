use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::mutation_sequence_support::{
    aggregate_naming_mutation_continuity_matrix_for_declarations,
    aggregate_topology_mutation_digest_for_declarations,
    topology_mutation_families_for_declarations,
};
use super::super::report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeMutationReplayStepRow,
};
use super::super::shared::{
    accepted_step_row_for_declaration, derived_validation_report_from_materialized,
    first_source_identity_for_relation_kind, replay_checked,
};
use super::local_successor_rewire::successor_relocation_declaration;
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{TopologyMutationDigest, TopologyMutationFamily};

struct MilestoneThreeAmbiguousLocalRewireRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    mutation_families: Vec<TopologyMutationFamily>,
    topology_mutation_digest: TopologyMutationDigest,
    naming_mutation_continuity_matrix: crate::topology_operators::NamingMutationContinuityMatrix,
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    derived_validation_report: crate::validation::DerivedTopologyValidationReport,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
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
    let continuity_outcome_class = chosen.naming_mutation_continuity_matrix.outcome_class();
    let continuity_rejection_class = chosen.naming_mutation_continuity_matrix.rejection_class();

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
        primitive_family: chosen.primitive_family,
        primitive: chosen.primitive,
        mutation_families: chosen.mutation_families,
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
        topology_mutation_digest: chosen.topology_mutation_digest,
        naming_mutation_continuity_matrix: chosen.naming_mutation_continuity_matrix,
        continuity_outcome_class,
        continuity_rejection_class,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_mutation_scope_report: None,
        derived_validation_report: Some(chosen.derived_validation_report),
        derived_materialization_fallback_class: chosen.derived_materialization_fallback_class,
        mutation_replay_parity_report: replay_report,
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = surfaces
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let topology_read = TopologyReadProofHarness::new();
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let neighborhood = topology_read
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, 6)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let old_successor_identity = neighborhood.old_successor_identity.clone();
    let chosen_successor_identity = neighborhood
        .cycle_identities
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| cycle_query_error("requested successor candidate should exist in cycle"))?;
    let declaration = successor_relocation_declaration(&neighborhood, &chosen_successor_identity)?;
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration.clone())
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_rows = vec![accepted_step_row_for_declaration(
        0,
        &declaration,
        &execution,
    )];
    let derived_validation_report =
        derived_validation_report_from_materialized(&execution.materialized)?;
    Ok(MilestoneThreeAmbiguousLocalRewireRun {
        primitive_family,
        primitive,
        mutation_families: topology_mutation_families_for_declarations(vec![declaration.clone()]),
        topology_mutation_digest: aggregate_topology_mutation_digest_for_declarations(vec![
            declaration.clone(),
        ]),
        naming_mutation_continuity_matrix:
            aggregate_naming_mutation_continuity_matrix_for_declarations(vec![declaration.clone()]),
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution.materialized,
        ),
        derived_validation_report,
        derived_materialization_fallback_class: execution.materialized.report().fallback_class,
        moved_half_edge_identity,
        old_successor_identity,
        chosen_successor_identity,
    })
}

fn cycle_query_error(detail: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(detail.to_string())
}
