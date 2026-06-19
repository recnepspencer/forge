#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerQueryDependencyConsumerKitPosture {
    QuerySupportSnapshotAndPinningAdopted,
    QuerySupportSnapshotAndPinningBlocked,
    QueryBoundaryAuditAdopted,
    QueryBoundaryAuditBlocked,
    QueryTestBackendResidueAuditAdopted,
    LocalFolklore,
}

impl ForgeServerQueryDependencyConsumerKitPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QuerySupportSnapshotAndPinningAdopted => {
                "query-support-snapshot-and-pinning-adopted"
            }
            Self::QuerySupportSnapshotAndPinningBlocked => {
                "query-support-snapshot-and-pinning-blocked"
            }
            Self::QueryBoundaryAuditAdopted => "query-boundary-audit-adopted",
            Self::QueryBoundaryAuditBlocked => "query-boundary-audit-blocked",
            Self::QueryTestBackendResidueAuditAdopted => "query-test-backend-residue-audit-adopted",
            Self::LocalFolklore => "local-folklore",
        }
    }
}
