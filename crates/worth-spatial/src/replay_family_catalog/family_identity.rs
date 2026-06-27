#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpatialReplayFamilyIdentity {
    BooleanEventLedgerReplay,
    ProjectionReceiptReplay,
}

impl SpatialReplayFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanEventLedgerReplay => "boolean-event-ledger-replay",
            Self::ProjectionReceiptReplay => "projection-receipt-replay",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialReplayFamilyIdentityAuthority {
    identity: SpatialReplayFamilyIdentity,
}

impl SpatialReplayFamilyIdentityAuthority {
    pub fn boolean_event_ledger() -> Self {
        Self {
            identity: SpatialReplayFamilyIdentity::BooleanEventLedgerReplay,
        }
    }

    pub fn projection_receipt() -> Self {
        Self {
            identity: SpatialReplayFamilyIdentity::ProjectionReceiptReplay,
        }
    }

    pub const fn identity(&self) -> SpatialReplayFamilyIdentity {
        self.identity
    }
}

pub fn admit_spatial_replay_family_identity(
    authority: SpatialReplayFamilyIdentityAuthority,
) -> SpatialReplayFamilyIdentity {
    authority.identity()
}
