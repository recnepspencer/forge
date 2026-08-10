use crate::basis::ExecutionPreflightBundle;
use crate::correspondence_history::CorrespondenceHistoricalEnvelope;
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::preview::{
    AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationRequest,
    PromotionParityPreviewComparisonAdmission,
};
use crate::workflow::WorkflowCounters;
use worth_relational::facade::history::BranchId;
use worth_runtime_bridge::facade::BridgePreviewSessionIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowBasisFamily {
    RuntimePreflight,
    PreviewFoundation,
    PreviewPromotionComparison,
    CorrespondenceHistorical,
}

impl WorkflowBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimePreflight => "runtime_preflight",
            Self::PreviewFoundation => "preview_foundation",
            Self::PreviewPromotionComparison => "preview_promotion_comparison",
            Self::CorrespondenceHistorical => "correspondence_historical",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowPreviewEvaluationClass {
    ReadOnly,
    PromotionEligible,
}

impl WorkflowPreviewEvaluationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PromotionEligible => "promotion_eligible",
        }
    }
}

pub enum WorkflowBindingSource<'a> {
    RuntimePreflight(&'a ExecutionPreflightBundle),
    PreviewFoundation(&'a AdmittedPreviewWorkflowFoundation),
    PreviewPromotionComparison(&'a PromotionParityPreviewComparisonAdmission),
    CorrespondenceHistorical(&'a CorrespondenceHistoricalEnvelope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowContextBinding {
    pub(super) binding_identity: WorthQueryEvidenceIdentity,
    pub(super) source_identity: WorthQueryEvidenceIdentity,
    pub(super) query_identity: WorthQueryEvidenceIdentity,
    pub(super) basis_family: WorkflowBasisFamily,
    pub(super) basis_identity: WorthQueryEvidenceIdentity,
    pub(super) runtime_snapshot_identity: Option<WorthQuerySnapshotIdentity>,
    pub(super) runtime_target_branch: Option<BranchId>,
    pub(super) preview_evaluation_class: Option<WorkflowPreviewEvaluationClass>,
    pub(super) preview_request_family: Option<PreviewWorkflowFoundationRequest>,
    pub(super) preview_session_identity: Option<BridgePreviewSessionIdentity>,
    pub(super) counters: WorkflowCounters,
}

impl WorkflowContextBinding {
    pub fn binding_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn source_for_reporting(&self) -> &str {
        self.source_identity.as_str()
    }

    pub fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn query_for_reporting(&self) -> &str {
        self.query_identity.as_str()
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn runtime_snapshot_identity(&self) -> Option<&WorthQuerySnapshotIdentity> {
        self.runtime_snapshot_identity.as_ref()
    }

    pub fn runtime_target_branch(&self) -> Option<&BranchId> {
        self.runtime_target_branch.as_ref()
    }

    pub fn preview_evaluation_class(&self) -> Option<&WorkflowPreviewEvaluationClass> {
        self.preview_evaluation_class.as_ref()
    }

    pub fn preview_request_family(&self) -> Option<&PreviewWorkflowFoundationRequest> {
        self.preview_request_family.as_ref()
    }

    pub fn preview_session_identity(&self) -> Option<&BridgePreviewSessionIdentity> {
        self.preview_session_identity.as_ref()
    }

    pub fn counters(&self) -> &WorkflowCounters {
        &self.counters
    }
}
