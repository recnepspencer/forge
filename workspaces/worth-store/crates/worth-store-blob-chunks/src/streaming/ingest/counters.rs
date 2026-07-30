use worth_store_io_scheduler::BackgroundPacingCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobStreamingIngestCounterSnapshot {
    windows_observed: u64,
    bytes_streamed: u64,
    chunks_read: u64,
    chunks_written: u64,
    backend_write_observations: u64,
    peak_resident_bytes: u64,
    allocation_count: u64,
    scheduler_yields: u64,
    scheduler_waits: u64,
    scheduler_throttles: u64,
    scheduler_admissions: u64,
    denials: u64,
}

impl BlobStreamingIngestCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            windows_observed: 0,
            bytes_streamed: 0,
            chunks_read: 0,
            chunks_written: 0,
            backend_write_observations: 0,
            peak_resident_bytes: 0,
            allocation_count: 0,
            scheduler_yields: 0,
            scheduler_waits: 0,
            scheduler_throttles: 0,
            scheduler_admissions: 0,
            denials: 0,
        }
    }

    pub(crate) const fn record_allocation(self) -> Self {
        Self {
            allocation_count: self.allocation_count + 1,
            ..self
        }
    }

    pub(crate) const fn record_background_pressure(
        self,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self {
            scheduler_yields: self.scheduler_yields + counters.yield_events(),
            scheduler_waits: self.scheduler_waits
                + counters.yield_events()
                + counters.deferred_events()
                + counters.denied_events()
                + counters.throttle_events()
                + counters.admitted_with_debt_events()
                + counters.violation_events()
                + counters.foreground_pressure_events(),
            scheduler_throttles: self.scheduler_throttles + counters.throttle_events(),
            scheduler_admissions: self.scheduler_admissions
                + counters.throttle_events()
                + counters.admitted_with_debt_events(),
            denials: self.denials
                + counters.yield_events()
                + counters.deferred_events()
                + counters.denied_events()
                + counters.violation_events(),
            ..self
        }
    }

    pub(crate) const fn observe_source_window(self, bytes: u64, resident_bytes: u64) -> Self {
        let peak_resident_bytes = if resident_bytes > self.peak_resident_bytes {
            resident_bytes
        } else {
            self.peak_resident_bytes
        };
        Self {
            windows_observed: self.windows_observed + 1,
            bytes_streamed: self.bytes_streamed + bytes,
            peak_resident_bytes,
            ..self
        }
    }

    pub(crate) const fn observe_chunk_write(self) -> Self {
        Self {
            chunks_written: self.chunks_written + 1,
            backend_write_observations: self.backend_write_observations + 1,
            ..self
        }
    }

    pub(crate) const fn observe_chunk_read(self) -> Self {
        Self {
            chunks_read: self.chunks_read + 1,
            ..self
        }
    }

    pub(crate) const fn observe_residency(self, resident_bytes: u64) -> Self {
        let peak_resident_bytes = if resident_bytes > self.peak_resident_bytes {
            resident_bytes
        } else {
            self.peak_resident_bytes
        };
        Self {
            peak_resident_bytes,
            ..self
        }
    }

    pub const fn windows_observed(self) -> u64 {
        self.windows_observed
    }

    pub const fn bytes_streamed(self) -> u64 {
        self.bytes_streamed
    }

    pub const fn chunks_read(self) -> u64 {
        self.chunks_read
    }

    pub const fn chunks_written(self) -> u64 {
        self.chunks_written
    }

    pub const fn backend_write_observations(self) -> u64 {
        self.backend_write_observations
    }

    pub const fn peak_resident_bytes(self) -> u64 {
        self.peak_resident_bytes
    }

    pub const fn allocation_count(self) -> u64 {
        self.allocation_count
    }

    pub const fn scheduler_yields(self) -> u64 {
        self.scheduler_yields
    }

    pub const fn scheduler_waits(self) -> u64 {
        self.scheduler_waits
    }

    pub const fn scheduler_throttles(self) -> u64 {
        self.scheduler_throttles
    }

    pub const fn scheduler_admissions(self) -> u64 {
        self.scheduler_admissions
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
