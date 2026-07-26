use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::execution_digest::hash_parts;

static NEXT_EXECUTION_ATTEMPT: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionAttemptIdentity(Arc<str>);

impl WorthQueryExecutionAttemptIdentity {
    pub(super) fn initial(lane: &str, resource_plan_identity: &str) -> Self {
        Self::mint(lane, resource_plan_identity, None)
    }

    pub(super) fn readmission(
        lane: &str,
        resource_plan_identity: &str,
        yielded_attempt_identity: &str,
    ) -> Self {
        Self::mint(lane, resource_plan_identity, Some(yielded_attempt_identity))
    }

    fn mint(
        lane: &str,
        resource_plan_identity: &str,
        yielded_attempt_identity: Option<&str>,
    ) -> Self {
        let ordinal = NEXT_EXECUTION_ATTEMPT.fetch_add(1, Ordering::Relaxed);
        Self(Arc::from(hash_parts(&[
            "worth_query_execution_attempt_v1".into(),
            format!("ordinal:{ordinal}"),
            format!("lane:{lane}"),
            format!("resource-plan:{resource_plan_identity}"),
            format!(
                "yielded-attempt:{}",
                yielded_attempt_identity.unwrap_or("initial")
            ),
        ])))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
