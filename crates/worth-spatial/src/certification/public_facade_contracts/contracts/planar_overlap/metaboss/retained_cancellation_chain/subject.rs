use worth_kernel::workload_composition::WorkloadCatalog;
use worth_spatial::facade::retained_cancellation_chain::{
    RetainedCancellationChainIntegrity, RetainedCancellationChainPredicate,
    RetainedCancellationChainReceipt, RetainedCancellationChainReplayPolicy,
    RetainedCancellationChainTransformPosture, RetainedCancellationChainWorkload,
    RetainedCancellationCheckpoint, RetainedCancellationCheckpointTrigger, RetainedReplaySampling,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;
use worth_spatial::facade::transform_workload::TransformReceiptSet;
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};

pub(crate) struct RetainedCancellationPlatformSubject {
    pub(crate) receipt: RetainedCancellationChainReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
    pub(crate) catalog_retained_artifact_count: usize,
    pub(crate) catalog_replay_checkpoint_count: usize,
    pub(crate) catalog_transform_stage_identity: String,
    pub(crate) catalog_retained_replay_stage_identity: String,
}

struct RetainedCancellationCatalogEvidence {
    ledger: CompleteWorkloadEvidenceLedger,
    transform_receipts: TransformReceiptSet,
    replay_receipts: ReplayReceiptSet,
    catalog_retained_artifact_count: usize,
    catalog_replay_checkpoint_count: usize,
}

pub(crate) fn certify_retained_cancellation_chain(
    world: &'static str,
) -> RetainedCancellationPlatformSubject {
    certify_retained_cancellation_chain_with_checkpoints(world, 32)
}

pub(crate) fn certify_retained_cancellation_chain_with_checkpoints(
    world: &'static str,
    checkpoint_count: usize,
) -> RetainedCancellationPlatformSubject {
    let catalog_evidence = retained_catalog_evidence(world);
    let receipt = retained_chain_workload(world, &catalog_evidence, checkpoint_count)
        .certify()
        .expect("retained cancellation chain should certify");
    let user_outcome = user_outcome_from_retained_chain_receipt(&receipt);

    RetainedCancellationPlatformSubject {
        receipt,
        user_outcome,
        catalog_retained_artifact_count: catalog_evidence.catalog_retained_artifact_count,
        catalog_replay_checkpoint_count: catalog_evidence.catalog_replay_checkpoint_count,
        catalog_transform_stage_identity: catalog_evidence
            .transform_receipts
            .stage_identity()
            .receipt_identity()
            .to_string(),
        catalog_retained_replay_stage_identity: catalog_evidence
            .replay_receipts
            .stage_identity()
            .receipt_identity()
            .to_string(),
    }
}

pub(crate) fn retained_cancellation_outcome_matrix(world: &'static str) -> Vec<WorthUserOutcome> {
    vec![
        policy_required_outcome(world),
        predicate_uncertain_outcome(world),
        retained_replay_mismatch_outcome(world),
        transform_invalidation_outcome(world),
        projection_consumed_mismatch_outcome(world),
    ]
}

pub(crate) fn retained_replay_mismatch_outcome(world: &'static str) -> WorthUserOutcome {
    retained_chain_error_outcome(world, |workload| {
        workload.requiring_integrity(RetainedCancellationChainIntegrity::RetainedReplayMismatch {
            step_index: 17,
        })
    })
}

pub(crate) fn projection_consumed_mismatch_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    checkpoints[25] = checkpoints[25]
        .clone()
        .with_projection_consumed_identity("projection consumed facts from another retained basis");
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 projection mismatch {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("projection-consumed mismatch must deny");
    user_outcome_from_retained_chain_error(error)
}

pub(crate) fn live_extraction_denial_outcome(world: &'static str) -> WorthUserOutcome {
    retained_chain_error_outcome(world, |workload| {
        workload
            .requiring_replay_policy(RetainedCancellationChainReplayPolicy::LiveExtractionRequested)
    })
}

pub(crate) fn duplicate_checkpoint_denial_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    checkpoints[7] = checkpoints[0].clone();
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 duplicate checkpoint {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("duplicate retained checkpoint evidence must deny");
    user_outcome_from_retained_chain_error(error)
}

pub(crate) fn missing_trigger_local_replay_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    checkpoints[9] = checkpoints[9]
        .clone()
        .with_trigger(RetainedCancellationCheckpointTrigger::PredicateUncertain);
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 missing trigger-local replay {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("unsampled trigger checkpoint must deny before outcome classification");
    user_outcome_from_retained_chain_error(error)
}

pub(crate) fn projection_consumed_forgery_denial_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    let forged_projection_identity = format!(
        "projection-consumed-checkpoint:checkpoint=25:replay-evidence:forged:{}",
        checkpoints[25].retained_basis_identity()
    );
    checkpoints[25] = checkpoints[25]
        .clone()
        .with_projection_consumed_identity(forged_projection_identity);
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 forged projection identity {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("projection evidence shaped like retained replay must still deny");
    user_outcome_from_retained_chain_error(error)
}

pub(crate) fn foreign_checkpoint_stage_denial_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let foreign_catalog_evidence = retained_catalog_evidence("foreign-retained-chain-stage");
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    checkpoints[11] = retained_checkpoints(&foreign_catalog_evidence, 32)[11].clone();
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 foreign checkpoint stage {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("foreign checkpoint stage must deny");
    user_outcome_from_retained_chain_error(error)
}

fn policy_required_outcome(world: &'static str) -> WorthUserOutcome {
    retained_chain_trigger_outcome(
        world,
        9,
        RetainedCancellationCheckpointTrigger::NearGrazePolicyRequired,
    )
}

fn predicate_uncertain_outcome(world: &'static str) -> WorthUserOutcome {
    retained_chain_error_outcome(world, |workload| {
        workload.requiring_predicate(RetainedCancellationChainPredicate::UncertainAtStep(13))
    })
}

fn transform_invalidation_outcome(world: &'static str) -> WorthUserOutcome {
    retained_chain_error_outcome(world, |workload| {
        workload.requiring_transform_posture(
            RetainedCancellationChainTransformPosture::InvalidatedAtStep(21),
        )
    })
}

fn retained_chain_trigger_outcome(
    world: &'static str,
    trigger_step: usize,
    trigger: RetainedCancellationCheckpointTrigger,
) -> WorthUserOutcome {
    let catalog_evidence = retained_catalog_evidence(world);
    let mut checkpoints = retained_checkpoints(&catalog_evidence, 32);
    checkpoints[trigger_step] = checkpoints[trigger_step]
        .clone()
        .with_trigger(trigger)
        .sampled_for_replay();
    let error = RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 retained trigger {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(checkpoints)
        .certify()
        .expect_err("retained trigger must deny");
    user_outcome_from_retained_chain_error(error)
}

fn retained_chain_error_outcome<F>(world: &'static str, configure: F) -> WorthUserOutcome
where
    F: for<'a> FnOnce(
        RetainedCancellationChainWorkload<'a>,
    ) -> RetainedCancellationChainWorkload<'a>,
{
    let catalog_evidence = retained_catalog_evidence(world);
    let error = configure(retained_chain_workload(world, &catalog_evidence, 32))
        .certify()
        .expect_err("configured retained chain branch must deny");
    user_outcome_from_retained_chain_error(error)
}

fn retained_chain_workload<'a>(
    world: &'static str,
    catalog_evidence: &'a RetainedCancellationCatalogEvidence,
    checkpoint_count: usize,
) -> RetainedCancellationChainWorkload<'a> {
    RetainedCancellationChainWorkload::from_platform_evidence(&catalog_evidence.ledger)
        .declared(format!("MB-M6-4 retained cancellation chain {world}"))
        .with_required_checkpoints(checkpoint_count)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(retained_checkpoints(catalog_evidence, checkpoint_count))
}

fn retained_checkpoints(
    catalog_evidence: &RetainedCancellationCatalogEvidence,
    checkpoint_count: usize,
) -> Vec<RetainedCancellationCheckpoint> {
    (0..checkpoint_count)
        .map(|index| {
            let checkpoint = RetainedCancellationCheckpoint::from_receipts(
                index,
                &catalog_evidence.transform_receipts,
                &catalog_evidence.replay_receipts,
            );
            if index % 4 == 0 {
                checkpoint.sampled_for_replay()
            } else {
                checkpoint
            }
        })
        .collect()
}

fn retained_catalog_evidence(world: &'static str) -> RetainedCancellationCatalogEvidence {
    let catalog = WorkloadCatalog::retained_cancellation_chain()
        .declared(format!(
            "MB-M6-4 retained cancellation chain catalog {world}"
        ))
        .build()
        .expect("retained cancellation catalog should build");
    let catalog_replay = catalog
        .workload()
        .evidence_ledger()
        .row_for_stage(WorkloadEvidenceStage::RetainedReplay)
        .expect("retained cancellation catalog replay row")
        .counters();
    RetainedCancellationCatalogEvidence {
        ledger: catalog.workload().evidence_ledger().clone(),
        transform_receipts: catalog.transform_receipts().clone(),
        replay_receipts: catalog
            .replay_receipts()
            .expect("retained cancellation catalog must expose replay receipts")
            .clone(),
        catalog_retained_artifact_count: catalog_replay.retained_artifact_count(),
        catalog_replay_checkpoint_count: catalog_replay.replay_checkpoint_count(),
    }
}

fn user_outcome_from_retained_chain_receipt(
    receipt: &RetainedCancellationChainReceipt,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_retained_cancellation_chain(receipt),
    )
    .declared("explain retained cancellation chain outcome")
    .respond()
    .expect("retained cancellation response should certify")
    .outcome()
    .clone()
}

fn user_outcome_from_retained_chain_error(
    error: worth_spatial::facade::retained_cancellation_chain::RetainedCancellationChainError,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_retained_cancellation_chain_error(error),
    )
    .declared("explain retained cancellation chain denial")
    .respond()
    .expect("retained cancellation denial response should certify")
    .outcome()
    .clone()
}
