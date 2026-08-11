use worth_store_io_scheduler::{execute_ready_queue_plan, QueueExecutionOutcome};
use worth_store_physical_backend::{
    CompletedScheduledRecoveryReopenRead, RecoveryReopenReadOutcome,
};
use worth_store_physical_format::{
    DurablePhysicalRootManifest, DurableRootSelector, RootSelectorRole,
};

use crate::physical_runtime::recovery_coordination::settlement::{
    settle, signal_completion_is_terminal,
};
#[cfg(feature = "certification-test-authority")]
use crate::physical_runtime::recovery_coordination::settlement::{
    settle_with_certification, PhysicalRecoverySettlementCertificationStage,
};
use crate::physical_runtime::work::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalRetryPayload,
};
use crate::physical_runtime::{PhysicalWorkSchedulerPosture, PhysicalWorkScope};

use super::{
    artifact, CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryFreshReopenCommand,
    PhysicalRecoveryFreshReopenDenial, PhysicalRecoveryFreshReopenDenialKind,
    PhysicalRecoveryFreshReopenOutcome, PhysicalRecoveryFreshReopenStage,
};
use crate::physical_runtime::recovery_coordination::{
    PerformedRecoveryPhysicalEffect, PhysicalRecoveryCoordination, RecoveryFreshReopenOccurrence,
};

pub(super) fn execute(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    command: PhysicalRecoveryFreshReopenCommand,
) -> PhysicalRecoveryFreshReopenOutcome {
    let generation = command.expected_root.generation();
    let selector = match read(
        coordination,
        media,
        PhysicalRecoveryFreshReopenStage::CurrentSelector,
        generation,
        worth_store_physical_format::ROOT_SELECTOR_BYTES as u64,
    ) {
        Ok(read) => read,
        Err(denial) => return PhysicalRecoveryFreshReopenOutcome::Denied(denial),
    };
    let observed_selector = match DurableRootSelector::decode(selector.physical.bytes()) {
        Ok(selector)
            if selector == command.expected_selector
                && selector.store_identity() == media.store_identity()
                && selector.format() == command.format
                && selector.role() == RootSelectorRole::Current
                && selector.root_generation() == generation =>
        {
            selector
        }
        _ => {
            return denied_binding(
                selector.physical,
                None,
                PhysicalRecoveryFreshReopenDenialKind::InvalidSelector,
            );
        }
    };
    let root_bytes = command.expected_root.encode(command.format).len() as u64;
    let root = match read(
        coordination,
        media,
        PhysicalRecoveryFreshReopenStage::RootManifest,
        generation,
        root_bytes,
    ) {
        Ok(read) => read,
        Err(mut denial) => {
            denial.selector = Some(selector.physical);
            return PhysicalRecoveryFreshReopenOutcome::Denied(denial);
        }
    };
    let observed_root = match DurablePhysicalRootManifest::decode(
        root.physical.bytes(),
        command.expected_root.node_capacity(),
    ) {
        Ok((root, format)) if format == command.format => root,
        _ => {
            return denied_binding(
                selector.physical,
                Some(root.physical),
                PhysicalRecoveryFreshReopenDenialKind::InvalidRoot,
            )
        }
    };
    if observed_selector != command.expected_selector || observed_root != command.expected_root {
        return denied_binding(
            selector.physical,
            Some(root.physical),
            PhysicalRecoveryFreshReopenDenialKind::BindingMismatch,
        );
    }
    let performed =
        PerformedRecoveryPhysicalEffect::record_fresh_reopen(RecoveryFreshReopenOccurrence::new(
            coordination.session_identity(),
            command.plan,
            generation,
            selector.physical,
            root.physical,
            selector.work,
            root.work,
            selector.signal,
            root.signal,
        ));
    PhysicalRecoveryFreshReopenOutcome::Completed(CompletedPhysicalRecoveryFreshReopen::new(
        observed_root,
        performed,
    ))
}

struct CompletedRead {
    physical: CompletedScheduledRecoveryReopenRead,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

fn read(
    coordination: &PhysicalRecoveryCoordination,
    media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    stage: PhysicalRecoveryFreshReopenStage,
    generation: u64,
    maximum_bytes: u64,
) -> Result<CompletedRead, PhysicalRecoveryFreshReopenDenial> {
    let artifact = artifact(stage, generation);
    let work = super::admission::admit(
        coordination,
        PhysicalWorkScope::artifact(artifact),
        maximum_bytes,
    )
    .map_err(|kind| PhysicalRecoveryFreshReopenDenial::new(stage, kind, None, None, None))?;
    let work_identity = work.intent().identity();
    let (dispatched, plan) = work.into_execution_parts(None).map_err(|denial| {
        PhysicalRecoveryFreshReopenDenial::new(
            stage,
            PhysicalRecoveryFreshReopenDenialKind::PreEffect(denial),
            None,
            None,
            None,
        )
    })?;
    match media.read_recovery_artifact_scheduled(
        artifact,
        maximum_bytes,
        plan.backend_completion_binding()
            .backend_execution_binding(),
    ) {
        RecoveryReopenReadOutcome::Completed(completed) => {
            let queue = completed.queue();
            #[cfg(feature = "certification-test-authority")]
            let queue = if coordination.take_certification_reopen_scheduler_failure(stage) {
                queue.with_foreign_plan_binding_for_certification()
            } else {
                queue
            };
            let scheduler = execute_ready_queue_plan(plan, queue);
            let posture = scheduler_posture(&scheduler);
            let dispatch = PhysicalExecutorDispatch::new(
                dispatched,
                PhysicalExecutorOutcome::ReadCompleted {
                    physical: completed.physical(),
                    bytes: completed.bytes().to_vec().into_boxed_slice(),
                    scheduler,
                },
                PhysicalEffectRecoveryObligation::Cleared,
            );
            #[cfg(feature = "certification-test-authority")]
            let signal = settle_with_certification(
                coordination,
                dispatch,
                PhysicalRecoverySettlementCertificationStage::FreshReopen(stage),
            );
            #[cfg(not(feature = "certification-test-authority"))]
            let signal = settle(coordination, dispatch);
            if posture != PhysicalWorkSchedulerPosture::Executed {
                return Err(completed_settlement_denial(
                    stage,
                    PhysicalRecoveryFreshReopenDenialKind::SchedulerSettlement(posture),
                    completed,
                ));
            }
            if !signal_completion_is_terminal(signal) {
                return Err(completed_settlement_denial(
                    stage,
                    PhysicalRecoveryFreshReopenDenialKind::SignalSettlement(signal),
                    completed,
                ));
            }
            Ok(CompletedRead {
                physical: completed,
                work: work_identity,
                signal,
            })
        }
        RecoveryReopenReadOutcome::Denied(denied) => {
            let _scheduler = denied
                .queue()
                .map(|queue| execute_ready_queue_plan(plan, queue));
            let _ = settle(
                coordination,
                PhysicalExecutorDispatch::new(
                    dispatched,
                    PhysicalExecutorOutcome::DeniedBeforeEffect {
                        failure: denied.failure(),
                        retry: PhysicalRetryPayload::Read,
                    },
                    PhysicalEffectRecoveryObligation::Cleared,
                ),
            );
            Err(PhysicalRecoveryFreshReopenDenial::new(
                stage,
                PhysicalRecoveryFreshReopenDenialKind::Media(denied.failure()),
                None,
                None,
                Some(denied),
            ))
        }
    }
}

fn completed_settlement_denial(
    stage: PhysicalRecoveryFreshReopenStage,
    kind: PhysicalRecoveryFreshReopenDenialKind,
    completed: CompletedScheduledRecoveryReopenRead,
) -> PhysicalRecoveryFreshReopenDenial {
    let (selector, root) = match stage {
        PhysicalRecoveryFreshReopenStage::CurrentSelector => (Some(completed), None),
        PhysicalRecoveryFreshReopenStage::RootManifest => (None, Some(completed)),
        PhysicalRecoveryFreshReopenStage::ExactBinding => {
            unreachable!("exact binding is not a physical read stage")
        }
    };
    PhysicalRecoveryFreshReopenDenial::new(stage, kind, selector, root, None)
}

fn denied_binding(
    selector: CompletedScheduledRecoveryReopenRead,
    root: Option<CompletedScheduledRecoveryReopenRead>,
    kind: PhysicalRecoveryFreshReopenDenialKind,
) -> PhysicalRecoveryFreshReopenOutcome {
    PhysicalRecoveryFreshReopenOutcome::Denied(PhysicalRecoveryFreshReopenDenial::new(
        PhysicalRecoveryFreshReopenStage::ExactBinding,
        kind,
        Some(selector),
        root,
        None,
    ))
}

fn scheduler_posture(outcome: &QueueExecutionOutcome) -> PhysicalWorkSchedulerPosture {
    if matches!(outcome, QueueExecutionOutcome::Executed(_)) {
        PhysicalWorkSchedulerPosture::Executed
    } else {
        PhysicalWorkSchedulerPosture::RejectedAfterEffect
    }
}
