/// Ordering posture for entries inside a command projection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionOrdering {
    Declaration,
    ByCommandId,
    ByCategoryThenCommandId,
}

impl CommandProjectionOrdering {
    pub fn digest_basis(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::ByCommandId => "by_command_id",
            Self::ByCategoryThenCommandId => "by_category_then_command_id",
        }
    }
}
