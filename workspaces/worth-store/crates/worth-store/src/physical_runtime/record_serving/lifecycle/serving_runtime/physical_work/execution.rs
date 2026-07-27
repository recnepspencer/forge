impl crate::physical_runtime::instance::PhysicalStoreWorkRuntime {
    pub(in crate::physical_runtime) fn execute_physical_work(
        &self,
        command: crate::physical_runtime::PhysicalExecutorCommand,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkExecutionOutcome,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.reconcile_signal_derivation();
        if command.is_cancelled() {
            return Err(crate::physical_runtime::PhysicalWorkPreEffectDenial::ConsumerCancelled);
        }
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.submission,
            command.intent(),
            &self.health,
        )?;
        let dispatch_guard = self.health.physical_dispatch_guard();
        let execution = match self.executor.dispatch(command) {
            Ok(execution) => execution,
            Err(denial) => {
                if denial
                    != crate::physical_runtime::PhysicalWorkPreEffectDenial::
                        RecoveryJournalUnavailable
                {
                    dispatch_guard.disarm();
                }
                return Err(denial);
            }
        };
        let settlement = crate::physical_runtime::PhysicalWorkSettlement::settle(execution);
        let (settled, revocation, effect_activity, residency_writeback) = settlement.into_parts();
        self.consume_settlement_revocation(&settled, revocation);
        dispatch_guard.disarm();
        self.submission.record_settled_causality(&settled);
        let signal = self.signal.record_settlement(&settled);
        self.submission
            .record_derived_completion_causality(settled.intent().identity(), signal);
        if signal
            == crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable
        {
            self.submission
                .record_derived_reconciliation_deferred(settled.intent().identity());
        }
        drop(effect_activity);
        Ok(crate::physical_runtime::PhysicalWorkExecutionOutcome::new(
            settled,
            signal,
            residency_writeback,
        ))
    }
}
