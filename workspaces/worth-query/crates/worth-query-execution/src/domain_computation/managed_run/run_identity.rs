use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

use crate::domain_computation::WorthQueryExecutionBoundOperationAuthority;

static NEXT_MANAGED_LOGICAL_RUN: AtomicU64 = AtomicU64::new(1);

pub(super) struct WorthQueryManagedRunIdentity {
    logical: Arc<str>,
    attempt: Arc<str>,
}

impl WorthQueryManagedRunIdentity {
    pub(super) fn initial(
        _lane: &str,
        _operation: &WorthQueryExecutionBoundOperationAuthority,
        resource_attempt_identity: &str,
        _bridge_basis: &BridgeBoundExecutionBasis,
        _relational_basis: &RelationalExecutionBasisLease,
    ) -> Self {
        let ordinal = next_managed_logical_run_ordinal(&NEXT_MANAGED_LOGICAL_RUN)
            .expect("managed logical-run identity space must not be exhausted");
        let logical = Arc::from(format!("managed-logical-run:{ordinal}"));
        let attempt = Arc::from(resource_attempt_identity);
        Self { logical, attempt }
    }

    pub(super) fn into_parts(self) -> (Arc<str>, Arc<str>) {
        (self.logical, self.attempt)
    }

    pub(super) fn resumed(
        _lane: &str,
        logical: Arc<str>,
        resource_attempt_identity: &str,
        _bridge_basis: &BridgeBoundExecutionBasis,
        _relational_basis: &RelationalExecutionBasisLease,
    ) -> Self {
        let attempt = Arc::from(resource_attempt_identity);
        Self { logical, attempt }
    }
}

fn next_managed_logical_run_ordinal(counter: &AtomicU64) -> Option<u64> {
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
    fn managed_logical_run_ordinal_exhaustion_cannot_wrap() {
        let counter = AtomicU64::new(u64::MAX - 1);

        assert_eq!(
            next_managed_logical_run_ordinal(&counter),
            Some(u64::MAX - 1)
        );
        assert_eq!(next_managed_logical_run_ordinal(&counter), None);
        assert_eq!(counter.load(Ordering::Relaxed), u64::MAX);
    }
}
