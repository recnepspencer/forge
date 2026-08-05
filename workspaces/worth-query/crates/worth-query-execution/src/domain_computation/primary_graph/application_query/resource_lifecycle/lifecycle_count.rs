use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) fn acquire(
    counter: &AtomicUsize,
    amount: usize,
) -> Result<usize, LifecycleCountOverflow> {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_add(amount)
        })
        .map(|previous| previous + amount)
        .map_err(|_| LifecycleCountOverflow)
}

pub(super) fn release(
    counter: &AtomicUsize,
    amount: usize,
) -> Result<usize, LifecycleCountUnderflow> {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_sub(amount)
        })
        .map(|previous| previous - amount)
        .map_err(|_| LifecycleCountUnderflow)
}

pub(super) fn record_one_saturating(counter: &AtomicUsize) {
    let _ = counter.fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
        Some(current.saturating_add(1))
    });
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LifecycleCountOverflow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LifecycleCountUnderflow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_overflow_and_underflow_leave_authoritative_count_unchanged() {
        let full = AtomicUsize::new(usize::MAX);
        assert_eq!(acquire(&full, 1), Err(LifecycleCountOverflow));
        assert_eq!(full.load(Ordering::Acquire), usize::MAX);

        let empty = AtomicUsize::new(0);
        assert_eq!(release(&empty, 1), Err(LifecycleCountUnderflow));
        assert_eq!(empty.load(Ordering::Acquire), 0);
    }

    #[test]
    fn lifetime_diagnostic_count_saturates_instead_of_wrapping() {
        let count = AtomicUsize::new(usize::MAX);
        record_one_saturating(&count);
        assert_eq!(count.load(Ordering::Acquire), usize::MAX);
    }
}
