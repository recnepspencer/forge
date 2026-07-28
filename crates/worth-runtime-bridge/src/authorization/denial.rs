#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAuthorizationDenialKind {
    EmptyPolicy,
    MissingAllowPath,
    DuplicateCorrespondence,
    UnknownCorrespondence,
    ObservationShapeMismatch,
    SignalInstallationRejected,
    SignalEvaluationRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationDenial {
    kind: BridgeAuthorizationDenialKind,
    subject: String,
}

impl BridgeAuthorizationDenial {
    pub(crate) fn new(kind: BridgeAuthorizationDenialKind, subject: impl Into<String>) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> BridgeAuthorizationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for BridgeAuthorizationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "authorization correspondence denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for BridgeAuthorizationDenial {}
