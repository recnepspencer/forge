use crate::runtime::{
    WorthUiLiveViewConditionalProjectionRebindReceipt,
    WorthUiLiveViewControlProjectionRebindReceipt, WorthUiLiveViewProjectionAdmissionReceipt,
    WorthUiRuntimeHost,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewProjectionRebindCounters {
    prior_control_count: usize,
    next_control_count: usize,
    prior_conditional_count: usize,
    next_conditional_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewProjectionRebindReceipt {
    control_rebind: WorthUiLiveViewControlProjectionRebindReceipt,
    conditional_rebind: WorthUiLiveViewConditionalProjectionRebindReceipt,
    counters: WorthUiLiveViewProjectionRebindCounters,
}

impl WorthUiRuntimeHost {
    pub fn rebind_live_view_projections(
        &self,
        prior: &WorthUiLiveViewProjectionAdmissionReceipt,
        next: &WorthUiLiveViewProjectionAdmissionReceipt,
    ) -> WorthUiLiveViewProjectionRebindReceipt {
        WorthUiLiveViewProjectionRebindReceipt::from_projection_admission_receipts(prior, next)
    }
}

impl WorthUiLiveViewProjectionRebindReceipt {
    fn from_projection_admission_receipts(
        prior: &WorthUiLiveViewProjectionAdmissionReceipt,
        next: &WorthUiLiveViewProjectionAdmissionReceipt,
    ) -> Self {
        let control_rebind =
            WorthUiLiveViewControlProjectionRebindReceipt::from_control_projection_receipts(
                prior.controls(),
                next.controls(),
            );
        let conditional_rebind =
            WorthUiLiveViewConditionalProjectionRebindReceipt::from_conditional_projection_receipts(
                prior.conditionals(),
                next.conditionals(),
            );
        let counters = WorthUiLiveViewProjectionRebindCounters {
            prior_control_count: prior.controls().len(),
            next_control_count: next.controls().len(),
            prior_conditional_count: prior.conditionals().len(),
            next_conditional_count: next.conditionals().len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        Self {
            control_rebind,
            conditional_rebind,
            counters,
        }
    }

    pub fn control_rebind(&self) -> &WorthUiLiveViewControlProjectionRebindReceipt {
        &self.control_rebind
    }

    pub fn conditional_rebind(&self) -> &WorthUiLiveViewConditionalProjectionRebindReceipt {
        &self.conditional_rebind
    }

    pub fn counters(&self) -> WorthUiLiveViewProjectionRebindCounters {
        self.counters
    }
}

impl WorthUiLiveViewProjectionRebindCounters {
    pub fn prior_control_count(self) -> usize {
        self.prior_control_count
    }

    pub fn next_control_count(self) -> usize {
        self.next_control_count
    }

    pub fn prior_conditional_count(self) -> usize {
        self.prior_conditional_count
    }

    pub fn next_conditional_count(self) -> usize {
        self.next_conditional_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
