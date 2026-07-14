use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::runtime::{
    WorthQueryOrdinaryAuthorityAdmission, WorthQueryPreviewCloseoutKind, WorthQueryPreviewOutcome,
    WorthQueryRuntimeError,
};
use crate::session_label::WorthQuerySessionLabel;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryWorkflowCounters {
    context_validation_count: usize,
    session_open_attempt_count: usize,
    lower_runtime_execution_attempt_count: usize,
    lower_runtime_execution_completed_count: usize,
    inspection_materialization_count: usize,
}

impl WorthQueryWorkflowCounters {
    pub fn context_validation_count(&self) -> usize {
        self.context_validation_count
    }

    pub fn session_open_attempt_count(&self) -> usize {
        self.session_open_attempt_count
    }

    pub fn lower_runtime_execution_attempt_count(&self) -> usize {
        self.lower_runtime_execution_attempt_count
    }

    pub fn lower_runtime_execution_completed_count(&self) -> usize {
        self.lower_runtime_execution_completed_count
    }

    pub fn inspection_materialization_count(&self) -> usize {
        self.inspection_materialization_count
    }

    pub(crate) fn context_checked() -> Self {
        Self {
            context_validation_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn execution_attempted(mut self) -> Self {
        self.session_open_attempt_count += 1;
        self.lower_runtime_execution_attempt_count += 1;
        self
    }

    pub(crate) fn execution_completed(mut self) -> Self {
        self.lower_runtime_execution_completed_count += 1;
        self.inspection_materialization_count += 1;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPromotionEligibility {
    session_label: WorthQuerySessionLabel,
    snapshot_identity: WorthQuerySnapshotIdentity,
    eligibility_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryPromotionEligibility {
    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.session_label
    }

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.eligibility_identity
    }

    pub fn identity_for_reporting(&self) -> &str {
        self.eligibility_identity.as_str()
    }

    pub(crate) fn from_authority(authority: &WorthQueryOrdinaryAuthorityAdmission) -> Self {
        let session_label = authority
            .session_label()
            .expect("preview authority must carry a session label")
            .clone();
        let snapshot_identity = authority.snapshot_identity().clone();
        let eligibility_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::PreviewPromotionContinuation,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "ordinary-promotion-eligibility",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authority"),
            authority.admission_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("snapshot"),
            &snapshot_identity.evidence_identity(),
        )
        .seal();
        Self {
            session_label,
            snapshot_identity,
            eligibility_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedWorkflowEffect {
    effect_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryAdmittedWorkflowEffect {
    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.effect_identity
    }

    pub(crate) fn new(
        declaration_identity: &WorthQueryEvidenceIdentity,
        authority: &WorthQueryOrdinaryAuthorityAdmission,
    ) -> Self {
        Self {
            effect_identity: WorthQueryEvidenceIdentity::compose(
                WorthQueryEvidenceScope::PreviewExecutionEvidence,
            )
            .field_shape(
                WorthQueryEvidenceTag::new("role"),
                "ordinary-admitted-workflow-effect",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("declaration"),
                declaration_identity,
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("authority"),
                authority.admission_identity(),
            )
            .seal(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLoweredWorkflowPlan {
    request_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryLoweredWorkflowPlan {
    pub fn request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub(crate) fn new(request_identity: WorthQueryEvidenceIdentity) -> Self {
        Self { request_identity }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowAftermath {
    closeout_kind: WorthQueryPreviewCloseoutKind,
    receipt_identity: WorthQueryEvidenceIdentity,
    aftermath_identity: WorthQueryEvidenceIdentity,
    inspection_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryWorkflowAftermath {
    pub fn closeout_kind(&self) -> WorthQueryPreviewCloseoutKind {
        self.closeout_kind
    }

    pub fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }

    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.aftermath_identity
    }

    pub fn inspection_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.inspection_identity
    }

    pub(crate) fn new(
        outcome: &WorthQueryPreviewOutcome,
        receipt_identity: WorthQueryEvidenceIdentity,
        aftermath_identity: WorthQueryEvidenceIdentity,
        inspection_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            closeout_kind: outcome.closeout_evidence().kind(),
            receipt_identity,
            aftermath_identity,
            inspection_identity,
        }
    }
}

pub struct WorthQueryWorkflowCompletion {
    eligibility: WorthQueryPromotionEligibility,
    admitted_effect: WorthQueryAdmittedWorkflowEffect,
    lowered_plan: WorthQueryLoweredWorkflowPlan,
    aftermath: WorthQueryWorkflowAftermath,
    preview_outcome: WorthQueryPreviewOutcome,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryWorkflowCompletion {
    pub fn promotion_eligibility(&self) -> &WorthQueryPromotionEligibility {
        &self.eligibility
    }

    pub fn admitted_effect(&self) -> &WorthQueryAdmittedWorkflowEffect {
        &self.admitted_effect
    }

    pub fn lowered_plan(&self) -> &WorthQueryLoweredWorkflowPlan {
        &self.lowered_plan
    }

    pub fn aftermath(&self) -> &WorthQueryWorkflowAftermath {
        &self.aftermath
    }

    pub fn preview_outcome(&self) -> &WorthQueryPreviewOutcome {
        &self.preview_outcome
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub(crate) fn new(
        eligibility: WorthQueryPromotionEligibility,
        admitted_effect: WorthQueryAdmittedWorkflowEffect,
        lowered_plan: WorthQueryLoweredWorkflowPlan,
        aftermath: WorthQueryWorkflowAftermath,
        preview_outcome: WorthQueryPreviewOutcome,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            eligibility,
            admitted_effect,
            lowered_plan,
            aftermath,
            preview_outcome,
            counters,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStopSource {
    CrossSession,
    ForeignAuthority,
    StalePreview,
    UnsupportedWriteback,
    LowerRuntime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowNextAction {
    ProvideAuthority,
    RefreshPreview,
    UseMatchingSession,
    RebindAuthoritativeWriteback,
    InspectRuntimeDenial,
}

pub struct WorthQueryWorkflowStop {
    source: WorthQueryWorkflowStopSource,
    error: Option<WorthQueryRuntimeError>,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryWorkflowStop {
    pub fn source(&self) -> WorthQueryWorkflowStopSource {
        self.source
    }

    pub fn error(&self) -> Option<&WorthQueryRuntimeError> {
        self.error.as_ref()
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryWorkflowNextAction {
        match self.source {
            WorthQueryWorkflowStopSource::CrossSession => {
                WorthQueryWorkflowNextAction::UseMatchingSession
            }
            WorthQueryWorkflowStopSource::ForeignAuthority => {
                WorthQueryWorkflowNextAction::ProvideAuthority
            }
            WorthQueryWorkflowStopSource::StalePreview => {
                WorthQueryWorkflowNextAction::RefreshPreview
            }
            WorthQueryWorkflowStopSource::UnsupportedWriteback => {
                WorthQueryWorkflowNextAction::RebindAuthoritativeWriteback
            }
            WorthQueryWorkflowStopSource::LowerRuntime => {
                WorthQueryWorkflowNextAction::InspectRuntimeDenial
            }
        }
    }

    pub(crate) fn denied(
        source: WorthQueryWorkflowStopSource,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source,
            error: None,
            counters,
        }
    }

    pub(crate) fn runtime(
        error: WorthQueryRuntimeError,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source: WorthQueryWorkflowStopSource::LowerRuntime,
            error: Some(error),
            counters,
        }
    }
}

pub enum WorthQueryWorkflowOutcome {
    Completed(WorthQueryWorkflowCompletion),
    Stopped(WorthQueryWorkflowStop),
}

impl WorthQueryWorkflowOutcome {
    pub fn completed(&self) -> Option<&WorthQueryWorkflowCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryWorkflowStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }
}
