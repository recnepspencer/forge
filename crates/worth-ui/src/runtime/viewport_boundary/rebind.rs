use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeHost, WorthUiViewportBoundaryReceipt};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiViewportBoundaryRebindCounters {
    prior_boundary_count: usize,
    next_boundary_count: usize,
    changed_viewport_fact_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
    artifact_scan_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiViewportBoundaryRebindReceipt {
    changed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiViewportBoundaryRebindCounters,
}

impl WorthUiRuntimeHost {
    pub fn rebind_viewport_boundaries(
        &self,
        prior: &WorthUiViewportBoundaryReceipt,
        next: &WorthUiViewportBoundaryReceipt,
    ) -> WorthUiViewportBoundaryRebindReceipt {
        WorthUiViewportBoundaryRebindReceipt::from_receipts(prior, next)
    }
}

impl WorthUiViewportBoundaryRebindReceipt {
    fn from_receipts(
        prior: &WorthUiViewportBoundaryReceipt,
        next: &WorthUiViewportBoundaryReceipt,
    ) -> Self {
        let changed_facts = if prior.receipt_digest() == next.receipt_digest() {
            Vec::new()
        } else {
            next.consumed_facts()
                .iter()
                .filter(|fact| viewport_family(fact))
                .cloned()
                .collect()
        };
        let counters = WorthUiViewportBoundaryRebindCounters {
            prior_boundary_count: prior.boundaries().len(),
            next_boundary_count: next.boundaries().len(),
            changed_viewport_fact_count: changed_facts.len(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
            artifact_scan_count: 0,
        };
        Self {
            changed_facts,
            counters,
        }
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn counters(&self) -> WorthUiViewportBoundaryRebindCounters {
        self.counters
    }
}

impl WorthUiViewportBoundaryRebindCounters {
    pub fn prior_boundary_count(self) -> usize {
        self.prior_boundary_count
    }

    pub fn next_boundary_count(self) -> usize {
        self.next_boundary_count
    }

    pub fn changed_viewport_fact_count(self) -> usize {
        self.changed_viewport_fact_count
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

fn viewport_family(fact: &WorthUiRuntimeFactId) -> bool {
    matches!(
        fact.family(),
        crate::runtime::WorthUiRuntimeFactFamily::ViewportBoundary
            | crate::runtime::WorthUiRuntimeFactFamily::ClipBoundary
            | crate::runtime::WorthUiRuntimeFactFamily::ScrollRestoration
            | crate::runtime::WorthUiRuntimeFactFamily::ViewportEventParticipation
    )
}
