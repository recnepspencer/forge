use super::open_class::OpenTopologyClass;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenClassTriadParityDenialKind {
    MissingDeclaration,
    MissingOpenClass,
    DuplicateOpenClass,
    UnsupportedOpenClass,
    ParityReceiptRejected,
    TopologyParityMismatch,
    BoundedConversionViolation,
    CrossClassCheckpointReplay,
    ProjectionConsumptionMismatch,
    StormExtractionUnsupported,
    DeniedLaneUpgrade,
    MissingLaneEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassTriadParityDenial {
    kind: OpenClassTriadParityDenialKind,
    source_class: Option<OpenTopologyClass>,
    target_class: Option<OpenTopologyClass>,
    lane: Option<ProjectionFactParityLane>,
    human_reason: String,
}

impl OpenClassTriadParityDenial {
    pub(crate) fn new(
        kind: OpenClassTriadParityDenialKind,
        target_class: Option<OpenTopologyClass>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_class: None,
            target_class,
            lane: None,
            human_reason: human_reason.into(),
        }
    }

    pub(crate) fn with_source(
        kind: OpenClassTriadParityDenialKind,
        source_class: OpenTopologyClass,
        target_class: OpenTopologyClass,
        lane: Option<ProjectionFactParityLane>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_class: Some(source_class),
            target_class: Some(target_class),
            lane,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> OpenClassTriadParityDenialKind {
        self.kind
    }

    pub fn source_class(&self) -> Option<OpenTopologyClass> {
        self.source_class
    }

    pub fn target_class(&self) -> Option<OpenTopologyClass> {
        self.target_class
    }

    pub fn lane(&self) -> Option<ProjectionFactParityLane> {
        self.lane
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
