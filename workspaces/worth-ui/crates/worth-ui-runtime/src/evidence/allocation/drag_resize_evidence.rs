#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDragResizeStrategy {
    LatestWinsPerResolvedFrame,
    TerminalDurableCommit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDragResizeEvidence {
    strategy: UiDragResizeStrategy,
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    admitted_samples: u16,
    preview_publications: u16,
    durable_mutations: u16,
    committed_receipts: u16,
}

impl UiDragResizeEvidence {
    pub(crate) fn new(
        strategy: UiDragResizeStrategy,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        counters: crate::runtime::UiDragResizeCounters,
    ) -> Self {
        Self {
            strategy,
            frame_epoch,
            admitted_samples: counters.admitted_samples(),
            preview_publications: counters.preview_publications(),
            durable_mutations: counters.durable_mutations(),
            committed_receipts: counters.committed_receipts(),
        }
    }
    pub fn strategy(&self) -> UiDragResizeStrategy {
        self.strategy
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn admitted_samples(&self) -> u16 {
        self.admitted_samples
    }
    pub fn preview_publications(&self) -> u16 {
        self.preview_publications
    }
    pub fn durable_mutations(&self) -> u16 {
        self.durable_mutations
    }
    pub fn committed_receipts(&self) -> u16 {
        self.committed_receipts
    }
}
