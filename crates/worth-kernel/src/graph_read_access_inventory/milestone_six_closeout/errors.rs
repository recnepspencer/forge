use super::super::inventory_lane::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::phase_six_closeout::{
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessMilestoneSixErrorKind {
    InventoryCloseoutFailed,
    DispositionCloseoutFailed,
    InventoryDispositionCountMismatch,
    DeclarationCandidateCountMismatch,
    CapabilityGapCountMismatch,
    DeletionItemCountMismatch,
    CertificationOnlyCountMismatch,
    OutOfScopeCountMismatch,
    DeletedSourceStillExists,
    OldGraphReadFolkloreInMilestoneSevenSeed,
    LaterMilestoneClaimed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessMilestoneSixError {
    kind: WorthGraphReadAccessMilestoneSixErrorKind,
    inventory_error_kind: Option<WorthGraphReadAccessInventoryErrorKind>,
    disposition_error_kind: Option<WorthGraphReadAccessPhaseSixErrorKind>,
    inventory_error_message: Option<String>,
}

impl WorthGraphReadAccessMilestoneSixError {
    pub(crate) const fn new(kind: WorthGraphReadAccessMilestoneSixErrorKind) -> Self {
        Self {
            kind,
            inventory_error_kind: None,
            disposition_error_kind: None,
            inventory_error_message: None,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessMilestoneSixErrorKind {
        self.kind
    }

    pub const fn inventory_error_kind(&self) -> Option<WorthGraphReadAccessInventoryErrorKind> {
        self.inventory_error_kind
    }

    pub const fn disposition_error_kind(&self) -> Option<WorthGraphReadAccessPhaseSixErrorKind> {
        self.disposition_error_kind
    }

    pub fn inventory_error_message(&self) -> Option<&str> {
        self.inventory_error_message.as_deref()
    }
}

impl From<WorthGraphReadAccessInventoryError> for WorthGraphReadAccessMilestoneSixError {
    fn from(error: WorthGraphReadAccessInventoryError) -> Self {
        Self {
            kind: WorthGraphReadAccessMilestoneSixErrorKind::InventoryCloseoutFailed,
            inventory_error_kind: Some(error.kind()),
            disposition_error_kind: None,
            inventory_error_message: error.message().map(str::to_owned),
        }
    }
}

impl From<WorthGraphReadAccessPhaseSixError> for WorthGraphReadAccessMilestoneSixError {
    fn from(error: WorthGraphReadAccessPhaseSixError) -> Self {
        Self {
            kind: WorthGraphReadAccessMilestoneSixErrorKind::DispositionCloseoutFailed,
            inventory_error_kind: None,
            disposition_error_kind: Some(error.kind()),
            inventory_error_message: None,
        }
    }
}
