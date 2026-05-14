use crate::basis_lifecycle::{BasisAuthorityPosture, BasisFamily, BasisLifecyclePosture};
use crate::identity::hash_parts;
use crate::workflow::{
    MergeLoweringInput, MutationLoweringInput, WorkflowContextBinding, WorkflowDeclarationRequest,
    WritebackLoweringInput,
};

use super::authoring_basis::EffectAuthoringBasis;
use super::counters::EffectLifecycleCounters;
use super::taxonomy::{EffectAuthorityLane, EffectFamily, EffectIntentDenialKind};

#[derive(Clone, Debug, PartialEq)]
pub enum EffectOperationInput {
    Mutation(MutationLoweringInput),
    Merge(MergeLoweringInput),
    Writeback(WritebackLoweringInput),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedEffectIntent {
    family: EffectFamily,
    authority_lane: EffectAuthorityLane,
    basis_family: BasisFamily,
    basis_authority: BasisAuthorityPosture,
    basis_lifecycle: BasisLifecyclePosture,
    capability_digest: String,
    scoped_basis_digest: String,
    expected_lower_runtime_binding_digest: Option<String>,
    workflow_binding: WorkflowContextBinding,
    workflow_request: WorkflowDeclarationRequest,
    operation_input: EffectOperationInput,
    source_path: &'static str,
    normalized_digest: String,
    counters: EffectLifecycleCounters,
}

impl NormalizedEffectIntent {
    pub(crate) fn new(
        authoring_basis: &EffectAuthoringBasis,
        family: EffectFamily,
        authority_lane: EffectAuthorityLane,
        workflow_binding: WorkflowContextBinding,
        workflow_request: WorkflowDeclarationRequest,
        operation_input: EffectOperationInput,
        source_path: &'static str,
    ) -> Self {
        let capability_digest = authoring_basis.capability_digest();
        let normalized_digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("authority_lane:{}", authority_lane.as_str()),
            format!("basis_family:{}", authoring_basis.family().as_str()),
            format!("basis_authority:{}", authoring_basis.authority().as_str()),
            format!("basis_lifecycle:{}", authoring_basis.lifecycle().as_str()),
            format!("capability:{capability_digest}"),
            format!("scoped_basis:{}", authoring_basis.scoped_basis_digest()),
            format!(
                "lower_runtime_binding:{}",
                authoring_basis
                    .expected_lower_runtime_binding_digest()
                    .unwrap_or("none")
            ),
            format!("workflow_binding:{}", workflow_binding.digest()),
            format!(
                "workflow_family:{}",
                workflow_request.declaration_family().as_str()
            ),
            format!(
                "workflow_target:{}",
                workflow_request.authority_target_family().as_str()
            ),
            format!("source_path:{source_path}"),
        ]);

        Self {
            family,
            authority_lane,
            basis_family: authoring_basis.family(),
            basis_authority: authoring_basis.authority(),
            basis_lifecycle: authoring_basis.lifecycle(),
            capability_digest,
            scoped_basis_digest: authoring_basis.scoped_basis_digest().to_string(),
            expected_lower_runtime_binding_digest: authoring_basis
                .expected_lower_runtime_binding_digest()
                .map(str::to_string),
            workflow_binding,
            workflow_request,
            operation_input,
            source_path,
            normalized_digest,
            counters: EffectLifecycleCounters::normalized(1, 1, 1),
        }
    }

    pub fn family(&self) -> EffectFamily {
        self.family
    }

    pub fn authority_lane(&self) -> EffectAuthorityLane {
        self.authority_lane
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn basis_authority(&self) -> BasisAuthorityPosture {
        self.basis_authority
    }

    pub fn basis_lifecycle(&self) -> BasisLifecyclePosture {
        self.basis_lifecycle
    }

    pub fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub fn scoped_basis_digest(&self) -> &str {
        &self.scoped_basis_digest
    }

    pub fn expected_lower_runtime_binding_digest(&self) -> Option<&str> {
        self.expected_lower_runtime_binding_digest.as_deref()
    }

    pub fn workflow_binding(&self) -> &WorkflowContextBinding {
        &self.workflow_binding
    }

    pub fn workflow_request(&self) -> &WorkflowDeclarationRequest {
        &self.workflow_request
    }

    pub fn operation_input(&self) -> &EffectOperationInput {
        &self.operation_input
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn normalized_digest(&self) -> &str {
        &self.normalized_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub(crate) fn admitted_digest(&self) -> String {
        hash_parts(&[
            "admitted_effect_intent_v1".to_string(),
            format!("normalized:{}", self.normalized_digest()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectIntentDenial {
    denial_kind: EffectIntentDenialKind,
    message: &'static str,
    counters: EffectLifecycleCounters,
}

impl EffectIntentDenial {
    pub(crate) fn new(
        denial_kind: EffectIntentDenialKind,
        message: &'static str,
        source_path: &'static str,
        workflow_authority_target_check_count: usize,
        basis_scope_check_count: usize,
    ) -> Self {
        Self {
            denial_kind,
            message,
            counters: EffectLifecycleCounters::intent_denial(
                usize::from(!source_path.is_empty()),
                workflow_authority_target_check_count,
                basis_scope_check_count,
            ),
        }
    }

    pub fn denial_kind(&self) -> EffectIntentDenialKind {
        self.denial_kind
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}
