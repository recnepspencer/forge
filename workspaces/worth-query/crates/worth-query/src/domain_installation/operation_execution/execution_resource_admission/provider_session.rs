use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

static NEXT_PROVIDER_SESSION: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct WorthQueryExecutionProviderSession {
    identity: Arc<str>,
    admission_identity: Arc<str>,
    strategy: Arc<str>,
}

impl WorthQueryExecutionProviderSession {
    pub(crate) fn mint(plan: &super::WorthQueryAdmittedExecutionResourcePlan) -> Self {
        let ordinal = NEXT_PROVIDER_SESSION.fetch_add(1, Ordering::Relaxed);
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_execution_provider_session_v1".into(),
            format!("admission:{}", plan.identity()),
            format!("strategy:{}", plan.strategy().as_str()),
            format!("ordinal:{ordinal}"),
        ]));
        Self {
            identity,
            admission_identity: Arc::from(plan.identity()),
            strategy: Arc::from(plan.strategy().as_str()),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }

    pub fn strategy(&self) -> &str {
        &self.strategy
    }
}
