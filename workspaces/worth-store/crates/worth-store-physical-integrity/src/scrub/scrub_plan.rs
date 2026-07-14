use crate::{
    ChunkIntegrityStreamingWindow, IntegrityEntryWitness, OfflineScrubInspectionInput,
    ScrubCounterSnapshot, ScrubMode, ScrubOverBudgetClass, ScrubPlanDenial, ScrubPlanDenialKind,
    ScrubPlanningMemoryEnvelope, ScrubWindow, ScrubWindowSource,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanBudget {
    resident_byte_limit: u64,
    pin_page_limit: u32,
    allocation_byte_limit: u64,
    streaming_window_byte_limit: u64,
    protected_read_limit: u64,
}

impl ScrubPlanBudget {
    pub fn new(
        witness: IntegrityEntryWitness,
        scrub_envelope: ScrubPlanningMemoryEnvelope,
        streaming_window: ChunkIntegrityStreamingWindow,
    ) -> Self {
        let witness_scrub_limit = witness.scrub_envelope_limits().allocation_bytes();
        let envelope_scrub_limit = scrub_envelope.allocation_bytes();
        let witness_pin_limit = witness.verifier_resident_limits().pinned_pages();
        let envelope_pin_limit = scrub_envelope.pinned_pages();
        Self {
            resident_byte_limit: witness.verifier_resident_limits().resident_bytes(),
            pin_page_limit: witness_pin_limit.min(envelope_pin_limit),
            allocation_byte_limit: witness_scrub_limit.min(envelope_scrub_limit),
            streaming_window_byte_limit: streaming_window.window_bytes(),
            protected_read_limit: witness.entry_basis().protected_view_count() as u64,
        }
    }

    pub fn constrained_by_policy(
        self,
        resident_byte_limit: u64,
        pin_page_limit: u32,
        allocation_byte_limit: u64,
        streaming_window_byte_limit: u64,
        protected_read_limit: u64,
    ) -> Self {
        Self {
            resident_byte_limit: self.resident_byte_limit.min(resident_byte_limit),
            pin_page_limit: self.pin_page_limit.min(pin_page_limit),
            allocation_byte_limit: self.allocation_byte_limit.min(allocation_byte_limit),
            streaming_window_byte_limit: self
                .streaming_window_byte_limit
                .min(streaming_window_byte_limit),
            protected_read_limit: self.protected_read_limit.min(protected_read_limit),
        }
    }

    pub const fn resident_byte_limit(self) -> u64 {
        self.resident_byte_limit
    }

    pub const fn pin_page_limit(self) -> u32 {
        self.pin_page_limit
    }

    pub const fn allocation_byte_limit(self) -> u64 {
        self.allocation_byte_limit
    }

    pub const fn streaming_window_byte_limit(self) -> u64 {
        self.streaming_window_byte_limit
    }

    pub const fn protected_read_limit(self) -> u64 {
        self.protected_read_limit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannedScrubWindowStatus {
    Inspect,
    Skip,
    DeferOverBudget(ScrubOverBudgetClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedScrubWindow<'lease> {
    window: ScrubWindow<'lease>,
    status: PlannedScrubWindowStatus,
}

impl<'lease> PlannedScrubWindow<'lease> {
    pub const fn window(self) -> ScrubWindow<'lease> {
        self.window
    }

    pub const fn status(self) -> PlannedScrubWindowStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrubPlanRequest<'lease> {
    mode: ScrubMode,
    windows: Vec<ScrubWindow<'lease>>,
    budget: ScrubPlanBudget,
    defer_over_budget_windows: bool,
    yield_after_windows: Option<u64>,
    skipped_ordinals: Vec<crate::ScrubWindowOrdinal>,
}

impl<'lease> ScrubPlanRequest<'lease> {
    pub fn online(windows: Vec<ScrubWindow<'lease>>, budget: ScrubPlanBudget) -> Self {
        Self::new(ScrubMode::Online, windows, budget)
    }

    pub fn offline(input: OfflineScrubInspectionInput<'lease>, budget: ScrubPlanBudget) -> Self {
        Self::new(ScrubMode::Offline, input.windows().to_vec(), budget)
    }

    pub fn with_deferred_over_budget_windows(mut self) -> Self {
        self.defer_over_budget_windows = true;
        self
    }

    pub fn with_yield_after_windows(mut self, windows: u64) -> Self {
        self.yield_after_windows = Some(windows);
        self
    }

    pub fn with_skipped_window(mut self, ordinal: crate::ScrubWindowOrdinal) -> Self {
        self.skipped_ordinals.push(ordinal);
        self
    }

    fn new(mode: ScrubMode, windows: Vec<ScrubWindow<'lease>>, budget: ScrubPlanBudget) -> Self {
        Self {
            mode,
            windows,
            budget,
            defer_over_budget_windows: false,
            yield_after_windows: None,
            skipped_ordinals: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrubPlan<'lease> {
    mode: ScrubMode,
    windows: Vec<PlannedScrubWindow<'lease>>,
    budget: ScrubPlanBudget,
    yield_after_windows: Option<u64>,
    plan_identity: u64,
}

impl<'lease> ScrubPlan<'lease> {
    pub fn build(request: ScrubPlanRequest<'lease>) -> Result<Self, ScrubPlanDenial> {
        if request.yield_after_windows == Some(0) {
            return Err(ScrubPlanDenial::new(
                ScrubPlanDenialKind::ZeroYieldWindowBudget,
                ScrubCounterSnapshot::empty(),
            ));
        }
        if request.windows.is_empty() {
            return Err(ScrubPlanDenial::new(
                ScrubPlanDenialKind::EmptyWindowSet,
                ScrubCounterSnapshot::empty(),
            ));
        }

        let mut cumulative_bytes = 0u64;
        let mut planned = Vec::with_capacity(request.windows.len());
        for (index, window) in request.windows.iter().copied().enumerate() {
            let status = if request.skipped_ordinals.contains(&window.ordinal()) {
                PlannedScrubWindowStatus::Skip
            } else {
                classify_window_for_budget(
                    index as u64 + 1,
                    window,
                    cumulative_bytes,
                    request.budget,
                )?
            };
            if status == PlannedScrubWindowStatus::Inspect {
                cumulative_bytes += window.len_bytes();
            } else if matches!(status, PlannedScrubWindowStatus::DeferOverBudget(_))
                && !request.defer_over_budget_windows
            {
                return Err(denial_for_status(status, window, request.budget));
            }
            planned.push(PlannedScrubWindow { window, status });
        }

        Ok(Self {
            mode: request.mode,
            plan_identity: super::scrub_plan_identity::scrub_plan_identity(
                request.mode,
                request.budget,
                request.yield_after_windows,
                &planned,
            ),
            windows: planned,
            budget: request.budget,
            yield_after_windows: request.yield_after_windows,
        })
    }

    pub const fn mode(&self) -> ScrubMode {
        self.mode
    }

    pub fn windows(&self) -> &[PlannedScrubWindow<'lease>] {
        &self.windows
    }

    pub const fn budget(&self) -> ScrubPlanBudget {
        self.budget
    }

    pub const fn yield_after_windows(&self) -> Option<u64> {
        self.yield_after_windows
    }

    pub const fn plan_identity(&self) -> u64 {
        self.plan_identity
    }
}

fn classify_window_for_budget(
    ordinal_read_count: u64,
    window: ScrubWindow<'_>,
    cumulative_bytes: u64,
    budget: ScrubPlanBudget,
) -> Result<PlannedScrubWindowStatus, ScrubPlanDenial> {
    if window.is_empty() {
        return Err(ScrubPlanDenial::new(
            ScrubPlanDenialKind::EmptyWindow {
                ordinal: window.ordinal(),
            },
            ScrubCounterSnapshot::empty(),
        ));
    }
    if window.source() == ScrubWindowSource::OnlineProtectedRead
        && ordinal_read_count > budget.protected_read_limit()
    {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::ProtectedRead,
        ));
    }
    if budget.pin_page_limit() == 0 {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::PinPage,
        ));
    }
    if window.len_bytes() > budget.streaming_window_byte_limit() {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::StreamingWindow,
        ));
    }
    let requested_bytes = cumulative_bytes + window.len_bytes();
    if requested_bytes > budget.resident_byte_limit() {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::ResidentMemory,
        ));
    }
    if requested_bytes > budget.allocation_byte_limit() {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::Allocation,
        ));
    }
    Ok(PlannedScrubWindowStatus::Inspect)
}

fn denial_for_status(
    status: PlannedScrubWindowStatus,
    window: ScrubWindow<'_>,
    budget: ScrubPlanBudget,
) -> ScrubPlanDenial {
    let kind = match status {
        PlannedScrubWindowStatus::Inspect | PlannedScrubWindowStatus::Skip => unreachable!(),
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ProtectedRead) => {
            ScrubPlanDenialKind::ProtectedReadLimitExceeded {
                requested: window.ordinal().get() + 1,
                limit: budget.protected_read_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::StreamingWindow) => {
            ScrubPlanDenialKind::StreamingWindowLimitExceeded {
                requested: window.len_bytes(),
                limit: budget.streaming_window_byte_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ResidentMemory) => {
            ScrubPlanDenialKind::ResidentMemoryLimitExceeded {
                requested: window.len_bytes(),
                limit: budget.resident_byte_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::PinPage) => {
            ScrubPlanDenialKind::PinPageLimitExceeded {
                requested: 1,
                limit: budget.pin_page_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::Allocation) => {
            ScrubPlanDenialKind::AllocationLimitExceeded {
                requested: window.len_bytes(),
                limit: budget.allocation_byte_limit(),
            }
        }
    };
    ScrubPlanDenial::new(kind, ScrubCounterSnapshot::empty())
}
