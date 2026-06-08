use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct ForgeServerCounters {
    registered_surface_families: AtomicU64,
    rejected_duplicate_surface_registrations: AtomicU64,
    serve_start_count: AtomicU64,
}

impl ForgeServerCounters {
    pub fn record_registered_surface_families(&self, count: usize) {
        self.registered_surface_families
            .store(count as u64, Ordering::Relaxed);
    }

    pub fn increment_rejected_duplicate_surface_registrations(&self) {
        self.rejected_duplicate_surface_registrations
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_serve_start_count(&self) {
        self.serve_start_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> ForgeServerCounterSnapshot {
        ForgeServerCounterSnapshot {
            registered_surface_families: self.registered_surface_families.load(Ordering::Relaxed),
            rejected_duplicate_surface_registrations: self
                .rejected_duplicate_surface_registrations
                .load(Ordering::Relaxed),
            serve_start_count: self.serve_start_count.load(Ordering::Relaxed),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCounterSnapshot {
    pub registered_surface_families: u64,
    pub rejected_duplicate_surface_registrations: u64,
    pub serve_start_count: u64,
}
