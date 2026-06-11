#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceKind {
    TopologySeed,
    TopologyCommit,
    PrimitiveCorpus,
    SpatialFixture,
    MetabossHarness,
    ReExtractionReplayHelper,
    CloseoutEvidenceFixture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceAuthority {
    QueryBackedTopology,
    QueryBackedSpatialContract,
    TestLocalConvenience,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPosture {
    OwnsTopologyTruth,
    ConsumesTopologyTruth,
    BypassesTopologyTruth,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptPosture {
    ProductionOwned,
    TestLocal,
    NoReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceScope {
    WorkloadCandidate,
    UnitSupportOnly,
    LegacyMigrationOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorkloadSurfaceId(&'static str);

impl WorkloadSurfaceId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LegacyFixtureClassification {
    surface_id: WorkloadSurfaceId,
    surface_kind: SurfaceKind,
    authority: SurfaceAuthority,
    topology_posture: TopologyPosture,
    receipt_posture: ReceiptPosture,
    scope: SurfaceScope,
    human_reason: &'static str,
}

impl LegacyFixtureClassification {
    pub const fn new(
        surface_id: WorkloadSurfaceId,
        surface_kind: SurfaceKind,
        authority: SurfaceAuthority,
        topology_posture: TopologyPosture,
        receipt_posture: ReceiptPosture,
        scope: SurfaceScope,
        human_reason: &'static str,
    ) -> Self {
        Self {
            surface_id,
            surface_kind,
            authority,
            topology_posture,
            receipt_posture,
            scope,
            human_reason,
        }
    }

    pub const fn surface_id(self) -> WorkloadSurfaceId {
        self.surface_id
    }

    pub const fn surface_kind(self) -> SurfaceKind {
        self.surface_kind
    }

    pub const fn authority(self) -> SurfaceAuthority {
        self.authority
    }

    pub const fn topology_posture(self) -> TopologyPosture {
        self.topology_posture
    }

    pub const fn receipt_posture(self) -> ReceiptPosture {
        self.receipt_posture
    }

    pub const fn scope(self) -> SurfaceScope {
        self.scope
    }

    pub const fn human_reason(self) -> &'static str {
        self.human_reason
    }
}
