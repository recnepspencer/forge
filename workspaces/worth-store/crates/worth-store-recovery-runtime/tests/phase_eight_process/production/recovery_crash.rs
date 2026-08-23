use worth_store::physical_runtime::PhysicalRecoveryYieldpointStage;

use super::harness::ProcessWorld;

#[path = "recovery_crash/cleanup_world.rs"]
mod cleanup_world;
#[path = "recovery_crash/interruption.rs"]
mod interruption;
#[path = "recovery_crash/seam.rs"]
mod seam;

const RECOVERY_SEAMS: [PhysicalRecoveryYieldpointStage; 11] = [
    PhysicalRecoveryYieldpointStage::StagingMaterialization,
    PhysicalRecoveryYieldpointStage::StagingSynchronization,
    PhysicalRecoveryYieldpointStage::CandidateMaterialization,
    PhysicalRecoveryYieldpointStage::CandidateSynchronization,
    PhysicalRecoveryYieldpointStage::RootProtocolReplacement,
    PhysicalRecoveryYieldpointStage::RecordNamespaceSynchronization,
    PhysicalRecoveryYieldpointStage::FreshReopenCurrentSelector,
    PhysicalRecoveryYieldpointStage::FreshReopenRootManifest,
    PhysicalRecoveryYieldpointStage::FreshReopenExactBinding,
    PhysicalRecoveryYieldpointStage::CleanupFreshnessRead,
    PhysicalRecoveryYieldpointStage::CleanupRemoval,
];
const RECOVERY_SEAM_OPERATION_COUNT: usize = 65;

#[test]
fn killed_recovery_process_reopens_after_each_named_c8_seam() {
    let publication_world = ProcessWorld::start_with_operation_count(
        "candidate-binding-record",
        0xC8_08_11,
        RECOVERY_SEAM_OPERATION_COUNT,
    );
    for (index, stage) in RECOVERY_SEAMS
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, stage)| !is_cleanup_stage(*stage))
    {
        seam::run(&publication_world, index, stage);
        publication_world.assert_within_budget("production/recovery seam case");
    }
    publication_world.finish_within_budget("production/recovery publication seams");

    let cleanup_world = ProcessWorld::start_cleanup_world_with_operation_count(
        0xC8_08_21,
        RECOVERY_SEAM_OPERATION_COUNT,
    );
    cleanup_world::require_raw_candidate(&cleanup_world);
    for (index, stage) in RECOVERY_SEAMS
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, stage)| is_cleanup_stage(*stage))
    {
        seam::run(&cleanup_world, index, stage);
        cleanup_world.assert_within_budget("production/recovery cleanup seam case");
    }
    cleanup_world.finish_within_budget("production/recovery cleanup seams");
}

#[test]
fn cancelled_recovery_process_reports_a_blocked_post_effect_outcome() {
    let publication_world = ProcessWorld::start_with_operation_count(
        "candidate-binding-record",
        0xC8_08_31,
        RECOVERY_SEAM_OPERATION_COUNT,
    );
    interruption::run_publication(&publication_world, 0);
    publication_world.finish_within_budget("production/cancelled publication proof");

    let cleanup_world = ProcessWorld::start_cleanup_world_with_operation_count(
        0xC8_08_32,
        RECOVERY_SEAM_OPERATION_COUNT,
    );
    cleanup_world::require_raw_candidate(&cleanup_world);
    interruption::run_cleanup(&cleanup_world, RECOVERY_SEAMS.len());
    cleanup_world.finish_within_budget("production/cancelled cleanup proof");
}

fn is_cleanup_stage(stage: PhysicalRecoveryYieldpointStage) -> bool {
    matches!(
        stage,
        PhysicalRecoveryYieldpointStage::CleanupFreshnessRead
            | PhysicalRecoveryYieldpointStage::CleanupRemoval
    )
}
