#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MarketRegime {
    Calm,
    HighVol,
    SpreadBlowout,
    CurveShock,
    FxDislocation,
}
