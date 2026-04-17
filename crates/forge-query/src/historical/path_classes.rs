#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RequestedHistoricalPathClass {
    RequestedRetainedSnapshotPath,
    RequestedDeltaReplayPath,
    RequestedFullReconstructionPath,
}

impl RequestedHistoricalPathClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequestedRetainedSnapshotPath => "requested_retained_snapshot_path",
            Self::RequestedDeltaReplayPath => "requested_delta_replay_path",
            Self::RequestedFullReconstructionPath => "requested_full_reconstruction_path",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AdmittedHistoricalPathClass {
    AdmittedRetainedSnapshotPath,
    AdmittedDeltaReplayPath,
    AdmittedFullReconstructionPath,
}

impl AdmittedHistoricalPathClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdmittedRetainedSnapshotPath => "admitted_retained_snapshot_path",
            Self::AdmittedDeltaReplayPath => "admitted_delta_replay_path",
            Self::AdmittedFullReconstructionPath => "admitted_full_reconstruction_path",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResolvedHistoricalPathClass {
    ResolvedRetainedSnapshotPath,
    ResolvedDeltaReplayPath,
    ResolvedFullReconstructionPath,
}

impl ResolvedHistoricalPathClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ResolvedRetainedSnapshotPath => "resolved_retained_snapshot_path",
            Self::ResolvedDeltaReplayPath => "resolved_delta_replay_path",
            Self::ResolvedFullReconstructionPath => "resolved_full_reconstruction_path",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoricalPathCompatibilityOutcome {
    Admitted,
    Denied,
    SubstitutionDenied,
}

impl HistoricalPathCompatibilityOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
            Self::SubstitutionDenied => "substitution_denied",
        }
    }
}
