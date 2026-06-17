use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

use super::super::replay_step_rows::{
    accepted_step_row_for_execution, rejected_step_row_for_declaration,
};
use super::super::report::{
    MilestoneThreeBowtieAdjacentWitness, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeMutationReplayStepRow, MilestoneThreeScenarioMutationSynopsis,
};
use super::super::shared::{
    derived_validation_report_from_materialized, entity_id_from_query_identity,
    first_source_identity_for_relation_kind, relation_id_from_query_identity_label, replay_checked,
    replay_checked_rejected,
};
use super::scenario_mutation_report_lowering::{
    accepted_mutation_synopsis_from_step_rows, accepted_semantic_summary_from_step_rows,
    hostile_scenario_mutation_synopsis_from_declaration,
    hostile_scenario_semantic_summary_from_rejected_declaration,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{
    RejectedMutationScopeReport, TopologyMutationRejectionClass,
    TopologySpliceRadialAdjacencyDeclaration,
};

struct MilestoneThreeBowtieAdjacentRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    declaration: TopologySpliceRadialAdjacencyDeclaration,
    declared_mutation_synopsis: Option<MilestoneThreeScenarioMutationSynopsis>,
    accepted_semantic_summary: Option<
        crate::certification::topology_operator_closeout::report::MilestoneThreeScenarioMutationSemanticSummary,
    >,
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: Option<crate::certification::DeterministicDigest>,
    outcome_class: MilestoneThreeHostileOutcomeClass,
    rejection_class: Option<TopologyMutationRejectionClass>,
    rejected_mutation_scope_report: Option<RejectedMutationScopeReport>,
    derived_validation_report: Option<crate::validation::DerivedTopologyValidationReport>,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeBowtieAdjacentWitness,
    detail: String,
}

pub(crate) fn certify_milestone_three_bowtie_adjacent_rewire_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_bowtie_adjacent_rewire(&mut runtime_factory, stem)?;
    let replay = execute_bowtie_adjacent_rewire(&mut runtime_factory, stem)?;
    let replay_report = match (
        left.outcome_class,
        left.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    ) {
        (
            MilestoneThreeHostileOutcomeClass::Accepted,
            Some(final_materialized_topology_digest),
            Some(replay_final_materialized_topology_digest),
        ) => replay_checked(
            left.step_rows.clone(),
            replay.step_rows.clone(),
            left.baseline_materialized_topology_digest.clone(),
            final_materialized_topology_digest,
            replay_final_materialized_topology_digest,
        ),
        (MilestoneThreeHostileOutcomeClass::Rejected, _, _) => replay_checked_rejected(
            left.step_rows.clone(),
            replay.step_rows.clone(),
            left.baseline_materialized_topology_digest.clone(),
        ),
        _ => {
            return Err(TopologyCertificationError::Query(
                "bowtie adjacent replay should preserve an honest final digest basis".to_string(),
            ))
        }
    };

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::BowtieAdjacentRewire,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        declared_mutation_synopsis: left
            .declared_mutation_synopsis
            .as_ref()
            .cloned()
            .map_or_else(
                || hostile_scenario_mutation_synopsis_from_declaration(&left.declaration),
                core::convert::identity,
            ),
        semantic_summary: left
            .accepted_semantic_summary
            .as_ref()
            .cloned()
            .map_or_else(
                || {
                    hostile_scenario_semantic_summary_from_rejected_declaration(
                        &left.declaration,
                        left.rejection_class,
                    )
                },
                core::convert::identity,
            ),
        bowtie_adjacent_witness: Some(left.witness),
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        outcome_class: left.outcome_class,
        rejection_class: left.rejection_class,
        rejected_mutation_scope_report: left.rejected_mutation_scope_report,
        derived_validation_report: left.derived_validation_report,
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        mutation_replay_parity_report: replay_report,
        detail: left.detail,
    })
}

fn execute_bowtie_adjacent_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeBowtieAdjacentRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let _verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.bowtie_adjacent"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.bowtie_adjacent.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized =
        crate::certification::support::current_head_materialized_topology::current_head_materialized_topology(
            &mut workspace,
            &surfaces,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_materialized);
    let topology_read = TopologyReadProofHarness::current_head();
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeRadialNext,
    )?;
    let source_half_edge_id = entity_id_from_query_identity(&source_identity)?;
    let bowtie_adjacent_witness =
        build_bowtie_adjacent_witness(&topology_read, &mut workspace, &source_identity)?;
    let radial = topology_read
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id_from_query_identity_label(radial.source_radial_next_relation_identity())?,
        source_half_edge_id,
        entity_id_from_query_identity(bowtie_adjacent_witness.target_half_edge_identity.as_str())?,
    );

    match execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration.clone())
    {
        Ok(execution) => {
            let step_rows = vec![accepted_step_row_for_execution(0, &execution)];
            let accepted_semantic_summary = accepted_semantic_summary_from_step_rows(
                &step_rows,
                "accepted bowtie-adjacent rewire",
            )?;
            let final_materialized_topology_digest =
                digest_materialized_topology_view(&execution.materialized());
            let derived_validation_report =
                derived_validation_report_from_materialized(&execution.materialized())?;
            Ok(MilestoneThreeBowtieAdjacentRun {
                primitive_family,
                primitive,
                declaration,
                declared_mutation_synopsis: Some(accepted_mutation_synopsis_from_step_rows(
                    &step_rows,
                )),
                accepted_semantic_summary: Some(accepted_semantic_summary),
                step_rows,
                baseline_materialized_topology_digest,
                final_materialized_topology_digest: Some(final_materialized_topology_digest),
                outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_mutation_scope_report: None,
                derived_validation_report: Some(derived_validation_report),
                derived_materialization_fallback_class: execution
                    .materialized()
                    .report()
                    .fallback_class,
                witness: bowtie_adjacent_witness,
                detail:
                    "bowtie-adjacent rewire committed successfully on the admitted runtime lane"
                        .to_string(),
            })
        }
        Err(error) => Ok(MilestoneThreeBowtieAdjacentRun {
            primitive_family,
            primitive,
            declaration: declaration.clone(),
            declared_mutation_synopsis: None,
            accepted_semantic_summary: None,
            step_rows: vec![rejected_step_row_for_declaration(0, &declaration, &error)],
            baseline_materialized_topology_digest,
            final_materialized_topology_digest: None,
            outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
            rejection_class: error.rejection_class(),
            rejected_mutation_scope_report: error.rejected_declaration_scope_report(&declaration),
            derived_validation_report: None,
            derived_materialization_fallback_class: None,
            witness: bowtie_adjacent_witness,
            detail: error.to_string(),
        }),
    }
}

pub(in crate::certification::topology_operator_closeout) fn rejected_branch_local_bowtie_adjacent_declaration<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<TopologySpliceRadialAdjacencyDeclaration, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let run = execute_bowtie_adjacent_rewire(runtime_factory, stem)?;
    if run.outcome_class != MilestoneThreeHostileOutcomeClass::Rejected {
        return Err(TopologyCertificationError::Query(
            "bowtie-adjacent hostile declaration unexpectedly admitted while building rejected branch-local parity witness".to_string(),
        ));
    }
    Ok(run.declaration)
}

fn build_bowtie_adjacent_witness(
    topology_read: &TopologyReadProofHarness,
    workspace: &mut ForgeQueryWorkspace,
    source_identity: &str,
) -> Result<MilestoneThreeBowtieAdjacentWitness, TopologyCertificationError> {
    let neighborhood = topology_read
        .shared_vertex_half_edge_neighborhood(workspace, source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let target = neighborhood
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .find(|candidate| !candidate.shared_vertex_identities().is_empty())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should provide a vertex-adjacent halfedge on a different edge"
                    .to_string(),
            )
        })?;
    let shared_vertex_identity = target
        .shared_vertex_identities()
        .first()
        .cloned()
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should expose a shared vertex for bowtie-adjacent witness"
                    .to_string(),
            )
        })?;
    let target_half_edge_identity = target.half_edge_identity().to_string();
    let target_edge_identity = target.edge_identity().to_string();
    Ok(MilestoneThreeBowtieAdjacentWitness {
        source_half_edge_identity: source_identity.to_string(),
        target_half_edge_identity,
        source_edge_identity: neighborhood.source_edge_identity,
        target_edge_identity,
        shared_vertex_identity,
    })
}
