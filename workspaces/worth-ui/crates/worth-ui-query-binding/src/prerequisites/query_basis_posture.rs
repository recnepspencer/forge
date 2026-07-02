#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBasisPosture {
    GraphAligned,
    WrongWorldProjection,
    RebindRequired,
    StaleReceipt,
    AmbiguousSources,
}
