#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPathCounterSnapshot {
    point_lookups: u16,
    range_lookups: u16,
    wal_replays: u16,
    publications: u16,
    maintenance_reads: u16,
    page_touches: u16,
    index_probes: u16,
    key_comparisons: u16,
    range_steps: u16,
    prefix_steps: u16,
    chunk_tree_node_reads: u16,
    manifest_reads: u16,
    bytes_read: u64,
    bytes_written: u64,
    write_fanout: u16,
    read_amplification: u16,
    write_amplification: u16,
    allocation_events: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannedCounterObservation {
    Exact,
    WithinEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterEnvelopeViolation {
    planned: AccessPathCounterSnapshot,
    observed: AccessPathCounterSnapshot,
}

impl CounterEnvelopeViolation {
    pub const fn planned(&self) -> AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn observed(&self) -> AccessPathCounterSnapshot {
        self.observed
    }
}

impl AccessPathCounterSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn exact(
        point_lookups: u16,
        range_lookups: u16,
        wal_replays: u16,
        publications: u16,
        maintenance_reads: u16,
        page_touches: u16,
        index_probes: u16,
        key_comparisons: u16,
        range_steps: u16,
        prefix_steps: u16,
        chunk_tree_node_reads: u16,
        manifest_reads: u16,
        bytes_read: u64,
        bytes_written: u64,
        write_fanout: u16,
        read_amplification: u16,
        write_amplification: u16,
    ) -> Self {
        Self {
            point_lookups,
            range_lookups,
            wal_replays,
            publications,
            maintenance_reads,
            page_touches,
            index_probes,
            key_comparisons,
            range_steps,
            prefix_steps,
            chunk_tree_node_reads,
            manifest_reads,
            bytes_read,
            bytes_written,
            write_fanout,
            read_amplification,
            write_amplification,
            allocation_events: 0,
        }
    }

    pub(crate) const fn with_allocation_events(mut self, allocation_events: u64) -> Self {
        self.allocation_events = allocation_events;
        self
    }

    pub(crate) const fn with_selected_plan_authority_allocation(mut self) -> Self {
        self.allocation_events += 1;
        self
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn wal_replays(self) -> u16 {
        self.wal_replays
    }

    pub const fn publications(self) -> u16 {
        self.publications
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.maintenance_reads
    }

    pub const fn page_touches(self) -> u16 {
        self.page_touches
    }

    pub const fn index_probes(self) -> u16 {
        self.index_probes
    }

    pub const fn key_comparisons(self) -> u16 {
        self.key_comparisons
    }

    pub const fn range_steps(self) -> u16 {
        self.range_steps
    }

    pub const fn prefix_steps(self) -> u16 {
        self.prefix_steps
    }

    pub const fn chunk_tree_node_reads(self) -> u16 {
        self.chunk_tree_node_reads
    }

    pub const fn manifest_reads(self) -> u16 {
        self.manifest_reads
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub const fn bytes_written(self) -> u64 {
        self.bytes_written
    }

    pub const fn write_fanout(self) -> u16 {
        self.write_fanout
    }

    pub const fn read_amplification(self) -> u16 {
        self.read_amplification
    }

    pub const fn write_amplification(self) -> u16 {
        self.write_amplification
    }

    pub const fn allocation_events(self) -> u64 {
        self.allocation_events
    }

    pub(crate) const fn validate_observation(
        self,
        observed: Self,
    ) -> Result<PlannedCounterObservation, CounterEnvelopeViolation> {
        if self.equals(observed) {
            Ok(PlannedCounterObservation::Exact)
        } else if observed.fits_within(self) {
            Ok(PlannedCounterObservation::WithinEnvelope)
        } else {
            Err(CounterEnvelopeViolation {
                planned: self,
                observed,
            })
        }
    }

    const fn equals(self, other: Self) -> bool {
        self.point_lookups == other.point_lookups
            && self.range_lookups == other.range_lookups
            && self.wal_replays == other.wal_replays
            && self.publications == other.publications
            && self.maintenance_reads == other.maintenance_reads
            && self.page_touches == other.page_touches
            && self.index_probes == other.index_probes
            && self.key_comparisons == other.key_comparisons
            && self.range_steps == other.range_steps
            && self.prefix_steps == other.prefix_steps
            && self.chunk_tree_node_reads == other.chunk_tree_node_reads
            && self.manifest_reads == other.manifest_reads
            && self.bytes_read == other.bytes_read
            && self.bytes_written == other.bytes_written
            && self.write_fanout == other.write_fanout
            && self.read_amplification == other.read_amplification
            && self.write_amplification == other.write_amplification
            && self.allocation_events == other.allocation_events
    }

    const fn fits_within(self, envelope: Self) -> bool {
        self.point_lookups <= envelope.point_lookups
            && self.range_lookups <= envelope.range_lookups
            && self.wal_replays <= envelope.wal_replays
            && self.publications <= envelope.publications
            && self.maintenance_reads <= envelope.maintenance_reads
            && self.page_touches <= envelope.page_touches
            && self.index_probes <= envelope.index_probes
            && self.key_comparisons <= envelope.key_comparisons
            && self.range_steps <= envelope.range_steps
            && self.prefix_steps <= envelope.prefix_steps
            && self.chunk_tree_node_reads <= envelope.chunk_tree_node_reads
            && self.manifest_reads <= envelope.manifest_reads
            && self.bytes_read <= envelope.bytes_read
            && self.bytes_written <= envelope.bytes_written
            && self.write_fanout <= envelope.write_fanout
            && self.read_amplification <= envelope.read_amplification
            && self.write_amplification <= envelope.write_amplification
            && self.allocation_events <= envelope.allocation_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocation_overrun_is_a_typed_violation_not_an_observation_state() {
        let planned =
            AccessPathCounterSnapshot::exact(1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 4_096, 0, 0, 1, 0)
                .with_allocation_events(2);
        let observed = planned.with_allocation_events(3);

        let violation = planned
            .validate_observation(observed)
            .expect_err("allocation overrun must deny successful counter observation");

        assert_eq!(violation.planned().allocation_events(), 2);
        assert_eq!(violation.observed().allocation_events(), 3);
    }
}
