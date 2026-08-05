#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ApplicationFieldPresence {
    Required,
    Optional,
}

impl ApplicationFieldPresence {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}
