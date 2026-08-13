use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::output::NodeEvaluationResult;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationOutput;
use crate::tests::domains::fintech::aspects::{PRICE, RISK};

use super::super::{
    FinancialEvaluationLedger, FinancialSemanticHandles, FinancialSemanticProjection,
    FinancialWorldDefinition, SemanticOutputKey,
};
use super::topology::{factor_signal_aspect, signal_aspect};

#[derive(Clone)]
pub(super) struct FinancialEvaluationProgram {
    definition: FinancialWorldDefinition,
    projection: FinancialSemanticProjection,
    handles: FinancialSemanticHandles,
    ledger: FinancialEvaluationLedger,
}

impl FinancialEvaluationProgram {
    pub(super) fn new(
        definition: FinancialWorldDefinition,
        projection: FinancialSemanticProjection,
        handles: FinancialSemanticHandles,
        ledger: FinancialEvaluationLedger,
    ) -> Self {
        Self {
            definition,
            projection,
            handles,
            ledger,
        }
    }

    pub(super) fn evaluator(
        &self,
    ) -> impl for<'ctx> Fn(&mut EvaluationContext<'ctx, ()>) -> Result<EvaluationOutput, SignalError>
           + Sync
           + '_ {
        move |view| self.evaluate(view)
    }

    pub(super) fn result_for(&self, key: SemanticOutputKey) -> NodeEvaluationResult {
        let projected = self.projection.output(key);
        let aspect = match key {
            SemanticOutputKey::Factor(factor) => factor_signal_aspect(&self.definition, factor),
            _ => signal_aspect(projected.aspect),
        };
        NodeEvaluationResult::from_version(AspectVersion::from_updates([(
            aspect,
            projected.revision,
        )]))
        .with_output_identity(format!(
            "financial:{key:?}:{}",
            projected.canonical_financial_value
        ))
    }

    pub(super) fn result_for_node(
        &self,
        node: crate::data::handle::NodeId,
    ) -> Result<NodeEvaluationResult, SignalError> {
        let key = self
            .semantic_key_for_node(node)
            .ok_or_else(|| SignalError::invalid_input(format!("unknown financial node {node}")))?;
        self.ledger.record(key);
        Ok(self.result_for(key))
    }

    fn semantic_key_for_node(
        &self,
        node: crate::data::handle::NodeId,
    ) -> Option<SemanticOutputKey> {
        self.handles
            .factors
            .iter()
            .find_map(|(factor, handle)| {
                (handle.0 == node).then_some(SemanticOutputKey::Factor(*factor))
            })
            .or_else(|| {
                self.handles
                    .positions
                    .iter()
                    .find_map(|(instrument, handles)| {
                        (handles.valuation == node)
                            .then_some(SemanticOutputKey::Valuation(*instrument))
                            .or_else(|| {
                                (handles.risk == node)
                                    .then_some(SemanticOutputKey::Risk(*instrument))
                            })
                    })
            })
            .or_else(|| {
                self.handles.consumers.iter().find_map(|(role, handle)| {
                    (handle.0 == node).then_some(SemanticOutputKey::Consumer(*role))
                })
            })
    }

    fn evaluate(
        &self,
        view: &mut EvaluationContext<'_, ()>,
    ) -> Result<EvaluationOutput, SignalError> {
        let node = view.node();
        for position in self.definition.positions() {
            let handles = self.handles.position(position.instrument);
            if node == handles.valuation {
                for subscription in &position.subscriptions {
                    let source = self.handles.factor(subscription.factor).0;
                    let _ = view.read_partitioned_aspect_version(
                        source,
                        factor_signal_aspect(&self.definition, subscription.factor),
                        crate::data::output::PartitionSubscription::partition_and_detail(
                            subscription.partition,
                            subscription.detail,
                        ),
                    )?;
                }
                let key = SemanticOutputKey::Valuation(position.instrument);
                self.ledger.record(key);
                return Ok(view.finish(self.result_for(key)));
            }
            if node == handles.risk {
                let _ = view.read_aspect_version(handles.valuation, PRICE)?;
                let key = SemanticOutputKey::Risk(position.instrument);
                self.ledger.record(key);
                return Ok(view.finish(self.result_for(key)));
            }
        }
        for declaration in self.definition.consumers() {
            let handle = self.handles.consumer(declaration.role).0;
            if node != handle {
                continue;
            }
            let risk = self.handles.position(declaration.position).risk;
            let _ = view.read_aspect_version(risk, RISK)?;
            let key = SemanticOutputKey::Consumer(declaration.role);
            self.ledger.record(key);
            return Ok(view.finish(self.result_for(key)));
        }
        Err(SignalError::invalid_input(format!(
            "node {node} is not part of the compiled financial evaluation program"
        )))
    }
}
