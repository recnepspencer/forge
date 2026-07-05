#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind {
    MissingPhaseFifteenPublicContractRow,
    MissingPhaseFifteenAntiTheatreGuard,
    ReadinessConsumerMismatch,
    ReadinessBindingMismatch,
    LoopLedgerMismatch,
    OverlapLedgerMismatch,
    RequestIdentityMismatch,
    ReplayParityMismatch,
    CheckpointParityMismatch,
    MissingReplayParityRow,
    MissingCanonicalIdentity,
    BoundaryOnlyAreaAdmission,
    OppositeSenseWindingInstability,
    NestedIdentityInstability,
    MixedBoundaryAreaCollapse,
    OrderingParityInstability,
    OverlapStormShapeViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutDenial {
    kind: PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind,
    subcase_name: &'static str,
    detail: String,
}

impl PlanarBooleanOverlapRegionSummumBonumCloseoutDenial {
    pub fn new(
        kind: PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind,
        subcase_name: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subcase_name,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn subcase_name(&self) -> &'static str {
        self.subcase_name
    }
}
