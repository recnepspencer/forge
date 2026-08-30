#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationReplayContract {
    NotSupported,
    ReExecutable,
    CertReplayable {
        comparator: WorthQueryOperationReplayComparatorContract,
    },
    CertReplayableWithNoise {
        comparator: WorthQueryOperationReplayComparatorContract,
        noise: WorthQueryOperationReplayNoiseContract,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationReplayComparatorContract {
    family: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationReplayComparatorDenial {
    InvalidPortableFamily,
}

impl WorthQueryOperationReplayComparatorContract {
    /// Retains one descriptive replay-comparator family without claiming that
    /// the current host has registered an implementation for it.
    pub fn new(
        family: impl Into<String>,
    ) -> Result<Self, WorthQueryOperationReplayComparatorDenial> {
        let family = family.into();
        if family.trim().is_empty()
            || family.trim() != family
            || family.chars().any(char::is_whitespace)
        {
            return Err(WorthQueryOperationReplayComparatorDenial::InvalidPortableFamily);
        }
        Ok(Self { family })
    }

    pub fn family(&self) -> &str {
        &self.family
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryOperationReplayNoiseContract {
    pub diagnostic_warnings: bool,
}
