#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpatialUndoFamilyIdentity {
    BooleanEventLedgerRollback,
    ProjectionReceiptRollback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialUndoFamilyIdentityAuthority {
    identity: SpatialUndoFamilyIdentity,
}

impl SpatialUndoFamilyIdentityAuthority {
    pub fn boolean_event_ledger() -> Self {
        Self {
            identity: SpatialUndoFamilyIdentity::BooleanEventLedgerRollback,
        }
    }

    pub fn projection_receipt() -> Self {
        Self {
            identity: SpatialUndoFamilyIdentity::ProjectionReceiptRollback,
        }
    }

    pub const fn identity(&self) -> SpatialUndoFamilyIdentity {
        self.identity
    }
}

impl SpatialUndoFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanEventLedgerRollback => "boolean-event-ledger-rollback",
            Self::ProjectionReceiptRollback => "projection-receipt-rollback",
        }
    }
}

pub fn admit_spatial_undo_family_identity(
    authority: SpatialUndoFamilyIdentityAuthority,
) -> SpatialUndoFamilyIdentity {
    authority.identity()
}
