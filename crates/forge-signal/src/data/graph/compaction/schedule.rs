use super::super::signal_graph::SignalGraph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::data::graph) enum CompactionFamily {
    DependencyEdges,
    Subscribers,
    Snapshots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::data::graph) struct CompactionEpochPlan {
    cursor_start: u8,
    family_budget: u8,
}

impl CompactionEpochPlan {
    pub(in crate::data::graph) fn family_budget(self) -> u8 {
        self.family_budget
    }

    pub(in crate::data::graph) fn families(self) -> impl Iterator<Item = CompactionFamily> {
        (0..self.family_budget).map(move |offset| match (self.cursor_start + offset) % 3 {
            0 => CompactionFamily::DependencyEdges,
            1 => CompactionFamily::Subscribers,
            _ => CompactionFamily::Snapshots,
        })
    }
}

impl SignalGraph {
    pub(in crate::data::graph) fn plan_compaction_epoch(&mut self) -> Option<CompactionEpochPlan> {
        if !self.should_run_compaction_epoch() {
            return None;
        }
        let plan = CompactionEpochPlan {
            cursor_start: self.compaction.cursor,
            family_budget: self.compaction_epoch_budget(),
        };
        self.compaction.cursor = (self.compaction.cursor + plan.family_budget) % 3;
        Some(plan)
    }
}
