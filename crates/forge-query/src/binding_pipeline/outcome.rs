#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingAmbiguity {
    reason: String,
    candidate_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingUnavailable {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingWrongWorld {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingWrongHandle {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingStale {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingRebindRequired {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingMissingRequiredAspect {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingAspectConflict {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingAuthorityMismatch {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingBasisMismatch {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingExplicitNarrowingRequired {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingUnsupported {
    reason: String,
}

macro_rules! reason_type {
    ($name:ident) => {
        impl $name {
            pub fn new(reason: impl Into<String>) -> Self {
                Self {
                    reason: reason.into(),
                }
            }

            pub fn reason(&self) -> &str {
                &self.reason
            }
        }
    };
}

reason_type!(ForgeQueryBindingUnavailable);
reason_type!(ForgeQueryBindingWrongWorld);
reason_type!(ForgeQueryBindingWrongHandle);
reason_type!(ForgeQueryBindingStale);
reason_type!(ForgeQueryBindingRebindRequired);
reason_type!(ForgeQueryBindingMissingRequiredAspect);
reason_type!(ForgeQueryBindingAspectConflict);
reason_type!(ForgeQueryBindingAuthorityMismatch);
reason_type!(ForgeQueryBindingBasisMismatch);
reason_type!(ForgeQueryBindingExplicitNarrowingRequired);
reason_type!(ForgeQueryBindingUnsupported);

impl ForgeQueryBindingAmbiguity {
    pub fn new(reason: impl Into<String>, candidate_count: usize) -> Self {
        Self {
            reason: reason.into(),
            candidate_count,
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ForgeQueryBindingOutcome<T> {
    Bound(T),
    Ambiguous(ForgeQueryBindingAmbiguity),
    Unavailable(ForgeQueryBindingUnavailable),
    WrongWorld(ForgeQueryBindingWrongWorld),
    WrongHandle(ForgeQueryBindingWrongHandle),
    Stale(ForgeQueryBindingStale),
    RebindRequired(ForgeQueryBindingRebindRequired),
    MissingRequiredAspect(ForgeQueryBindingMissingRequiredAspect),
    AspectConflict(ForgeQueryBindingAspectConflict),
    AuthorityMismatch(ForgeQueryBindingAuthorityMismatch),
    BasisMismatch(ForgeQueryBindingBasisMismatch),
    ExplicitNarrowingRequired(ForgeQueryBindingExplicitNarrowingRequired),
    Unsupported(ForgeQueryBindingUnsupported),
}

impl<T: Clone> Clone for ForgeQueryBindingOutcome<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Bound(value) => Self::Bound(value.clone()),
            Self::Ambiguous(value) => Self::Ambiguous(value.clone()),
            Self::Unavailable(value) => Self::Unavailable(value.clone()),
            Self::WrongWorld(value) => Self::WrongWorld(value.clone()),
            Self::WrongHandle(value) => Self::WrongHandle(value.clone()),
            Self::Stale(value) => Self::Stale(value.clone()),
            Self::RebindRequired(value) => Self::RebindRequired(value.clone()),
            Self::MissingRequiredAspect(value) => Self::MissingRequiredAspect(value.clone()),
            Self::AspectConflict(value) => Self::AspectConflict(value.clone()),
            Self::AuthorityMismatch(value) => Self::AuthorityMismatch(value.clone()),
            Self::BasisMismatch(value) => Self::BasisMismatch(value.clone()),
            Self::ExplicitNarrowingRequired(value) => {
                Self::ExplicitNarrowingRequired(value.clone())
            }
            Self::Unsupported(value) => Self::Unsupported(value.clone()),
        }
    }
}

impl<T> ForgeQueryBindingOutcome<T> {
    pub fn is_bound(&self) -> bool {
        matches!(self, Self::Bound(_))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingChecked<T> {
    outcome: ForgeQueryBindingOutcome<T>,
    binding_digest: String,
    linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
}

impl<T> ForgeQueryBindingChecked<T> {
    pub(crate) fn new(
        outcome: ForgeQueryBindingOutcome<T>,
        binding_digest: String,
        linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            binding_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &ForgeQueryBindingOutcome<T> {
        &self.outcome
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryBindingOutcome<T>,
        String,
        crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    ) {
        (self.outcome, self.binding_digest, self.linked_artifacts)
    }
}
