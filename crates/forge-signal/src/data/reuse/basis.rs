use serde::{Deserialize, Serialize};

/// Compact hot-path classification for how an artifact became current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseBasis {
    #[default]
    FreshCompute,
    Reused {
        source: ReuseSource,
        crossing: ReuseCrossing,
    },
}

/// The source lane from which an existing artifact was reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseSource {
    #[default]
    None,
    MemoizedArtifact,
    SnapshotArtifact,
    AuthorityReconciliation,
}

/// The runtime boundary crossed, if any, while reusing an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReuseCrossing {
    #[default]
    None,
    SnapshotRestore,
    AuthorityBoundary,
}
