use crate::{
    PlannedScrubWindow, PlannedScrubWindowStatus, ScrubCounterSnapshot, ScrubOverBudgetClass,
    ScrubPlan, ScrubPlanDenial, ScrubPlanDenialKind, ScrubPlanRequest, ScrubWindow,
    ScrubWindowSource,
};

impl<'runtime, 'lease> ScrubPlan<'runtime, 'lease> {
    pub fn build(request: ScrubPlanRequest<'runtime, 'lease>) -> Result<Self, ScrubPlanDenial> {
        if request.yield_after_windows == Some(0) {
            return Err(denial(ScrubPlanDenialKind::ZeroYieldWindowBudget));
        }
        if request.windows.is_empty() {
            return Err(denial(ScrubPlanDenialKind::EmptyWindowSet));
        }

        let mut cumulative_bytes = 0_u64;
        let mut planned = Vec::with_capacity(request.windows.len());
        for (index, window) in request.windows.iter().copied().enumerate() {
            require_matching_online_store_authority(window, &request)?;
            let status = if request.skipped_ordinals.contains(&window.ordinal()) {
                PlannedScrubWindowStatus::Skip
            } else {
                classify_window(
                    index as u64 + 1,
                    window,
                    cumulative_bytes,
                    request.allocation.bytes(),
                    request.policy,
                )?
            };
            if status == PlannedScrubWindowStatus::Inspect {
                cumulative_bytes += window.len_bytes();
            } else if matches!(status, PlannedScrubWindowStatus::DeferOverBudget(_))
                && !request.defer_over_budget_windows
            {
                return Err(denial_for_status(status, window, &request));
            }
            planned.push(PlannedScrubWindow::new(window, status));
        }

        let plan_identity = super::identity::scrub_plan_identity(
            &request.allocation,
            request.mode,
            request.policy,
            request.yield_after_windows,
            &planned,
        );
        Ok(Self {
            allocation: request.allocation,
            mode: request.mode,
            windows: planned,
            policy: request.policy,
            yield_after_windows: request.yield_after_windows,
            plan_identity,
        })
    }
}

fn require_matching_online_store_authority(
    window: ScrubWindow<'_>,
    request: &ScrubPlanRequest<'_, '_>,
) -> Result<(), ScrubPlanDenial> {
    let Some(chunk) = window.store_chunk_basis() else {
        return Ok(());
    };
    let expected_store = request.allocation.store_identity();
    if chunk.store_identity() != expected_store {
        return Err(denial(ScrubPlanDenialKind::OnlineWindowStoreMismatch {
            ordinal: window.ordinal(),
            expected: expected_store,
            actual: chunk.store_identity(),
        }));
    }
    let expected_generation = request.allocation.store_generation();
    if chunk.store_generation() != expected_generation {
        return Err(denial(
            ScrubPlanDenialKind::OnlineWindowGenerationMismatch {
                ordinal: window.ordinal(),
                expected: expected_generation,
                actual: chunk.store_generation(),
            },
        ));
    }
    Ok(())
}

fn classify_window(
    ordinal_read_count: u64,
    window: ScrubWindow<'_>,
    cumulative_bytes: u64,
    allocation_bytes: u64,
    policy: super::ScrubPlanPolicy,
) -> Result<PlannedScrubWindowStatus, ScrubPlanDenial> {
    if window.is_empty() {
        return Err(denial(ScrubPlanDenialKind::EmptyWindow {
            ordinal: window.ordinal(),
        }));
    }
    if window.source() == ScrubWindowSource::OnlineProtectedRead
        && ordinal_read_count > policy.protected_read_limit()
    {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::ProtectedRead,
        ));
    }
    if window.len_bytes() > policy.streaming_window_byte_limit() {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::StreamingWindow,
        ));
    }
    if cumulative_bytes.saturating_add(window.len_bytes()) > allocation_bytes {
        return Ok(PlannedScrubWindowStatus::DeferOverBudget(
            ScrubOverBudgetClass::Allocation,
        ));
    }
    Ok(PlannedScrubWindowStatus::Inspect)
}

fn denial_for_status(
    status: PlannedScrubWindowStatus,
    window: ScrubWindow<'_>,
    request: &ScrubPlanRequest<'_, '_>,
) -> ScrubPlanDenial {
    let kind = match status {
        PlannedScrubWindowStatus::Inspect | PlannedScrubWindowStatus::Skip => unreachable!(),
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ProtectedRead) => {
            ScrubPlanDenialKind::ProtectedReadLimitExceeded {
                requested: window.ordinal().get() + 1,
                limit: request.policy.protected_read_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::StreamingWindow) => {
            ScrubPlanDenialKind::StreamingWindowLimitExceeded {
                requested: window.len_bytes(),
                limit: request.policy.streaming_window_byte_limit(),
            }
        }
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::Allocation) => {
            ScrubPlanDenialKind::AllocationLimitExceeded {
                requested: window.len_bytes(),
                limit: request.allocation.bytes(),
            }
        }
    };
    denial(kind)
}

fn denial(kind: ScrubPlanDenialKind) -> ScrubPlanDenial {
    ScrubPlanDenial::new(kind, ScrubCounterSnapshot::empty())
}
