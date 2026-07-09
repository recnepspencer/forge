use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::WORTHStoreBuilder;

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn unique_test_store_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}.json"))
}

pub fn unique_test_sqlite_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}.sqlite"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StoreLane {
    InMemory,
    LocalFile,
    Sqlite,
}

impl StoreLane {
    pub fn label(self) -> &'static str {
        match self {
            StoreLane::InMemory => "in_memory",
            StoreLane::LocalFile => "local_file",
            StoreLane::Sqlite => "sqlite",
        }
    }
}

pub fn build_store_for_lane(lane: StoreLane, prefix: &str) -> crate::WORTHStore {
    match lane {
        StoreLane::InMemory => WORTHStoreBuilder::new().in_memory().build().unwrap(),
        StoreLane::LocalFile => WORTHStoreBuilder::new()
            .local_file(unique_test_store_path(prefix))
            .build()
            .unwrap(),
        StoreLane::Sqlite => WORTHStoreBuilder::new()
            .sqlite_file(unique_test_sqlite_path(prefix))
            .build()
            .unwrap(),
    }
}
