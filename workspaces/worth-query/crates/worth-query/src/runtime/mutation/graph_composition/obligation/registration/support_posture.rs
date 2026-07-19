use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use crate::runtime::WorthQueryGraphObligationExecutionBudget;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationSupportLane {
    AssemblyIndexSelection,
    GraphComposition,
    AuthoritativeCommandBatch,
    ScalarMutation,
    EffectTriggeredWriteIntent,
    DeclarationEntry,
    ContributionOrchestration,
    ReadFamily,
    LiveRead,
    PreviewMutation,
    PreviewIntent,
    BranchIntent,
    PolicyAwareGraphMutation,
    PrimitiveConstructionBirth,
    WorthTopoOperatorCatalog,
    WorthKernelPhaseChain,
}

impl WorthQueryGraphObligationSupportLane {
    pub const ALL: [Self; 16] = [
        Self::AssemblyIndexSelection,
        Self::GraphComposition,
        Self::AuthoritativeCommandBatch,
        Self::ScalarMutation,
        Self::EffectTriggeredWriteIntent,
        Self::DeclarationEntry,
        Self::ContributionOrchestration,
        Self::ReadFamily,
        Self::LiveRead,
        Self::PreviewMutation,
        Self::PreviewIntent,
        Self::BranchIntent,
        Self::PolicyAwareGraphMutation,
        Self::PrimitiveConstructionBirth,
        Self::WorthTopoOperatorCatalog,
        Self::WorthKernelPhaseChain,
    ];

    pub const MILESTONE_9_9_COVERED: [Self; 15] = [
        Self::GraphComposition,
        Self::AuthoritativeCommandBatch,
        Self::ScalarMutation,
        Self::EffectTriggeredWriteIntent,
        Self::DeclarationEntry,
        Self::ContributionOrchestration,
        Self::ReadFamily,
        Self::LiveRead,
        Self::PreviewMutation,
        Self::PreviewIntent,
        Self::BranchIntent,
        Self::PolicyAwareGraphMutation,
        Self::PrimitiveConstructionBirth,
        Self::WorthTopoOperatorCatalog,
        Self::WorthKernelPhaseChain,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::AssemblyIndexSelection => "assembly-index-selection",
            Self::GraphComposition => "graph-composition",
            Self::AuthoritativeCommandBatch => "authoritative-command-batch",
            Self::ScalarMutation => "scalar-mutation",
            Self::EffectTriggeredWriteIntent => "effect-triggered-write-intent",
            Self::DeclarationEntry => "declaration-entry",
            Self::ContributionOrchestration => "contribution-orchestration",
            Self::ReadFamily => "read-family",
            Self::LiveRead => "live-read",
            Self::PreviewMutation => "preview-mutation",
            Self::PreviewIntent => "preview-intent",
            Self::BranchIntent => "branch-intent",
            Self::PolicyAwareGraphMutation => "policy-aware-graph-mutation",
            Self::PrimitiveConstructionBirth => "primitive-construction-birth",
            Self::WorthTopoOperatorCatalog => "worth-topo-operator-catalog",
            Self::WorthKernelPhaseChain => "worth-kernel-phase-chain",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationSupportStatus {
    Supported,
    Unsupported,
    NotApplicable,
    DiagnosticOnly,
    DeferredToBackstop,
}

impl WorthQueryGraphObligationSupportStatus {
    pub const ALL: [Self; 5] = [
        Self::Supported,
        Self::Unsupported,
        Self::NotApplicable,
        Self::DiagnosticOnly,
        Self::DeferredToBackstop,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not-applicable",
            Self::DiagnosticOnly => "diagnostic-only",
            Self::DeferredToBackstop => "deferred-to-backstop",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSupportPosture {
    lane: WorthQueryGraphObligationSupportLane,
    status: WorthQueryGraphObligationSupportStatus,
    execution_budget: WorthQueryGraphObligationExecutionBudget,
    posture_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationSupportPosture {
    pub fn supported(lane: WorthQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, WorthQueryGraphObligationSupportStatus::Supported)
    }

    pub fn unsupported(lane: WorthQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, WorthQueryGraphObligationSupportStatus::Unsupported)
    }

    pub fn not_applicable(lane: WorthQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, WorthQueryGraphObligationSupportStatus::NotApplicable)
    }

    pub fn diagnostic_only(lane: WorthQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, WorthQueryGraphObligationSupportStatus::DiagnosticOnly)
    }

    pub fn deferred_to_backstop(lane: WorthQueryGraphObligationSupportLane) -> Self {
        Self::new(
            lane,
            WorthQueryGraphObligationSupportStatus::DeferredToBackstop,
        )
    }

    pub fn with_execution_budget(
        mut self,
        execution_budget: WorthQueryGraphObligationExecutionBudget,
    ) -> Self {
        self.execution_budget = execution_budget;
        self.posture_digest = self.build_digest();
        self
    }

    pub fn lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.lane
    }

    pub fn lane_label(&self) -> &'static str {
        self.lane.as_str()
    }

    pub fn status(&self) -> WorthQueryGraphObligationSupportStatus {
        self.status
    }

    pub fn execution_budget(&self) -> &WorthQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn posture_digest(&self) -> &str {
        self.posture_digest.as_str()
    }

    pub(crate) fn posture_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.posture_digest
    }

    pub(crate) fn default_selection_posture() -> Self {
        Self::deferred_to_backstop(WorthQueryGraphObligationSupportLane::AssemblyIndexSelection)
    }

    fn new(
        lane: WorthQueryGraphObligationSupportLane,
        status: WorthQueryGraphObligationSupportStatus,
    ) -> Self {
        let mut posture = Self {
            lane,
            status,
            execution_budget:
                WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
            posture_digest: worth_query_evidence_identity(
                WorthQueryEvidenceScope::GraphObligationSupportPosture,
            )
            .seal(),
        };
        posture.posture_digest = posture.build_digest();
        posture
    }

    fn build_digest(&self) -> WorthQueryEvidenceIdentity {
        worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationSupportPosture)
            .field_shape(WorthQueryEvidenceTag::new("lane"), self.lane.as_str())
            .field_shape(WorthQueryEvidenceTag::new("status"), self.status.as_str())
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("execution_budget"),
                self.execution_budget.budget_evidence_digest(),
            )
            .seal()
    }
}
