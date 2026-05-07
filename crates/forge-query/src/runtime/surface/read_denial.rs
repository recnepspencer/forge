use super::{
    read_composition::ForgeQueryReadScopeClass, ForgeQueryReadBuiltInOperator,
    ForgeQueryReadBuiltInOperatorDenial, ForgeQueryReadBuiltInOperatorDenialReason,
    ForgeQueryReadRelationshipProofDenial,
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
pub struct ForgeQueryReadDenial {
    kind: ForgeQueryReadDenialKind,
    message: String,
    built_in_operator_denial: Option<ForgeQueryReadBuiltInOperatorDenial>,
    relationship_proof_denial: Option<ForgeQueryReadRelationshipProofDenial>,
    scope_shape_mismatch: Option<ForgeQueryReadScopeShapeMismatch>,
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

    pub(in crate::runtime) fn new(
        kind: ForgeQueryReadDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            built_in_operator_denial: None,
            relationship_proof_denial: None,
            scope_shape_mismatch: None,
        }
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
