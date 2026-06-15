use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthPolicyDecision, WorthUserOutcomeCauseKind,
    WorthUserResponseSource,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanUserResponseClass {
    Admitted,
    Unsupported,
    Blocked,
    Denied,
    PolicyRequired,
    IntegrityMismatch,
    NoOptions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanUserResponseSource {
    class: PlanarBooleanUserResponseClass,
    message: String,
    evidence_digest: String,
    source_identity: String,
}

impl PlanarBooleanUserResponseSource {
    pub fn admitted(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::Admitted,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn unsupported(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::Unsupported,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn blocked(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::Blocked,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn denied(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::Denied,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn policy_required(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::PolicyRequired,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn integrity_mismatch(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::IntegrityMismatch,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn no_options(
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanUserResponseClass::NoOptions,
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn class(&self) -> PlanarBooleanUserResponseClass {
        self.class
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    fn new(
        class: PlanarBooleanUserResponseClass,
        message: impl Into<String>,
        evidence_digest: impl Into<String>,
        source_identity: impl Into<String>,
    ) -> Self {
        Self {
            class,
            message: message.into(),
            evidence_digest: evidence_digest.into(),
            source_identity: source_identity.into(),
        }
    }
}

impl WorthUserResponseSource {
    pub fn from_planar_boolean_outcome(source: &PlanarBooleanUserResponseSource) -> Self {
        if source.class() == PlanarBooleanUserResponseClass::PolicyRequired {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: source.message().to_string(),
                    evidence_digest: source.evidence_digest().to_string(),
                    source_identity: source.source_identity().to_string(),
                    choices: vec![WorthPolicyDecision::pause_for_manual_inspection()],
                },
            };
        }
        if source.class() == PlanarBooleanUserResponseClass::Admitted {
            return Self {
                kind: WorthUserResponseSourceKind::Admitted {
                    message: source.message().to_string(),
                    evidence_digest: source.evidence_digest().to_string(),
                    source_identity: source.source_identity().to_string(),
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(source.class()),
                message: source.message().to_string(),
                evidence_digest: source.evidence_digest().to_string(),
                source_identity: source.source_identity().to_string(),
            },
        }
    }
}

fn cause_kind(class: PlanarBooleanUserResponseClass) -> WorthUserOutcomeCauseKind {
    match class {
        PlanarBooleanUserResponseClass::Unsupported => WorthUserOutcomeCauseKind::UnsupportedInput,
        PlanarBooleanUserResponseClass::Blocked | PlanarBooleanUserResponseClass::NoOptions => {
            WorthUserOutcomeCauseKind::MissingEvidence
        }
        PlanarBooleanUserResponseClass::Denied => WorthUserOutcomeCauseKind::OverlapDenied,
        PlanarBooleanUserResponseClass::PolicyRequired => WorthUserOutcomeCauseKind::PolicyRequired,
        PlanarBooleanUserResponseClass::IntegrityMismatch => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        PlanarBooleanUserResponseClass::Admitted => {
            unreachable!("admitted branches do not route through no-options causes")
        }
    }
}
