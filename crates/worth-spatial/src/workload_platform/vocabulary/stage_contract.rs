#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadStage {
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    BatchAdmissionExecution,
    Diagnostics,
    Response,
}

impl WorkloadStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GeometryBinding => "geometry binding",
            Self::SurfaceSupport => "surface support",
            Self::Projection => "projection",
            Self::Transform => "transform",
            Self::RetainedReplay => "retained replay",
            Self::BatchAdmissionExecution => "batch admission execution",
            Self::Diagnostics => "diagnostics",
            Self::Response => "response",
        }
    }
}

pub type SpatialWorkloadStage = WorkloadStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadStageSupport {
    Admitted,
    Unsupported,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadStagePosture {
    stage: WorkloadStage,
    support: WorkloadStageSupport,
    reason: String,
}

impl WorkloadStagePosture {
    pub fn admitted(stage: WorkloadStage, reason: impl Into<String>) -> Self {
        Self::new(stage, WorkloadStageSupport::Admitted, reason)
    }

    pub fn unsupported(stage: WorkloadStage, reason: impl Into<String>) -> Self {
        Self::new(stage, WorkloadStageSupport::Unsupported, reason)
    }

    pub fn blocked(stage: WorkloadStage, reason: impl Into<String>) -> Self {
        Self::new(stage, WorkloadStageSupport::Blocked, reason)
    }

    pub(crate) fn new(
        stage: WorkloadStage,
        support: WorkloadStageSupport,
        reason: impl Into<String>,
    ) -> Self {
        let reason = normalize_reason(reason);
        Self {
            stage,
            support,
            reason,
        }
    }

    pub fn stage(&self) -> WorkloadStage {
        self.stage
    }

    pub fn support(&self) -> WorkloadStageSupport {
        self.support
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "workload stage support posture requires a human-readable reason".to_string()
    } else {
        reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadStageIdentity {
    stage: WorkloadStage,
    declaration: String,
    upstream_receipt: String,
}

impl WorkloadStageIdentity {
    pub(crate) fn new(stage: WorkloadStage, declaration: String, upstream_receipt: String) -> Self {
        Self {
            stage,
            declaration,
            upstream_receipt,
        }
    }

    pub fn stage(&self) -> WorkloadStage {
        self.stage
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn upstream_receipt(&self) -> &str {
        &self.upstream_receipt
    }

    pub fn receipt_identity(&self) -> String {
        format!(
            "stage={};declaration={};upstream={}",
            self.stage.as_str(),
            self.declaration,
            self.upstream_receipt
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadStageEnvelope {
    identity: WorkloadStageIdentity,
    posture: WorkloadStagePosture,
}

impl WorkloadStageEnvelope {
    pub(crate) fn new(identity: WorkloadStageIdentity, posture: WorkloadStagePosture) -> Self {
        Self { identity, posture }
    }

    pub fn identity(&self) -> &WorkloadStageIdentity {
        &self.identity
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadStageDenial {
    MissingDeclaration,
    MissingUpstreamReceipt,
    UnsupportedStage,
    BlockedStage,
}

impl WorkloadStageDenial {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::MissingDeclaration => "workload stage requires a declaration",
            Self::MissingUpstreamReceipt => "workload stage requires an upstream receipt",
            Self::UnsupportedStage => "workload stage is unsupported by this runtime",
            Self::BlockedStage => "workload stage is blocked by an unmet prerequisite",
        }
    }
}

pub(crate) fn certify_stage(
    stage: WorkloadStage,
    declaration: String,
    upstream_receipt: String,
    support: WorkloadStageSupport,
    reason: impl Into<String>,
) -> Result<WorkloadStageEnvelope, WorkloadStageDenial> {
    if declaration.trim().is_empty() {
        return Err(WorkloadStageDenial::MissingDeclaration);
    }
    if upstream_receipt.trim().is_empty() {
        return Err(WorkloadStageDenial::MissingUpstreamReceipt);
    }

    match support {
        WorkloadStageSupport::Admitted => Ok(WorkloadStageEnvelope::new(
            WorkloadStageIdentity::new(stage, declaration, upstream_receipt),
            WorkloadStagePosture::new(stage, support, reason),
        )),
        WorkloadStageSupport::Unsupported => Err(WorkloadStageDenial::UnsupportedStage),
        WorkloadStageSupport::Blocked => Err(WorkloadStageDenial::BlockedStage),
    }
}
