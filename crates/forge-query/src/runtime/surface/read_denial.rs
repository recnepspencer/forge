use super::{
    read_composition::ForgeQueryReadScopeClass, ForgeQueryReadBuiltInOperator,
    ForgeQueryReadBuiltInOperatorDenial, ForgeQueryReadBuiltInOperatorDenialReason,
    ForgeQueryReadRelationshipProofDenial,
};
use crate::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessExecutionCounters,
    ForgeQueryGraphReadPersistentArtifactAudit,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryReadDenialKind {
    InvalidRoot,
    BuiltInOperatorDenied,
    RelationshipProofAdmissionDenied,
    ScopeShapeDenied,
    AuthoringDenied,
    CanonicalizationDenied,
    ValidationDenied,
    PlanningDenied,
    BasisResolutionDenied,
    BasisPreflightDenied,
    ExecutionDenied,
}

impl ForgeQueryReadDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidRoot => "invalid_root",
            Self::BuiltInOperatorDenied => "built_in_operator_denied",
            Self::RelationshipProofAdmissionDenied => "relationship_proof_admission_denied",
            Self::ScopeShapeDenied => "scope_shape_denied",
            Self::AuthoringDenied => "authoring_denied",
            Self::CanonicalizationDenied => "canonicalization_denied",
            Self::ValidationDenied => "validation_denied",
            Self::PlanningDenied => "planning_denied",
            Self::BasisResolutionDenied => "basis_resolution_denied",
            Self::BasisPreflightDenied => "basis_preflight_denied",
            Self::ExecutionDenied => "execution_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadScopeShapeMismatch {
    expected: ForgeQueryReadScopeClass,
    actual: ForgeQueryReadScopeClass,
}

impl ForgeQueryReadScopeShapeMismatch {
    pub fn expected(&self) -> &ForgeQueryReadScopeClass {
        &self.expected
    }

    pub fn actual(&self) -> &ForgeQueryReadScopeClass {
        &self.actual
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadAccessPlanBindingMismatch {
    admitted_read_graph_digest: String,
    execution_read_graph_digest: String,
    provided_plan_digest: String,
    provided_admission_digest: String,
}

impl ForgeQueryReadAccessPlanBindingMismatch {
    pub fn admitted_read_graph_digest(&self) -> &str {
        &self.admitted_read_graph_digest
    }

    pub fn execution_read_graph_digest(&self) -> &str {
        &self.execution_read_graph_digest
    }

    pub fn provided_plan_digest(&self) -> &str {
        &self.provided_plan_digest
    }

    pub fn provided_admission_digest(&self) -> &str {
        &self.provided_admission_digest
    }

    pub(in crate::runtime) fn new(
        admitted_read_graph_digest: impl Into<String>,
        execution_read_graph_digest: impl Into<String>,
        provided_plan_digest: impl Into<String>,
        provided_admission_digest: impl Into<String>,
    ) -> Self {
        Self {
            admitted_read_graph_digest: admitted_read_graph_digest.into(),
            execution_read_graph_digest: execution_read_graph_digest.into(),
            provided_plan_digest: provided_plan_digest.into(),
            provided_admission_digest: provided_admission_digest.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadDenial {
    kind: ForgeQueryReadDenialKind,
    message: String,
    built_in_operator_denial: Option<ForgeQueryReadBuiltInOperatorDenial>,
    relationship_proof_denial: Option<ForgeQueryReadRelationshipProofDenial>,
    scope_shape_mismatch: Option<ForgeQueryReadScopeShapeMismatch>,
    access_plan_binding_mismatch: Option<ForgeQueryReadAccessPlanBindingMismatch>,
    graph_read_access_admission: Option<ForgeQueryGraphReadAccessAdmission>,
    graph_read_access_execution_counters: Option<ForgeQueryGraphReadAccessExecutionCounters>,
    graph_read_persistent_artifact_audit: Option<ForgeQueryGraphReadPersistentArtifactAudit>,
}

impl ForgeQueryReadDenial {
    pub fn kind(&self) -> &ForgeQueryReadDenialKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn built_in_operator_denial(&self) -> Option<&ForgeQueryReadBuiltInOperatorDenial> {
        self.built_in_operator_denial.as_ref()
    }

    pub fn relationship_proof_denial(&self) -> Option<&ForgeQueryReadRelationshipProofDenial> {
        self.relationship_proof_denial.as_ref()
    }

    pub fn scope_shape_mismatch(&self) -> Option<&ForgeQueryReadScopeShapeMismatch> {
        self.scope_shape_mismatch.as_ref()
    }

    pub fn access_plan_binding_mismatch(&self) -> Option<&ForgeQueryReadAccessPlanBindingMismatch> {
        self.access_plan_binding_mismatch.as_ref()
    }

    pub fn graph_read_access_admission(&self) -> Option<&ForgeQueryGraphReadAccessAdmission> {
        self.graph_read_access_admission.as_ref()
    }

    pub fn graph_read_access_execution_counters(
        &self,
    ) -> Option<&ForgeQueryGraphReadAccessExecutionCounters> {
        self.graph_read_access_execution_counters.as_ref()
    }

    pub fn graph_read_persistent_artifact_audit(
        &self,
    ) -> Option<&ForgeQueryGraphReadPersistentArtifactAudit> {
        self.graph_read_persistent_artifact_audit.as_ref()
    }

    pub(crate) fn new(kind: ForgeQueryReadDenialKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            built_in_operator_denial: None,
            relationship_proof_denial: None,
            scope_shape_mismatch: None,
            access_plan_binding_mismatch: None,
            graph_read_access_admission: None,
            graph_read_access_execution_counters: None,
            graph_read_persistent_artifact_audit: None,
        }
    }

    pub(crate) fn with_access_plan_binding_mismatch(
        mut self,
        access_plan_binding_mismatch: ForgeQueryReadAccessPlanBindingMismatch,
    ) -> Self {
        self.access_plan_binding_mismatch = Some(access_plan_binding_mismatch);
        self
    }

    pub(crate) fn with_graph_read_access_admission(
        mut self,
        graph_read_access_admission: ForgeQueryGraphReadAccessAdmission,
    ) -> Self {
        self.graph_read_access_admission = Some(graph_read_access_admission);
        self
    }

    pub(crate) fn with_graph_read_access_execution_counters(
        mut self,
        graph_read_access_execution_counters: ForgeQueryGraphReadAccessExecutionCounters,
    ) -> Self {
        self.graph_read_access_execution_counters = Some(graph_read_access_execution_counters);
        self
    }

    pub(crate) fn with_graph_read_persistent_artifact_audit(
        mut self,
        graph_read_persistent_artifact_audit: ForgeQueryGraphReadPersistentArtifactAudit,
    ) -> Self {
        self.graph_read_persistent_artifact_audit = Some(graph_read_persistent_artifact_audit);
        self
    }

    pub(crate) fn with_graph_read_persistent_artifact_audit_for_admission(
        self,
        admission: &ForgeQueryGraphReadAccessAdmission,
    ) -> Self {
        if admission.persistent_index_requirement().is_some() {
            return self.with_graph_read_persistent_artifact_audit(
                ForgeQueryGraphReadPersistentArtifactAudit::declaration_only_stop(),
            );
        }
        self
    }

    pub(in crate::runtime) fn new_built_in_operator_denied(
        operator: ForgeQueryReadBuiltInOperator,
        reason: ForgeQueryReadBuiltInOperatorDenialReason,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: ForgeQueryReadDenialKind::BuiltInOperatorDenied,
            message: message.into(),
            built_in_operator_denial: Some(ForgeQueryReadBuiltInOperatorDenial::new(
                operator, reason,
            )),
            relationship_proof_denial: None,
            scope_shape_mismatch: None,
            access_plan_binding_mismatch: None,
            graph_read_access_admission: None,
            graph_read_access_execution_counters: None,
            graph_read_persistent_artifact_audit: None,
        }
    }

    pub(in crate::runtime) fn new_relationship_proof_admission_denied(
        denial: ForgeQueryReadRelationshipProofDenial,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: ForgeQueryReadDenialKind::RelationshipProofAdmissionDenied,
            message: message.into(),
            built_in_operator_denial: None,
            relationship_proof_denial: Some(denial),
            scope_shape_mismatch: None,
            access_plan_binding_mismatch: None,
            graph_read_access_admission: None,
            graph_read_access_execution_counters: None,
            graph_read_persistent_artifact_audit: None,
        }
    }

    pub(in crate::runtime) fn new_scope_shape_denied(
        expected: ForgeQueryReadScopeClass,
        actual: ForgeQueryReadScopeClass,
    ) -> Self {
        let message = format!(
            "requested `{}` read shape lowers to `{}` under current kernel classification",
            expected.as_str(),
            actual.as_str()
        );
        Self {
            kind: ForgeQueryReadDenialKind::ScopeShapeDenied,
            message,
            built_in_operator_denial: None,
            relationship_proof_denial: None,
            scope_shape_mismatch: Some(ForgeQueryReadScopeShapeMismatch { expected, actual }),
            access_plan_binding_mismatch: None,
            graph_read_access_admission: None,
            graph_read_access_execution_counters: None,
            graph_read_persistent_artifact_audit: None,
        }
    }
}

impl std::fmt::Display for ForgeQueryReadDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "read composition {}: {}",
            self.kind.as_str(),
            self.message
        )
    }
}

impl std::error::Error for ForgeQueryReadDenial {}
