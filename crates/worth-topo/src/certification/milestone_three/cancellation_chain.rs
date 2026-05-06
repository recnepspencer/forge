use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::{WorthCreateKey, WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::report::{
    WorthMilestoneThreeEditReplayStepRow, WorthMilestoneThreeHostileOutcomeClass,
    WorthMilestoneThreeHostileScenario, WorthMilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    find_loop_id_by_label, replay_checked,
};
use crate::certification::error::WorthTopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract,
};
use crate::parity::digest_materialized_topology_view;
use crate::query::{
    worth_topology_runtime, WorthTopologyDomainQuery, WorthTopologyQueryAssembly,
    WorthTopologyRuntimeAdapters,
};

struct WorthMilestoneThreeCancellationRun {
    primitive_family: String,
    primitive: WorthMilestoneOnePrimitiveCase,
    edit_families: Vec<crate::edit::WorthTopologyEditFamily>,
    topology_edit_digest: crate::edit::WorthTopologyEditDigest,
    naming_edit_continuity_matrix: crate::edit::WorthNamingEditContinuityMatrix,
    step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::WorthDeterministicDigest,
    final_materialized_topology_digest: crate::certification::WorthDeterministicDigest,
}

pub(crate) fn certify_milestone_three_cancellation_chain_parity_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneThreeHostileScenarioReport, WorthTopologyCertificationError>
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

    Ok(WorthMilestoneThreeHostileScenarioReport {
        scenario: WorthMilestoneThreeHostileScenario::CancellationChainParity,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        edit_families: left.edit_families,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: left.topology_edit_digest,
        continuity_outcome_class: left.naming_edit_continuity_matrix.outcome_class(),
        continuity_rejection_class: left.naming_edit_continuity_matrix.rejection_class(),
        naming_edit_continuity_matrix: left.naming_edit_continuity_matrix,
        outcome_class: WorthMilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        edit_replay_parity_report: replay_report,
        detail: format!(
            "cancellation chain preserved accepted replay parity with status `{parity_status:?}` and returned_to_baseline={returned_to_baseline}"
        ),
    })
}

fn execute_cancellation_chain<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<WorthMilestoneThreeCancellationRun, WorthTopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(&mut runtime, stem, &primitive)?;
    let face_id = runtime
        .read_truth()
        .read_snapshot(verified.read_basis.snapshot())
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(
                "seeded SheetDisk(n) snapshot should remain readable".to_string(),
            )
        })?
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == worth_schema::facade::WorthEntityKind::Topology(WorthTopologyEntityKind::Face)
                    .kind_id()
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(
                "seeded SheetDisk(n) should expose one face".to_string(),
            )
        })?;
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, &format!("{stem}.runtime"))
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);

    let loop_key = WorthCreateKey::new(format!("{stem}.cancellation.inner_loop"));
    let batch_one = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            loop_key.as_str(),
            WorthTopologyEntityKind::Loop,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            format!("{stem}.cancellation.face-inner-loop"),
            WorthBoundaryMembershipKind::FaceInnerLoop,
            face_id,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("cancellation-chain first batch should be non-empty");
    let execution_one = assembly
        .apply_edit(
            &mut workspace,
            batch_one.clone(),
            WorthTopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let step_one = accepted_step_row(0, &batch_one, &execution_one);
    let loop_id = find_loop_id_by_label(&execution_one.materialized, loop_key.as_str())?;
    let domain_query = WorthTopologyDomainQuery::load(&workspace, &assembly)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let face_identity = domain_query
        .find_entity_identity_by_id(face_id)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let loop_identity = domain_query
        .find_entity_identity_by_id(loop_id)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let inner_loop_relation_id = domain_query
        .relation_id_by_kind_and_endpoints(
            face_identity.as_str(),
            loop_identity.as_str(),
            WorthTopologyRelationKind::FaceInnerLoop,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;

    let batch_two =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            inner_loop_relation_id,
            WorthBoundaryMembershipKind::FaceInnerLoop,
        )])
        .expect("cancellation-chain second batch should be non-empty");
    let execution_two = assembly
        .apply_edit(
            &mut workspace,
            batch_two.clone(),
            WorthTopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let step_two = accepted_step_row(1, &batch_two, &execution_two);

    let batch_three =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
            loop_id,
            WorthTopologyEntityKind::Loop,
        )])
        .expect("cancellation-chain third batch should be non-empty");
    let execution_three = assembly
        .apply_edit(
            &mut workspace,
            batch_three.clone(),
            WorthTopologyEditApplicationMode::Mainline,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let step_three = accepted_step_row(2, &batch_three, &execution_three);

    let batches = vec![batch_one, batch_two, batch_three];
    Ok(WorthMilestoneThreeCancellationRun {
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
    })
}
