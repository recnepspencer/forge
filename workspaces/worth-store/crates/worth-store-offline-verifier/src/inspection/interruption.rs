pub(crate) fn reject_inspection_interruption(
    budget: crate::OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), crate::OfflineInspectionDenial> {
    if cancellation.is_cancelled() {
        return Err(crate::OfflineInspectionDenial::Cancelled);
    }
    if let Some(deadline) = budget.deadline() {
        if std::time::SystemTime::now() >= deadline {
            return Err(crate::OfflineInspectionDenial::AbsoluteDeadlineReached { deadline });
        }
    }
    if let Some(limit) = budget.max_elapsed() {
        let elapsed = started_at.elapsed();
        if elapsed >= limit {
            return Err(crate::OfflineInspectionDenial::DeadlineExceeded { elapsed, limit });
        }
    }
    Ok(())
}
