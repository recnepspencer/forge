#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeExactCounterWitness {
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

impl BaselineBTreeExactCounterWitness {
    pub(super) const fn new(
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
}
