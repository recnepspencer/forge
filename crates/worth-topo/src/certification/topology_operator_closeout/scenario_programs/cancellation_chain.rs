use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::replay_step_rows::accepted_step_row_for_execution;
use super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeMutationReplayStepRow,
    MilestoneThreeScenarioMutationSynopsis,
};
use super::super::shared::{
    derived_validation_report_from_materialized, entity_id_from_query_identity,
    find_loop_id_by_label, relation_id_from_query_identity, replay_checked,
};
use super::scenario_mutation_report_lowering::{
    accepted_mutation_synopsis_from_step_rows, accepted_semantic_summary_from_step_rows,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::query_native_runtime_boundary::{row_text_at, TopologyNativeQueryRowField};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyCreateInnerLoopOnExistingFaceDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
};

struct MilestoneThreeCancellationRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    declared_mutation_synopsis: MilestoneThreeScenarioMutationSynopsis,
    accepted_semantic_summary:
        crate::certification::topology_operator_closeout::report::MilestoneThreeScenarioMutationSemanticSummary,
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    derived_validation_report: crate::validation::DerivedTopologyValidationReport,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
}

pub(crate) fn certify_milestone_three_cancellation_chain_parity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_cancellation_chain(&mut runtime_factory, stem)?;
    let replay = execute_cancellation_chain(&mut runtime_factory, stem)?;
    let replay_report = replay_checked(
        left.step_rows.clone(),
        replay.step_rows.clone(),
        left.baseline_materialized_topology_digest.clone(),
        left.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    );
    let returned_to_baseline = replay_report.returned_to_baseline.unwrap_or(false);
    let parity_status = replay_report.parity_status;

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::CancellationChainParity,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        declared_mutation_synopsis: left.declared_mutation_synopsis,
        semantic_summary: left.accepted_semantic_summary,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_mutation_scope_report: None,
        derived_validation_report: Some(left.derived_validation_report),
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        mutation_replay_parity_report: replay_report,
        detail: format!(
            "cancellation chain preserved accepted replay parity with status `{parity_status:?}` and returned_to_baseline={returned_to_baseline}"
        ),
    })
}

fn execute_cancellation_chain<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeCancellationRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified =
        seed_milestone_one_primitive_through_schema_execution(&mut runtime, stem, &primitive)?;
    let face_id = runtime
        .read_truth()
        .read_snapshot(verified.read_basis().snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded SheetDisk(n) snapshot should remain readable".to_string(),
            )
        })?
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == schema::facade::platform::entities::EntityKind::Topology(
                    TopologyEntityKind::Face,
                )
                .kind_id()
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded SheetDisk(n) should expose one face".to_string(),
            )
        })?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.runtime"))
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

    let loop_key = CreateKey::new(format!("{stem}.cancellation.inner_loop"));
    let create_inner_loop = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        loop_key.as_str(),
        format!("{stem}.cancellation.face-inner-loop"),
        face_id,
    );
    let execution_one = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        create_inner_loop.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_one = accepted_step_row_for_execution(0, &execution_one);
    let loop_id = find_loop_id_by_label(&execution_one.materialized(), loop_key.as_str())?;
    let inner_loop_relation_id =
        face_inner_loop_relation_id(&mut workspace, &surfaces, face_id, loop_id)?;

    let detach_inner_loop = TopologyDetachBoundaryMembershipDeclaration::new(
        inner_loop_relation_id,
        BoundaryMembershipKind::FaceInnerLoop,
    );
    let execution_two = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        detach_inner_loop.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_two = accepted_step_row_for_execution(1, &execution_two);

    let retire_loop =
        TopologyRetireTopologyEntityDeclaration::new(loop_id, TopologyEntityKind::Loop);
    let execution_three =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, retire_loop.clone())
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_three = accepted_step_row_for_execution(2, &execution_three);
    let step_rows = vec![step_one, step_two, step_three];
    let declared_mutation_synopsis = accepted_mutation_synopsis_from_step_rows(&step_rows);
    let accepted_semantic_summary =
        accepted_semantic_summary_from_step_rows(&step_rows, "accepted cancellation chain")?;
    let derived_validation_report =
        derived_validation_report_from_materialized(&execution_three.materialized())?;
    Ok(MilestoneThreeCancellationRun {
        primitive_family,
        primitive,
        declared_mutation_synopsis,
        accepted_semantic_summary,
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution_three.materialized(),
        ),
        derived_validation_report,
        derived_materialization_fallback_class: execution_three
            .materialized()
            .report()
            .fallback_class,
    })
}

fn face_inner_loop_relation_id(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    face_id: forge_relational::facade::identity::EntityId,
    loop_id: forge_relational::facade::identity::EntityId,
) -> Result<forge_relational::facade::identity::RelationId, TopologyCertificationError> {
    workspace
        .read(surfaces.relations())
        .iter()
        .find(|row| {
            row_text_at(row, TopologyNativeQueryRowField::TopologyKind.row_segments())
                == Some(schema::facade::platform::relations::TopologyRelationKind::FaceInnerLoop.kind_name())
                && row_text_at(
                    row,
                    TopologyNativeQueryRowField::TopologySourceIdentity.row_segments(),
                )
                    .and_then(|identity| entity_id_from_query_identity(identity).ok())
                    == Some(face_id)
                && row_text_at(
                    row,
                    TopologyNativeQueryRowField::TopologyTargetIdentity.row_segments(),
                )
                    .and_then(|identity| entity_id_from_query_identity(identity).ok())
                    == Some(loop_id)
        })
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "cancellation-chain topology rows did not expose the created face-inner-loop relation"
                    .to_string(),
            )
        })
        .and_then(|row| {
            relation_id_from_query_identity(row.identity())
                .map_err(|error| TopologyCertificationError::Query(error.to_string()))
        })
}
