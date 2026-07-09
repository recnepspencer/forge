mod bindings;
mod inspection;
mod mutation;
mod read;
mod routing;
mod unified_inspection;

pub(crate) const INTENT_ADMISSION_HANDOFFS_MODULE_ROOT: &str = "intent_admission/handoffs/mod.rs";
pub(crate) const INTENT_ADMISSION_HANDOFFS_CHILD_MODULES: &[&str] = &[
    "bindings",
    "inspection",
    "mutation",
    "read",
    "routing",
    "unified_inspection",
];
pub(crate) const INTENT_ADMISSION_HANDOFFS_EXPORTED_SURFACE: &[&str] = &[
    "WorthQueryAuthoritativeIntentExecutionBinding",
    "WorthQueryAuthoritativeMutationBatchExecutionBinding",
    "WorthQueryAuthoritativeMutationExecutionBinding",
    "WorthQueryDerivedInspectionExecutionBinding",
    "WorthQueryDerivedMaterializationExecutionBinding",
    "WorthQueryEffectTriggeredIntentExecutionBinding",
    "WorthQueryExistingTruthProbeExecutionBinding",
    "WorthQueryLiveReadExecutionBinding",
    "WorthQueryReadExecutionBinding",
    "WorthQueryUnifiedInspectionExecutionBinding",
    "WorthQueryDerivedInspectionExecutionHandoff",
    "WorthQueryDerivedMaterializationExecutionHandoff",
    "WorthQueryAuthoritativeMutationBatchExecutionHandoff",
    "WorthQueryAuthoritativeMutationExecutionHandoff",
    "WorthQueryLiveReadExecutionHandoff",
    "WorthQueryReadExecutionHandoff",
    "WorthQueryExistingTruthProbeExecutionHandoff",
    "WorthQueryUnifiedInspectionExecutionHandoff",
    "WorthQueryAuthoritativeIntentExecutionHandoff",
    "WorthQueryEffectTriggeredIntentExecutionHandoff",
    "WorthQueryAdmittedIntentExecutionHandoff",
];

use crate::identity::hash_parts;
use crate::runtime::{
    admit_authoritative_intent_execution, WorthQueryIntentAdmissionDenial,
    WorthQueryIntentDeclaration, WorthQueryIntentExecution,
};

use super::{
    WorthQueryAdmittedIntentPlan, WorthQueryAuthoritativeIntentExecutionPlan,
    WorthQueryEffectTriggeredIntentExecutionPlan, WorthQueryIntentAdmissionCoveredEntrypoint,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentEligibilityTraceEvidence, WorthQueryIntentViolationDecision,
};
pub use bindings::{
    WorthQueryAuthoritativeIntentExecutionBinding,
    WorthQueryAuthoritativeMutationBatchExecutionBinding,
    WorthQueryAuthoritativeMutationExecutionBinding, WorthQueryDerivedInspectionExecutionBinding,
    WorthQueryDerivedMaterializationExecutionBinding,
    WorthQueryEffectTriggeredIntentExecutionBinding, WorthQueryExistingTruthProbeExecutionBinding,
    WorthQueryLiveReadExecutionBinding, WorthQueryReadExecutionBinding,
    WorthQueryUnifiedInspectionExecutionBinding,
};
pub use inspection::{
    WorthQueryDerivedInspectionExecutionHandoff, WorthQueryDerivedMaterializationExecutionHandoff,
};
pub use mutation::{
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationExecutionHandoff,
};
pub use read::{WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff};
pub use routing::WorthQueryExistingTruthProbeExecutionHandoff;
pub use unified_inspection::WorthQueryUnifiedInspectionExecutionHandoff;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeIntentExecutionHandoff {
    declaration: WorthQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryEffectTriggeredIntentExecutionHandoff {
    declaration: WorthQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryAdmittedIntentExecutionHandoff {
    Authoritative(WorthQueryAuthoritativeIntentExecutionHandoff),
    EffectTriggered(WorthQueryEffectTriggeredIntentExecutionHandoff),
    AuthoritativeMutation(WorthQueryAuthoritativeMutationExecutionHandoff),
    AuthoritativeMutationBatch(WorthQueryAuthoritativeMutationBatchExecutionHandoff),
    ReadExecution(WorthQueryReadExecutionHandoff),
    LiveReadExecution(WorthQueryLiveReadExecutionHandoff),
    DerivedMaterialization(WorthQueryDerivedMaterializationExecutionHandoff),
    DerivedInspection(WorthQueryDerivedInspectionExecutionHandoff),
    UnifiedInspection(WorthQueryUnifiedInspectionExecutionHandoff),
    ExistingTruthProbeRouting(WorthQueryExistingTruthProbeExecutionHandoff),
}

impl WorthQueryAuthoritativeIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryAuthoritativeIntentExecutionPlan) -> Self {
        Self {
            declaration: plan.declaration().clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("authoritative runtime handoff requires execution seam"),
                plan.decision_digest(),
                plan.declaration(),
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        &self.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

impl WorthQueryEffectTriggeredIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryEffectTriggeredIntentExecutionPlan) -> Self {
        Self {
            declaration: plan.declaration().clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("effect runtime handoff requires execution seam"),
                plan.decision_digest(),
                plan.declaration(),
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::EffectTriggeredWriteIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        &self.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

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

impl WorthQueryAdmittedIntentPlan {
    pub fn into_execution_handoff(self) -> Option<WorthQueryAdmittedIntentExecutionHandoff> {
        match self {
            WorthQueryAdmittedIntentPlan::Authoritative(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::Authoritative(
                    WorthQueryAuthoritativeIntentExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::EffectTriggered(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::EffectTriggered(
                    WorthQueryEffectTriggeredIntentExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::AuthoritativeMutation(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::AuthoritativeMutation(
                    WorthQueryAuthoritativeMutationExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::AuthoritativeMutationBatch(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::AuthoritativeMutationBatch(
                    WorthQueryAuthoritativeMutationBatchExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::ReadExecution(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::ReadExecution(
                    WorthQueryReadExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::LiveReadExecution(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::LiveReadExecution(
                    WorthQueryLiveReadExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::DerivedMaterialization(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::DerivedMaterialization(
                    WorthQueryDerivedMaterializationExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::DerivedInspection(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::DerivedInspection(
                    WorthQueryDerivedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::UnifiedInspection(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::UnifiedInspection(
                    WorthQueryUnifiedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::ExistingTruthProbeRouting(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::ExistingTruthProbeRouting(
                    WorthQueryExistingTruthProbeExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::BasisObservation(_)
            | WorthQueryAdmittedIntentPlan::ProjectionConsumption(_) => None,
        }
    }
}

pub(crate) fn admit_authoritative_execution(
    handoff: &WorthQueryAuthoritativeIntentExecutionHandoff,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_execution_with_proof(
        handoff.family(),
        handoff.entrypoint(),
        handoff.request_digest(),
        handoff.eligibility_digest(),
        handoff.declaration(),
        execution,
    )
}

pub(crate) fn admit_effect_execution(
    handoff: &WorthQueryEffectTriggeredIntentExecutionHandoff,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_execution_with_proof(
        handoff.family(),
        handoff.entrypoint(),
        handoff.request_digest(),
        handoff.eligibility_digest(),
        handoff.declaration(),
        execution,
    )
}

fn admit_execution_with_proof(
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    request_digest: &str,
    eligibility_digest: &str,
    declaration: &WorthQueryIntentDeclaration,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_authoritative_intent_execution(declaration, execution).map_err(
        |denial: WorthQueryIntentAdmissionDenial| {
            WorthQueryIntentViolationDecision::new(
                family,
                entrypoint,
                denial.stage(),
                denial.message(),
                request_digest,
                eligibility_digest,
            )
        },
    )
}

fn handoff_digest(
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    decision_digest: &str,
    declaration: &WorthQueryIntentDeclaration,
) -> String {
    hash_parts(&[
        "worth_query_admitted_intent_execution_handoff_v1".to_string(),
        format!("family:{}", family.as_str()),
        format!("entrypoint:{}", entrypoint.as_str()),
        format!("execution-seam:{}", execution_seam.as_str()),
        format!("decision:{decision_digest}"),
        format!("intent:{}", declaration.input_digest()),
    ])
}
