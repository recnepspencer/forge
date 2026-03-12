#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MarketRegime {
    Calm,
    HighVol,
    SpreadBlowout,
    CurveShock,
    FxDislocation,
}
