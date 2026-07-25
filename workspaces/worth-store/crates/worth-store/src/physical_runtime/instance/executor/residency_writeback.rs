use sha2::{Digest, Sha256};
use worth_store_physical_backend::BackendQueueExecutionAdaptation;

use super::PhysicalWorkExecutor;
use crate::physical_runtime::{
    record_serving::residency::{
        artifact_tree::PhysicalRecordArtifactTree,
        scheduled_writeback::{PhysicalScheduledWriteback, PhysicalScheduledWritebackOutcome},
    },
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalResidencyWritebackExecutorCommand,
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
        let physical = writeback.execute(
            &PhysicalRecordArtifactTree::new(&self.media),
            BackendQueueExecutionAdaptation::None,
        );
        let (outcome, physical_recovery) = classify_residency_writeback(physical);
        let recovery = self.finish_effect_recovery(prepared, physical_recovery);
        Ok(PhysicalExecutorDispatch::new(dispatched, outcome, recovery))
    }
}

fn classify_residency_writeback(
    physical: PhysicalScheduledWritebackOutcome,
) -> (PhysicalExecutorOutcome, PhysicalEffectRecoveryObligation) {
    match physical {
        PhysicalScheduledWritebackOutcome::RetryableBeforeEffect(failure) => (
            PhysicalExecutorOutcome::DeniedBeforeEffect {
                failure,
                retry: crate::physical_runtime::work::PhysicalRetryPayload::ResidencyWriteback,
            },
            PhysicalEffectRecoveryObligation::Cleared,
        ),
        PhysicalScheduledWritebackOutcome::InspectionRequired(failure) => (
            PhysicalExecutorOutcome::Indeterminate(failure),
            PhysicalEffectRecoveryObligation::Retained,
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
        ),
        PhysicalScheduledWritebackOutcome::Applied {
            physical,
            execution,
        } => (
            PhysicalExecutorOutcome::ResidencyWritebackCompleted {
                physical,
                scheduler: execution,
            },
            PhysicalEffectRecoveryObligation::Cleared,
        ),
        PhysicalScheduledWritebackOutcome::ResidencyTerminal {
            physical,
            execution,
            denial,
        } => (
            PhysicalExecutorOutcome::ResidencyTerminal {
                physical,
                scheduler: execution,
                denial,
            },
            PhysicalEffectRecoveryObligation::Retained,
        ),
    }
}
