use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

use crate::domain_computation::WorthQueryExecutionBoundOperationAuthority;
use crate::execution_digest::hash_parts;

static NEXT_MANAGED_LOGICAL_RUN: AtomicU64 = AtomicU64::new(1);

pub(super) struct WorthQueryManagedRunIdentity {
    logical: Arc<str>,
    attempt: Arc<str>,
}

impl WorthQueryManagedRunIdentity {
    pub(super) fn initial(
        lane: &str,
        operation: &WorthQueryExecutionBoundOperationAuthority,
        resource_attempt_identity: &str,
        bridge_basis: &BridgeBoundExecutionBasis,
        relational_basis: &RelationalExecutionBasisLease,
    ) -> Self {
        let ordinal = NEXT_MANAGED_LOGICAL_RUN.fetch_add(1, Ordering::Relaxed);
        let logical = Arc::from(hash_parts(&[
            "worth_query_managed_logical_run_v1".into(),
            format!("ordinal:{ordinal}"),
            format!("lane:{lane}"),
            format!("operation:{}", operation.binding_identity()),
        ]));
        let attempt = Arc::from(hash_parts(&[
            "worth_query_managed_run_attempt_v1".into(),
            format!("logical:{logical}"),
            format!("resources:{resource_attempt_identity}"),
            format!("bridge:{}", bridge_basis.identity().as_str()),
            format!(
                "relational:{}:{}",
                relational_basis.identity().runtime_instance_id(),
                relational_basis.identity().lease_ordinal()
            ),
        ]));
        Self { logical, attempt }
    }

    pub(super) fn into_parts(self) -> (Arc<str>, Arc<str>) {
        (self.logical, self.attempt)
    }

    pub(super) fn resumed(
        lane: &str,
        logical: Arc<str>,
        resource_attempt_identity: &str,
        bridge_basis: &BridgeBoundExecutionBasis,
        relational_basis: &RelationalExecutionBasisLease,
    ) -> Self {
        let attempt = Arc::from(hash_parts(&[
            "worth_query_managed_run_attempt_v1".into(),
            format!("logical:{logical}"),
            format!("lane:{lane}"),
            format!("resources:{resource_attempt_identity}"),
            format!("bridge:{}", bridge_basis.identity().as_str()),
            format!(
                "relational:{}:{}",
                relational_basis.identity().runtime_instance_id(),
                relational_basis.identity().lease_ordinal()
            ),
        ]));
        Self { logical, attempt }
    }
}
