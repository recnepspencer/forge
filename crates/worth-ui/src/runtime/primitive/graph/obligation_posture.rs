#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveConstructionObligationKind {
    SchemaContract,
    CapabilitySupport,
    OperatingContext,
    DependencyContract,
    QuerySupport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveConstructionObligationPosture {
    Selected,
    NotApplicable,
    Unsupported,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveConstructionObligationRow {
    kind: WorthUiPrimitiveConstructionObligationKind,
    posture: WorthUiPrimitiveConstructionObligationPosture,
    evidence: String,
}

impl WorthUiPrimitiveConstructionObligationRow {
    pub(crate) fn new(
        kind: WorthUiPrimitiveConstructionObligationKind,
        posture: WorthUiPrimitiveConstructionObligationPosture,
        evidence: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            posture,
            evidence: evidence.into(),
        }
    }

    pub fn kind(&self) -> WorthUiPrimitiveConstructionObligationKind {
        self.kind
    }

    pub fn posture(&self) -> WorthUiPrimitiveConstructionObligationPosture {
        self.posture
    }

    pub fn evidence(&self) -> &str {
        &self.evidence
    }
}
