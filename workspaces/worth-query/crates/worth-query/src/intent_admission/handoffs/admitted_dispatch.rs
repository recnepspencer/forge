use crate::intent_admission::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};
use crate::runtime::WorthQueryIntentDeclaration;

use super::{
    WorthQueryAdmittedIntentExecutionHandoff, WorthQueryAuthoritativeIntentExecutionHandoff,
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationExecutionHandoff, WorthQueryDerivedInspectionExecutionHandoff,
    WorthQueryDerivedMaterializationExecutionHandoff,
    WorthQueryEffectTriggeredIntentExecutionHandoff, WorthQueryExistingTruthProbeExecutionHandoff,
    WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff,
    WorthQueryUnifiedInspectionExecutionHandoff,
};

impl WorthQueryAdmittedIntentExecutionHandoff {
    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        match self {
            Self::Authoritative(handoff) => handoff.family(),
            Self::EffectTriggered(handoff) => handoff.family(),
            Self::AuthoritativeMutation(handoff) => handoff.family(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.family(),
            Self::ReadExecution(handoff) => handoff.family(),
            Self::LiveReadExecution(handoff) => handoff.family(),
            Self::DerivedMaterialization(handoff) => handoff.family(),
            Self::DerivedInspection(handoff) => handoff.family(),
            Self::UnifiedInspection(handoff) => handoff.family(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.family(),
        }
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        match self {
            Self::Authoritative(handoff) => handoff.entrypoint(),
            Self::EffectTriggered(handoff) => handoff.entrypoint(),
            Self::AuthoritativeMutation(handoff) => handoff.entrypoint(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.entrypoint(),
            Self::ReadExecution(handoff) => handoff.entrypoint(),
            Self::LiveReadExecution(handoff) => handoff.entrypoint(),
            Self::DerivedMaterialization(handoff) => handoff.entrypoint(),
            Self::DerivedInspection(handoff) => handoff.entrypoint(),
            Self::UnifiedInspection(handoff) => handoff.entrypoint(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.entrypoint(),
        }
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        match self {
            Self::Authoritative(handoff) => handoff.execution_seam(),
            Self::EffectTriggered(handoff) => handoff.execution_seam(),
            Self::AuthoritativeMutation(handoff) => handoff.execution_seam(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.execution_seam(),
            Self::ReadExecution(handoff) => handoff.execution_seam(),
            Self::LiveReadExecution(handoff) => handoff.execution_seam(),
            Self::DerivedMaterialization(handoff) => handoff.execution_seam(),
            Self::DerivedInspection(handoff) => handoff.execution_seam(),
            Self::UnifiedInspection(handoff) => handoff.execution_seam(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.execution_seam(),
        }
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        match self {
            Self::Authoritative(handoff) => handoff.declaration(),
            Self::EffectTriggered(handoff) => handoff.declaration(),
            Self::AuthoritativeMutation(_)
            | Self::AuthoritativeMutationBatch(_)
            | Self::ReadExecution(_)
            | Self::LiveReadExecution(_)
            | Self::DerivedMaterialization(_)
            | Self::DerivedInspection(_)
            | Self::UnifiedInspection(_)
            | Self::ExistingTruthProbeRouting(_) => {
                panic!("mutation handoffs do not preserve intent declarations")
            }
        }
    }

    pub fn request_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.request_digest(),
            Self::EffectTriggered(handoff) => handoff.request_digest(),
            Self::AuthoritativeMutation(handoff) => handoff.request_digest(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.request_digest(),
            Self::ReadExecution(handoff) => handoff.request_digest(),
            Self::LiveReadExecution(handoff) => handoff.request_digest(),
            Self::DerivedMaterialization(handoff) => handoff.request_digest(),
            Self::DerivedInspection(handoff) => handoff.request_digest(),
            Self::UnifiedInspection(handoff) => handoff.request_digest(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.request_digest(),
        }
    }

    pub fn eligibility_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.eligibility_digest(),
            Self::EffectTriggered(handoff) => handoff.eligibility_digest(),
            Self::AuthoritativeMutation(handoff) => handoff.eligibility_digest(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.eligibility_digest(),
            Self::ReadExecution(handoff) => handoff.eligibility_digest(),
            Self::LiveReadExecution(handoff) => handoff.eligibility_digest(),
            Self::DerivedMaterialization(handoff) => handoff.eligibility_digest(),
            Self::DerivedInspection(handoff) => handoff.eligibility_digest(),
            Self::UnifiedInspection(handoff) => handoff.eligibility_digest(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.eligibility_digest(),
        }
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        match self {
            Self::Authoritative(handoff) => handoff.eligibility_trace(),
            Self::EffectTriggered(handoff) => handoff.eligibility_trace(),
            Self::AuthoritativeMutation(handoff) => handoff.eligibility_trace(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.eligibility_trace(),
            Self::ReadExecution(handoff) => handoff.eligibility_trace(),
            Self::LiveReadExecution(handoff) => handoff.eligibility_trace(),
            Self::DerivedMaterialization(handoff) => handoff.eligibility_trace(),
            Self::DerivedInspection(handoff) => handoff.eligibility_trace(),
            Self::UnifiedInspection(handoff) => handoff.eligibility_trace(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.eligibility_trace(),
        }
    }

    pub fn decision_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.decision_digest(),
            Self::EffectTriggered(handoff) => handoff.decision_digest(),
            Self::AuthoritativeMutation(handoff) => handoff.decision_digest(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.decision_digest(),
            Self::ReadExecution(handoff) => handoff.decision_digest(),
            Self::LiveReadExecution(handoff) => handoff.decision_digest(),
            Self::DerivedMaterialization(handoff) => handoff.decision_digest(),
            Self::DerivedInspection(handoff) => handoff.decision_digest(),
            Self::UnifiedInspection(handoff) => handoff.decision_digest(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.decision_digest(),
        }
    }

    pub fn handoff_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.handoff_digest(),
            Self::EffectTriggered(handoff) => handoff.handoff_digest(),
            Self::AuthoritativeMutation(handoff) => handoff.handoff_digest(),
            Self::AuthoritativeMutationBatch(handoff) => handoff.handoff_digest(),
            Self::ReadExecution(handoff) => handoff.handoff_digest(),
            Self::LiveReadExecution(handoff) => handoff.handoff_digest(),
            Self::DerivedMaterialization(handoff) => handoff.handoff_digest(),
            Self::DerivedInspection(handoff) => handoff.handoff_digest(),
            Self::UnifiedInspection(handoff) => handoff.handoff_digest(),
            Self::ExistingTruthProbeRouting(handoff) => handoff.handoff_digest(),
        }
    }
}

impl From<WorthQueryAuthoritativeIntentExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryAuthoritativeIntentExecutionHandoff) -> Self {
        Self::Authoritative(handoff)
    }
}

impl From<WorthQueryEffectTriggeredIntentExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryEffectTriggeredIntentExecutionHandoff) -> Self {
        Self::EffectTriggered(handoff)
    }
}

impl From<WorthQueryAuthoritativeMutationExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryAuthoritativeMutationExecutionHandoff) -> Self {
        Self::AuthoritativeMutation(handoff)
    }
}

impl From<WorthQueryAuthoritativeMutationBatchExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryAuthoritativeMutationBatchExecutionHandoff) -> Self {
        Self::AuthoritativeMutationBatch(handoff)
    }
}

impl From<WorthQueryReadExecutionHandoff> for WorthQueryAdmittedIntentExecutionHandoff {
    fn from(handoff: WorthQueryReadExecutionHandoff) -> Self {
        Self::ReadExecution(handoff)
    }
}

impl From<WorthQueryLiveReadExecutionHandoff> for WorthQueryAdmittedIntentExecutionHandoff {
    fn from(handoff: WorthQueryLiveReadExecutionHandoff) -> Self {
        Self::LiveReadExecution(handoff)
    }
}

impl From<WorthQueryDerivedMaterializationExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryDerivedMaterializationExecutionHandoff) -> Self {
        Self::DerivedMaterialization(handoff)
    }
}

impl From<WorthQueryDerivedInspectionExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryDerivedInspectionExecutionHandoff) -> Self {
        Self::DerivedInspection(handoff)
    }
}

impl From<WorthQueryUnifiedInspectionExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryUnifiedInspectionExecutionHandoff) -> Self {
        Self::UnifiedInspection(handoff)
    }
}

impl From<WorthQueryExistingTruthProbeExecutionHandoff>
    for WorthQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: WorthQueryExistingTruthProbeExecutionHandoff) -> Self {
        Self::ExistingTruthProbeRouting(handoff)
    }
}
