use std::collections::BTreeSet;

use crate::data::handle::NodeId;

use super::apply::TaskPatch;
use super::types::ParallelExecutionPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PatchFootprint {
    target_nodes: BTreeSet<NodeId>,
}

impl PatchFootprint {
    pub(super) fn from_task(task: &TaskPatch) -> Self {
        let mut target_nodes = BTreeSet::new();
        target_nodes.insert(task.node);
        target_nodes.extend(task.current_dependencies.iter().map(|edge| edge.source()));
        target_nodes.extend(task.next_dependencies.iter().map(|edge| edge.source()));
        Self { target_nodes }
    }

    fn conflicts_with(&self, other: &Self) -> bool {
        self.target_nodes
            .iter()
            .any(|node| other.target_nodes.contains(node))
    }

    pub(super) fn conflicts_with_nodes(&self, nodes: &BTreeSet<NodeId>) -> bool {
        self.target_nodes.iter().any(|node| nodes.contains(node))
    }

    fn merge(&mut self, other: &Self) {
        self.target_nodes.extend(other.target_nodes.iter().copied());
    }
}

#[derive(Debug, Clone)]
pub(super) struct ApplyGroup {
    pub tasks: Vec<TaskPatch>,
    pub footprint: PatchFootprint,
}

#[derive(Debug, Clone)]
pub(super) struct StageApplyPlan {
    pub groups: Vec<ApplyGroup>,
}

pub(super) fn build_stage_apply_plan(
    tasks: Vec<TaskPatch>,
    policy: ParallelExecutionPolicy,
) -> StageApplyPlan {
    if tasks.is_empty() {
        return StageApplyPlan { groups: Vec::new() };
    }

    let max_groups = policy.max_apply_group_count_for(tasks.len());
    let chunk_size = tasks.len().div_ceil(max_groups).max(1);
    let mut groups = Vec::<ApplyGroup>::new();

    for task in tasks {
        let footprint = PatchFootprint::from_task(&task);
        let mut placed = false;
        for group in &mut groups {
            if group.tasks.len() >= chunk_size {
                continue;
            }
            if group.footprint.conflicts_with(&footprint) {
                continue;
            }
            group.footprint.merge(&footprint);
            group.tasks.push(task.clone());
            placed = true;
            break;
        }
        if !placed {
            groups.push(ApplyGroup {
                tasks: vec![task],
                footprint,
            });
        }
    }

    StageApplyPlan { groups }
}
