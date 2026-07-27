use crate::physical_runtime::{
    PhysicalExecutorCommand, PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition,
};

use super::{
    super::{
        failure::{PhysicalWritebackFailureCause, PhysicalWritebackTransitionFailure},
        outcome::{
            PhysicalWritebackExecution, PhysicalWritebackInspectionRequired,
            PhysicalWritebackSettlement, RetryablePhysicalWriteback,
        },
    },
    AdmittedPhysicalWriteback, FrameWritebackPort,
};
use crate::physical_runtime::record_serving::residency::scheduled_writeback::PhysicalScheduledWriteback;

impl FrameWritebackPort {
    pub(in crate::physical_runtime::record_serving) fn execute(
        &self,
        admitted: AdmittedPhysicalWriteback,
    ) -> Result<PhysicalWritebackExecution, PhysicalWritebackTransitionFailure> {
        let AdmittedPhysicalWriteback { work, claim, dirty } = admitted;
        if let Err(denial) = PhysicalScheduledWriteback::validate(&claim, work.queue_plan()) {
            return Err(PhysicalWritebackTransitionFailure::new(
                PhysicalWritebackFailureCause::WritebackAdmission(denial),
                dirty,
            ));
        }
        let command = PhysicalExecutorCommand::residency_writeback(work, claim);
        let outcome = match self.execution.execute_physical_work(command) {
            Ok(outcome) => outcome,
            Err(denial) => {
                return Err(PhysicalWritebackTransitionFailure::new(
                    PhysicalWritebackFailureCause::PreEffect(denial),
                    dirty,
                ))
            }
        };
        let (settled, signal, completion) = outcome.into_residency_writeback_parts();
        let settlement = PhysicalWritebackSettlement::from_settled(&settled, signal);
        if settled.retry_is_physically_safe() && completion.is_none() {
            self.frame_ports.observe_retryable_writeback();
            return Ok(PhysicalWritebackExecution::Retryable(
                RetryablePhysicalWriteback::new(settled, settlement, dirty),
            ));
        }
        let settled_success = settlement.effect_fate() == PhysicalWorkEffectFate::WriteCompleted
            && settlement.recovery() != PhysicalWorkRecoveryDisposition::InspectionRequired;
        let cleaned = if settled_success {
            completion
                .map(|completion| {
                    completion
                        .publish_clean(self.frame_ports.writeback_clean_authority())
                        .is_ok()
                })
                .unwrap_or(false)
        } else {
            drop(completion);
            false
        };
        if cleaned {
            self.frame_ports.observe_exact_writeback_receipt();
            drop(dirty.into_frame());
            return Ok(PhysicalWritebackExecution::Clean(settlement));
        }
        self.frame_ports.observe_writeback_inspection(
            settlement.effect_fate() == PhysicalWorkEffectFate::Indeterminate,
        );
        drop(dirty.into_frame());
        if let Some(runtime) = self.runtime.upgrade() {
            runtime.health.revoke();
        }
        Ok(PhysicalWritebackExecution::InspectionRequired(
            PhysicalWritebackInspectionRequired::new(settlement),
        ))
    }
}
