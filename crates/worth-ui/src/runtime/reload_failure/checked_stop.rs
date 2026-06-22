#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadCheckedStopPosture {
    NotQueryCaused,
    QuerySupportDenied,
    QueryRecoveryPreserved,
}

impl WorthUiReloadCheckedStopPosture {
    pub(crate) fn ordinary() -> Self {
        Self::NotQueryCaused
    }

    pub(crate) fn query_support_denied() -> Self {
        Self::QuerySupportDenied
    }

    pub(crate) fn query_recovery_preserved() -> Self {
        Self::QueryRecoveryPreserved
    }

    pub fn is_query_checked_stop(self) -> bool {
        !matches!(self, Self::NotQueryCaused)
    }
}
