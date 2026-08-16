use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::NodeEvaluationResult;
use crate::logic::evaluation::EvaluationVerdict;
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};

use super::super::condition_resolution::{resolve_condition, ConditionDisposition};
use super::super::{
    InstalledSignalConditionResolver, SignalConditionalDecisionClass,
    SignalConditionalDecisionCounters,
};
use super::{ConditionalExecutionProviders, SignalConditionalExecutionRequest};

pub(super) struct ConditionalResolutionAttempt<
    'graph,
    'request,
    'contract,
    'provider_borrow,
    'providers,
    'counter,
    Condition,
    Comparator,
    Compute,
> {
    pub(super) graph: &'graph mut SignalGraph,
    pub(super) request: &'request SignalConditionalExecutionRequest<'contract>,
    pub(super) providers: &'provider_borrow mut ConditionalExecutionProviders<
        'providers,
        Condition,
        Comparator,
        Compute,
    >,
    pub(super) dependencies: PreparedDependencyCapture,
    pub(super) ready_invalidation:
        Option<crate::data::proof::invalidation::progression::ReadyInvalidationBatch>,
    pub(super) counters: &'counter mut SignalConditionalDecisionCounters,
}

impl<Condition, Comparator, Compute>
    ConditionalResolutionAttempt<'_, '_, '_, '_, '_, '_, Condition, Comparator, Compute>
where
    Condition: InstalledSignalConditionResolver,
    Comparator: ComparatorPolicyResolver,
    Compute: FnOnce() -> Result<NodeEvaluationResult, SignalError>,
{
    pub(super) fn resolve(self) -> Result<SignalConditionalDecisionClass, SignalError> {
        let disposition = resolve_condition(self.graph, self.request, self.providers.condition)?;
        match disposition {
            ConditionDisposition::Eligible => self.apply_computed(),
            ConditionDisposition::Suppressed => {
                self.counters.application_contacts += 1;
                apply_passive(
                    self.graph,
                    self.request.contract.node(),
                    self.dependencies,
                    self.providers.comparator,
                )?;
                Ok(SignalConditionalDecisionClass::SuppressedBeforeCompute)
            }
            ConditionDisposition::Deferred => {
                self.apply_deferred(SignalConditionalDecisionClass::DeferredByCondition)
            }
            ConditionDisposition::DeferredTemporal => {
                self.apply_deferred(SignalConditionalDecisionClass::DeferredTemporal)
            }
            ConditionDisposition::DeferredOnDemand => {
                self.apply_deferred(SignalConditionalDecisionClass::DeferredOnDemand)
            }
        }
    }

    fn apply_computed(self) -> Result<SignalConditionalDecisionClass, SignalError> {
        self.counters.compute_contacts += 1;
        let compute = self.providers.compute.take().ok_or_else(|| {
            SignalError::internal("conditional compute provider was already consumed")
        })?;
        let result = match self.ready_invalidation {
            Some(ready) => {
                crate::logic::invalidation::scheduling::execute_ready(&*self.graph, ready, compute)?
            }
            None => compute()?,
        };
        let prepared = PreparedEvaluation::from_result(result).with_dependencies(self.dependencies);
        self.counters.application_contacts += 1;
        let applied = crate::logic::evaluation::apply_prepared_evaluation_with_policy(
            self.graph,
            self.request.contract.node(),
            prepared,
            self.providers.comparator,
            None,
        )?;
        self.counters.semantic_classifications += 1;
        match applied.report.verdict {
            EvaluationVerdict::Recomputed => {
                self.counters.semantic_changes += 1;
                Ok(SignalConditionalDecisionClass::ComputedChanged)
            }
            EvaluationVerdict::Suppressed { .. } => {
                Ok(SignalConditionalDecisionClass::ComputedRevertedClean)
            }
            EvaluationVerdict::Deferred { .. } => Err(SignalError::internal(
                "computed conditional output cannot become a deferred verdict",
            )),
        }
    }

    fn apply_deferred(
        self,
        class: SignalConditionalDecisionClass,
    ) -> Result<SignalConditionalDecisionClass, SignalError> {
        self.counters.application_contacts += 1;
        apply_deferred(
            self.graph,
            self.request.contract.node(),
            self.dependencies,
            self.providers.comparator,
        )?;
        Ok(class)
    }
}

pub(super) fn apply_passive(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    dependencies: PreparedDependencyCapture,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError> {
    let prepared =
        PreparedEvaluation::reverted_clean_by_condition().with_dependencies(dependencies);
    crate::logic::evaluation::apply_prepared_evaluation_with_policy(
        graph, node, prepared, resolver, None,
    )?;
    Ok(())
}

fn apply_deferred(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    dependencies: PreparedDependencyCapture,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<(), SignalError> {
    let prepared = PreparedEvaluation::deferred_by_condition().with_dependencies(dependencies);
    crate::logic::evaluation::apply_prepared_evaluation_with_policy(
        graph, node, prepared, resolver, None,
    )?;
    Ok(())
}
