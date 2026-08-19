//! Translate harness profiles and observations at the presentation boundary.

use worth_harness::facade::{DiagnosticsLevel, ExecutionMode, ObservationStatus};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::diagnostics::profile::DiagnosticsTier;
use crate::logic::planner::{EvaluationPlan, StageExecutor};
use crate::runtime_policy::SignalRuntimePolicy;

use super::SignalHarnessBridge;

impl SignalHarnessBridge {
    #[cfg(test)]
    pub(super) fn requires_condition_aware_execution(
        graph: &SignalGraph,
        plan: &EvaluationPlan,
    ) -> Result<bool, SignalError> {
        for task in plan.stages.iter().flat_map(|stage| &stage.tasks) {
            let config = graph.node_eval_config(task.node)?;
            if !matches!(config.condition, EvaluationCondition::Always)
                || config.comparator.is_some()
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(in crate::presentation::harness) fn diagnostics_profile(
        level: DiagnosticsLevel,
    ) -> DiagnosticsTier {
        match level {
            DiagnosticsLevel::Off | DiagnosticsLevel::Operational => DiagnosticsTier::Operational,
            DiagnosticsLevel::Development => DiagnosticsTier::Development,
            DiagnosticsLevel::Forensic => DiagnosticsTier::Forensic,
        }
    }

    pub(in crate::presentation::harness) fn runtime_policy(
        level: DiagnosticsLevel,
    ) -> SignalRuntimePolicy {
        SignalRuntimePolicy::for_tier(Self::diagnostics_profile(level))
    }

    pub(super) fn executor(mode: ExecutionMode) -> Result<StageExecutor, SignalError> {
        match mode {
            ExecutionMode::RuntimeDefault | ExecutionMode::Serial => Ok(StageExecutor::Serial),
            ExecutionMode::StagedParallel => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::staged_parallel_precompute(2))
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(super::error_mapping::staged_parallel_unavailable())
                }
            }
            ExecutionMode::FullParallel => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::full_parallel(2))
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(super::error_mapping::full_parallel_unavailable())
                }
            }
        }
    }

    pub(super) fn observation_status(state: NodeState) -> ObservationStatus {
        match state {
            NodeState::Clean => ObservationStatus::Clean,
            NodeState::MaybeStale => ObservationStatus::MaybeStale,
            NodeState::Dirty => ObservationStatus::Dirty,
        }
    }
}
