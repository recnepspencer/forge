#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoverySurfaceGapPosture {
    Closed,
    DeferredToLaterPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoverySurfaceGap {
    pub surface: &'static str,
    pub posture: RecoverySurfaceGapPosture,
    pub resolution: &'static str,
}

const GAPS: &[RecoverySurfaceGap] = &[
    RecoverySurfaceGap {
        surface: "behavioral operations vocabulary",
        posture: RecoverySurfaceGapPosture::Closed,
        resolution: "crate deleted; owner capabilities are local",
    },
    RecoverySurfaceGap {
        surface: "milestone-shaped custody and repair handoffs",
        posture: RecoverySurfaceGapPosture::Closed,
        resolution: "replaced by domain-named owner admission",
    },
    RecoverySurfaceGap {
        surface: "malicious control-media authenticity",
        posture: RecoverySurfaceGapPosture::DeferredToLaterPhase,
        resolution: "S.11 authenticated record chain",
    },
    RecoverySurfaceGap {
        surface: "restore drill certification",
        posture: RecoverySurfaceGapPosture::DeferredToLaterPhase,
        resolution: "S.10 Phase 13 ordinary restore and reopen",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentRecoverySurfaceGapReport;

impl CurrentRecoverySurfaceGapReport {
    pub const fn current() -> Self {
        Self
    }
    pub const fn gaps(self) -> &'static [RecoverySurfaceGap] {
        GAPS
    }
}
