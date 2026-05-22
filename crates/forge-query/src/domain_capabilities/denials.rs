use forge_proof::TransitionOutcome;

use super::targets::ForgeQueryDomainCapabilityTargetKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityProgressionDenialKind {
    EmptySemanticCode,
    EmptyDetail,
    UnsupportedCanonicalMaterializationPosture,
    MissingCanonicalMaterializationSemantics,
    InconsistentCanonicalMaterializationSemantics,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityProgressionDenial {
    kind: ForgeQueryDomainCapabilityProgressionDenialKind,
    category: &'static str,
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    request_digest: String,
    message: String,
}

impl ForgeQueryDomainCapabilityProgressionDenial {
    pub(crate) fn new(
        kind: ForgeQueryDomainCapabilityProgressionDenialKind,
        category: &'static str,
        target_kind: ForgeQueryDomainCapabilityTargetKind,
        request_digest: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            category,
            target_kind,
            request_digest: request_digest.into(),
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryDomainCapabilityProgressionDenialKind {
        self.kind
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn target_kind(&self) -> ForgeQueryDomainCapabilityTargetKind {
        self.target_kind
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityProgressionFailure {
    message: String,
}

impl ForgeQueryDomainCapabilityProgressionFailure {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityStale {
    category: &'static str,
    bound_target_digest: String,
    current_target_digest: String,
}

impl ForgeQueryDomainCapabilityStale {
    pub(crate) fn new(
        category: &'static str,
        bound_target_digest: impl Into<String>,
        current_target_digest: impl Into<String>,
    ) -> Self {
        Self {
            category,
            bound_target_digest: bound_target_digest.into(),
            current_target_digest: current_target_digest.into(),
        }
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn bound_target_digest(&self) -> &str {
        &self.bound_target_digest
    }

    pub fn current_target_digest(&self) -> &str {
        &self.current_target_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityRebindRequired {
    category: &'static str,
    bound_target_digest: String,
    current_target_digest: String,
}

impl ForgeQueryDomainCapabilityRebindRequired {
    pub(crate) fn new(
        category: &'static str,
        bound_target_digest: impl Into<String>,
        current_target_digest: impl Into<String>,
    ) -> Self {
        Self {
            category,
            bound_target_digest: bound_target_digest.into(),
            current_target_digest: current_target_digest.into(),
        }
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn bound_target_digest(&self) -> &str {
        &self.bound_target_digest
    }

    pub fn current_target_digest(&self) -> &str {
        &self.current_target_digest
    }
}

pub type ForgeQueryDomainCapabilityTransitionOutcome<S> = TransitionOutcome<
    S,
    ForgeQueryDomainCapabilityProgressionDenial,
    std::convert::Infallible,
    ForgeQueryDomainCapabilityStale,
    ForgeQueryDomainCapabilityRebindRequired,
    ForgeQueryDomainCapabilityProgressionFailure,
>;
