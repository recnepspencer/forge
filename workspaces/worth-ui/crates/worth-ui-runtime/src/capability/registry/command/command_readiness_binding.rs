#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandReadinessStatus {
    Admitted,
    Deferred,
    Unsupported,
    InvalidBasis,
}

/// UI-owned command readiness meaning. Query adapters must translate at the
/// binding edge; command registration never retains Query reports or digests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandReadinessBinding {
    status: CommandReadinessStatus,
}

impl CommandReadinessBinding {
    pub fn always_admitted() -> Self {
        Self { status: CommandReadinessStatus::Admitted }
    }

    pub fn from_status(status: CommandReadinessStatus) -> Self {
        Self { status }
    }

    pub fn strongest_status(&self) -> CommandReadinessStatus {
        self.status
    }

    pub fn readiness_digest(&self) -> Option<&str> {
        None
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self.status {
            CommandReadinessStatus::Admitted => "admitted",
            CommandReadinessStatus::Deferred => "deferred",
            CommandReadinessStatus::Unsupported => "unsupported",
            CommandReadinessStatus::InvalidBasis => "invalid_basis",
        }.to_owned()
    }
}
