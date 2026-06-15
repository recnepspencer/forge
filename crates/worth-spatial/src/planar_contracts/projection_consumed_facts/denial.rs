#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumedPlanarFactsDenialKind {
    MissingRetainedPlanarFactsReceipt,
    InvalidMaterializationBasis,
    MissingProjectionReceipts,
    DuplicateProjectionReceipt,
    MismatchedProjectionClosure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsDenial {
    kind: ProjectionConsumedPlanarFactsDenialKind,
    reason: String,
}

impl ProjectionConsumedPlanarFactsDenial {
    pub(crate) fn new(
        kind: ProjectionConsumedPlanarFactsDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> ProjectionConsumedPlanarFactsDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
