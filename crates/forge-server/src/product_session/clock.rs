use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

pub trait ForgeServerProductSessionClock: std::fmt::Debug + Send + Sync {
    fn current_time_millis(&self) -> u64;
}

#[derive(Clone, Debug, Default)]
pub struct ForgeServerSystemProductSessionClock;

impl ForgeServerProductSessionClock for ForgeServerSystemProductSessionClock {
    fn current_time_millis(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_millis() as u64
    }
}

pub(crate) fn default_product_session_clock() -> Arc<dyn ForgeServerProductSessionClock> {
    Arc::new(ForgeServerSystemProductSessionClock)
}
