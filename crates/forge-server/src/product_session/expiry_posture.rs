#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductSessionExpiryPosture {
    Active {
        expires_at_epoch_millis: u64,
    },
    Expired {
        expires_at_epoch_millis: u64,
        observed_at_epoch_millis: u64,
    },
    Closed {
        closed_at_epoch_millis: u64,
    },
}

impl ForgeServerProductSessionExpiryPosture {
    pub fn expires_at_epoch_millis(&self) -> Option<u64> {
        match self {
            Self::Active {
                expires_at_epoch_millis,
            }
            | Self::Expired {
                expires_at_epoch_millis,
                ..
            } => Some(*expires_at_epoch_millis),
            Self::Closed { .. } => None,
        }
    }
}
