//! Operation-specific next-action contracts derived from published posture.
//!
//! Irreversible outcomes expose no undo surface at the type level (R8.21).

use super::published_posture::PublishedAftermathPosture;

/// Next actions installed for `Reversible` posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReversibleNextActionContract {
    _private: (),
}

impl ReversibleNextActionContract {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    /// Legal next action marker for a recorded inverse.
    pub const fn undo_via_recorded_inverse(self) -> UndoViaRecordedInverse {
        UndoViaRecordedInverse { _private: () }
    }
}

/// Proof that undo-via-recorded-inverse is installed for this outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UndoViaRecordedInverse {
    _private: (),
}

/// Next actions installed for `Compensatable` posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompensatableNextActionContract {
    _private: (),
}

impl CompensatableNextActionContract {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    /// Legal next action marker for compensation.
    pub const fn compensate(self) -> CompensateNextAction {
        CompensateNextAction { _private: () }
    }
}

/// Proof that compensate is installed for this outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompensateNextAction {
    _private: (),
}

/// Next actions installed for `Reconcilable` posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReconcilableNextActionContract {
    _private: (),
}

impl ReconcilableNextActionContract {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }

    /// Legal next action marker for reconciliation.
    pub const fn reconcile(self) -> ReconcileNextAction {
        ReconcileNextAction { _private: () }
    }
}

/// Proof that reconcile is installed for this outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReconcileNextAction {
    _private: (),
}

/// Next actions installed for `Irreversible` posture.
///
/// This type intentionally exposes no `undo`, `compensate`, or `reconcile`
/// method. Calling those is a type error, not a runtime denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IrreversibleNextActionContract {
    _private: (),
}

impl IrreversibleNextActionContract {
    pub(crate) const fn new() -> Self {
        Self { _private: () }
    }
}

/// Installed next-action contract discriminated by published posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstalledAftermathNextActionContract {
    Reversible(ReversibleNextActionContract),
    Compensatable(CompensatableNextActionContract),
    Reconcilable(ReconcilableNextActionContract),
    Irreversible(IrreversibleNextActionContract),
}

impl InstalledAftermathNextActionContract {
    pub(crate) const fn for_posture(posture: PublishedAftermathPosture) -> Self {
        match posture {
            PublishedAftermathPosture::Reversible => {
                Self::Reversible(ReversibleNextActionContract::new())
            }
            PublishedAftermathPosture::Compensatable => {
                Self::Compensatable(CompensatableNextActionContract::new())
            }
            PublishedAftermathPosture::Reconcilable => {
                Self::Reconcilable(ReconcilableNextActionContract::new())
            }
            PublishedAftermathPosture::Irreversible => {
                Self::Irreversible(IrreversibleNextActionContract::new())
            }
        }
    }

    pub const fn posture(self) -> PublishedAftermathPosture {
        match self {
            Self::Reversible(_) => PublishedAftermathPosture::Reversible,
            Self::Compensatable(_) => PublishedAftermathPosture::Compensatable,
            Self::Reconcilable(_) => PublishedAftermathPosture::Reconcilable,
            Self::Irreversible(_) => PublishedAftermathPosture::Irreversible,
        }
    }
}
