use crate::temporal::BridgeTemporalBasisKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeTemporalSubscriptionFamilyKind {
    WakeDriven,
    HistoricalReplay,
}

impl BridgeTemporalSubscriptionFamilyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WakeDriven => "wake_driven",
            Self::HistoricalReplay => "historical_replay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeTemporalSubscriptionFamily {
    kind: BridgeTemporalSubscriptionFamilyKind,
}

impl BridgeTemporalSubscriptionFamily {
    pub const fn for_kind(kind: BridgeTemporalSubscriptionFamilyKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> BridgeTemporalSubscriptionFamilyKind {
        self.kind
    }

    pub const fn supports_basis_kind(self, basis_kind: BridgeTemporalBasisKind) -> bool {
        match self.kind {
            BridgeTemporalSubscriptionFamilyKind::WakeDriven => matches!(
                basis_kind,
                BridgeTemporalBasisKind::Authoritative | BridgeTemporalBasisKind::BranchHead
            ),
            BridgeTemporalSubscriptionFamilyKind::HistoricalReplay => matches!(
                basis_kind,
                BridgeTemporalBasisKind::Historical | BridgeTemporalBasisKind::CdcCursor
            ),
        }
    }
}
