#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationQueryLane {
    OneShot,
    Continuation,
    Historical,
    Live,
    Preview,
}

impl WorthQueryApplicationQueryLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneShot => "one_shot",
            Self::Continuation => "continuation",
            Self::Historical => "historical",
            Self::Live => "live",
            Self::Preview => "preview",
        }
    }
}
