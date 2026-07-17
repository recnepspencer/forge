#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalArtifactPolicy {
    RetainOnInterruption,
    AbandonWithDurableDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalExecutionPolicy {
    deadline_tick: Option<u64>,
    maximum_resident_bytes: u64,
    maximum_in_flight_io: u64,
    artifact_policy: OperationalArtifactPolicy,
}

impl OperationalExecutionPolicy {
    pub const fn bounded(
        deadline_tick: Option<u64>,
        maximum_resident_bytes: u64,
        maximum_in_flight_io: u64,
        artifact_policy: OperationalArtifactPolicy,
    ) -> Option<Self> {
        if maximum_resident_bytes == 0 || maximum_in_flight_io == 0 {
            return None;
        }
        Some(Self {
            deadline_tick,
            maximum_resident_bytes,
            maximum_in_flight_io,
            artifact_policy,
        })
    }

    pub const fn deadline_tick(self) -> Option<u64> {
        self.deadline_tick
    }
    pub const fn maximum_resident_bytes(self) -> u64 {
        self.maximum_resident_bytes
    }
    pub const fn maximum_in_flight_io(self) -> u64 {
        self.maximum_in_flight_io
    }
    pub const fn artifact_policy(self) -> OperationalArtifactPolicy {
        self.artifact_policy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalComplexityContract {
    kind: super::OperationalSessionKind,
    time_variables: &'static [&'static str],
    space_variables: &'static [&'static str],
    reconstructive: bool,
}

impl OperationalComplexityContract {
    pub const fn for_kind(kind: super::OperationalSessionKind) -> Self {
        use super::OperationalSessionKind as Kind;
        match kind {
            Kind::ReplicaBootstrap => Self::new(
                kind,
                &["source_bytes", "wal_tail_bytes", "blob_bytes"],
                &["buffer_budget", "in_flight_io"],
                true,
            ),
            Kind::ReplicaPromotion => Self::new(
                kind,
                &["candidate_reports", "fence_round_trips"],
                &["candidate_metadata"],
                false,
            ),
            Kind::ForensicAcquisition => Self::new(
                kind,
                &["source_bytes", "source_files"],
                &["buffer_budget", "source_metadata"],
                true,
            ),
            Kind::OfflineVerification => Self::new(
                kind,
                &["media_bytes", "artifact_count"],
                &["buffer_budget", "closure_metadata"],
                false,
            ),
            Kind::Backup => Self::new(
                kind,
                &["reachable_bytes", "wal_tail_bytes"],
                &["buffer_budget", "reachability_metadata"],
                true,
            ),
            Kind::Restore | Kind::PointInTimeRecovery | Kind::Rollback => Self::new(
                kind,
                &["output_bytes", "wal_tail_bytes"],
                &["buffer_budget", "owner_dag"],
                true,
            ),
            Kind::Repair => Self::new(
                kind,
                &["damaged_region_bytes", "owner_nodes"],
                &["buffer_budget", "owner_dag"],
                true,
            ),
        }
    }

    const fn new(
        kind: super::OperationalSessionKind,
        time_variables: &'static [&'static str],
        space_variables: &'static [&'static str],
        reconstructive: bool,
    ) -> Self {
        Self {
            kind,
            time_variables,
            space_variables,
            reconstructive,
        }
    }

    pub const fn kind(self) -> super::OperationalSessionKind {
        self.kind
    }
    pub const fn time_variables(self) -> &'static [&'static str] {
        self.time_variables
    }
    pub const fn space_variables(self) -> &'static [&'static str] {
        self.space_variables
    }
    pub const fn reconstructive(self) -> bool {
        self.reconstructive
    }
}
