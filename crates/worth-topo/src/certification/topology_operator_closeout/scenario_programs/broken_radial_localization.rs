use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

use super::super::replay_step_rows::{
    accepted_step_row_for_execution, rejected_step_row_for_declaration,
};
use super::super::report::{
    MilestoneThreeBrokenRadialWitness, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeScenarioMutationSynopsis,
};
use super::super::shared::{
    derived_validation_report_from_materialized, entity_id_from_query_identity,
    first_source_identity_for_relation_kind, relation_id_from_query_identity, replay_checked,
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
use crate::query_domain::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{
    TopologyMutationRejectionClass, TopologySpliceRadialAdjacencyDeclaration,
};

struct MilestoneThreeBrokenRadialRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    declaration: TopologySpliceRadialAdjacencyDeclaration,
    declared_mutation_synopsis: Option<MilestoneThreeScenarioMutationSynopsis>,
    accepted_semantic_summary: Option<
        crate::certification::topology_operator_closeout::report::MilestoneThreeScenarioMutationSemanticSummary,
    >,
    step_rows: Vec<super::super::report::MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: Option<crate::certification::DeterministicDigest>,
    outcome_class: MilestoneThreeHostileOutcomeClass,
    rejection_class: Option<TopologyMutationRejectionClass>,
    rejected_mutation_scope_report: Option<crate::topology_operators::RejectedMutationScopeReport>,
    derived_validation_report: Option<crate::validation::DerivedTopologyValidationReport>,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeBrokenRadialWitness,
    detail: String,
}

pub(crate) fn certify_milestone_three_broken_radial_localization_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_broken_radial_localization(&mut runtime_factory, stem)?;
    let replay = execute_broken_radial_localization(&mut runtime_factory, stem)?;
    let replay_report =
        match (
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
            _ => return Err(TopologyCertificationError::Query(
                "broken radial localization replay should preserve an honest final digest basis"
                    .to_string(),
            )),
        };

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::BrokenRadialLocalization,
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
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: Some(left.witness),
        outcome_class: left.outcome_class,
        rejection_class: left.rejection_class,
        rejected_mutation_scope_report: left.rejected_mutation_scope_report,
        derived_validation_report: left.derived_validation_report,
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        mutation_replay_parity_report: replay_report,
        detail: left.detail,
    })
}

fn execute_broken_radial_localization<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeBrokenRadialRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let _verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.broken_radial_localization"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        &format!("{stem}.broken_radial_localization.runtime"),
    )
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
    let radial = topology_read
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let shared_vertex = topology_read
        .shared_vertex_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let current_target_identity = radial.current_target_half_edge_identity().to_string();
    let source_radial_next_relation_identity =
        radial.source_radial_next_relation_identity().to_string();
    let witness = build_broken_radial_witness(radial, shared_vertex)?;
    let illegal_target_half_edge_id =
        entity_id_from_query_identity(&witness.illegal_target_half_edge_identity)?;
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id_from_query_identity(&source_radial_next_relation_identity)?,
        source_half_edge_id,
        illegal_target_half_edge_id,
    );

    match execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration.clone())
    {
        Ok(execution) => {
            let step_rows = vec![accepted_step_row_for_execution(0, &execution)];
            let declared_mutation_synopsis = accepted_mutation_synopsis_from_step_rows(&step_rows);
            let accepted_semantic_summary = accepted_semantic_summary_from_step_rows(
                &step_rows,
                "accepted broken-radial localization",
            )?;
            let detail = format!(
                "radial splice from `{source_identity}` to illegal target `{}` unexpectedly admitted from current target `{current_target_identity}`",
                witness.illegal_target_half_edge_identity
            );
            let derived_validation_report =
                derived_validation_report_from_materialized(&execution.materialized())?;
            Ok(MilestoneThreeBrokenRadialRun {
                primitive_family,
                primitive,
                declaration,
                declared_mutation_synopsis: Some(declared_mutation_synopsis),
                accepted_semantic_summary: Some(accepted_semantic_summary),
                step_rows,
                baseline_materialized_topology_digest,
                final_materialized_topology_digest: Some(digest_materialized_topology_view(
                    &execution.materialized(),
                )),
                outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_mutation_scope_report: None,
                derived_validation_report: Some(derived_validation_report),
                derived_materialization_fallback_class: execution
                    .materialized()
                    .report()
                    .fallback_class,
                witness,
                detail,
            })
        }
        Err(error) => Ok(MilestoneThreeBrokenRadialRun {
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
            witness,
            detail: error.to_string(),
        }),
    }
}

pub(in crate::certification::topology_operator_closeout) fn rejected_branch_local_broken_radial_declaration<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<TopologySpliceRadialAdjacencyDeclaration, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let run = execute_broken_radial_localization(runtime_factory, stem)?;
    if run.outcome_class != MilestoneThreeHostileOutcomeClass::Rejected {
        return Err(TopologyCertificationError::Query(
            "broken-radial hostile declaration unexpectedly admitted while building rejected branch-local parity witness".to_string(),
        ));
    }
    Ok(run.declaration)
}

fn build_broken_radial_witness(
    radial: TopologyHalfEdgeRadialNeighborhoodView,
    shared_vertex: TopologyHalfEdgeSharedVertexNeighborhoodView,
) -> Result<MilestoneThreeBrokenRadialWitness, TopologyCertificationError> {
    let illegal_target = shared_vertex
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .find(|candidate| {
            candidate.half_edge_identity() != radial.current_target_half_edge_identity()
        })
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should expose an illegal radial target on a different edge"
                    .to_string(),
            )
        })?;
    let illegal_target_half_edge_identity = illegal_target.half_edge_identity().to_string();
    let illegal_target_edge_identity = illegal_target.edge_identity().to_string();
    Ok(MilestoneThreeBrokenRadialWitness {
        source_half_edge_identity: radial.source_half_edge_identity().to_string(),
        current_target_half_edge_identity: radial.current_target_half_edge_identity,
        illegal_target_half_edge_identity,
        source_edge_identity: radial.source_edge_identity,
        current_target_edge_identity: radial.current_target_edge_identity,
        illegal_target_edge_identity,
    })
}
