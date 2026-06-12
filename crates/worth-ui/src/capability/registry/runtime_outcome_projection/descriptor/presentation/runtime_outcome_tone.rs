#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeOutcomeTone {
    Neutral,
    Progress,
    Positive,
    Advisory,
    Blocking,
    Destructive,
}

impl RuntimeOutcomeTone {
    pub fn neutral() -> Self {
        Self::Neutral
    }

    pub fn progress() -> Self {
        Self::Progress
    }

    pub fn positive() -> Self {
        Self::Positive
    }

    pub fn advisory() -> Self {
        Self::Advisory
    }

    pub fn blocking() -> Self {
        Self::Blocking
    }

    pub fn destructive() -> Self {
        Self::Destructive
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Progress => "progress",
            Self::Positive => "positive",
            Self::Advisory => "advisory",
            Self::Blocking => "blocking",
            Self::Destructive => "destructive",
        }
    }
}
