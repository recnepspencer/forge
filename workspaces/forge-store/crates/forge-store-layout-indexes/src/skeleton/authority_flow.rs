#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ForbiddenAuthoritySource {
    CertificationCloseout,
    PhysicalCertificationHarness,
    TestSupportFixture,
    OfflineVerifierObservation,
    FoundationalMaterializedReport,
    CopiedCounterRow,
    TerminalProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AuthorityFlowEdge {
    from_lane: &'static str,
    to_lane: &'static str,
    meaning: &'static str,
}

const REQUIRED_FLOW: &[S8AuthorityFlowEdge] = &[
    S8AuthorityFlowEdge::new(
        "family declaration",
        "layout grammar",
        "family crates admit their local artifact families into S.8 grammar",
    ),
    S8AuthorityFlowEdge::new(
        "layout grammar",
        "family execution",
        "layout grammar lowers execution posture without owning execution truth",
    ),
    S8AuthorityFlowEdge::new(
        "family execution",
        "foundational boundary evidence",
        "executed Store counters may materialize boundary evidence after execution",
    ),
    S8AuthorityFlowEdge::new(
        "foundational boundary evidence",
        "courtroom proof",
        "certification proves executed Store law without minting new runtime authority",
    ),
];

const FORBIDDEN_SOURCES: &[S8ForbiddenAuthoritySource] = &[
    S8ForbiddenAuthoritySource::CertificationCloseout,
    S8ForbiddenAuthoritySource::PhysicalCertificationHarness,
    S8ForbiddenAuthoritySource::TestSupportFixture,
    S8ForbiddenAuthoritySource::OfflineVerifierObservation,
    S8ForbiddenAuthoritySource::FoundationalMaterializedReport,
    S8ForbiddenAuthoritySource::CopiedCounterRow,
    S8ForbiddenAuthoritySource::TerminalProjection,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8CrossCrateAuthorityFlowReport;

impl S8AuthorityFlowEdge {
    pub const fn new(
        from_lane: &'static str,
        to_lane: &'static str,
        meaning: &'static str,
    ) -> Self {
        Self {
            from_lane,
            to_lane,
            meaning,
        }
    }

    pub const fn from_lane(&self) -> &'static str {
        self.from_lane
    }

    pub const fn to_lane(&self) -> &'static str {
        self.to_lane
    }

    pub const fn meaning(&self) -> &'static str {
        self.meaning
    }
}

impl S8CrossCrateAuthorityFlowReport {
    pub const fn current() -> Self {
        Self
    }

    pub const fn required_edges(&self) -> &'static [S8AuthorityFlowEdge] {
        REQUIRED_FLOW
    }

    pub const fn forbidden_sources(&self) -> &'static [S8ForbiddenAuthoritySource] {
        FORBIDDEN_SOURCES
    }
}
