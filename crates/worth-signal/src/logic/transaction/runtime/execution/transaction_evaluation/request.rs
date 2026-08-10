use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{ExecutionReport, StageExecutor};

use super::super::super::transaction::SignalTransaction;
use super::super::shared::executor_for_strategy;

pub(super) enum TransactionExecutionIntent<'a> {
    Targets {
        targets: &'a [NodeId],
        request_mode: EvaluationRequestMode,
        stage_task_candidates: bool,
    },
    Dirty,
}

pub struct TransactionExecutionRequest<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    targets: Vec<NodeId>,
    request_mode: EvaluationRequestMode,
    executor: Option<StageExecutor>,
}

impl<'tx, 'a, D, I, E, Ctx, T> TransactionExecutionRequest<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub(super) fn new(
        tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
        targets: Vec<NodeId>,
        request_mode: EvaluationRequestMode,
    ) -> Self {
        Self {
            tx,
            targets,
            request_mode,
            executor: None,
        }
    }

    pub fn on_demand(mut self) -> Self {
        self.request_mode = EvaluationRequestMode::ForceOnDemand;
        self
    }

    pub fn with_mode(mut self, request_mode: EvaluationRequestMode) -> Self {
        self.request_mode = request_mode;
        self
    }

    pub fn with_executor(mut self, executor: StageExecutor) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn run<F, O>(self, evaluator: &F) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let executor = self
            .executor
            .unwrap_or_else(|| executor_for_strategy(self.tx.graph.derive_evaluation_strategy()));
        self.tx.execute_evaluation(
            TransactionExecutionIntent::Targets {
                targets: &self.targets,
                request_mode: self.request_mode,
                stage_task_candidates: false,
            },
            evaluator,
            executor,
        )
    }

    pub fn read<F, O>(self, evaluator: &F) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let [node] = self.targets.as_slice() else {
            return Err(SignalError::invalid_input(
                "guided read requires exactly one target; use read_many for multiple targets",
            ));
        };
        let executor = self
            .executor
            .unwrap_or_else(|| executor_for_strategy(self.tx.graph.derive_evaluation_strategy()));
        if !matches!(
            self.tx.graph.get_state(*node)?,
            crate::data::node::NodeState::Clean
        ) {
            self.tx.execute_evaluation(
                TransactionExecutionIntent::Targets {
                    targets: &self.targets,
                    request_mode: self.request_mode,
                    stage_task_candidates: false,
                },
                evaluator,
                executor,
            )?;
        }
        self.tx.graph.node_aspect_version(*node)
    }

    pub fn read_many<F, O>(self, evaluator: &F) -> Result<Vec<AspectVersion>, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let pending = self
            .targets
            .iter()
            .copied()
            .filter(|node| {
                !matches!(
                    self.tx.graph.get_state(*node),
                    Ok(crate::data::node::NodeState::Clean)
                )
            })
            .collect::<Vec<_>>();
        if !pending.is_empty() {
            let executor = self.executor.unwrap_or_else(|| {
                executor_for_strategy(self.tx.graph.derive_evaluation_strategy())
            });
            self.tx.execute_evaluation(
                TransactionExecutionIntent::Targets {
                    targets: &pending,
                    request_mode: self.request_mode,
                    stage_task_candidates: false,
                },
                evaluator,
                executor,
            )?;
        }
        self.targets
            .into_iter()
            .map(|node| self.tx.graph.node_aspect_version(node))
            .collect()
    }
}
