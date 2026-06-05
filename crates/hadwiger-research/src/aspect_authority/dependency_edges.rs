use super::aspect_kinds::{
    HadwigerAspectAuthorityError, HadwigerAspectKind, HadwigerAspectPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerAspectDependencyRole {
    MathematicalRequirement,
    StructuralRequirement,
    AdvisoryContext,
    FailureContext,
}

impl HadwigerAspectDependencyRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MathematicalRequirement => "mathematical_requirement",
            Self::StructuralRequirement => "structural_requirement",
            Self::AdvisoryContext => "advisory_context",
            Self::FailureContext => "failure_context",
        }
    }

    pub(crate) fn requires_admitted_posture(self) -> bool {
        matches!(
            self,
            Self::MathematicalRequirement | Self::StructuralRequirement
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerAspectInvalidationScope {
    ExactAspect,
    DependencyClosure,
    ConservativeEscalation,
    UnsafeLocalScope,
    Unknown,
}

impl HadwigerAspectInvalidationScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactAspect => "exact_aspect",
            Self::DependencyClosure => "dependency_closure",
            Self::ConservativeEscalation => "conservative_escalation",
            Self::UnsafeLocalScope => "unsafe_local_scope",
            Self::Unknown => "unknown",
        }
    }

    pub(crate) fn requires_conservative_invalidation(self) -> bool {
        matches!(
            self,
            Self::ConservativeEscalation | Self::UnsafeLocalScope | Self::Unknown
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HadwigerAspectDependencyEdge {
    required_by: HadwigerAspectKind,
    required_aspect: HadwigerAspectKind,
    dependency_role: HadwigerAspectDependencyRole,
    required_posture: HadwigerAspectPosture,
    invalidation_scope: HadwigerAspectInvalidationScope,
}

impl HadwigerAspectDependencyEdge {
    pub fn new(
        required_by: HadwigerAspectKind,
        required_aspect: HadwigerAspectKind,
        dependency_role: HadwigerAspectDependencyRole,
        invalidation_scope: HadwigerAspectInvalidationScope,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        if required_by == required_aspect {
            return Err(HadwigerAspectAuthorityError::SelfDependency {
                aspect_kind: required_by,
            });
        }
        Ok(Self {
            required_by,
            required_aspect,
            dependency_role,
            required_posture: HadwigerAspectPosture::Admitted,
            invalidation_scope,
        })
    }

    pub fn required_by(&self) -> HadwigerAspectKind {
        self.required_by
    }

    pub fn required_aspect(&self) -> HadwigerAspectKind {
        self.required_aspect
    }

    pub fn dependency_role(&self) -> HadwigerAspectDependencyRole {
        self.dependency_role
    }

    pub fn required_posture(&self) -> HadwigerAspectPosture {
        self.required_posture
    }

    pub fn invalidation_scope(&self) -> HadwigerAspectInvalidationScope {
        self.invalidation_scope
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.required_by.as_str(),
            self.required_aspect.as_str(),
            self.dependency_role.as_str(),
            self.required_posture.as_str(),
            self.invalidation_scope.as_str()
        )
    }

    pub(crate) fn is_satisfied_by(&self, posture: HadwigerAspectPosture) -> bool {
        if self.dependency_role.requires_admitted_posture() {
            posture == self.required_posture
        } else {
            matches!(
                posture,
                HadwigerAspectPosture::Admitted | HadwigerAspectPosture::Advisory
            )
        }
    }
}
