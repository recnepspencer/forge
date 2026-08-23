#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationAsyncPosture {
    Pending,
    Current,
    Stale,
    Failed,
    Cancelled,
    Superseded,
    Unresolved,
}

impl WorthUiPresentationAsyncPosture {
    pub fn from_query(
        posture: worth_query::facade::runtime::WorthQueryRuntimeAsyncResultStateKind,
    ) -> Self {
        use worth_query::facade::runtime::WorthQueryRuntimeAsyncResultStateKind as Query;
        match posture {
            Query::Pending => Self::Pending,
            Query::Current => Self::Current,
            Query::Stale => Self::Stale,
            Query::Failed => Self::Failed,
            Query::Cancelled => Self::Cancelled,
            Query::Superseded => Self::Superseded,
            Query::Unresolved => Self::Unresolved,
            Query::Retried | Query::Revalidating => Self::Pending,
            Query::Denied => Self::Failed,
        }
    }
}
