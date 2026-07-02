#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionQueryBasis {
    GraphAligned,
    WrongWorldProjection,
    RebindRequired,
    StaleReceipt,
    AmbiguousSources,
}

impl UiAdmissionQueryBasis {
    pub const fn graph_aligned() -> Self {
        Self::GraphAligned
    }
}
