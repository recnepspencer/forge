use forge_proof::TransitionOutcome;

use super::targets::ForgeQueryDomainCapabilityTargetKind;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
    request_identity: ForgeQueryEvidenceIdentity,
    message: String,
    failure_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryDomainCapabilityProgressionDenial {
    pub(crate) fn new(
        kind: ForgeQueryDomainCapabilityProgressionDenialKind,
        category: &'static str,
        target_kind: ForgeQueryDomainCapabilityTargetKind,
        request_identity: ForgeQueryEvidenceIdentity,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let failure_identity = compose_progression_denial_failure_identity(
            kind,
            category,
            target_kind,
            &request_identity,
            &message,
        );
        Self {
            kind,
            category,
            target_kind,
            request_identity,
            message,
            failure_identity,
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

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_for_reporting(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn failure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.failure_identity
    }

    pub fn failure_for_reporting(&self) -> &str {
        self.failure_identity.as_str()
    }
}

fn compose_progression_denial_failure_identity(
    kind: ForgeQueryDomainCapabilityProgressionDenialKind,
    category: &'static str,
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    request_identity: &ForgeQueryEvidenceIdentity,
    message: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_progression_denial_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), format!("{kind:?}"))
        .field_shape(ForgeQueryEvidenceTag::new("category"), category)
        .field_shape(ForgeQueryEvidenceTag::new("target_kind"), target_kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("request"), request_identity)
        .field_value(ForgeQueryEvidenceTag::new("message"), message)
        .seal()
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
    bound_target_identity: ForgeQueryEvidenceIdentity,
    current_target_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryDomainCapabilityStale {
    pub(crate) fn new(
        category: &'static str,
        bound_target_identity: ForgeQueryEvidenceIdentity,
        current_target_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            category,
            bound_target_identity,
            current_target_identity,
        }
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn bound_target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bound_target_identity
    }

    pub fn current_target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.current_target_identity
    }

    pub fn bound_target_for_reporting(&self) -> &str {
        self.bound_target_identity.as_str()
    }

    pub fn current_target_for_reporting(&self) -> &str {
        self.current_target_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityRebindRequired {
    category: &'static str,
    bound_target_identity: ForgeQueryEvidenceIdentity,
    current_target_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryDomainCapabilityRebindRequired {
    pub(crate) fn new(
        category: &'static str,
        bound_target_identity: ForgeQueryEvidenceIdentity,
        current_target_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            category,
            bound_target_identity,
            current_target_identity,
        }
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn bound_target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bound_target_identity
    }

    pub fn current_target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.current_target_identity
    }

    pub fn bound_target_for_reporting(&self) -> &str {
        self.bound_target_identity.as_str()
    }

    pub fn current_target_for_reporting(&self) -> &str {
        self.current_target_identity.as_str()
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
