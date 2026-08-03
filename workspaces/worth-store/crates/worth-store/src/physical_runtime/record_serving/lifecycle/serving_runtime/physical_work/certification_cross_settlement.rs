impl crate::physical_runtime::instance::PhysicalStoreWorkRuntime {
    pub(in crate::physical_runtime) fn certification_cross_settle_physical_writes(
        &self,
        first: crate::physical_runtime::PhysicalExecutorCommand,
        second: crate::physical_runtime::PhysicalExecutorCommand,
    ) -> Result<
        [crate::physical_runtime::PhysicalWorkEffectFate; 2],
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.require_certification_command(&first)?;
        self.require_certification_command(&second)?;
        let first_guard = self.health.physical_dispatch_guard();
        let first_execution = self.executor.dispatch(first)?;
        let second_guard = self.health.physical_dispatch_guard();
        let second_execution = self.executor.dispatch(second)?;
        let (first_dispatched, first_outcome, first_recovery, first_writeback) =
            first_execution.into_parts();
        let (second_dispatched, second_outcome, second_recovery, second_writeback) =
            second_execution.into_parts();
        let recovery = first_recovery.join(second_recovery);

        let first = crate::physical_runtime::PhysicalWorkSettlement::settle(
            crate::physical_runtime::PhysicalExecutorDispatch::from_parts(
                first_dispatched,
                second_outcome,
                recovery,
                second_writeback,
            ),
        );
        let second = crate::physical_runtime::PhysicalWorkSettlement::settle(
            crate::physical_runtime::PhysicalExecutorDispatch::from_parts(
                second_dispatched,
                first_outcome,
                recovery,
                first_writeback,
            ),
        );
        let (first, first_revocation, first_activity, first_writeback) = first.into_parts();
        self.consume_settlement_revocation(&first, first_revocation);
        self.submission.record_settled_causality(&first);
        let first_signal = self.signal.record_settlement(&first);
        self.submission
            .record_derived_completion_causality(first.intent().identity(), first_signal);
        if first_signal
            == crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable
        {
            self.submission
                .record_derived_reconciliation_deferred(first.intent().identity());
        }

        let (second, second_revocation, second_activity, second_writeback) = second.into_parts();
        self.consume_settlement_revocation(&second, second_revocation);
        self.submission.record_settled_causality(&second);
        let second_signal = self.signal.record_settlement(&second);
        self.submission
            .record_derived_completion_causality(second.intent().identity(), second_signal);
        if second_signal
            == crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable
        {
            self.submission
                .record_derived_reconciliation_deferred(second.intent().identity());
        }

        let fates = [first.evidence().fate(), second.evidence().fate()];
        drop(first_activity);
        drop(second_activity);
        drop(first_writeback);
        drop(second_writeback);
        first_guard.disarm();
        second_guard.disarm();
        Ok(fates)
    }

    fn require_certification_command(
        &self,
        command: &crate::physical_runtime::PhysicalExecutorCommand,
    ) -> Result<(), crate::physical_runtime::PhysicalWorkPreEffectDenial> {
        if command.is_cancelled() {
            return Err(crate::physical_runtime::PhysicalWorkPreEffectDenial::ConsumerCancelled);
        }
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.submission,
            command.intent(),
            &self.health,
        )
    }
}
