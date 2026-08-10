use std::fmt;

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::logic::evaluation::EvaluationRequestMode;

use super::task::{EligibleTask, StageBarrier};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionStage {
    pub index: u32,
    pub tasks: Vec<EligibleTask>,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StageCursor {
    pub index: u32,
    pub start: usize,
    pub end: usize,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanSummary {
    pub requested_target_count: u32,
    pub stage_count: u32,
    pub task_count: u32,
    pub max_stage_width: u32,
    pub contract_pruned_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPlan {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub stages: Vec<ExecutionStage>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EvaluationCursor {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub tasks: Vec<EligibleTask>,
    pub stages: Vec<StageCursor>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone)]
pub(crate) struct SessionScratch<'a> {
    pub targets: &'a [NodeId],
    pub tasks: &'a [EligibleTask],
    pub stages: &'a [StageCursor],
    pub summary: PlanSummary,
}

impl fmt::Display for PlanSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "targets={} stages={} tasks={} max_stage_width={} contract_pruned={}",
            self.requested_target_count,
            self.stage_count,
            self.task_count,
            self.max_stage_width,
            self.contract_pruned_count
        )
    }
}

impl fmt::Display for EvaluationPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EvaluationPlan {}", self.summary)?;
        for stage in &self.stages {
            writeln!(f, "  stage {} tasks={}", stage.index, stage.tasks.len())?;
            for task in &stage.tasks {
                writeln!(
                    f,
                    "    {} direct={} reason={:?} mode={:?}",
                    task.node, task.direct_request, task.reason, task.request_mode
                )?;
            }
        }
        Ok(())
    }
}
