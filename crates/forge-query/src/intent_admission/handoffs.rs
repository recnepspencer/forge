use crate::identity::hash_parts;
use crate::runtime::{
    admit_authoritative_intent_execution, ForgeQueryIntentAdmissionDenial,
    ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
};

use super::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryAuthoritativeIntentExecutionPlan,
    ForgeQueryEffectTriggeredIntentExecutionPlan, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentViolationDecision,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeIntentExecutionHandoff {
    declaration: ForgeQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectTriggeredIntentExecutionHandoff {
    declaration: ForgeQueryIntentDeclaration,
    request_digest: String,
    eligibility_digest: String,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmittedIntentExecutionHandoff {
    Authoritative(ForgeQueryAuthoritativeIntentExecutionHandoff),
    EffectTriggered(ForgeQueryEffectTriggeredIntentExecutionHandoff),
}

impl ForgeQueryAuthoritativeIntentExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryAuthoritativeIntentExecutionPlan) -> Self {
        Self {
            declaration: plan.declaration().clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam(),
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
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam(),
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
        }
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        match self {
            Self::Authoritative(handoff) => handoff.entrypoint(),
            Self::EffectTriggered(handoff) => handoff.entrypoint(),
        }
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        match self {
            Self::Authoritative(handoff) => handoff.execution_seam(),
            Self::EffectTriggered(handoff) => handoff.execution_seam(),
        }
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        match self {
            Self::Authoritative(handoff) => handoff.declaration(),
            Self::EffectTriggered(handoff) => handoff.declaration(),
        }
    }

    pub fn request_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.request_digest(),
            Self::EffectTriggered(handoff) => handoff.request_digest(),
        }
    }

    pub fn eligibility_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.eligibility_digest(),
            Self::EffectTriggered(handoff) => handoff.eligibility_digest(),
        }
    }

    pub fn decision_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.decision_digest(),
            Self::EffectTriggered(handoff) => handoff.decision_digest(),
        }
    }

    pub fn handoff_digest(&self) -> &str {
        match self {
            Self::Authoritative(handoff) => handoff.handoff_digest(),
            Self::EffectTriggered(handoff) => handoff.handoff_digest(),
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

impl ForgeQueryAdmittedIntentPlan {
    pub fn into_execution_handoff(self) -> ForgeQueryAdmittedIntentExecutionHandoff {
        match self {
            ForgeQueryAdmittedIntentPlan::Authoritative(plan) => {
                ForgeQueryAdmittedIntentExecutionHandoff::Authoritative(
                    ForgeQueryAuthoritativeIntentExecutionHandoff::from_plan(plan),
                )
            }
            ForgeQueryAdmittedIntentPlan::EffectTriggered(plan) => {
                ForgeQueryAdmittedIntentExecutionHandoff::EffectTriggered(
                    ForgeQueryEffectTriggeredIntentExecutionHandoff::from_plan(plan),
                )
            }
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
