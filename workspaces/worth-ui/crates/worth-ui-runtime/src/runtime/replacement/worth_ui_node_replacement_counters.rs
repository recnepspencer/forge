#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiNodeReplacementCounters {
    active_nodes_classified: usize,
    candidate_nodes_classified: usize,
    preserved_node_count: usize,
    replaced_node_count: usize,
    dropped_node_count: usize,
    created_node_count: usize,
    moved_node_count: usize,
    rebound_node_count: usize,
    lane_changed_node_count: usize,
    ambiguous_node_count: usize,
}

impl WorthUiNodeReplacementCounters {
    pub(crate) fn record_active_node_classified(&mut self) {
        self.active_nodes_classified += 1;
    }

    pub(crate) fn record_candidate_node_classified(&mut self) {
        self.candidate_nodes_classified += 1;
    }

    pub(crate) fn record_transition(
        &mut self,
        transition: crate::runtime::WorthUiNodeLifecycleTransition,
    ) {
        match transition {
            crate::runtime::WorthUiNodeLifecycleTransition::Preserve => {
                self.preserved_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::Replace => {
                self.replaced_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::Drop => {
                self.dropped_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::Create => {
                self.created_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::Move => {
                self.moved_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::Rebind => {
                self.rebound_node_count += 1;
            }
            crate::runtime::WorthUiNodeLifecycleTransition::LaneChange => {
                self.lane_changed_node_count += 1;
            }
        }
    }

    pub(crate) fn record_ambiguous_node(&mut self) {
        self.ambiguous_node_count += 1;
    }

    pub fn active_nodes_classified(&self) -> usize {
        self.active_nodes_classified
    }

    pub fn candidate_nodes_classified(&self) -> usize {
        self.candidate_nodes_classified
    }

    pub fn preserved_node_count(&self) -> usize {
        self.preserved_node_count
    }

    pub fn replaced_node_count(&self) -> usize {
        self.replaced_node_count
    }

    pub fn dropped_node_count(&self) -> usize {
        self.dropped_node_count
    }

    pub fn created_node_count(&self) -> usize {
        self.created_node_count
    }

    pub fn moved_node_count(&self) -> usize {
        self.moved_node_count
    }

    pub fn rebound_node_count(&self) -> usize {
        self.rebound_node_count
    }

    pub fn lane_changed_node_count(&self) -> usize {
        self.lane_changed_node_count
    }

    pub fn ambiguous_node_count(&self) -> usize {
        self.ambiguous_node_count
    }
}
