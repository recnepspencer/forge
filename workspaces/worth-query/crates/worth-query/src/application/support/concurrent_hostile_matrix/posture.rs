#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConcurrentHostileMatrixPosture {
    Open,
    Partial,
    Closed,
}

impl WorthQueryConcurrentHostileMatrixPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Partial => "partial",
            Self::Closed => "closed",
        }
    }
}

pub(super) fn classify_concurrent_hostile_matrix_posture(
    topology_satisfied: bool,
    artifact_replay_equal: bool,
    repeated_run_equal: bool,
    counter_residue_count: usize,
    registry_lease_count: usize,
    sabotage_sensitive: bool,
) -> WorthQueryConcurrentHostileMatrixPosture {
    if topology_satisfied
        && artifact_replay_equal
        && repeated_run_equal
        && counter_residue_count == 0
        && registry_lease_count > 0
        && sabotage_sensitive
    {
        return WorthQueryConcurrentHostileMatrixPosture::Closed;
    }
    if counter_residue_count > 0 || !topology_satisfied {
        return WorthQueryConcurrentHostileMatrixPosture::Open;
    }
    WorthQueryConcurrentHostileMatrixPosture::Partial
}
