#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandProjectionSelectionMode {
    SingleSelect,
    MultiSelect,
}

impl CommandProjectionSelectionMode {
    pub const fn token(self) -> &'static str {
        match self {
            Self::SingleSelect => "single_select",
            Self::MultiSelect => "multi_select",
        }
    }
}
