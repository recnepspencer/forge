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
    pub(super) const fn new(values: BaselineBTreeExactCounterValues) -> Self {
        Self {
            point_lookups: values.point_lookups,
            range_lookups: values.range_lookups,
            wal_replays: values.wal_replays,
            publications: values.publications,
            maintenance_reads: values.maintenance_reads,
            page_touches: values.page_touches,
            index_probes: values.index_probes,
            key_comparisons: values.key_comparisons,
            range_steps: values.range_steps,
            prefix_steps: values.prefix_steps,
            chunk_tree_node_reads: values.chunk_tree_node_reads,
            manifest_reads: values.manifest_reads,
            bytes_read: values.bytes_read,
            bytes_written: values.bytes_written,
            write_fanout: values.write_fanout,
            read_amplification: values.read_amplification,
            write_amplification: values.write_amplification,
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

    pub(super) fn from_replay_reads(
        reads: [forge_store_physical_format::PhysicalLayoutAccessCounterSnapshot; 3],
    ) -> Self {
        let page_touches = reads
            .iter()
            .map(|read| read.page_touches())
            .fold(0_u16, u16::saturating_add);
        let index_probes = reads
            .iter()
            .map(|read| read.index_probes())
            .fold(0_u16, u16::saturating_add);
        let key_comparisons = reads
            .iter()
            .map(|read| read.key_comparisons())
            .fold(0_u16, u16::saturating_add);
        let bytes_read = reads
            .iter()
            .map(|read| read.bytes_read())
            .fold(0_u64, u64::saturating_add);

        Self::new(BaselineBTreeExactCounterValues {
            wal_replays: 1,
            maintenance_reads: page_touches,
            page_touches,
            index_probes,
            key_comparisons,
            manifest_reads: 1,
            bytes_read,
            read_amplification: page_touches,
            ..BaselineBTreeExactCounterValues::default()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct BaselineBTreeExactCounterValues {
    pub(super) point_lookups: u16,
    pub(super) range_lookups: u16,
    pub(super) wal_replays: u16,
    pub(super) publications: u16,
    pub(super) maintenance_reads: u16,
    pub(super) page_touches: u16,
    pub(super) index_probes: u16,
    pub(super) key_comparisons: u16,
    pub(super) range_steps: u16,
    pub(super) prefix_steps: u16,
    pub(super) chunk_tree_node_reads: u16,
    pub(super) manifest_reads: u16,
    pub(super) bytes_read: u64,
    pub(super) bytes_written: u64,
    pub(super) write_fanout: u16,
    pub(super) read_amplification: u16,
    pub(super) write_amplification: u16,
}
