use crate::data::error::SignalError;
use crate::data::output::PartitionSubscription;

use super::super::{FinancialWorldDefinition, InstrumentId, MarketFactorKey, SemanticOutputKey};
use super::evaluation::FinancialEvaluationProgram;
use super::runtime_finance::runtime_financial_snapshot;
use super::topology::factor_signal_aspect;
use super::{source_result, CompiledFinancialWorld};

pub(in crate::tests::domains::fintech) struct FinancialFactorSequenceEvidence {
    pending_scopes: Vec<PartitionSubscription>,
    gated_consumer_was_pending: bool,
}

pub(in crate::tests::domains::fintech) struct FinancialGatedSequenceEvidence {
    baseline_revision: u64,
    final_revision: u64,
}

impl FinancialGatedSequenceEvidence {
    pub(in crate::tests::domains::fintech) const fn revision_delta(&self) -> u64 {
        self.final_revision.abs_diff(self.baseline_revision)
    }
}

impl FinancialFactorSequenceEvidence {
    pub(in crate::tests::domains::fintech) fn pending_scopes(&self) -> &[PartitionSubscription] {
        &self.pending_scopes
    }

    pub(in crate::tests::domains::fintech) const fn gated_consumer_was_pending(&self) -> bool {
        self.gated_consumer_was_pending
    }
}

impl CompiledFinancialWorld {
    pub(in crate::tests::domains::fintech) fn apply_factor_change_sequence(
        &mut self,
        changes: &[(FinancialWorldDefinition, MarketFactorKey)],
        affected_instrument: InstrumentId,
    ) -> Result<FinancialFactorSequenceEvidence, SignalError> {
        self.ledger.clear();
        for (next_definition, factor) in changes {
            let next_snapshot = runtime_financial_snapshot(next_definition);
            let next_projection = self.projection.advance(&next_snapshot);
            let program = FinancialEvaluationProgram::new(
                next_definition.clone(),
                next_projection.clone(),
                self.handles.clone(),
                self.ledger.clone(),
            );
            let source = self.handles.factor(*factor).0;
            let result = source_result(&program, *factor);
            self.runtime.transaction(&mut (), |tx| {
                tx.mark_changed(source, factor_signal_aspect(next_definition, *factor))?;
                self.ledger.record(SemanticOutputKey::Factor(*factor));
                tx.target(source)
                    .on_demand()
                    .read(&move |view| Ok(view.finish(result.clone())))?;
                Ok(())
            })?;
            self.definition = next_definition.clone();
            self.economic_snapshot = next_snapshot;
            self.projection = next_projection;
        }

        let valuation = self.handles.position(affected_instrument).valuation;
        let pending_scopes = self
            .runtime
            .graph()
            .pending_causes(valuation)?
            .iter()
            .flat_map(|cause| cause.changed_scopes.iter().cloned())
            .collect::<Vec<_>>();
        let program = self.program();
        let evaluator = program.evaluator();
        let consumers = self
            .handles
            .consumers
            .values()
            .map(|handle| handle.0)
            .collect::<Vec<_>>();
        let gated_consumer_was_pending = consumers.iter().any(|consumer| {
            self.runtime
                .graph()
                .pending_dependency_revalidation(*consumer)
                .ok()
                .flatten()
                .is_some_and(|pending| !pending.is_resolved())
        });
        self.runtime.transaction(&mut (), |tx| {
            for consumer in &consumers {
                tx.read(*consumer, &evaluator)?;
            }
            Ok(())
        })?;
        Ok(FinancialFactorSequenceEvidence {
            pending_scopes,
            gated_consumer_was_pending,
        })
    }

    pub(in crate::tests::domains::fintech) fn apply_gated_factor_sequence(
        &mut self,
        changes: &[(FinancialWorldDefinition, MarketFactorKey)],
        affected_instrument: InstrumentId,
        consumer_role: super::super::FinancialConsumerRole,
    ) -> Result<FinancialGatedSequenceEvidence, SignalError> {
        let risk_key = SemanticOutputKey::Risk(affected_instrument);
        let baseline_revision = self.projection.output(risk_key).revision;
        self.ledger.clear();
        for (next_definition, factor) in changes {
            let next_snapshot = runtime_financial_snapshot(next_definition);
            let next_projection = self.projection.advance(&next_snapshot);
            let program = FinancialEvaluationProgram::new(
                next_definition.clone(),
                next_projection.clone(),
                self.handles.clone(),
                self.ledger.clone(),
            );
            let evaluator = program.evaluator();
            let source = self.handles.factor(*factor).0;
            let risk = self.handles.position(affected_instrument).risk;
            let result = source_result(&program, *factor);
            self.runtime.transaction(&mut (), |tx| {
                tx.mark_changed(source, factor_signal_aspect(next_definition, *factor))?;
                self.ledger.record(SemanticOutputKey::Factor(*factor));
                tx.target(source)
                    .on_demand()
                    .read(&move |view| Ok(view.finish(result.clone())))?;
                tx.read(risk, &evaluator)?;
                Ok(())
            })?;
            self.definition = next_definition.clone();
            self.economic_snapshot = next_snapshot;
            self.projection = next_projection;
        }
        let program = self.program();
        let evaluator = program.evaluator();
        let consumer = self.handles.consumer(consumer_role).0;
        self.runtime.transaction(&mut (), |tx| {
            tx.read(consumer, &evaluator)?;
            Ok(())
        })?;
        Ok(FinancialGatedSequenceEvidence {
            baseline_revision,
            final_revision: self.projection.output(risk_key).revision,
        })
    }
}
