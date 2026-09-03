use crate::branch::ProductBranchCreationIntent;
use worth_relational::facade::branch::AdmittedRelationalForkSourceBasis;
use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;
use worth_relational::facade::mvcc::RelationalTransactionIntent;
use worth_signal::facade::branch::SignalOwnerCancellationToken;
use worth_signal::facade::branch::ValidatedSignalBranchName;
use worth_signal::facade::{SignalError, SignalTransaction};

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

/// The only mutation callback accepted by the Runtime World Signal seam. The
/// callback is bound to the actual Signal transaction type and can live only
/// for the synchronous owner call.
pub type SignalTransactionMutation<'a, D, I, E, Ctx, T> = Box<
    dyn for<'tx> FnOnce(&mut SignalTransaction<'tx, D, I, E, Ctx, T>) -> Result<(), SignalError>
        + 'a,
>;

/// The only synchronous Signal execution borrow accepted by the publication
/// port. It cannot be cloned, erased to `()`, or stored as a `'static`
/// callback.
pub enum CompositeExecutionBorrow<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    WithoutSignal,
    Signal {
        context: &'a mut Ctx,
        mutation: SignalTransactionMutation<'a, D, I, E, Ctx, T>,
        cancellation: &'a SignalOwnerCancellationToken,
    },
}

impl<'a, D, I, E, Ctx, T> CompositeExecutionBorrow<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn without_signal() -> Self {
        Self::WithoutSignal
    }

    pub fn signal<F>(
        context: &'a mut Ctx,
        cancellation: &'a SignalOwnerCancellationToken,
        mutation: F,
    ) -> Self
    where
        F: for<'tx> FnOnce(&mut SignalTransaction<'tx, D, I, E, Ctx, T>) -> Result<(), SignalError>
            + 'a,
    {
        Self::Signal {
            context,
            mutation: Box::new(mutation),
            cancellation,
        }
    }
}

/// First compiler-visible publication phase. Construction is owner-internal;
/// callers submit a validated branch-creation meaning instead.
#[derive(Debug)]
pub struct ProductBranchIntent {
    creation: ProductBranchCreationIntent,
    component_postures: crate::branch::ProductBranchComponentPostures,
    component_intent: CompositeComponentIntent,
    prepared_relational_candidate: Option<PreparedRelationalCommitCandidate>,
    relational_fork_source: Option<AdmittedRelationalForkSourceBasis>,
    signal_fork_name: Option<ValidatedSignalBranchName>,
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
            prepared_relational_candidate: None,
            relational_fork_source: None,
            signal_fork_name: None,
        }
    }

    /// Attach the one owner-issued Relational candidate that corresponds to
    /// this intent. The candidate remains move-only and is consumed by plan
    /// lowering or dropped with the intent on a rejected route.
    pub fn with_prepared_relational_candidate(
        mut self,
        candidate: PreparedRelationalCommitCandidate,
    ) -> Self {
        self.prepared_relational_candidate = Some(candidate);
        self
    }

    /// Attach the one owner-issued Relational fork source for a fork posture.
    /// A descriptive branch id is never accepted in its place.
    pub fn with_relational_fork_source(
        mut self,
        source: AdmittedRelationalForkSourceBasis,
    ) -> Self {
        self.relational_fork_source = Some(source);
        self
    }

    /// Attach the owner-validated Signal destination for a fork route. A
    /// product name is not promoted implicitly into a component identity.
    pub fn with_signal_fork_name(mut self, name: ValidatedSignalBranchName) -> Self {
        self.signal_fork_name = Some(name);
        self
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

    pub(crate) fn take_plan_inputs(
        &mut self,
    ) -> (
        Option<PreparedRelationalCommitCandidate>,
        Option<AdmittedRelationalForkSourceBasis>,
        Option<ValidatedSignalBranchName>,
    ) {
        (
            self.prepared_relational_candidate.take(),
            self.relational_fork_source.take(),
            self.signal_fork_name.take(),
        )
    }
}
