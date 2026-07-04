#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoPlannerRouteFamily {
    Replay,
    Undo,
    Transaction,
}

impl ReplayUndoPlannerRouteFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::Undo => "undo",
            Self::Transaction => "transaction",
        }
    }
}
