#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationRevocationObservation {
    NotRevoked {
        observed_at: u64,
    },
    Revoked {
        observed_at: u64,
        reason_fingerprint: [u8; 32],
    },
    Unavailable {
        observed_at: u64,
    },
}

impl AuthorizationRevocationObservation {
    pub const fn observed_at(self) -> u64 {
        match self {
            Self::NotRevoked { observed_at }
            | Self::Revoked { observed_at, .. }
            | Self::Unavailable { observed_at } => observed_at,
        }
    }
}
