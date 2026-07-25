use std::sync::{Arc, Weak};

use crate::physical_runtime::{
    work::PhysicalWorkAdmissionAuthority, AdmittedPhysicalWork, BlockedPhysicalWork,
    PhysicalEffectObligation, PhysicalMutationSubmission, PhysicalReadSubmission,
    PhysicalWorkAdmission, PhysicalWorkCancellationFailure, PhysicalWorkCancellationJoin,
    PhysicalWorkConsumerHandle, PhysicalWorkObservation, PhysicalWorkPreEffectDenial,
    PhysicalWorkReadiness, PhysicalWorkRecoveryLocator, PhysicalWorkRetryAdmission,
    PhysicalWorkRetryFailure, PhysicalWorkRetrySchedule, PhysicalWorkRetryScheduleOutcome,
    PhysicalWorkSubmissionReceipt, PhysicalWorkTimeoutJoin, ReadyPhysicalWork, SettledPhysicalWork,
};

use super::PhysicalStoreWorkRuntime;

/// Cloneable access to one Store instance's canonical physical-work lifecycle.
///
/// It owns no effect authority. Every transition upgrades the one runtime
/// owner and rechecks its admission/health fence.
#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalWorkLifecycle {
    read_submission: PhysicalReadSubmission,
    mutation_submission: PhysicalMutationSubmission,
    observation: PhysicalWorkObservation,
    admission: PhysicalWorkAdmissionAuthority,
    runtime: Weak<PhysicalStoreWorkRuntime>,
}

impl PhysicalWorkLifecycle {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        admission: PhysicalWorkAdmissionAuthority,
    ) -> Self {
        Self {
            read_submission: runtime.submission.read_submission(),
            mutation_submission: runtime.submission.mutation_submission(),
            observation: runtime.submission.observation(),
            admission,
            runtime: Arc::downgrade(runtime),
        }
    }

    pub(in crate::physical_runtime) fn read_submission(&self) -> PhysicalReadSubmission {
        self.read_submission.clone()
    }

    pub(in crate::physical_runtime) fn mutation_submission(&self) -> PhysicalMutationSubmission {
        self.mutation_submission.clone()
    }

    pub(in crate::physical_runtime) fn observation(&self) -> PhysicalWorkObservation {
        self.observation.clone()
    }

    pub(in crate::physical_runtime) fn admit(
        &self,
        receipt: PhysicalWorkSubmissionReceipt,
    ) -> Result<AdmittedPhysicalWork, PhysicalWorkPreEffectDenial> {
        let runtime = self.runtime()?;
        PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.admission,
            &runtime.health,
        )
    }

    pub(in crate::physical_runtime) fn request(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let runtime = self.runtime()?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            admitted.intent(),
            &runtime.health,
        )?;
        runtime.signal.request(admitted)
    }

    pub(in crate::physical_runtime) fn revalidate_ready(
        &self,
        ready: ReadyPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let runtime = self.runtime()?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            ready.intent(),
            &runtime.health,
        )?;
        runtime.signal.revalidate(ready)
    }

    pub(in crate::physical_runtime) fn revalidate_blocked(
        &self,
        blocked: BlockedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let runtime = self.runtime()?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            blocked.intent(),
            &runtime.health,
        )?;
        runtime.signal.revalidate_blocked(blocked)
    }

    pub(in crate::physical_runtime) fn cancel(
        &self,
        consumer: PhysicalWorkConsumerHandle,
    ) -> Result<PhysicalWorkCancellationJoin, PhysicalWorkCancellationFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(PhysicalWorkCancellationFailure::DerivedStateUnavailable)?;
        let cancelled_before_dispatch = runtime
            .submission
            .cancel_before_dispatch(consumer.identity());
        if !cancelled_before_dispatch {
            let _ = runtime
                .submission
                .mark_consumer_cancelled(consumer.identity());
        }
        let report = runtime
            .signal
            .cancel(consumer)
            .map_err(|_| PhysicalWorkCancellationFailure::DerivedStateUnavailable)?;
        let obligation = if cancelled_before_dispatch {
            PhysicalEffectObligation::NotDispatched
        } else {
            PhysicalEffectObligation::SettlementContinues
        };
        Ok(PhysicalWorkCancellationJoin::new(report, obligation))
    }

    pub(in crate::physical_runtime) fn schedule_retry(
        &self,
        settled: &SettledPhysicalWork,
    ) -> Result<PhysicalWorkRetryScheduleOutcome, PhysicalWorkRetryFailure> {
        if !settled.retry_is_physically_safe() {
            return Err(PhysicalWorkRetryFailure::EffectNotProvenSafe);
        }
        self.runtime()
            .map_err(|_| PhysicalWorkRetryFailure::DerivedStateUnavailable)?
            .signal
            .schedule_retry(settled)
            .map_err(|_| PhysicalWorkRetryFailure::DerivedStateUnavailable)
    }

    pub(in crate::physical_runtime) fn advance_clock(
        &self,
        consumer: PhysicalWorkConsumerHandle,
        request: worth_signal::facade::ClockAdvanceRequest,
    ) -> Result<worth_signal::facade::ValidatedClockAdvance, PhysicalWorkRetryFailure> {
        self.runtime()
            .map_err(|_| PhysicalWorkRetryFailure::DerivedStateUnavailable)?
            .signal
            .advance_clock(consumer.route(), request)
            .map_err(|_| PhysicalWorkRetryFailure::DerivedStateUnavailable)
    }

    pub(in crate::physical_runtime) fn admit_retry(
        &self,
        retry: &PhysicalWorkRetrySchedule,
        settled: SettledPhysicalWork,
    ) -> Result<PhysicalWorkRetryAdmission, PhysicalWorkRetryFailure> {
        if retry.identity() != settled.intent().identity() || !settled.retry_is_physically_safe() {
            return Err(PhysicalWorkRetryFailure::EffectNotProvenSafe);
        }
        let report = self
            .runtime()
            .map_err(|_| PhysicalWorkRetryFailure::DerivedStateUnavailable)?
            .signal
            .admit_retry(retry)
            .map_err(|_| PhysicalWorkRetryFailure::RetryWakeNotReady)?;
        let Some(admitted_retry) = report.admitted_retry().cloned() else {
            return Err(PhysicalWorkRetryFailure::SignalDenied);
        };
        let Some((ready, command)) = settled.into_retry_parts(admitted_retry) else {
            return Err(PhysicalWorkRetryFailure::EffectNotProvenSafe);
        };
        Ok(PhysicalWorkRetryAdmission::new(
            retry.identity(),
            retry.route(),
            report,
            ready,
            command,
        ))
    }

    pub(in crate::physical_runtime) fn timeout(
        &self,
        consumer: PhysicalWorkConsumerHandle,
    ) -> Result<PhysicalWorkTimeoutJoin, PhysicalWorkCancellationFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(PhysicalWorkCancellationFailure::DerivedStateUnavailable)?;
        let report = runtime
            .signal
            .timeout(consumer)
            .map_err(|_| PhysicalWorkCancellationFailure::DerivedStateUnavailable)?;
        let cancelled_before_dispatch = report.timed_out_request().is_some()
            && runtime
                .submission
                .cancel_before_dispatch(consumer.identity());
        let obligation = if cancelled_before_dispatch {
            PhysicalEffectObligation::NotDispatched
        } else {
            let _ = runtime
                .submission
                .mark_consumer_cancelled(consumer.identity());
            PhysicalEffectObligation::SettlementContinues
        };
        Ok(PhysicalWorkTimeoutJoin::new(report, obligation))
    }

    pub(in crate::physical_runtime) fn recovery_obligations(
        &self,
    ) -> Option<Box<[PhysicalWorkRecoveryLocator]>> {
        self.runtime
            .upgrade()
            .map(|runtime| runtime.recovery.obligations().to_vec().into_boxed_slice())
    }

    pub(in crate::physical_runtime) fn recovery_evidence_damaged(&self) -> Option<bool> {
        self.runtime
            .upgrade()
            .map(|runtime| runtime.recovery.evidence_damaged())
    }

    fn runtime(&self) -> Result<Arc<PhysicalStoreWorkRuntime>, PhysicalWorkPreEffectDenial> {
        self.runtime
            .upgrade()
            .ok_or(PhysicalWorkPreEffectDenial::AdmissionStopped)
    }
}
