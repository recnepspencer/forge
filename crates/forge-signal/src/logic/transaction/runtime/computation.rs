use crate::data::aspect::AspectVersion;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeContract;
use crate::data::output::{ComputationFamily, ComputationKey, KeyedComputation, StructuralMemoKey};
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::ExecutionReport;

use super::state::SignalRuntime;
use super::transaction::SignalTransaction;

pub struct ComputationSpec<T, F> {
    pub family: ComputationFamily,
    pub contract: NodeContract,
    pub tier: T,
    pub comparator: VersionComparatorPolicy,
    pub evaluator: F,
}

pub struct DefinedComputation<T, F> {
    family: ComputationFamily,
    contract: NodeContract,
    tier: T,
    comparator: VersionComparatorPolicy,
    evaluator: F,
}

pub struct DefinedKeyedComputation<'a, T, F> {
    definition: &'a DefinedComputation<T, F>,
    key: ComputationKey,
}

impl<T: Copy, F> DefinedComputation<T, F> {
    pub(crate) fn from_spec(spec: ComputationSpec<T, F>) -> Self {
        Self {
            family: spec.family,
            contract: spec.contract,
            tier: spec.tier,
            comparator: spec.comparator,
            evaluator: spec.evaluator,
        }
    }

    pub fn family(&self) -> &ComputationFamily {
        &self.family
    }

    pub fn contract(&self) -> &NodeContract {
        &self.contract
    }

    pub fn tier(&self) -> T {
        self.tier
    }

    pub fn comparator(&self) -> &VersionComparatorPolicy {
        &self.comparator
    }

    pub fn keyed(&self, key: impl Into<ComputationKey>) -> DefinedKeyedComputation<'_, T, F> {
        DefinedKeyedComputation {
            definition: self,
            key: key.into(),
        }
    }
}

impl<'a, T: Copy, F> DefinedKeyedComputation<'a, T, F> {
    pub fn key(&self) -> &ComputationKey {
        &self.key
    }

    pub fn family(&self) -> &ComputationFamily {
        self.definition.family()
    }

    pub fn node<D, I, E, Ctx>(&self, runtime: &mut SignalRuntime<D, I, E, Ctx, T>) -> NodeId
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        T: Ord,
    {
        runtime
            .config
            .resolve_defined_node(&mut runtime.graph, self.definition.family(), self.key.clone())
    }

    pub fn node_in_transaction<D, I, E, Ctx>(
        &self,
        tx: &mut SignalTransaction<'_, D, I, E, Ctx, T>,
    ) -> NodeId
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        T: Ord,
    {
        tx.resolve_defined_node(self.definition.family(), self.key.clone())
    }

    pub fn metadata(&self) -> KeyedComputation {
        KeyedComputation::new(self.definition.family().clone(), self.key.clone())
    }

    pub fn memoized(&self, memo_key: impl Into<StructuralMemoKey>) -> KeyedComputation {
        self.metadata().with_memo_key(memo_key)
    }
}

impl<'a, T, F> DefinedKeyedComputation<'a, T, F>
where
    T: Copy + Ord,
{
    pub fn execute<D, I, E, Ctx, O>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
        runtime_ctx: &Ctx,
    ) -> Result<ExecutionReport, SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let node = self.node(runtime);
        runtime.evaluate_with_plan(node, runtime_ctx, &self.definition.evaluator, EvaluationRequestMode::Default)
    }

    pub fn read<D, I, E, Ctx, O>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
        runtime_ctx: &Ctx,
    ) -> Result<AspectVersion, SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let node = self.node(runtime);
        runtime.read(node, runtime_ctx, &self.definition.evaluator)
    }

    pub fn evaluate<D, I, E, Ctx, O>(
        &self,
        tx: &mut SignalTransaction<'_, D, I, E, Ctx, T>,
    ) -> Result<(), SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let node = self.node_in_transaction(tx);
        tx.evaluate_keyed(node, &self.metadata(), &self.definition.evaluator)
    }

    pub fn evaluate_memoized<D, I, E, Ctx, O>(
        &self,
        tx: &mut SignalTransaction<'_, D, I, E, Ctx, T>,
        memo_key: impl Into<StructuralMemoKey>,
    ) -> Result<(), SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let node = self.node_in_transaction(tx);
        tx.evaluate_keyed(node, &self.memoized(memo_key), &self.definition.evaluator)
    }
}
