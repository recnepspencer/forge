use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use super::{
    UiCollectionProjectionWorkCounters, UiNativeTextValue, UiProjectionAvailability,
    UiProjectionFactReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionCompleteness {
    Complete,
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionProjectionDelivery {
    Snapshot,
    Patch,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionContinuation {
    query_continuation_identity: WorthQueryEvidenceIdentity,
}

impl UiCollectionContinuation {
    pub(crate) fn query_issued(query_continuation_identity: WorthQueryEvidenceIdentity) -> Self {
        Self {
            query_continuation_identity,
        }
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_continuation_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionRowReference {
    query_row_identity: WorthQueryEvidenceIdentity,
}

impl UiCollectionProjectionRowReference {
    pub(crate) fn query_issued(query_row_identity: WorthQueryEvidenceIdentity) -> Self {
        Self { query_row_identity }
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_row_identity
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionTextRow {
    row: UiCollectionProjectionRowReference,
    selected_values: Box<[UiNativeTextValue]>,
}

impl UiCollectionProjectionTextRow {
    pub(crate) fn admitted(
        row: UiCollectionProjectionRowReference,
        selected_values: impl Into<Box<[UiNativeTextValue]>>,
    ) -> Self {
        Self {
            row,
            selected_values: selected_values.into(),
        }
    }

    pub fn row(&self) -> &UiCollectionProjectionRowReference {
        &self.row
    }

    pub fn selected_values(&self) -> &[UiNativeTextValue] {
        &self.selected_values
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionValue {
    rows: Box<[UiCollectionProjectionTextRow]>,
    completeness: UiCollectionCompleteness,
    continuation: Option<UiCollectionContinuation>,
}

impl UiCollectionProjectionValue {
    pub(crate) fn admitted(
        rows: impl Into<Box<[UiCollectionProjectionTextRow]>>,
        completeness: UiCollectionCompleteness,
        continuation: Option<UiCollectionContinuation>,
    ) -> Self {
        Self {
            rows: rows.into(),
            completeness,
            continuation,
        }
    }

    pub fn rows(&self) -> &[UiCollectionProjectionTextRow] {
        &self.rows
    }

    pub fn completeness(&self) -> UiCollectionCompleteness {
        self.completeness
    }

    pub fn continuation(&self) -> Option<&UiCollectionContinuation> {
        self.continuation.as_ref()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum UiCollectionProjectionChange {
    Insert {
        row: UiCollectionProjectionRowReference,
        at: usize,
    },
    Remove {
        row: UiCollectionProjectionRowReference,
        from: usize,
    },
    Move {
        row: UiCollectionProjectionRowReference,
        from: usize,
        to: usize,
    },
    Update {
        row: UiCollectionProjectionRowReference,
    },
    WindowShift,
    ResetRequired {
        reason: crate::WorthUiCollectionResetReason,
    },
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionFactReceipt {
    core: UiProjectionFactReceipt,
    delivery: UiCollectionProjectionDelivery,
    availability: UiProjectionAvailability<UiCollectionProjectionValue>,
    work: UiCollectionProjectionWorkCounters,
    changes: Box<[UiCollectionProjectionChange]>,
}

impl UiCollectionProjectionFactReceipt {
    pub(crate) fn admitted(
        core: UiProjectionFactReceipt,
        delivery: UiCollectionProjectionDelivery,
        availability: UiProjectionAvailability<UiCollectionProjectionValue>,
        work: UiCollectionProjectionWorkCounters,
        changes: impl Into<Box<[UiCollectionProjectionChange]>>,
    ) -> Self {
        Self {
            core,
            delivery,
            availability,
            work,
            changes: changes.into(),
        }
    }

    pub fn core(&self) -> &UiProjectionFactReceipt {
        &self.core
    }

    pub const fn delivery(&self) -> UiCollectionProjectionDelivery {
        self.delivery
    }

    pub fn availability(&self) -> &UiProjectionAvailability<UiCollectionProjectionValue> {
        &self.availability
    }

    pub fn work(&self) -> UiCollectionProjectionWorkCounters {
        self.work
    }

    pub fn changes(&self) -> &[UiCollectionProjectionChange] {
        &self.changes
    }

    pub fn into_observation(self) -> crate::UiCollectionProjectionObservation {
        crate::UiCollectionProjectionObservation::query_issued(self)
    }
}
