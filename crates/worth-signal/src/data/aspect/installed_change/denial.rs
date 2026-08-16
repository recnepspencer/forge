use worth_proof::TransitionOutcome;

use crate::data::error::SignalError;

use super::InstalledSignalScopedChangeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalInstalledScopedChangeDenial {
    EmptyChangeSet,
    ForeignCapability,
    DuplicateTarget,
    MissingOrStaleTarget,
}

pub type SignalInstalledScopedChangeOutcome = TransitionOutcome<
    InstalledSignalScopedChangeSet,
    SignalInstalledScopedChangeDenial,
    std::convert::Infallible,
    std::convert::Infallible,
    std::convert::Infallible,
    SignalError,
>;
