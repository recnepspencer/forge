use super::{WorthPolicyDecision, WorthUserOutcomeCauseKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUserResponseSource {
    pub(super) kind: WorthUserResponseSourceKind,
}

impl WorthUserResponseSource {
    pub(crate) fn kind(&self) -> &WorthUserResponseSourceKind {
        &self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUserResponseSourceKind {
    Admitted {
        message: String,
        evidence_digest: String,
        source_identity: String,
    },
    PolicyRequired {
        message: String,
        evidence_digest: String,
        source_identity: String,
        choices: Vec<WorthPolicyDecision>,
    },
    NoOptions {
        cause_kind: WorthUserOutcomeCauseKind,
        message: String,
        evidence_digest: String,
        source_identity: String,
    },
}
