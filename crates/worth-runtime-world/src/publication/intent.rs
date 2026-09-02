use crate::branch::ProductBranchCreationIntent;
use worth_relational::facade::mvcc::RelationalTransactionIntent;

/// Which owner components a future operation is allowed to change. Omission
/// is not interpreted as an implicit refresh or a latest-head lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositeComponentIntent {
    RelationalOnly(RelationalTransactionIntent),
    SignalOnly,
    RelationalAndSignal(RelationalTransactionIntent),
}

impl CompositeComponentIntent {
    pub fn relational_only(change: RelationalTransactionIntent) -> Self {
        Self::RelationalOnly(change)
    }

    pub const fn signal_only() -> Self {
        Self::SignalOnly
    }

    pub fn relational_and_signal(change: RelationalTransactionIntent) -> Self {
        Self::RelationalAndSignal(change)
    }

    pub const fn changes_relational(&self) -> bool {
        matches!(self, Self::RelationalOnly(_) | Self::RelationalAndSignal(_))
    }

    pub const fn changes_signal(&self) -> bool {
        matches!(self, Self::SignalOnly | Self::RelationalAndSignal(_))
    }

    pub fn relational_change(&self) -> Option<&RelationalTransactionIntent> {
        match self {
            Self::RelationalOnly(change) | Self::RelationalAndSignal(change) => Some(change),
            Self::SignalOnly => None,
        }
    }
}

/// The only synchronous Signal execution borrow accepted by the future
/// publication port. It cannot be cloned or stored as a `'static` callback.
pub enum CompositeExecutionBorrow<'a, Ctx, F> {
    WithoutSignal,
    Signal { context: &'a mut Ctx, mutation: F },
}

impl<'a, Ctx, F> CompositeExecutionBorrow<'a, Ctx, F> {
    pub fn without_signal() -> Self {
        Self::WithoutSignal
    }

    pub fn signal(context: &'a mut Ctx, mutation: F) -> Self {
        Self::Signal { context, mutation }
    }
}

/// First compiler-visible publication phase. Construction is owner-internal;
/// callers submit a validated branch-creation meaning instead.
#[derive(Debug)]
pub struct ProductBranchIntent {
    creation: ProductBranchCreationIntent,
    component_postures: crate::branch::ProductBranchComponentPostures,
    component_intent: CompositeComponentIntent,
}

impl ProductBranchIntent {
    pub fn new(
        creation: ProductBranchCreationIntent,
        component_postures: crate::branch::ProductBranchComponentPostures,
        component_intent: CompositeComponentIntent,
    ) -> Self {
        Self {
            creation,
            component_postures,
            component_intent,
        }
    }

    pub fn creation(&self) -> &ProductBranchCreationIntent {
        &self.creation
    }

    pub fn component_intent(&self) -> CompositeComponentIntent {
        self.component_intent.clone()
    }

    pub const fn component_postures(&self) -> crate::branch::ProductBranchComponentPostures {
        self.component_postures
    }
}
