use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use crate::runtime::ForgeQueryGraphObligationExecutionBudget;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationSupportLane {
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

impl ForgeQueryGraphObligationSupportLane {
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
pub enum ForgeQueryGraphObligationSupportStatus {
    Supported,
    Unsupported,
    NotApplicable,
    DiagnosticOnly,
    DeferredToBackstop,
}

impl ForgeQueryGraphObligationSupportStatus {
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
pub struct ForgeQueryGraphObligationSupportPosture {
    lane: ForgeQueryGraphObligationSupportLane,
    status: ForgeQueryGraphObligationSupportStatus,
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
    posture_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationSupportPosture {
    pub fn supported(lane: ForgeQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, ForgeQueryGraphObligationSupportStatus::Supported)
    }

    pub fn unsupported(lane: ForgeQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, ForgeQueryGraphObligationSupportStatus::Unsupported)
    }

    pub fn not_applicable(lane: ForgeQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, ForgeQueryGraphObligationSupportStatus::NotApplicable)
    }

    pub fn diagnostic_only(lane: ForgeQueryGraphObligationSupportLane) -> Self {
        Self::new(lane, ForgeQueryGraphObligationSupportStatus::DiagnosticOnly)
    }

    pub fn deferred_to_backstop(lane: ForgeQueryGraphObligationSupportLane) -> Self {
        Self::new(
            lane,
            ForgeQueryGraphObligationSupportStatus::DeferredToBackstop,
        )
    }

    pub fn with_execution_budget(
        mut self,
        execution_budget: ForgeQueryGraphObligationExecutionBudget,
    ) -> Self {
        self.execution_budget = execution_budget;
        self.posture_digest = self.build_digest();
        self
    }

    pub fn lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.lane
    }

    pub fn lane_label(&self) -> &'static str {
        self.lane.as_str()
    }

    pub fn status(&self) -> ForgeQueryGraphObligationSupportStatus {
        self.status
    }

    pub fn execution_budget(&self) -> &ForgeQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn posture_digest(&self) -> &str {
        self.posture_digest.as_str()
    }

    pub(crate) fn posture_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.posture_digest
    }

    pub(crate) fn default_selection_posture() -> Self {
        Self::deferred_to_backstop(ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection)
    }

    fn new(
        lane: ForgeQueryGraphObligationSupportLane,
        status: ForgeQueryGraphObligationSupportStatus,
    ) -> Self {
        let mut posture = Self {
            lane,
            status,
            execution_budget:
                ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
            posture_digest: forge_query_evidence_identity(
                ForgeQueryEvidenceScope::GraphObligationSupportPosture,
            )
            .seal(),
        };
        posture.posture_digest = posture.build_digest();
        posture
    }

    fn build_digest(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationSupportPosture)
            .field_shape(ForgeQueryEvidenceTag::new("lane"), self.lane.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("status"), self.status.as_str())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("execution_budget"),
                self.execution_budget.budget_evidence_digest(),
            )
            .seal()
    }
}
