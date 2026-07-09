use crate::storage::overlay::{
    BorrowedWorkingState, OverlayStateView, PartitionAccess, WorkingState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InvariantObservationKind {
    Committed,
    Speculative,
}

impl InvariantObservationKind {
    pub const fn diagnostic_label(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::Speculative => "speculative",
        }
    }
}

#[derive(Clone)]
pub(crate) struct CommittedInvariantView<'runtime> {
    state: BorrowedWorkingState<'runtime>,
}

impl<'runtime> CommittedInvariantView<'runtime> {
    pub(crate) fn new(state: BorrowedWorkingState<'runtime>) -> Self {
        Self { state }
    }

    pub(crate) fn partition_access(&self) -> &dyn PartitionAccess {
        &self.state
    }
}

#[derive(Clone)]
pub(crate) struct SpeculativeInvariantView<'runtime> {
    state: OverlayStateView<'runtime, WorkingState>,
}

impl<'runtime> SpeculativeInvariantView<'runtime> {
    pub(crate) fn new(state: OverlayStateView<'runtime, WorkingState>) -> Self {
        Self { state }
    }

    pub(crate) fn partition_access(&self) -> &dyn PartitionAccess {
        &self.state
    }
}

#[derive(Clone)]
pub(crate) enum InvariantObservation<'runtime> {
    Committed(CommittedInvariantView<'runtime>),
    Speculative(SpeculativeInvariantView<'runtime>),
}

impl<'runtime> InvariantObservation<'runtime> {
    pub(crate) fn committed(state: BorrowedWorkingState<'runtime>) -> Self {
        Self::Committed(CommittedInvariantView::new(state))
    }

    pub(crate) fn speculative(state: OverlayStateView<'runtime, WorkingState>) -> Self {
        Self::Speculative(SpeculativeInvariantView::new(state))
    }

    pub(crate) fn kind(&self) -> InvariantObservationKind {
        match self {
            Self::Committed(_) => InvariantObservationKind::Committed,
            Self::Speculative(_) => InvariantObservationKind::Speculative,
        }
    }

    pub(crate) fn partition_access(&self) -> &dyn PartitionAccess {
        match self {
            Self::Committed(view) => view.partition_access(),
            Self::Speculative(view) => view.partition_access(),
        }
    }
}
