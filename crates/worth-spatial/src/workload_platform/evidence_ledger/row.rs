#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceStage {
    Topology,
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    Diagnostics,
    Response,
    Operator,
    BooleanDeclarationEntry,
    BooleanRoutePlan,
    BooleanOperandPairConstruction,
    BooleanBlockerProvenance,
    BooleanSplit,
    BooleanClassify,
    BooleanAssemble,
    BooleanCleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceRow {
    stage: WorkloadEvidenceStage,
    evidence_identity: String,
    backing: WorkloadEvidenceBacking,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl WorkloadEvidenceRow {
    pub fn new(stage: WorkloadEvidenceStage, evidence_identity: impl Into<String>) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Manual,
            support: WorkloadEvidenceSupport::Manual,
            counters: WorkloadEvidenceStageCounters::default(),
        }
    }

    pub(crate) fn receipt_backed(
        stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        counters: WorkloadEvidenceStageCounters,
    ) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Receipt,
            support: WorkloadEvidenceSupport::Admitted,
            counters,
        }
    }

    pub(crate) fn receipt_backed_with_support(
        stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        support: WorkloadEvidenceSupport,
        counters: WorkloadEvidenceStageCounters,
    ) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Receipt,
            support,
            counters,
        }
    }

    pub fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn backing(&self) -> WorkloadEvidenceBacking {
        self.backing
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn is_receipt_backed(&self) -> bool {
        self.backing == WorkloadEvidenceBacking::Receipt
    }

    pub fn is_admitted(&self) -> bool {
        self.support == WorkloadEvidenceSupport::Admitted
    }

    pub fn from_boolean_evidence_receipt(receipt: &impl BooleanEvidenceReceipt) -> Self {
        Self::receipt_backed_with_support(
            receipt.boolean_stage().evidence_stage(),
            receipt.evidence_identity(),
            receipt.evidence_support(),
            receipt.evidence_counters(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceBacking {
    Receipt,
    Manual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceSupport {
    Admitted,
    Unsupported,
    Blocked,
    Manual,
}

pub trait BooleanEvidenceReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind;
    fn evidence_identity(&self) -> &str;
    fn evidence_support(&self) -> WorkloadEvidenceSupport;
    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BooleanEvidenceStageKind {
    DeclarationEntry,
    RoutePlan,
    OperandPairConstruction,
    BlockerProvenance,
    Split,
    Classify,
    Assemble,
    Cleanup,
}

impl WorkloadEvidenceStage {
    pub const AUTHORITY_STAGES: [Self; 8] = [
        Self::Topology,
        Self::GeometryBinding,
        Self::SurfaceSupport,
        Self::Projection,
        Self::Transform,
        Self::RetainedReplay,
        Self::Diagnostics,
        Self::Response,
    ];

    pub const BOOLEAN_STAGES: [Self; 8] = [
        Self::BooleanDeclarationEntry,
        Self::BooleanRoutePlan,
        Self::BooleanOperandPairConstruction,
        Self::BooleanBlockerProvenance,
        Self::BooleanSplit,
        Self::BooleanClassify,
        Self::BooleanAssemble,
        Self::BooleanCleanup,
    ];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Topology => "topology evidence",
            Self::GeometryBinding => "geometry binding evidence",
            Self::SurfaceSupport => "surface support evidence",
            Self::Projection => "projection evidence",
            Self::Transform => "transform evidence",
            Self::RetainedReplay => "retained replay evidence",
            Self::Diagnostics => "diagnostic evidence",
            Self::Response => "response evidence",
            Self::Operator => "operator evidence",
            Self::BooleanDeclarationEntry => "boolean declaration entry evidence",
            Self::BooleanRoutePlan => "boolean route plan evidence",
            Self::BooleanOperandPairConstruction => "boolean operand pair construction evidence",
            Self::BooleanBlockerProvenance => "boolean blocker provenance evidence",
            Self::BooleanSplit => "boolean split evidence",
            Self::BooleanClassify => "boolean classify evidence",
            Self::BooleanAssemble => "boolean assemble evidence",
            Self::BooleanCleanup => "boolean cleanup evidence",
        }
    }

    pub fn is_boolean_stage(self) -> bool {
        Self::BOOLEAN_STAGES.contains(&self)
    }
}

impl BooleanEvidenceStageKind {
    fn evidence_stage(self) -> WorkloadEvidenceStage {
        match self {
            Self::DeclarationEntry => WorkloadEvidenceStage::BooleanDeclarationEntry,
            Self::RoutePlan => WorkloadEvidenceStage::BooleanRoutePlan,
            Self::OperandPairConstruction => WorkloadEvidenceStage::BooleanOperandPairConstruction,
            Self::BlockerProvenance => WorkloadEvidenceStage::BooleanBlockerProvenance,
            Self::Split => WorkloadEvidenceStage::BooleanSplit,
            Self::Classify => WorkloadEvidenceStage::BooleanClassify,
            Self::Assemble => WorkloadEvidenceStage::BooleanAssemble,
            Self::Cleanup => WorkloadEvidenceStage::BooleanCleanup,
        }
    }
}
use super::WorkloadEvidenceStageCounters;
