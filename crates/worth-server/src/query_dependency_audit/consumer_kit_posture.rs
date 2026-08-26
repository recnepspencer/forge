#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthServerQueryDependencyConsumerKitPosture {
    QuerySupportSnapshotAndPinningAdopted,
    QuerySupportSnapshotAndPinningBlocked,
}

impl WorthServerQueryDependencyConsumerKitPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuerySupportSnapshotAndPinningAdopted => {
                "query-support-snapshot-and-pinning-adopted"
            }
            Self::QuerySupportSnapshotAndPinningBlocked => {
                "query-support-snapshot-and-pinning-blocked"
            }
        }
    }
}
