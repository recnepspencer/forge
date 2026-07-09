#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthServerOperationFamily {
    QueryDirectRead,
    QueryDirectSubmission,
    QueryDirectProjection,
    ProductApplicationRead,
    ProductApplicationMutation,
    ProductSessionCoordination,
    BinaryTransfer,
    SyncLease,
}

impl WorthServerOperationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDirectRead => "query-direct-read",
            Self::QueryDirectSubmission => "query-direct-submission",
            Self::QueryDirectProjection => "query-direct-projection",
            Self::ProductApplicationRead => "product-application-read",
            Self::ProductApplicationMutation => "product-application-mutation",
            Self::ProductSessionCoordination => "product-session-coordination",
            Self::BinaryTransfer => "binary-transfer",
            Self::SyncLease => "sync-lease",
        }
    }
}
