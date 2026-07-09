use worth_foundational::{FoundationalPerformanceBudgetKind, FoundationalPolicyAdmissionReceipt};

use crate::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit, IoResourceUnitKind,
    QueueSlot, ReadAheadWindow, ReclaimPermit, SyncDebt, WorkerPermit, WriteBackWindow,
};

use super::BackgroundPacingDenial;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BackgroundResourceBudget {
    queue_slots: u64,
    bandwidth_tokens: u64,
    flush_permits: u64,
    sync_debt: u64,
    read_ahead_window: u64,
    write_back_window: u64,
    dirty_page_budget: u64,
    worker_permits: u64,
    cache_residency_hints: u64,
    reclaim_permits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundResourceShortfall {
    Unit {
        unit: IoResourceUnitKind,
        requested: u64,
        available: u64,
    },
}

impl BackgroundResourceBudget {
    pub const fn new() -> Self {
        Self {
            queue_slots: 0,
            bandwidth_tokens: 0,
            flush_permits: 0,
            sync_debt: 0,
            read_ahead_window: 0,
            write_back_window: 0,
            dirty_page_budget: 0,
            worker_permits: 0,
            cache_residency_hints: 0,
            reclaim_permits: 0,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.queue_slots == 0
            && self.bandwidth_tokens == 0
            && self.flush_permits == 0
            && self.sync_debt == 0
            && self.read_ahead_window == 0
            && self.write_back_window == 0
            && self.dirty_page_budget == 0
            && self.worker_permits == 0
            && self.cache_residency_hints == 0
            && self.reclaim_permits == 0
    }

    pub const fn amount_for(self, unit: IoResourceUnitKind) -> u64 {
        match unit {
            IoResourceUnitKind::QueueSlot => self.queue_slots,
            IoResourceUnitKind::BandwidthToken => self.bandwidth_tokens,
            IoResourceUnitKind::FlushPermit => self.flush_permits,
            IoResourceUnitKind::SyncDebt => self.sync_debt,
            IoResourceUnitKind::ReadAheadWindow => self.read_ahead_window,
            IoResourceUnitKind::WriteBackWindow => self.write_back_window,
            IoResourceUnitKind::DirtyPageBudget => self.dirty_page_budget,
            IoResourceUnitKind::WorkerPermit => self.worker_permits,
            IoResourceUnitKind::CacheResidencyHint => self.cache_residency_hints,
            IoResourceUnitKind::ReclaimPermit => self.reclaim_permits,
        }
    }

    pub const fn with_queue_slots(mut self, unit: QueueSlot) -> Self {
        self.queue_slots = unit.get();
        self
    }

    pub const fn with_bandwidth(mut self, unit: BandwidthToken) -> Self {
        self.bandwidth_tokens = unit.get();
        self
    }

    pub const fn with_flush_permits(mut self, unit: FlushPermit) -> Self {
        self.flush_permits = unit.get();
        self
    }

    pub const fn with_sync_debt(mut self, unit: SyncDebt) -> Self {
        self.sync_debt = unit.get();
        self
    }

    pub const fn with_read_ahead(mut self, unit: ReadAheadWindow) -> Self {
        self.read_ahead_window = unit.get();
        self
    }

    pub const fn with_write_back(mut self, unit: WriteBackWindow) -> Self {
        self.write_back_window = unit.get();
        self
    }

    pub const fn with_dirty_pages(mut self, unit: DirtyPageBudget) -> Self {
        self.dirty_page_budget = unit.get();
        self
    }

    pub const fn with_worker_permits(mut self, unit: WorkerPermit) -> Self {
        self.worker_permits = unit.get();
        self
    }

    pub const fn with_cache_residency(mut self, unit: CacheResidencyHint) -> Self {
        self.cache_residency_hints = unit.get();
        self
    }

    pub const fn with_reclaim_permits(mut self, unit: ReclaimPermit) -> Self {
        self.reclaim_permits = unit.get();
        self
    }

    pub(crate) const fn min_with(self, available: Self) -> Self {
        Self {
            queue_slots: min(self.queue_slots, available.queue_slots),
            bandwidth_tokens: min(self.bandwidth_tokens, available.bandwidth_tokens),
            flush_permits: min(self.flush_permits, available.flush_permits),
            sync_debt: min(self.sync_debt, available.sync_debt),
            read_ahead_window: min(self.read_ahead_window, available.read_ahead_window),
            write_back_window: min(self.write_back_window, available.write_back_window),
            dirty_page_budget: min(self.dirty_page_budget, available.dirty_page_budget),
            worker_permits: min(self.worker_permits, available.worker_permits),
            cache_residency_hints: min(self.cache_residency_hints, available.cache_residency_hints),
            reclaim_permits: min(self.reclaim_permits, available.reclaim_permits),
        }
    }

    pub(crate) const fn debt_after(self, admitted: Self) -> Self {
        Self {
            queue_slots: self.queue_slots.saturating_sub(admitted.queue_slots),
            bandwidth_tokens: self
                .bandwidth_tokens
                .saturating_sub(admitted.bandwidth_tokens),
            flush_permits: self.flush_permits.saturating_sub(admitted.flush_permits),
            sync_debt: self.sync_debt.saturating_sub(admitted.sync_debt),
            read_ahead_window: self
                .read_ahead_window
                .saturating_sub(admitted.read_ahead_window),
            write_back_window: self
                .write_back_window
                .saturating_sub(admitted.write_back_window),
            dirty_page_budget: self
                .dirty_page_budget
                .saturating_sub(admitted.dirty_page_budget),
            worker_permits: self.worker_permits.saturating_sub(admitted.worker_permits),
            cache_residency_hints: self
                .cache_residency_hints
                .saturating_sub(admitted.cache_residency_hints),
            reclaim_permits: self
                .reclaim_permits
                .saturating_sub(admitted.reclaim_permits),
        }
    }

    pub const fn queue_slots(self) -> u64 {
        self.queue_slots
    }
    pub const fn bandwidth_tokens(self) -> u64 {
        self.bandwidth_tokens
    }
    pub const fn flush_permits(self) -> u64 {
        self.flush_permits
    }
    pub const fn sync_debt(self) -> u64 {
        self.sync_debt
    }
    pub const fn read_ahead_window(self) -> u64 {
        self.read_ahead_window
    }
    pub const fn write_back_window(self) -> u64 {
        self.write_back_window
    }
    pub const fn dirty_page_budget(self) -> u64 {
        self.dirty_page_budget
    }
    pub const fn worker_permits(self) -> u64 {
        self.worker_permits
    }
    pub const fn cache_residency_hints(self) -> u64 {
        self.cache_residency_hints
    }
    pub const fn reclaim_permits(self) -> u64 {
        self.reclaim_permits
    }
}

pub(crate) fn require_policy_receipt(
    receipt: &FoundationalPolicyAdmissionReceipt,
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> Result<(), BackgroundPacingDenial> {
    if receipt.budget_decisions().is_empty() {
        return Err(BackgroundPacingDenial::PolicyReceiptHasNoBudgetDecision);
    }
    if !receipt.denied_work().is_empty() || !receipt.widened_work().is_empty() {
        return Err(BackgroundPacingDenial::PolicyReceiptRejectedOrWidenedWork);
    }
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        breadth_units(requested)?,
        breadth_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        density_units(requested)?,
        density_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        locality_units(requested)?,
        locality_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness_units(requested)?,
        freshness_units(admitted)?,
    )
}

fn require_policy_budget_kind(
    receipt: &FoundationalPolicyAdmissionReceipt,
    kind: FoundationalPerformanceBudgetKind,
    requested: u32,
    admitted: u32,
) -> Result<(), BackgroundPacingDenial> {
    let mut matched = None;
    for decision in receipt.budget_decisions() {
        if decision.kind() != kind {
            continue;
        }
        if matched.is_some() {
            return Err(BackgroundPacingDenial::PolicyReceiptDuplicateBudgetKind(
                kind,
            ));
        }
        matched = Some(decision);
    }
    let Some(decision) = matched else {
        if requested == 0 && admitted == 0 {
            return Ok(());
        }
        return Err(BackgroundPacingDenial::PolicyReceiptMissingBudgetKind(kind));
    };
    if decision.requested_units() != requested || decision.admitted_units() != admitted {
        return Err(BackgroundPacingDenial::PolicyReceiptBudgetMismatch {
            kind,
            requested_units: decision.requested_units(),
            admitted_units: decision.admitted_units(),
            expected_requested_units: requested,
            expected_admitted_units: admitted,
        });
    }
    Ok(())
}

const fn min(left: u64, right: u64) -> u64 {
    if left < right {
        left
    } else {
        right
    }
}

fn breadth_units(budget: BackgroundResourceBudget) -> Result<u32, BackgroundPacingDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Breadth,
        &[budget.queue_slots(), budget.worker_permits()],
    )
}

fn density_units(budget: BackgroundResourceBudget) -> Result<u32, BackgroundPacingDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Density,
        &[
            budget.bandwidth_tokens(),
            budget.dirty_page_budget(),
            budget.cache_residency_hints(),
        ],
    )
}

fn locality_units(budget: BackgroundResourceBudget) -> Result<u32, BackgroundPacingDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Locality,
        &[
            budget.read_ahead_window(),
            budget.write_back_window(),
            budget.reclaim_permits(),
        ],
    )
}

fn freshness_units(budget: BackgroundResourceBudget) -> Result<u32, BackgroundPacingDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        &[budget.flush_permits(), budget.sync_debt()],
    )
}

fn u32_budget_sum(
    kind: FoundationalPerformanceBudgetKind,
    units: &[u64],
) -> Result<u32, BackgroundPacingDenial> {
    let mut total = 0_u64;
    for unit in units {
        total = total
            .checked_add(*unit)
            .ok_or(BackgroundPacingDenial::PolicyReceiptBudgetOverflow(kind))?;
    }
    u32::try_from(total).map_err(|_| BackgroundPacingDenial::PolicyReceiptBudgetOverflow(kind))
}
