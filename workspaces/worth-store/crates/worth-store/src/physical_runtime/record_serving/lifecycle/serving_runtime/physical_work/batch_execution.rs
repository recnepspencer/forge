use super::super::ServingPhysicalRuntime;

struct BatchDerivedCompletionGuard<'submission> {
    submission: &'submission crate::physical_runtime::work::PhysicalWorkSubmissionOwner,
    retained: Vec<crate::physical_runtime::PhysicalWorkIdentity>,
    armed: bool,
}

impl ServingPhysicalRuntime {
    pub fn execute_physical_work_batch(
        &self,
        commands: Box<[crate::physical_runtime::PhysicalExecutorCommand]>,
    ) -> crate::physical_runtime::PhysicalWorkExecutionBatchOutcome {
        self.physical_work_execution()
            .execute_physical_work_batch(commands)
    }
}

impl crate::physical_runtime::instance::PhysicalStoreWorkRuntime {
    pub(in crate::physical_runtime) fn execute_physical_work_batch(
        &self,
        commands: Box<[crate::physical_runtime::PhysicalExecutorCommand]>,
        execution_generation: crate::physical_runtime::LifecycleGeneration,
    ) -> crate::physical_runtime::PhysicalWorkExecutionBatchOutcome {
        let mut settled = Vec::with_capacity(commands.len());
        let mut effect_activities = Vec::with_capacity(commands.len());
        let mut residency_writebacks = Vec::with_capacity(commands.len());
        let mut denied = Vec::new();
        let mut derived_guard = BatchDerivedCompletionGuard::new(&self.submission);
        let mut derivation_reconciled = false;
        for command in commands {
            if let Err(failure) = require_execution_generation(&command, execution_generation) {
                denied.push(failure);
                continue;
            }
            if !derivation_reconciled {
                self.reconcile_signal_derivation();
                derivation_reconciled = true;
            }
            match self.execute_batch_command(command) {
                Ok((work, effect_activity, residency_writeback)) => {
                    self.submission.record_settled_causality(&work);
                    if self.signal.settlement_requires_derived_completion(&work) {
                        if self.signal.retain_settlement_obligation(&work) {
                            derived_guard.retain(work.intent().identity());
                        } else {
                            self.submission
                                .record_derived_reconciliation_deferred(work.intent().identity());
                        }
                    }
                    settled.push(work);
                    effect_activities.push(effect_activity);
                    residency_writebacks.push(residency_writeback);
                }
                Err(failure) => denied.push(failure),
            }
        }
        let signals = self.signal.record_settlement_batch(&settled);
        for (work, signal) in settled.iter().zip(signals.iter()) {
            self.submission
                .record_derived_completion_causality(work.intent().identity(), *signal);
            if *signal
                == crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable
            {
                self.submission
                    .record_derived_reconciliation_deferred(work.intent().identity());
            }
        }
        derived_guard.complete();
        drop(effect_activities);
        let executions = settled
            .into_iter()
            .zip(signals)
            .zip(residency_writebacks)
            .map(|((work, signal), residency_writeback)| {
                crate::physical_runtime::PhysicalWorkExecutionOutcome::new(
                    work,
                    signal,
                    residency_writeback,
                )
            })
            .collect();
        crate::physical_runtime::PhysicalWorkExecutionBatchOutcome::new(executions, denied)
    }

    fn execute_batch_command(
        &self,
        command: crate::physical_runtime::PhysicalExecutorCommand,
    ) -> Result<
        (
            crate::physical_runtime::SettledPhysicalWork,
            crate::physical_runtime::work::PhysicalEffectActivity,
            Option<crate::physical_runtime::PhysicalResidencyWritebackCompletion>,
        ),
        crate::physical_runtime::PhysicalWorkBatchDenial,
    > {
        let identity = command.intent().identity();
        if command.is_cancelled() {
            return Err(crate::physical_runtime::PhysicalWorkBatchDenial::new(
                identity,
                crate::physical_runtime::PhysicalWorkPreEffectDenial::ConsumerCancelled,
            ));
        }
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.submission,
            command.intent(),
            &self.health,
        )
        .map_err(|denial| {
            crate::physical_runtime::PhysicalWorkBatchDenial::new(identity, denial)
        })?;
        let dispatch_guard = self.health.physical_dispatch_guard();
        let execution = match self.executor.dispatch(command) {
            Ok(execution) => execution,
            Err(denial) => {
                if denial
                        != crate::physical_runtime::PhysicalWorkPreEffectDenial::RecoveryJournalUnavailable
                    {
                        dispatch_guard.disarm();
                    }
                return Err(crate::physical_runtime::PhysicalWorkBatchDenial::new(
                    identity, denial,
                ));
            }
        };
        let settlement = crate::physical_runtime::PhysicalWorkSettlement::settle(execution);
        let (work, revocation, effect_activity, residency_writeback) = settlement.into_parts();
        self.consume_settlement_revocation(&work, revocation);
        dispatch_guard.disarm();
        Ok((work, effect_activity, residency_writeback))
    }
}

fn require_execution_generation(
    command: &crate::physical_runtime::PhysicalExecutorCommand,
    execution_generation: crate::physical_runtime::LifecycleGeneration,
) -> Result<(), crate::physical_runtime::PhysicalWorkBatchDenial> {
    let identity = command.intent().identity();
    (identity.generation().lifecycle() == execution_generation)
        .then_some(())
        .ok_or_else(|| {
            crate::physical_runtime::PhysicalWorkBatchDenial::new(
                identity,
                crate::physical_runtime::PhysicalWorkPreEffectDenial::StaleGeneration,
            )
        })
}

impl<'submission> BatchDerivedCompletionGuard<'submission> {
    fn new(
        submission: &'submission crate::physical_runtime::work::PhysicalWorkSubmissionOwner,
    ) -> Self {
        Self {
            submission,
            retained: Vec::new(),
            armed: true,
        }
    }

    fn retain(&mut self, identity: crate::physical_runtime::PhysicalWorkIdentity) {
        self.retained.push(identity);
    }

    fn complete(mut self) {
        self.armed = false;
    }
}

impl Drop for BatchDerivedCompletionGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            for identity in self.retained.iter().copied() {
                self.submission
                    .record_derived_reconciliation_deferred(identity);
            }
        }
    }
}
