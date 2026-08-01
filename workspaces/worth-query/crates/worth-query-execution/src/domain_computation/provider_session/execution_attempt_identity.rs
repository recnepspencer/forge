use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

static NEXT_EXECUTION_ATTEMPT: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionAttemptIdentity(Arc<str>);

impl WorthQueryExecutionAttemptIdentity {
    pub(super) fn initial(_lane: &str, _resource_plan_identity: &str) -> Self {
        Self::mint()
    }

    pub(super) fn readmission(
        _lane: &str,
        _resource_plan_identity: &str,
        _yielded_attempt_identity: &str,
    ) -> Self {
        Self::mint()
    }

    pub(super) fn graph_work(
        session_identity: &worth_foundational::facade::CanonicalDigestId,
    ) -> Self {
        Self(Arc::from(session_identity.render_hex()))
    }

    fn mint() -> Self {
        let ordinal = next_execution_attempt_ordinal(&NEXT_EXECUTION_ATTEMPT)
            .expect("execution attempt identity space must not be exhausted");
        Self(Arc::from(format!("execution-attempt:{ordinal}")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn next_execution_attempt_ordinal(counter: &AtomicU64) -> Option<u64> {
    counter
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_attempt_ordinal_exhaustion_cannot_wrap() {
        let counter = AtomicU64::new(u64::MAX - 1);

        assert_eq!(next_execution_attempt_ordinal(&counter), Some(u64::MAX - 1));
        assert_eq!(next_execution_attempt_ordinal(&counter), None);
        assert_eq!(counter.load(Ordering::Relaxed), u64::MAX);
    }
}
