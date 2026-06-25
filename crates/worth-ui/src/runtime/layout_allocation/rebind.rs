use crate::runtime::{
    WorthUiLayoutAllocationReceipt, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeHost,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLayoutAllocationRebindCounters {
    prior_child_count: usize,
    next_child_count: usize,
    changed_layout_fact_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
    artifact_scan_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutAllocationRebindReceipt {
    root_node_id: String,
    changed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiLayoutAllocationRebindCounters,
}

impl WorthUiRuntimeHost {
    pub fn rebind_layout_allocation(
        &self,
        prior: &WorthUiLayoutAllocationReceipt,
        next: &WorthUiLayoutAllocationReceipt,
    ) -> WorthUiLayoutAllocationRebindReceipt {
        WorthUiLayoutAllocationRebindReceipt::from_allocations(prior, next)
    }
}

impl WorthUiLayoutAllocationRebindReceipt {
    fn from_allocations(
        prior: &WorthUiLayoutAllocationReceipt,
        next: &WorthUiLayoutAllocationReceipt,
    ) -> Self {
        let mut changed_facts = Vec::new();
        if prior.receipt_digest() != next.receipt_digest() {
            changed_facts.push(WorthUiRuntimeFactId::layout_allocation(format!(
                "{}:{}",
                next.root_node_id(),
                next.host_measurement_basis_digest()
            )));
        }
        let counters = WorthUiLayoutAllocationRebindCounters {
            prior_child_count: prior.children().len(),
            next_child_count: next.children().len(),
            changed_layout_fact_count: changed_facts.len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
            artifact_scan_count: 0,
        };
        Self {
            root_node_id: next.root_node_id().to_owned(),
            changed_facts,
            counters,
        }
    }

    pub fn root_node_id(&self) -> &str {
        &self.root_node_id
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn changed_fact_families(&self) -> impl Iterator<Item = WorthUiRuntimeFactFamily> + '_ {
        self.changed_facts.iter().map(WorthUiRuntimeFactId::family)
    }

    pub fn counters(&self) -> WorthUiLayoutAllocationRebindCounters {
        self.counters
    }
}

impl WorthUiLayoutAllocationRebindCounters {
    pub fn prior_child_count(self) -> usize {
        self.prior_child_count
    }

    pub fn next_child_count(self) -> usize {
        self.next_child_count
    }

    pub fn changed_layout_fact_count(self) -> usize {
        self.changed_layout_fact_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }

    pub fn artifact_scan_count(self) -> usize {
        self.artifact_scan_count
    }
}
