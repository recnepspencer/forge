use sha2::{Digest, Sha256};
use worth_store_physical_backend::BackendQueueExecutionAdaptation;

use super::PhysicalWorkExecutor;
use crate::physical_runtime::{
    record_serving::residency::{
        artifact_tree::PhysicalRecordArtifactTree,
        scheduled_writeback::{
            PhysicalScheduledWriteback, PhysicalScheduledWritebackDispatch,
            PhysicalScheduledWritebackOutcome,
        },
    },
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalResidencyWritebackCompletion, PhysicalResidencyWritebackExecutorCommand,
};

impl PhysicalWorkExecutor {
    pub(super) fn dispatch_residency_writeback(
        &self,
        command: PhysicalResidencyWritebackExecutorCommand,
    ) -> Result<PhysicalExecutorDispatch, crate::physical_runtime::PhysicalWorkPreEffectDenial>
    {
        let PhysicalResidencyWritebackExecutorCommand { work, claim } = command;
        let coordinate = claim.frames()[0].coordinate();
        let payload_digest: [u8; 32] =
            Sha256::digest(claim.frame_bytes(0).expect("validated writeback claim")).into();
        let (dispatched, plan) = work.into_execution_parts(Some(payload_digest))?;
        let writeback = PhysicalScheduledWriteback::admit(claim, plan)
            .expect("canonical writeback command was validated before dispatch");
        let prepared = self.prepare_effect_recovery(
            &dispatched,
            crate::physical_runtime::PhysicalWorkRecoveryTarget::Range(coordinate),
            Some(payload_digest),
        )?;
        let physical = writeback.execute_effect(
            &PhysicalRecordArtifactTree::new(&self.media),
            BackendQueueExecutionAdaptation::None,
        );
        let physical = match physical {
            PhysicalScheduledWritebackDispatch::Terminal(outcome) => outcome,
            PhysicalScheduledWritebackDispatch::EffectCompleted(effect) => {
                #[cfg(feature = "certification-test-authority")]
                self.certification_yieldpoints.pause(
                    super::CertificationPhysicalExecutionCheckpoint::
                        AfterResidencyWriteBeforeSchedulerSettlement,
                );
                effect.settle()
            }
        };
        let identity = dispatched.intent().identity();
        let (outcome, physical_recovery, completion) =
            classify_residency_writeback(identity, physical);
        let recovery = self.finish_effect_recovery(prepared, physical_recovery);
        Ok(match completion {
            Some(completion) => PhysicalExecutorDispatch::with_residency_writeback_completion(
                dispatched, outcome, recovery, completion,
            ),
            None => PhysicalExecutorDispatch::new(dispatched, outcome, recovery),
        })
    }
}

fn classify_residency_writeback(
    identity: crate::physical_runtime::PhysicalWorkIdentity,
    physical: PhysicalScheduledWritebackOutcome,
) -> (
    PhysicalExecutorOutcome,
    PhysicalEffectRecoveryObligation,
    Option<PhysicalResidencyWritebackCompletion>,
) {
    match physical {
        PhysicalScheduledWritebackOutcome::RetryableBeforeEffect(failure) => (
            PhysicalExecutorOutcome::DeniedBeforeEffect {
                failure,
                retry: crate::physical_runtime::work::PhysicalRetryPayload::ResidencyWriteback,
            },
            PhysicalEffectRecoveryObligation::Cleared,
            None,
        ),
        PhysicalScheduledWritebackOutcome::InspectionRequired(failure) => (
            PhysicalExecutorOutcome::Indeterminate(failure),
            PhysicalEffectRecoveryObligation::Retained,
            None,
        ),
        PhysicalScheduledWritebackOutcome::WrittenButNotApplied {
            physical,
            execution,
        } => (
            PhysicalExecutorOutcome::WriteCompleted {
                physical,
                scheduler: execution,
            },
            PhysicalEffectRecoveryObligation::Retained,
            None,
        ),
        PhysicalScheduledWritebackOutcome::Completed {
            physical,
            execution,
            claim,
        } => {
            let completion =
                PhysicalResidencyWritebackCompletion::new(identity, claim, physical.clone());
            (
                PhysicalExecutorOutcome::ResidencyWritebackCompleted {
                    physical,
                    scheduler: execution,
                },
                PhysicalEffectRecoveryObligation::Cleared,
                Some(completion),
            )
        }
    }
}
