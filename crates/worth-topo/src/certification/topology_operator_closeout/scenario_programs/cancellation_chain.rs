use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::TopologyEntityKind;
<<<<<<< HEAD
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
=======
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
>>>>>>> origin/master

use super::super::mutation_sequence_support::{
    aggregate_naming_mutation_continuity_matrix_for_declarations,
    aggregate_topology_mutation_digest_for_declarations,
    topology_mutation_families_for_declarations, TopologyCloseoutDeclaration,
};
use super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeMutationReplayStepRow,
};
use super::super::shared::{
    accepted_step_row_for_declaration, derived_validation_report_from_materialized,
    find_loop_id_by_label, replay_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::parse_relation_identity;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyCreateInnerLoopOnExistingFaceDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
};

struct MilestoneThreeCancellationRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    mutation_families: Vec<crate::topology_operators::TopologyMutationFamily>,
    topology_mutation_digest: crate::topology_operators::TopologyMutationDigest,
    naming_mutation_continuity_matrix: crate::topology_operators::NamingMutationContinuityMatrix,
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
        mutation_families: left.mutation_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_mutation_digest: left.topology_mutation_digest,
        continuity_outcome_class: left.naming_mutation_continuity_matrix.outcome_class(),
        continuity_rejection_class: left.naming_mutation_continuity_matrix.rejection_class(),
        naming_mutation_continuity_matrix: left.naming_mutation_continuity_matrix,
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
    let verified = seed_milestone_one_primitive(&mut runtime, stem, &primitive)?;
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
    let baseline_snapshot = surfaces
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);

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
    let step_one = accepted_step_row_for_declaration(0, &create_inner_loop, &execution_one);
    let loop_id = find_loop_id_by_label(&execution_one.materialized, loop_key.as_str())?;
    let inner_loop_relation_id = created_relation_id(&execution_one.receipt)?;

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
    let step_two = accepted_step_row_for_declaration(1, &detach_inner_loop, &execution_two);

    let retire_loop =
        TopologyRetireTopologyEntityDeclaration::new(loop_id, TopologyEntityKind::Loop);
    let execution_three =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, retire_loop.clone())
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_three = accepted_step_row_for_declaration(2, &retire_loop, &execution_three);

    let declarations = vec![
        TopologyCloseoutDeclaration::CreateInnerLoopOnExistingFace(create_inner_loop),
        TopologyCloseoutDeclaration::DetachBoundaryMembership(detach_inner_loop),
        TopologyCloseoutDeclaration::RetireTopologyEntity(retire_loop),
    ];
    let derived_validation_report =
        derived_validation_report_from_materialized(&execution_three.materialized)?;
    Ok(MilestoneThreeCancellationRun {
        primitive_family,
        primitive,
        mutation_families: topology_mutation_families_for_declarations(declarations.clone()),
        topology_mutation_digest: aggregate_topology_mutation_digest_for_declarations(
            declarations.clone(),
        ),
        naming_mutation_continuity_matrix:
            aggregate_naming_mutation_continuity_matrix_for_declarations(declarations),
        step_rows: vec![step_one, step_two, step_three],
        baseline_materialized_topology_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution_three.materialized,
        ),
        derived_validation_report,
        derived_materialization_fallback_class: execution_three
            .materialized
            .report()
            .fallback_class,
    })
}

fn created_relation_id(
    receipt: &forge_query::facade::ForgeQueryBatchWriteReceipt,
) -> Result<forge_relational::facade::identity::RelationId, TopologyCertificationError> {
    receipt
        .write_receipts()
        .iter()
        .flat_map(|write_receipt| write_receipt.deltas())
        .find(|delta| {
            delta.collection == "TopologyRelation"
                && delta.kind == forge_query::facade::ForgeQueryMutationKind::Created
        })
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "cancellation-chain relation-create receipt did not expose a created topology relation"
                    .to_string(),
            )
        })
        .and_then(|delta| {
            parse_relation_identity(delta.entity_identity.as_str())
                .map_err(|error| TopologyCertificationError::Query(error.to_string()))
        })
}
