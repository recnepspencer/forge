#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthTopologyDomainQueryFallbackPosture {
    None,
    SnapshotIndexedFallback,
    WholeViewDebt,
}

#[allow(dead_code)]
impl WorthTopologyDomainQueryFallbackPosture {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SnapshotIndexedFallback => "snapshot_indexed_fallback",
            Self::WholeViewDebt => "whole_view_debt",
        }
    }
}
