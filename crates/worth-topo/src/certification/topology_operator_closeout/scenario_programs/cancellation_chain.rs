use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::report::{
    MilestoneThreeEditReplayStepRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
};
use super::super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    derived_validation_report_from_materialized, find_loop_id_by_label, replay_checked,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::parse_relation_identity;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract,
};

struct MilestoneThreeCancellationRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    edit_families: Vec<crate::topology_operators::TopologyEditFamily>,
    topology_edit_digest: crate::topology_operators::TopologyEditDigest,
    naming_edit_continuity_matrix: crate::topology_operators::NamingEditContinuityMatrix,
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
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
        edit_families: left.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: left.topology_edit_digest,
        continuity_outcome_class: left.naming_edit_continuity_matrix.outcome_class(),
        continuity_rejection_class: left.naming_edit_continuity_matrix.rejection_class(),
        naming_edit_continuity_matrix: left.naming_edit_continuity_matrix,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        derived_validation_report: Some(left.derived_validation_report),
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        edit_replay_parity_report: replay_report,
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
        .read_snapshot(verified.read_basis.snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded SheetDisk(n) snapshot should remain readable".to_string(),
            )
        })?
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Face).kind_id()
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
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);

    let loop_key = CreateKey::new(format!("{stem}.cancellation.inner_loop"));
    let batch_one = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(loop_key.as_str(), TopologyEntityKind::Loop),
        TopologyEditContract::attach_boundary_membership(
            format!("{stem}.cancellation.face-inner-loop"),
            BoundaryMembershipKind::FaceInnerLoop,
            face_id,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("cancellation-chain first batch should be non-empty");
    let execution_one = assembly
        .apply_edit(
            &mut workspace,
            batch_one.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_one = accepted_step_row(0, &batch_one, &execution_one);
    let loop_id = find_loop_id_by_label(&execution_one.materialized, loop_key.as_str())?;
    let inner_loop_relation_id = created_relation_id(&execution_one.receipt)?;

    let batch_two = TopologyEditBatch::new(vec![TopologyEditContract::detach_boundary_membership(
        inner_loop_relation_id,
        BoundaryMembershipKind::FaceInnerLoop,
    )])
    .expect("cancellation-chain second batch should be non-empty");
    let execution_two = assembly
        .apply_edit(
            &mut workspace,
            batch_two.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_two = accepted_step_row(1, &batch_two, &execution_two);

    let batch_three = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
        loop_id,
        TopologyEntityKind::Loop,
    )])
    .expect("cancellation-chain third batch should be non-empty");
    let execution_three = assembly
        .apply_edit(
            &mut workspace,
            batch_three.clone(),
            TopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let step_three = accepted_step_row(2, &batch_three, &execution_three);

    let batches = vec![batch_one, batch_two, batch_three];
    let derived_validation_report =
        derived_validation_report_from_materialized(&execution_three.materialized)?;
    Ok(MilestoneThreeCancellationRun {
        primitive_family,
        primitive,
        edit_families: batches.iter().flat_map(|batch| batch.families()).collect(),
        topology_edit_digest: aggregate_topology_edit_digest(&batches),
        naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
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




