use std::path::PathBuf;

#[derive(Debug)]
pub enum OfflineMediaReadDenial {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    NotAFile {
        path: PathBuf,
    },
    ConcurrentMutationIndeterminate {
        path: PathBuf,
    },
    UnprovenCrossFileConsistency,
    ContentClosureMissingArtifact {
        path: PathBuf,
    },
    ContentClosureUnexpectedArtifact {
        path: PathBuf,
    },
    ContentClosureArtifactMismatch {
        path: PathBuf,
    },
    InvalidFileIndex,
    InvalidReadOffset {
        path: PathBuf,
        offset: u64,
        length: u64,
    },
    UnexpectedEof {
        path: PathBuf,
        offset: u64,
        length: u64,
    },
    ZeroReadBudget,
    AllocationFailed,
    OwnedAllocationBudgetExceeded {
        admitted: u64,
        limit: u64,
    },
    CounterOverflow,
}
