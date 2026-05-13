#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EffectFamily {
    Mutation,
    Merge,
    Writeback,
}

impl EffectFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectAuthorityLane {
    Relational,
    RuntimeBridge,
}

impl EffectAuthorityLane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relational => "relational",
            Self::RuntimeBridge => "runtime_bridge",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectIntentDenialKind {
    WorkflowDeclarationFamilyMismatch,
    WorkflowAuthorityTargetMismatch,
    BasisWorkflowBindingMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeniedEffectEligibilityKind {
    UnsupportedForBasisFamily,
    PreviewRebindRequired,
    DeferredToLaterMilestone,
    WorkflowAdmissionDenied,
}

impl DeniedEffectEligibilityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedForBasisFamily => "unsupported_for_basis_family",
            Self::PreviewRebindRequired => "preview_rebind_required",
            Self::DeferredToLaterMilestone => "deferred_to_later_milestone",
            Self::WorkflowAdmissionDenied => "workflow_admission_denied",
        }
    }
}
