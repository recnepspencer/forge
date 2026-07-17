use super::{
    BackgroundIdleCapacityLease, BackgroundIdleCapacityLeaseRequest, BackgroundIoDebt,
    BackgroundPacingAdmittedWithDebt, BackgroundPacingCounterSnapshot, BackgroundPacingDeferred,
    BackgroundPacingDenial, BackgroundPacingDenied, BackgroundPacingOutcome,
    BackgroundPacingStaleRebindKind, BackgroundPacingStaleRebindRequired, BackgroundPacingThrottle,
    BackgroundPacingViolation, BackgroundPacingYield, BackgroundResourceBudget,
    BackgroundResourceShortfall,
};

pub fn admit_background_pacing(
    request: BackgroundIdleCapacityLeaseRequest,
) -> BackgroundPacingOutcome {
    match super::proof::prove_background_pacing_current(
        request.capacity().pressure().class(),
        request.capacity().freshness(),
    )
    .into_raw()
    {
        worth_proof::TransitionOutcome::Success(ready) => {
            let _authority = ready.authority_witness();
        }
        worth_proof::TransitionOutcome::Denied(denial) => return denied(&request, denial),
        worth_proof::TransitionOutcome::Deferred(_) => return deferred(&request),
        worth_proof::TransitionOutcome::Stale(_) => {
            return stale_or_rebind(&request, BackgroundPacingStaleRebindKind::Stale);
        }
        worth_proof::TransitionOutcome::RebindRequired(_) => {
            return stale_or_rebind(&request, BackgroundPacingStaleRebindKind::RebindRequired);
        }
        worth_proof::TransitionOutcome::Failed(failed) => match failed {},
    }

    if request.foreground_pressure_events() > 0 && !request.late_yield() {
        return yield_now(&request);
    }
    let requested = request.capacity().pressure().requested_budget();
    if request.capacity().policy_admitted().is_empty() {
        return deferred(&request);
    }
    let available_under_policy = request
        .capacity()
        .idle_available()
        .min_with(request.capacity().policy_admitted());
    let admitted = requested.min_with(available_under_policy);
    let debt = requested.debt_after(admitted);
    if admitted.is_empty() {
        if debt.is_empty() {
            return deferred(&request);
        }
        return throttled(&request, admitted, debt);
    }
    if !debt.is_empty() && request.capacity().debt_limit().is_empty() {
        return throttled(&request, admitted, debt);
    }
    if !debt.is_empty() && !debt_fits_limit(debt, request.capacity().debt_limit()) {
        return denied(
            &request,
            BackgroundPacingDenial::InsufficientIdleCapacity(first_shortfall(debt)),
        );
    }
    if request.late_yield() {
        return violation(&request, admitted, debt);
    }
    admitted_with_debt(&request, admitted, debt)
}

fn yield_now(request: &BackgroundIdleCapacityLeaseRequest) -> BackgroundPacingOutcome {
    BackgroundPacingOutcome::Yield(BackgroundPacingYield::new(
        request.capacity().pressure().class(),
        BackgroundPacingCounterSnapshot::yield_now(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
            request.foreground_pressure_events(),
        ),
    ))
}

fn deferred(request: &BackgroundIdleCapacityLeaseRequest) -> BackgroundPacingOutcome {
    BackgroundPacingOutcome::Deferred(BackgroundPacingDeferred::new(
        request.capacity().pressure().class(),
        BackgroundPacingCounterSnapshot::deferred(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
        ),
    ))
}

fn stale_or_rebind(
    request: &BackgroundIdleCapacityLeaseRequest,
    kind: BackgroundPacingStaleRebindKind,
) -> BackgroundPacingOutcome {
    BackgroundPacingOutcome::StaleRebindRequired(BackgroundPacingStaleRebindRequired::new(
        request.capacity().pressure().class(),
        kind,
        BackgroundPacingCounterSnapshot::deferred(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
        ),
    ))
}

fn denied(
    request: &BackgroundIdleCapacityLeaseRequest,
    denial: BackgroundPacingDenial,
) -> BackgroundPacingOutcome {
    BackgroundPacingOutcome::Denied(BackgroundPacingDenied::new(
        request.capacity().pressure().class(),
        denial,
        BackgroundPacingCounterSnapshot::denied(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
            request.capacity().pressure().requested_budget(),
        ),
    ))
}

fn throttled(
    request: &BackgroundIdleCapacityLeaseRequest,
    admitted: BackgroundResourceBudget,
    throttled_units: BackgroundResourceBudget,
) -> BackgroundPacingOutcome {
    BackgroundPacingOutcome::Throttled(Box::new(BackgroundPacingThrottle::new(
        request.capacity().pressure().class(),
        admitted,
        throttled_units,
        BackgroundPacingCounterSnapshot::throttled(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
            admitted,
            throttled_units,
        ),
    )))
}

fn admitted_with_debt(
    request: &BackgroundIdleCapacityLeaseRequest,
    admitted: BackgroundResourceBudget,
    debt_units: BackgroundResourceBudget,
) -> BackgroundPacingOutcome {
    let debt = BackgroundIoDebt::new(request.capacity().pressure().class(), debt_units);
    let counters = BackgroundPacingCounterSnapshot::admitted_with_debt(
        request.capacity().pressure().requested_budget(),
        request.capacity().idle_available(),
        admitted,
        debt_units,
        debt.kind(),
    );
    BackgroundPacingOutcome::AdmittedWithDebt(Box::new(BackgroundPacingAdmittedWithDebt::new(
        BackgroundIdleCapacityLease::new(
            request.capacity().pressure().class(),
            admitted,
            debt,
            request.capacity().basis(),
            counters,
            request.capacity().secure_io(),
        ),
    )))
}

fn violation(
    request: &BackgroundIdleCapacityLeaseRequest,
    admitted: BackgroundResourceBudget,
    debt_units: BackgroundResourceBudget,
) -> BackgroundPacingOutcome {
    let debt = BackgroundIoDebt::new(request.capacity().pressure().class(), debt_units);
    BackgroundPacingOutcome::Violation(BackgroundPacingViolation::new(
        debt,
        BackgroundPacingCounterSnapshot::violation(
            request.capacity().pressure().requested_budget(),
            request.capacity().idle_available(),
            admitted,
            debt_units,
            debt.kind(),
            request.foreground_pressure_events(),
        ),
    ))
}

fn debt_fits_limit(debt: BackgroundResourceBudget, limit: BackgroundResourceBudget) -> bool {
    units()
        .into_iter()
        .all(|unit| debt.amount_for(unit) <= limit.amount_for(unit))
}

fn first_shortfall(debt: BackgroundResourceBudget) -> BackgroundResourceShortfall {
    for unit in units() {
        let requested = debt.amount_for(unit);
        if requested > 0 {
            return BackgroundResourceShortfall::Unit {
                unit,
                requested,
                available: 0,
            };
        }
    }
    BackgroundResourceShortfall::Unit {
        unit: crate::IoResourceUnitKind::QueueSlot,
        requested: 0,
        available: 0,
    }
}

const fn units() -> [crate::IoResourceUnitKind; 10] {
    [
        crate::IoResourceUnitKind::QueueSlot,
        crate::IoResourceUnitKind::BandwidthToken,
        crate::IoResourceUnitKind::FlushPermit,
        crate::IoResourceUnitKind::SyncDebt,
        crate::IoResourceUnitKind::ReadAheadWindow,
        crate::IoResourceUnitKind::WriteBackWindow,
        crate::IoResourceUnitKind::DirtyPageBudget,
        crate::IoResourceUnitKind::WorkerPermit,
        crate::IoResourceUnitKind::CacheResidencyHint,
        crate::IoResourceUnitKind::ReclaimPermit,
    ]
}
