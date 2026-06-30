#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventLedgerLookupExecutionDenialKind {
    SpatialTouchAuthority,
    FamilyCatalog,
    BroadBooleanResidue,
    InputAdmission,
    PlanSelection,
    IndexProduct,
    Execution,
    WitnessMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventLedgerLookupExecutionDenial {
    kind: PlanarBooleanEventLedgerLookupExecutionDenialKind,
    detail: String,
}

impl PlanarBooleanEventLedgerLookupExecutionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEventLedgerLookupExecutionDenialKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEventLedgerLookupExecutionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
