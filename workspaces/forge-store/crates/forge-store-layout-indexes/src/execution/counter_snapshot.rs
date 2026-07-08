#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPathCounterSnapshot {
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
}

impl S8AccessPathCounterSnapshot {
    pub(crate) const fn new(
        point_lookups: u16,
        range_lookups: u16,
        wal_replays: u16,
        publications: u16,
        maintenance_reads: u16,
    ) -> Self {
        let page_touches = point_lookups
            .saturating_add(range_lookups)
            .saturating_add(wal_replays)
            .saturating_add(publications)
            .saturating_add(maintenance_reads);
        let index_probes = point_lookups.saturating_add(range_lookups);
        let key_comparisons = index_probes;
        let range_steps = range_lookups;
        let prefix_steps = 0;
        let chunk_tree_node_reads = 0;
        let manifest_reads = publications.saturating_add(maintenance_reads);
        let bytes_read = page_touches as u64 * 4_096;
        let bytes_written = publications as u64 * 4_096;
        let write_fanout = publications;
        let read_amplification = page_touches;
        let write_amplification = write_fanout;
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
        }
    }

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
        }
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

    pub(crate) const fn with_page_touches(mut self, page_touches: u16) -> Self {
        self.page_touches = page_touches;
        self
    }

    pub(crate) const fn with_index_probes(mut self, index_probes: u16) -> Self {
        self.index_probes = index_probes;
        self
    }

    pub(crate) const fn with_key_comparisons(mut self, key_comparisons: u16) -> Self {
        self.key_comparisons = key_comparisons;
        self
    }

    pub(crate) const fn with_range_steps(mut self, range_steps: u16) -> Self {
        self.range_steps = range_steps;
        self
    }

    pub(crate) const fn with_prefix_steps(mut self, prefix_steps: u16) -> Self {
        self.prefix_steps = prefix_steps;
        self
    }

    pub(crate) const fn with_chunk_tree_node_reads(mut self, chunk_tree_node_reads: u16) -> Self {
        self.chunk_tree_node_reads = chunk_tree_node_reads;
        self
    }

    pub(crate) const fn with_manifest_reads(mut self, manifest_reads: u16) -> Self {
        self.manifest_reads = manifest_reads;
        self
    }

    pub(crate) const fn with_bytes_read(mut self, bytes_read: u64) -> Self {
        self.bytes_read = bytes_read;
        self
    }

    pub(crate) const fn with_bytes_written(mut self, bytes_written: u64) -> Self {
        self.bytes_written = bytes_written;
        self
    }

    pub(crate) const fn with_write_fanout(mut self, write_fanout: u16) -> Self {
        self.write_fanout = write_fanout;
        self
    }

    pub(crate) const fn with_read_amplification(mut self, read_amplification: u16) -> Self {
        self.read_amplification = read_amplification;
        self
    }

    pub(crate) const fn with_write_amplification(mut self, write_amplification: u16) -> Self {
        self.write_amplification = write_amplification;
        self
    }
}
