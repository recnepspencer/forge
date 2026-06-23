use crate::runtime::WorthUiQueryRuntimeFactLoweringInput;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryRuntimeFactLoweringCounters {
    bindings_compared: usize,
    live_rebind_entries: usize,
    consumed_projection_fact_count: usize,
    consumed_state_snapshot_count: usize,
    consumed_effect_posture_count: usize,
    virtualized_frame_target_count: usize,
    changed_fact_count: usize,
    support_denial_count: usize,
}

impl WorthUiQueryRuntimeFactLoweringCounters {
    pub(super) fn from_input(
        input: &WorthUiQueryRuntimeFactLoweringInput,
        changed_fact_count: usize,
        support_denial_count: usize,
        query_proofs_consumed: bool,
    ) -> Self {
        Self {
            bindings_compared: input.binding_comparison().counters().bindings_compared(),
            live_rebind_entries: input.live_rebind_plan().entries().len(),
            consumed_projection_fact_count: consumed_count(
                query_proofs_consumed,
                input.projection_fact_receipts().len(),
            ),
            consumed_state_snapshot_count: consumed_count(
                query_proofs_consumed,
                input.state_snapshot_receipts().len(),
            ),
            consumed_effect_posture_count: consumed_count(
                query_proofs_consumed,
                input.effect_posture_receipts().len(),
            ),
            virtualized_frame_target_count: consumed_count(
                query_proofs_consumed,
                input.virtualized_frame_targets().len(),
            ),
            changed_fact_count,
            support_denial_count,
        }
    }

    pub fn bindings_compared(self) -> usize {
        self.bindings_compared
    }

    pub fn live_rebind_entries(self) -> usize {
        self.live_rebind_entries
    }

    pub fn consumed_projection_fact_count(self) -> usize {
        self.consumed_projection_fact_count
    }

    pub fn consumed_state_snapshot_count(self) -> usize {
        self.consumed_state_snapshot_count
    }

    pub fn consumed_effect_posture_count(self) -> usize {
        self.consumed_effect_posture_count
    }

    pub fn virtualized_frame_target_count(self) -> usize {
        self.virtualized_frame_target_count
    }

    pub fn changed_fact_count(self) -> usize {
        self.changed_fact_count
    }

    pub fn support_denial_count(self) -> usize {
        self.support_denial_count
    }
}

fn consumed_count(query_proofs_consumed: bool, input_count: usize) -> usize {
    if query_proofs_consumed {
        input_count
    } else {
        0
    }
}
