use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

static NEXT_PROVIDER_SESSION: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct WorthQueryExecutionProviderSession {
    identity: Arc<str>,
    attempt_identity: Arc<str>,
}

impl WorthQueryExecutionProviderSession {
    pub(crate) fn mint(attempt_identity: &str) -> Self {
        let ordinal = NEXT_PROVIDER_SESSION.fetch_add(1, Ordering::Relaxed);
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_execution_provider_session_v1".into(),
            format!("attempt:{attempt_identity}"),
            format!("ordinal:{ordinal}"),
        ]));
        Self {
            identity,
            attempt_identity: Arc::from(attempt_identity),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn attempt_identity(&self) -> &str {
        &self.attempt_identity
    }
}
