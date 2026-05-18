mod inspection;
mod mutation;
mod read;
mod routing;
mod unified_inspection;

use crate::identity::hash_parts;
use crate::runtime::{
    admit_authoritative_intent_execution, ForgeQueryIntentAdmissionDenial,
    ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
};

use super::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryAuthoritativeIntentExecutionPlan,
    ForgeQueryEffectTriggeredIntentExecutionPlan, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentEligibilityTraceEvidence, ForgeQueryIntentViolationDecision,
};
pub use inspection::{
    ForgeQueryDerivedInspectionExecutionHandoff, ForgeQueryDerivedMaterializationExecutionHandoff,
};
pub use mutation::{
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ForgeQueryAuthoritativeMutationExecutionHandoff,
};
pub use read::{ForgeQueryLiveReadExecutionHandoff, ForgeQueryReadExecutionHandoff};
pub use routing::ForgeQueryExistingTruthProbeExecutionHandoff;
pub use unified_inspection::ForgeQueryUnifiedInspectionExecutionHandoff;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeIntentExecutionHandoff {
    declaration: ForgeQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryEffectTriggeredIntentExecutionHandoff {
    declaration: ForgeQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryAdmittedIntentExecutionHandoff {
    Authoritative(ForgeQueryAuthoritativeIntentExecutionHandoff),
    EffectTriggered(ForgeQueryEffectTriggeredIntentExecutionHandoff),
    AuthoritativeMutation(ForgeQueryAuthoritativeMutationExecutionHandoff),
    AuthoritativeMutationBatch(ForgeQueryAuthoritativeMutationBatchExecutionHandoff),
    ReadExecution(ForgeQueryReadExecutionHandoff),
    LiveReadExecution(ForgeQueryLiveReadExecutionHandoff),
    DerivedMaterialization(ForgeQueryDerivedMaterializationExecutionHandoff),
    DerivedInspection(ForgeQueryDerivedInspectionExecutionHandoff),
    UnifiedInspection(ForgeQueryUnifiedInspectionExecutionHandoff),
    ExistingTruthProbeRouting(ForgeQueryExistingTruthProbeExecutionHandoff),
}

impl ForgeQueryAuthoritativeIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryAuthoritativeIntentExecutionPlan) -> Self {
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

impl ForgeQueryEffectTriggeredIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryEffectTriggeredIntentExecutionPlan) -> Self {
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        &self.declaration
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

impl ForgeQueryAdmittedIntentExecutionHandoff {
    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
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

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
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

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
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

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
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

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
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

impl From<ForgeQueryAuthoritativeIntentExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryAuthoritativeIntentExecutionHandoff) -> Self {
        Self::Authoritative(handoff)
    }
}

impl From<ForgeQueryEffectTriggeredIntentExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryEffectTriggeredIntentExecutionHandoff) -> Self {
        Self::EffectTriggered(handoff)
    }
}

impl From<ForgeQueryAuthoritativeMutationExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryAuthoritativeMutationExecutionHandoff) -> Self {
        Self::AuthoritativeMutation(handoff)
    }
}

impl From<ForgeQueryAuthoritativeMutationBatchExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff) -> Self {
        Self::AuthoritativeMutationBatch(handoff)
    }
}

impl From<ForgeQueryReadExecutionHandoff> for ForgeQueryAdmittedIntentExecutionHandoff {
    fn from(handoff: ForgeQueryReadExecutionHandoff) -> Self {
        Self::ReadExecution(handoff)
    }
}

impl From<ForgeQueryLiveReadExecutionHandoff> for ForgeQueryAdmittedIntentExecutionHandoff {
    fn from(handoff: ForgeQueryLiveReadExecutionHandoff) -> Self {
        Self::LiveReadExecution(handoff)
    }
}

impl From<ForgeQueryDerivedMaterializationExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryDerivedMaterializationExecutionHandoff) -> Self {
        Self::DerivedMaterialization(handoff)
    }
}

impl From<ForgeQueryDerivedInspectionExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryDerivedInspectionExecutionHandoff) -> Self {
        Self::DerivedInspection(handoff)
    }
}

impl From<ForgeQueryUnifiedInspectionExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryUnifiedInspectionExecutionHandoff) -> Self {
        Self::UnifiedInspection(handoff)
    }
}

impl From<ForgeQueryExistingTruthProbeExecutionHandoff>
    for ForgeQueryAdmittedIntentExecutionHandoff
{
    fn from(handoff: ForgeQueryExistingTruthProbeExecutionHandoff) -> Self {
        Self::ExistingTruthProbeRouting(handoff)
    }
}

impl ForgeQueryAdmittedIntentPlan {
    pub fn into_execution_handoff(self) -> Option<ForgeQueryAdmittedIntentExecutionHandoff> {
        match self {
            ForgeQueryAdmittedIntentPlan::Authoritative(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::Authoritative(
                    ForgeQueryAuthoritativeIntentExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::EffectTriggered(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::EffectTriggered(
                    ForgeQueryEffectTriggeredIntentExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::AuthoritativeMutation(plan) => Some(
                ForgeQueryAdmittedIntentExecutionHandoff::AuthoritativeMutation(
                    ForgeQueryAuthoritativeMutationExecutionHandoff::from_plan(plan),
                ),
            ),
            ForgeQueryAdmittedIntentPlan::AuthoritativeMutationBatch(plan) => Some(
                ForgeQueryAdmittedIntentExecutionHandoff::AuthoritativeMutationBatch(
                    ForgeQueryAuthoritativeMutationBatchExecutionHandoff::from_plan(plan),
                ),
            ),
            ForgeQueryAdmittedIntentPlan::ReadExecution(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::ReadExecution(
                    ForgeQueryReadExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::LiveReadExecution(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::LiveReadExecution(
                    ForgeQueryLiveReadExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::DerivedMaterialization(plan) => Some(
                ForgeQueryAdmittedIntentExecutionHandoff::DerivedMaterialization(
                    ForgeQueryDerivedMaterializationExecutionHandoff::from_plan(plan),
                ),
            ),
            ForgeQueryAdmittedIntentPlan::DerivedInspection(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::DerivedInspection(
                    ForgeQueryDerivedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::UnifiedInspection(plan) => {
                Some(ForgeQueryAdmittedIntentExecutionHandoff::UnifiedInspection(
                    ForgeQueryUnifiedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            ForgeQueryAdmittedIntentPlan::ExistingTruthProbeRouting(plan) => Some(
                ForgeQueryAdmittedIntentExecutionHandoff::ExistingTruthProbeRouting(
                    ForgeQueryExistingTruthProbeExecutionHandoff::from_plan(plan),
                ),
            ),
            ForgeQueryAdmittedIntentPlan::BasisObservation(_)
            | ForgeQueryAdmittedIntentPlan::ProjectionConsumption(_) => None,
        }
    }
}

pub(crate) fn admit_authoritative_execution(
    handoff: &ForgeQueryAuthoritativeIntentExecutionHandoff,
    execution: &ForgeQueryIntentExecution,
) -> Result<(), ForgeQueryIntentViolationDecision> {
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
    handoff: &ForgeQueryEffectTriggeredIntentExecutionHandoff,
    execution: &ForgeQueryIntentExecution,
) -> Result<(), ForgeQueryIntentViolationDecision> {
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
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    request_digest: &str,
    eligibility_digest: &str,
    declaration: &ForgeQueryIntentDeclaration,
    execution: &ForgeQueryIntentExecution,
) -> Result<(), ForgeQueryIntentViolationDecision> {
    admit_authoritative_intent_execution(declaration, execution).map_err(
        |denial: ForgeQueryIntentAdmissionDenial| {
            ForgeQueryIntentViolationDecision::new(
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
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    decision_digest: &str,
    declaration: &ForgeQueryIntentDeclaration,
) -> String {
    hash_parts(&[
        "forge_query_admitted_intent_execution_handoff_v1".to_string(),
        format!("family:{}", family.as_str()),
        format!("entrypoint:{}", entrypoint.as_str()),
        format!("execution-seam:{}", execution_seam.as_str()),
        format!("decision:{decision_digest}"),
        format!("intent:{}", declaration.input_digest()),
    ])
}
