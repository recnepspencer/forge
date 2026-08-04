#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDurabilityPerformanceClaim {
    GroupCommitAmplification,
    CheckpointBoundedness,
    PageBasisBoundedness,
    IdempotencyRetention,
    TerminalCloseout,
}

impl PhysicalDurabilityPerformanceClaim {
    pub const fn label(self) -> &'static str {
        match self {
            Self::GroupCommitAmplification => "group-commit-amplification",
            Self::CheckpointBoundedness => "checkpoint-boundedness",
            Self::PageBasisBoundedness => "page-basis-boundedness",
            Self::IdempotencyRetention => "idempotency-retention",
            Self::TerminalCloseout => "terminal-closeout",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalTrafficPerformanceExpectation {
    operations: u64,
    groups: u64,
    terminal: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIoPerformanceExpectation {
    operations: u64,
    bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalQueuePerformanceExpectation {
    peak_members: u64,
    member_limit: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupCommitPerformanceExpectation {
    traffic: PhysicalTrafficPerformanceExpectation,
    wal: PhysicalIoPerformanceExpectation,
    data: PhysicalIoPerformanceExpectation,
    root_publications: u64,
    queue: PhysicalQueuePerformanceExpectation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointPerformanceExpectation {
    traffic: PhysicalTrafficPerformanceExpectation,
    stream: PhysicalIoPerformanceExpectation,
    dirty_records: u64,
    retained_wal_segments: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageBasisPerformanceExpectation {
    writes: u64,
    bytes: u64,
    records: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdempotencyPerformanceExpectation {
    values: [u64; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloseoutPerformanceExpectation {
    values: [u64; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDurabilityPerformanceContract {
    GroupCommit(GroupCommitPerformanceExpectation),
    Checkpoint(CheckpointPerformanceExpectation),
    PageBasis(PageBasisPerformanceExpectation),
    Idempotency(IdempotencyPerformanceExpectation),
    Closeout(CloseoutPerformanceExpectation),
}

impl PhysicalTrafficPerformanceExpectation {
    pub const fn new(operations: u64, groups: u64, terminal: u64) -> Self {
        Self {
            operations,
            groups,
            terminal,
        }
    }

    const fn values(self) -> [u64; 3] {
        [self.operations, self.groups, self.terminal]
    }
}

impl PhysicalIoPerformanceExpectation {
    pub const fn new(operations: u64, bytes: u64) -> Self {
        Self { operations, bytes }
    }

    const fn values(self) -> [u64; 2] {
        [self.operations, self.bytes]
    }
}

impl PhysicalQueuePerformanceExpectation {
    pub const fn new(peak_members: u64, member_limit: u64) -> Self {
        Self {
            peak_members,
            member_limit,
        }
    }

    const fn values(self) -> [u64; 2] {
        [self.peak_members, self.member_limit]
    }
}

impl GroupCommitPerformanceExpectation {
    pub const fn new(
        traffic: PhysicalTrafficPerformanceExpectation,
        wal: PhysicalIoPerformanceExpectation,
        data: PhysicalIoPerformanceExpectation,
        root_publications: u64,
        queue: PhysicalQueuePerformanceExpectation,
    ) -> Self {
        Self {
            traffic,
            wal,
            data,
            root_publications,
            queue,
        }
    }

    fn rows(self) -> Vec<(&'static str, u64)> {
        let [mutations, groups, acknowledgments] = self.traffic.values();
        let [wal_frames, wal_bytes] = self.wal.values();
        let [data_writes, data_bytes] = self.data.values();
        let [queue_peak_members, queue_member_limit] = self.queue.values();
        vec![
            ("store.durability.mutations", mutations),
            ("store.durability.groups", groups),
            ("store.durability.acknowledgments", acknowledgments),
            ("store.durability.wal.frames", wal_frames),
            ("store.durability.wal.bytes", wal_bytes),
            ("store.durability.data.writes", data_writes),
            ("store.durability.data.bytes", data_bytes),
            ("store.durability.root.publications", self.root_publications),
            (
                "store.durability.group_queue.peak_members",
                queue_peak_members,
            ),
            (
                "store.durability.group_queue.member_limit",
                queue_member_limit,
            ),
        ]
    }
}

impl CheckpointPerformanceExpectation {
    pub const fn new(
        traffic: PhysicalTrafficPerformanceExpectation,
        stream: PhysicalIoPerformanceExpectation,
        dirty_records: u64,
        retained_wal_segments: u64,
    ) -> Self {
        Self {
            traffic,
            stream,
            dirty_records,
            retained_wal_segments,
        }
    }

    fn rows(self) -> Vec<(&'static str, u64)> {
        let [started, completed, terminal] = self.traffic.values();
        let [streams, bytes] = self.stream.values();
        vec![
            ("store.checkpoint.started", started),
            ("store.checkpoint.completed", completed),
            ("store.checkpoint.terminal", terminal),
            ("store.checkpoint.streams", streams),
            ("store.checkpoint.bytes", bytes),
            ("store.checkpoint.dirty_records", self.dirty_records),
            (
                "store.checkpoint.retained_wal_segments",
                self.retained_wal_segments,
            ),
        ]
    }
}

impl PageBasisPerformanceExpectation {
    pub const fn new(writes: u64, bytes: u64, records: u64) -> Self {
        Self {
            writes,
            bytes,
            records,
        }
    }

    fn rows(self) -> Vec<(&'static str, u64)> {
        vec![
            ("store.page_basis.writes", self.writes),
            ("store.page_basis.bytes", self.bytes),
            ("store.page_basis.records", self.records),
        ]
    }
}

impl IdempotencyPerformanceExpectation {
    pub const fn from_values(values: [u64; 6]) -> Self {
        Self { values }
    }

    pub const fn from_counts(
        live_bindings: u64,
        counts: crate::physical_runtime::PhysicalRecoveryOperationFateCounts,
    ) -> Self {
        Self {
            values: [
                live_bindings,
                counts.unresolved(),
                counts.completed(),
                counts.proven_no_effect(),
                counts.indeterminate(),
                counts.completed_unobserved(),
            ],
        }
    }

    fn rows(self) -> Vec<(&'static str, u64)> {
        named_rows(&IDEMPOTENCY_COUNTER_NAMES, self.values)
    }
}

impl CloseoutPerformanceExpectation {
    pub const fn from_values(values: [u64; 6]) -> Self {
        Self { values }
    }

    fn rows(self) -> Vec<(&'static str, u64)> {
        named_rows(&CLOSEOUT_COUNTER_NAMES, self.values)
    }
}

impl PhysicalDurabilityPerformanceContract {
    pub const fn claim(self) -> PhysicalDurabilityPerformanceClaim {
        match self {
            Self::GroupCommit(_) => PhysicalDurabilityPerformanceClaim::GroupCommitAmplification,
            Self::Checkpoint(_) => PhysicalDurabilityPerformanceClaim::CheckpointBoundedness,
            Self::PageBasis(_) => PhysicalDurabilityPerformanceClaim::PageBasisBoundedness,
            Self::Idempotency(_) => PhysicalDurabilityPerformanceClaim::IdempotencyRetention,
            Self::Closeout(_) => PhysicalDurabilityPerformanceClaim::TerminalCloseout,
        }
    }

    pub(super) fn rows(self) -> Vec<(&'static str, u64)> {
        match self {
            Self::GroupCommit(value) => value.rows(),
            Self::Checkpoint(value) => value.rows(),
            Self::PageBasis(value) => value.rows(),
            Self::Idempotency(value) => value.rows(),
            Self::Closeout(value) => value.rows(),
        }
    }
}

const IDEMPOTENCY_COUNTER_NAMES: [&str; 6] = [
    "store.idempotency.live_bindings",
    "store.idempotency.unresolved",
    "store.idempotency.completed",
    "store.idempotency.proven_no_effect",
    "store.idempotency.indeterminate",
    "store.idempotency.completed_unobserved",
];

const CLOSEOUT_COUNTER_NAMES: [&str; 6] = [
    "store.closeout.mutation_terminal",
    "store.closeout.checkpoint_terminal",
    "store.closeout.work_residual",
    "store.closeout.live_record_handles",
    "store.closeout.live_residency_bytes",
    "store.closeout.residue_classes",
];

fn named_rows<const N: usize>(
    names: &[&'static str; N],
    values: [u64; N],
) -> Vec<(&'static str, u64)> {
    names.iter().copied().zip(values).collect()
}
