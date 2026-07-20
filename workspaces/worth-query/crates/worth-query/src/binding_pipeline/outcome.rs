#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingAmbiguity {
    reason: String,
    candidate_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingUnavailable {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingWrongWorld {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingWrongHandle {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingStale {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingRebindRequired {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingMissingRequiredAspect {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingAspectConflict {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingAuthorityMismatch {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingBasisMismatch {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingExplicitNarrowingRequired {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingUnsupported {
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

reason_type!(WorthQueryBindingUnavailable);
reason_type!(WorthQueryBindingWrongWorld);
reason_type!(WorthQueryBindingWrongHandle);
reason_type!(WorthQueryBindingStale);
reason_type!(WorthQueryBindingRebindRequired);
reason_type!(WorthQueryBindingMissingRequiredAspect);
reason_type!(WorthQueryBindingAspectConflict);
reason_type!(WorthQueryBindingAuthorityMismatch);
reason_type!(WorthQueryBindingBasisMismatch);
reason_type!(WorthQueryBindingExplicitNarrowingRequired);
reason_type!(WorthQueryBindingUnsupported);

impl WorthQueryBindingAmbiguity {
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
pub enum WorthQueryBindingOutcome<T> {
    Bound(T),
    Ambiguous(WorthQueryBindingAmbiguity),
    Unavailable(WorthQueryBindingUnavailable),
    WrongWorld(WorthQueryBindingWrongWorld),
    WrongHandle(WorthQueryBindingWrongHandle),
    Stale(WorthQueryBindingStale),
    RebindRequired(WorthQueryBindingRebindRequired),
    MissingRequiredAspect(WorthQueryBindingMissingRequiredAspect),
    AspectConflict(WorthQueryBindingAspectConflict),
    AuthorityMismatch(WorthQueryBindingAuthorityMismatch),
    BasisMismatch(WorthQueryBindingBasisMismatch),
    ExplicitNarrowingRequired(WorthQueryBindingExplicitNarrowingRequired),
    Unsupported(WorthQueryBindingUnsupported),
}

impl<T: Clone> Clone for WorthQueryBindingOutcome<T> {
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

impl<T> WorthQueryBindingOutcome<T> {
    pub fn is_bound(&self) -> bool {
        matches!(self, Self::Bound(_))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingChecked<T> {
    outcome: WorthQueryBindingOutcome<T>,
    binding_digest: String,
    linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
}

impl<T> WorthQueryBindingChecked<T> {
    pub(crate) fn new(
        outcome: WorthQueryBindingOutcome<T>,
        binding_digest: String,
        linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            outcome,
            binding_digest,
            linked_artifacts,
        }
    }

    pub fn outcome(&self) -> &WorthQueryBindingOutcome<T> {
        &self.outcome
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn linked_artifacts(&self) -> &crate::binding_pipeline::WorthQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryBindingOutcome<T>,
        String,
        crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    ) {
        (self.outcome, self.binding_digest, self.linked_artifacts)
    }
}
